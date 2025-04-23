import { Form, Input, Button } from "antd";

import { QubitroIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  initialValues: QubitroIntegration;
  onFinish: (obj: QubitroIntegration) => void;
}

interface FormValues {
  project_id: string;
  webhook_signing_key: string;
}

function QubitroIntegrationForm(props: IProps) {
  const onFinish = (values: FormValues) => {
    const i = new QubitroIntegration();
    i.setProjectId(values.project_id);
    i.setWebhookSigningKey(values.webhook_signing_key);

    props.onFinish(i);
  };

  const initialValues: FormValues = {
    project_id: props.initialValues.getProjectId(),
    webhook_signing_key: props.initialValues.getWebhookSigningKey(),
  };

  return (
    <Form layout="vertical" initialValues={initialValues} onFinish={onFinish}>
      <Form.Item
        label="Project ID"
        name="project_id"
        rules={[{ required: true, message: "Please enter your Qubitro Project ID" }]}
      >
        <Input />
      </Form.Item>
      <Form.Item
        label="Webhook Signing Key"
        name="webhook_signing_key"
        rules={[{ required: true, message: "Please enter your Qubitro Webhook Signing Key" }]}
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