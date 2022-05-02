import React, { Component } from "react";

import { Form, Input, Button, notification } from "antd";

interface FormValues {
  password: string;
  passwordConfirm: string;
}

interface IProps {
  onFinish: (pw: string) => void;
}

class PasswordForm extends Component<IProps> {
  onFinish = (v: FormValues) => {
    if (v.password !== v.passwordConfirm) {
      notification.error({
        message: "The password does not match!",
        duration: 3,
      });
    } else {
      this.props.onFinish(v.password);
    }
  };

  render() {
    return (
      <Form layout="vertical" onFinish={this.onFinish}>
        <Form.Item label="Password" name="password" rules={[{ required: true, message: "Please enter a password!" }]}>
          <Input type="password" />
        </Form.Item>
        <Form.Item
          label="Confirm password"
          name="passwordConfirm"
          rules={[{ required: true, message: "Please enter a password!" }]}
        >
          <Input type="password" />
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

export default PasswordForm;
