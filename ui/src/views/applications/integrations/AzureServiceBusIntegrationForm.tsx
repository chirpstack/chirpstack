import React, { Component } from "react";

import { Form, Input, Button, Select } from "antd";

import { AzureServiceBusIntegration, Encoding } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  initialValues: AzureServiceBusIntegration;
  onFinish: (obj: AzureServiceBusIntegration) => void;
}

class AzureServiceBusIntegrationForm extends Component<IProps> {
  onFinish = (values: AzureServiceBusIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let i = new AzureServiceBusIntegration();

    i.setApplicationId(v.applicationId);
    i.setEncoding(v.encoding);
    i.setConnectionString(v.connectionString);
    i.setPublishName(v.publishName);

    this.props.onFinish(i);
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item
          label="Payload encoding"
          name="encoding"
          rules={[{ required: true, message: "Please select an encoding!" }]}
        >
          <Select>
            <Select.Option value={Encoding.JSON}>JSON</Select.Option>
            <Select.Option value={Encoding.PROTOBUF}>Protobuf (binary)</Select.Option>
          </Select>
        </Form.Item>
        <Form.Item
          label="Azure Service-Bus connection string"
          name="connectionString"
          tooltip="This string can be obtained after creating a 'Shared access policy' with 'Send' permission."
          rules={[{ required: true, message: "Please enter an Azure Service-Bus connection string!" }]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          label="Azure Service-Bus topic / queue name"
          name="publishName"
          rules={[{ required: true, message: "Please enter an Azure Service-Bus topic / queue name!" }]}
        >
          <Input />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default AzureServiceBusIntegrationForm;
