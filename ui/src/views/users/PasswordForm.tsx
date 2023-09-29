import { Form, Input, Button, notification } from "antd";

import { onFinishFailed } from "../helpers";

interface FormValues {
  password: string;
  passwordConfirm: string;
}

interface IProps {
  onFinish: (pw: string) => void;
}

function PasswordForm(props: IProps) {
  const onFinish = (v: FormValues) => {
    if (v.password !== v.passwordConfirm) {
      notification.error({
        message: "The password does not match!",
        duration: 3,
      });
    } else {
      props.onFinish(v.password);
    }
  };

  return (
    <Form layout="vertical" onFinish={onFinish} onFinishFailed={onFinishFailed}>
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

export default PasswordForm;
