import React, { Component } from "react";

import { Form, Input, Button } from "antd";

import { PilotThingsIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  initialValues: PilotThingsIntegration;
  onFinish: (obj: PilotThingsIntegration) => void;
}

class PilotThingsIntegrationForm extends Component<IProps> {
  onFinish = (values: PilotThingsIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let i = new PilotThingsIntegration();

    i.setApplicationId(v.applicationId);
    i.setServer(v.server);
    i.setToken(v.token);

    this.props.onFinish(i);
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item
          label="Pilot Things server"
          name="server"
          rules={[{ required: true, message: "Please enter a Pilot Things server!" }]}
        >
          <Input placeholder="https://host:port" />
        </Form.Item>
        <Form.Item
          label="Authentication token"
          name="token"
          rules={[{ required: true, message: "Please enter a Pilot Things token!" }]}
        >
          <Input.Password />
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

export default PilotThingsIntegrationForm;
