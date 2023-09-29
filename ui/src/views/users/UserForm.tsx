import { Form, Input, Switch, Row, Col, Button } from "antd";

import { User } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import { onFinishFailed } from "../helpers";

interface IProps {
  initialValues: User;
  onFinish: (obj: User, password: string) => void;
  password: boolean;
}

interface UserWithPassword extends User.AsObject {
  password: string;
}

function UserForm(props: IProps) {
  const onFinish = (v: UserWithPassword) => {
    const values = Object.assign(props.initialValues.toObject(), v);

    let user = new User();
    user.setId(values.id);
    user.setEmail(values.email);
    user.setNote(values.note);
    user.setIsActive(values.isActive);
    user.setIsAdmin(values.isAdmin);

    props.onFinish(user, v.password);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Form.Item label="Email" name="email" rules={[{ required: true, message: "Please enter an email address!" }]}>
        <Input />
      </Form.Item>
      <Form.Item label="Optional notes" name="note">
        <Input.TextArea />
      </Form.Item>
      {props.password && (
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

export default UserForm;
