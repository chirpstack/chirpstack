import React, { Component } from "react";

import { Form, Input, Button } from "antd";

import { ApiKey } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

interface IProps {
  initialValues: ApiKey;
  onFinish: (obj: ApiKey) => void;
}

interface IState {}

class ApiKeyForm extends Component<IProps, IState> {
  onFinish = (values: ApiKey.AsObject) => {
    let apiKey = new ApiKey();
    apiKey.setName(values.name);
    this.props.onFinish(apiKey);
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
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

export default ApiKeyForm;
