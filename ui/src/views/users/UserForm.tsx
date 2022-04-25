import React, { Component } from "react";

import { Form, Input, Switch, Row, Col, Button } from "antd";

import { User } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

interface IProps {
  initialValues: User;
  onFinish: (obj: User, password: string) => void;
  password: boolean;
}

interface IState {}

interface UserWithPassword extends User.AsObject {
  password: string;
}

class UserForm extends Component<IProps, IState> {
  onFinish = (v: UserWithPassword) => {
    const values = Object.assign(this.props.initialValues.toObject(), v);

    let user = new User();
    user.setId(values.id);
    user.setEmail(values.email);
    user.setNote(values.note);
    user.setIsActive(values.isActive);
    user.setIsAdmin(values.isAdmin);

    this.props.onFinish(user, v.password);
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item label="Email" name="email" rules={[{ required: true, message: "Please enter an email address!" }]}>
          <Input />
        </Form.Item>
        <Form.Item label="Optional notes" name="note">
          <Input.TextArea />
        </Form.Item>
        {this.props.password && (
          <Form.Item label="Password" name="password" rules={[{ required: true, message: "Please enter a password!" }]}>
            <Input type="password" />
          </Form.Item>
        )}
        <Row>
          <Col span={12}>
            <Form.Item label="Is active" name="isActive" valuePropName="checked">
              <Switch />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="Is admin" name="isAdmin" valuePropName="checked">
              <Switch />
            </Form.Item>
          </Col>
        </Row>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default UserForm;
