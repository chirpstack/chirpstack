import { Form, Input, Button } from "antd";

import { BlynkIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import { onFinishFailed } from "../../helpers";

interface IProps {
  initialValues: BlynkIntegration;
  onFinish: (obj: BlynkIntegration) => void;
}

function BlynkIntegrationForm(props: IProps) {
  const onFinish = (values: BlynkIntegration.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    const i = new BlynkIntegration();

    i.setApplicationId(v.applicationId);
    i.setToken(v.token);

    props.onFinish(i);
  };

  return (
    <Form
      layout="vertical"
      initialValues={props.initialValues.toObject()}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
    >
      <Form.Item
        label="Blynk API token"
        name="token"
        rules={[{ required: true, message: "Please enter a Blynk API token!" }]}
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

export default BlynkIntegrationForm;
