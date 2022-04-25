import React, { Component } from "react";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Form, Input, Button } from "antd";

interface IProps {
  initialValues: Application;
  onFinish: (obj: Application) => void;
  disabled?: boolean;
}

class ApplicationForm extends Component<IProps> {
  onFinish = (values: Application.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let app = new Application();

    app.setId(v.id);
    app.setTenantId(v.tenantId);
    app.setName(v.name);
    app.setDescription(v.description);

    this.props.onFinish(app);
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
          <Input disabled={this.props.disabled} />
        </Form.Item>
        <Form.Item label="Description" name="description">
          <Input.TextArea disabled={this.props.disabled} />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit" disabled={this.props.disabled}>
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default ApplicationForm;
