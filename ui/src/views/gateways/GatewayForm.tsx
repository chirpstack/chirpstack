import React, { useState, useEffect } from "react";

import { Form, Input, InputNumber, Row, Col, Button, Tabs, Space, Card } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import { Location } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { Gateway } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import { onFinishFailed } from "../helpers";
import EuiInput from "../../components/EuiInput";
import Map, { Marker } from "../../components/Map";
import LocationStore from "../../stores/LocationStore";

interface IProps {
  initialValues: Gateway;
  onFinish: (obj: Gateway) => void;
  update?: boolean;
  disabled?: boolean;
}

function GatewayForm(props: IProps) {
  const [form] = Form.useForm();
  const [latValue, setLatValue] = useState<number>(0);
  const [lonValue, setLonValue] = useState<number>(0);
  const [locationPending, setLocationPending] = useState<boolean>(false);

  useEffect(() => {
    if (!props.update) {
      getCurrentLocation();
    } else {
      const loc = props.initialValues.getLocation();
      if (loc) {
        setLatValue(loc.getLatitude());
        setLonValue(loc.getLongitude());
      }
    }
  }, [props]);

  const getCurrentLocation = () => {
    setLocationPending(true);

    LocationStore.getLocation((loc: [number, number]) => {
      setLatValue(loc[0]);
      setLonValue(loc[1]);
      setLocationPending(false);
      setLocationFields(loc[0], loc[1]);
    });
  };

  const onFinish = (values: Gateway.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let gw = new Gateway();
    let loc = new Location();

    if (v.location) {
      loc.setLatitude(v.location.latitude);
      loc.setLongitude(v.location.longitude);
    }

    gw.setTenantId(v.tenantId);
    gw.setName(v.name);
    gw.setDescription(v.description);
    gw.setGatewayId(v.gatewayId);
    gw.setStatsInterval(v.statsInterval);
    gw.setLocation(loc);

    // tags
    for (const elm of v.tagsMap) {
      gw.getTagsMap().set(elm[0], elm[1]);
    }

    props.onFinish(gw);
  };

  const updateLocation = (e: any) => {
    const loc = e.target.getLatLng();
    setLatValue(loc.lat);
    setLonValue(loc.lng);
    setLocationFields(loc.lat, loc.lng);
  };

  const setLocationFields = (lat: number, lon: number) => {
    form.setFieldsValue({
      location: {
        latitude: lat,
        longitude: lon,
      },
    });
  };

  const location: [number, number] = [latValue, lonValue];

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed} form={form}>
      <Tabs>
        <Tabs.TabPane tab="General" key="1">
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input disabled={props.disabled} />
          </Form.Item>
          <Form.Item label="Description" name="description">
            <Input.TextArea disabled={props.disabled} />
          </Form.Item>
          <Row gutter={24}>
            <Col span={12}>
              <EuiInput
                label="Gateway ID (EUI64)"
                name="gatewayId"
                value={props.initialValues.getGatewayId()}
                disabled={props.update || props.disabled}
                required
              />
            </Col>
            <Col span={12}>
              <Form.Item
                label="Stats interval (secs)"
                tooltip="The expected interval in seconds in which the gateway sends its statistics"
                name="statsInterval"
                rules={[{ required: true, message: "Please enter a stats interval!" }]}
              >
                <InputNumber min={0} disabled={props.disabled} />
              </Form.Item>
            </Col>
          </Row>
          <Form.Item label="Location">
            <Form.Item name={["location", "latitude"]} noStyle>
              <Input hidden />
            </Form.Item>
            <Form.Item name={["location", "longitude"]} noStyle>
              <Input hidden />
            </Form.Item>
            <Space direction="vertical" style={{ width: "100%" }}>
              <Map height={500} center={location}>
                <Marker
                  position={location}
                  faIcon="wifi"
                  color="blue"
                  draggable={!props.disabled}
                  eventHandlers={{ dragend: updateLocation }}
                />
              </Map>
              <Button onClick={getCurrentLocation} disabled={locationPending} type="link" style={{ float: "right" }}>
                Set to current location
              </Button>
            </Space>
          </Form.Item>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Tags" key="2">
          <Form.List name="tagsMap">
            {(fields, { add, remove }) => (
              <>
                {fields.map(({ key, name, ...restField }) => (
                  <Row gutter={24}>
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 0]}
                        fieldKey={[name, 0]}
                        rules={[{ required: true, message: "Please enter a key!" }]}
                      >
                        <Input placeholder="Key" disabled={props.disabled} />
                      </Form.Item>
                    </Col>
                    <Col span={16}>
                      <Form.Item
                        {...restField}
                        name={[name, 1]}
                        fieldKey={[name, 1]}
                        rules={[{ required: true, message: "Please enter a value!" }]}
                      >
                        <Input placeholder="Value" disabled={props.disabled} />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} disabled={props.disabled} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button type="dashed" disabled={props.disabled} onClick={() => add()} block icon={<PlusOutlined />}>
                    Add tag
                  </Button>
                </Form.Item>
              </>
            )}
          </Form.List>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Metadata" key="3">
          <Card bordered={false}>
            <p>
              Metadata is pushed by the gateway on every stats update and can be used to expose information about the
              gateway like ip / hostname, serial number, HAL version.
            </p>
          </Card>
          <Form.List name="metadataMap">
            {(fields, { add, remove }) => (
              <>
                {fields.map(({ key, name, ...restField }) => (
                  <Row gutter={24}>
                    <Col span={6}>
                      <Form.Item {...restField} name={[name, 0]} fieldKey={[name, 0]}>
                        <Input placeholder="Key" disabled />
                      </Form.Item>
                    </Col>
                    <Col span={18}>
                      <Form.Item {...restField} name={[name, 1]} fieldKey={[name, 1]}>
                        <Input placeholder="Value" disabled />
                      </Form.Item>
                    </Col>
                  </Row>
                ))}
              </>
            )}
          </Form.List>
        </Tabs.TabPane>
      </Tabs>
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default GatewayForm;
