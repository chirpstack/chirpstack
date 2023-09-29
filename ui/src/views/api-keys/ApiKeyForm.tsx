import { Form, Input, Button } from "antd";

import { ApiKey } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import { onFinishFailed } from "../helpers";

interface IProps {
  initialValues: ApiKey;
  onFinish: (obj: ApiKey) => void;
}

function ApiKeyForm(props: IProps) {
  const onFinish = (values: ApiKey.AsObject) => {
    let apiKey = new ApiKey();
    apiKey.setName(values.name);
    props.onFinish(apiKey);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
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

export default ApiKeyForm;
