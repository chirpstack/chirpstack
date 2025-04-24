import { Form, Input, Button } from "antd";

import { QubitroIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface FormProps {
  initialValues: QubitroIntegration.AsObject;
  onFinish: (values: QubitroIntegration.AsObject) => void;
}

function QubitroIntegrationForm({ initialValues, onFinish }: FormProps) {
  return (
    <Form layout="vertical" initialValues={initialValues} onFinish={onFinish}>
      <Form.Item
        label="Project ID"
        name="projectId"
        rules={[{ required: true, message: "Please enter a Project ID!" }]}
      >
        <Input />
      </Form.Item>
      <Form.Item
        label="Webhook Signing Key"
        name="webhookSigningKey"
        rules={[{ required: true, message: "Please enter a Webhook Signing Key!" }]}
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

export default QubitroIntegrationForm; 