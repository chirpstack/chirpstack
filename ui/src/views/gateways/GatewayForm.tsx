import React, { Component } from "react";

import { Form, Input, Row, Col, Button, Tabs, Space } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import { Location } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { Gateway } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import EuiInput from "../../components/EuiInput";
import Map, { Marker } from "../../components/Map";
import LocationStore from "../../stores/LocationStore";

interface IProps {
  initialValues: Gateway;
  onFinish: (obj: Gateway) => void;
  update?: boolean;
  disabled?: boolean;
}

interface IState {
  latValue: number;
  lonValue: number;
  locationPending: boolean;
}

class GatewayForm extends Component<IProps, IState> {
  formRef = React.createRef<any>();

  constructor(props: IProps) {
    super(props);
    this.state = {
      latValue: 0,
      lonValue: 0,
      locationPending: false,
    };
  }

  componentDidMount() {
    if (!this.props.update) {
      this.getCurrentLocation();
    } else {
      const loc = this.props.initialValues.getLocation();
      if (loc) {
        this.setState({
          latValue: loc.getLatitude(),
          lonValue: loc.getLongitude(),
        });
      }
    }
  }

  getCurrentLocation = () => {
    this.setState({
      locationPending: true,
    });

    LocationStore.getLocation((loc: [number, number]) => {
      this.setState(
        {
          latValue: loc[0],
          lonValue: loc[1],
          locationPending: false,
        },
        this.setLocationFields,
      );
    });
  };

  onFinish = (values: Gateway.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
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
    gw.setLocation(loc);

    // tags
    for (const elm of v.tagsMap) {
      gw.getTagsMap().set(elm[0], elm[1]);
    }

    this.props.onFinish(gw);
  };

  updateLocation = (e: any) => {
    const loc = e.target.getLatLng();

    this.setState(
      {
        latValue: loc.lat,
        lonValue: loc.lng,
      },
      this.setLocationFields,
    );
  };

  setLocationFields = () => {
    this.formRef.current.setFieldsValue({
      location: {
        latitude: this.state.latValue,
        longitude: this.state.lonValue,
      },
    });
  };

  render() {
    const location: [number, number] = [this.state.latValue, this.state.lonValue];

    return (
      <Form
        layout="vertical"
        initialValues={this.props.initialValues.toObject()}
        onFinish={this.onFinish}
        ref={this.formRef}
      >
        <Tabs>
          <Tabs.TabPane tab="General" key="1">
            <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
              <Input disabled={this.props.disabled} />
            </Form.Item>
            <Form.Item label="Description" name="description">
              <Input.TextArea disabled={this.props.disabled} />
            </Form.Item>
            <EuiInput
              label="Gateway ID (EUI64)"
              name="gatewayId"
              value={this.props.initialValues.getGatewayId()}
              formRef={this.formRef}
              disabled={this.props.update || this.props.disabled}
              required
            />
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
                    draggable={!this.props.disabled}
                    eventHandlers={{ dragend: this.updateLocation }}
                  />
                </Map>
                <Button
                  onClick={this.getCurrentLocation}
                  disabled={this.state.locationPending}
                  type="link"
                  style={{ float: "right" }}
                >
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
                          <Input placeholder="Key" disabled={this.props.disabled} />
                        </Form.Item>
                      </Col>
                      <Col span={16}>
                        <Form.Item
                          {...restField}
                          name={[name, 1]}
                          fieldKey={[name, 1]}
                          rules={[{ required: true, message: "Please enter a value!" }]}
                        >
                          <Input placeholder="Value" disabled={this.props.disabled} />
                        </Form.Item>
                      </Col>
                      <Col span={2}>
                        <MinusCircleOutlined onClick={() => remove(name)} disabled={this.props.disabled} />
                      </Col>
                    </Row>
                  ))}
                  <Form.Item>
                    <Button
                      type="dashed"
                      disabled={this.props.disabled}
                      onClick={() => add()}
                      block
                      icon={<PlusOutlined />}
                    >
                      Add tag
                    </Button>
                  </Form.Item>
                </>
              )}
            </Form.List>
          </Tabs.TabPane>
        </Tabs>
        <Form.Item>
          <Button type="primary" htmlType="submit" disabled={this.props.disabled}>
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default GatewayForm;
