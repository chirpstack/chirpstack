import { Form, Input, Button } from "antd";

import { PilotThingsIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import { onFinishFailed } from "../../helpers";

interface IProps {
  initialValues: PilotThingsIntegration;
  onFinish: (obj: PilotThingsIntegration) => void;
}

function PilotThingsIntegrationForm(props: IProps) {
  const onFinish = (values: PilotThingsIntegration.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let i = new PilotThingsIntegration();

    i.setApplicationId(v.applicationId);
    i.setServer(v.server);
    i.setToken(v.token);

    props.onFinish(i);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Form.Item
        label="Pilot Things server"
        name="server"
        rules={[{ required: true, message: "Please enter a Pilot Things server!" }]}
      >
        <Input placeholder="https://host:port" />
      </Form.Item>
      <Form.Item
        label="Authentication token"
        name="token"
        rules={[{ required: true, message: "Please enter a Pilot Things token!" }]}
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

export default PilotThingsIntegrationForm;
