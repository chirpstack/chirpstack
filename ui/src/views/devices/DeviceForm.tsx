import React, { Component } from "react";

import { Form, Input, Row, Col, Button, Tabs } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import {
  ListDeviceProfilesRequest,
  ListDeviceProfilesResponse,
  GetDeviceProfileRequest,
  GetDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import EuiInput from "../../components/EuiInput";
import { OptionsCallbackFunc, OptionCallbackFunc } from "../../components/Autocomplete";
import AutocompleteInput from "../../components/AutocompleteInput";
import DeviceProfileStore from "../../stores/DeviceProfileStore";

interface IProps {
  tenant: Tenant;
  initialValues: Device;
  onFinish: (obj: Device) => void;
  update?: boolean;
}

class DeviceForm extends Component<IProps> {
  formRef = React.createRef<any>();

  onFinish = (values: Device.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let d = new Device();

    d.setApplicationId(v.applicationId);
    d.setName(v.name);
    d.setDescription(v.description);
    d.setDevEui(v.devEui);
    d.setDeviceProfileId(v.deviceProfileId);

    // tags
    for (const elm of v.tagsMap) {
      d.getTagsMap().set(elm[0], elm[1]);
    }

    // variables
    for (const elm of v.variablesMap) {
      d.getVariablesMap().set(elm[0], elm[1]);
    }

    this.props.onFinish(d);
  };

  getDeviceProfileOptions = (search: string, fn: OptionsCallbackFunc) => {
    let req = new ListDeviceProfilesRequest();
    req.setTenantId(this.props.tenant.getId());
    req.setSearch(search);
    req.setLimit(10);

    DeviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
      const options = resp.getResultList().map((o, i) => {
        return { label: o.getName(), value: o.getId() };
      });
      fn(options);
    });
  };

  getDeviceProfileOption = (id: string, fn: OptionCallbackFunc) => {
    let req = new GetDeviceProfileRequest();
    req.setId(id);

    DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
      const dp = resp.getDeviceProfile();
      if (dp) {
        fn({ label: dp.getName(), value: dp.getId() });
      }
    });
  };

  render() {
    return (
      <Form
        layout="vertical"
        initialValues={this.props.initialValues.toObject()}
        onFinish={this.onFinish}
        ref={this.formRef}
      >
        <Tabs>
          <Tabs.TabPane tab="Device" key="1">
            <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
              <Input />
            </Form.Item>
            <Form.Item label="Description" name="description">
              <Input.TextArea />
            </Form.Item>
            <EuiInput
              label="Device EUI (EUI64)"
              name="devEui"
              value={this.props.initialValues.getDevEui()}
              formRef={this.formRef}
              disabled={this.props.update}
              required
            />
            <AutocompleteInput
              label="Device profile"
              name="deviceProfileId"
              formRef={this.formRef}
              getOption={this.getDeviceProfileOption}
              getOptions={this.getDeviceProfileOptions}
              required
            />
          </Tabs.TabPane>
          <Tabs.TabPane tab="Tags" key="2">
            <Form.List name="tagsMap">
              {(fields, { add, remove }) => (
                <>
                  {fields.map(( {key, name, ...restField} ) => (
                    <Row gutter={24}>
                      <Col span={6}>
                        <Form.Item
                          {...restField}
                          name={[name, 0]}
                          fieldKey={[name, 0]}
                          rules={[{ required: true, message: "Please enter a key!" }]}
                        >
                          <Input placeholder="Key" />
                        </Form.Item>
                      </Col>
                      <Col span={16}>
                        <Form.Item
                          {...restField}
                          name={[name, 1]}
                          fieldKey={[name, 1]}
                          rules={[{ required: true, message: "Please enter a value!" }]}
                        >
                          <Input placeholder="Value" />
                        </Form.Item>
                      </Col>
                      <Col span={2}>
                        <MinusCircleOutlined onClick={() => remove(name)} />
                      </Col>
                    </Row>
                  ))}
                  <Form.Item>
                    <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                      Add tag
                    </Button>
                  </Form.Item>
                </>
              )}
            </Form.List>
          </Tabs.TabPane>
          <Tabs.TabPane tab="Variables" key="3">
            <Form.List name="variablesMap">
              {(fields, { add, remove }) => (
                <>
                  {fields.map(( {key, name, ...restField} ) => (
                    <Row gutter={24}>
                      <Col span={6}>
                        <Form.Item
                          {...restField}
                          name={[name, 0]}
                          fieldKey={[name, 0]}
                          rules={[{ required: true, message: "Please enter a key!" }]}
                        >
                          <Input placeholder="Key" />
                        </Form.Item>
                      </Col>
                      <Col span={16}>
                        <Form.Item
                          {...restField}
                          name={[name, 1]}
                          fieldKey={[name, 1]}
                          rules={[{ required: true, message: "Please enter a value!" }]}
                        >
                          <Input placeholder="Value" />
                        </Form.Item>
                      </Col>
                      <Col span={2}>
                        <MinusCircleOutlined onClick={() => remove(name)} />
                      </Col>
                    </Row>
                  ))}
                  <Form.Item>
                    <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                      Add variable
                    </Button>
                  </Form.Item>
                </>
              )}
            </Form.List>
          </Tabs.TabPane>
        </Tabs>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default DeviceForm;
