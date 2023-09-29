import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Form, Input, Button } from "antd";

import { onFinishFailed } from "../helpers";

interface IProps {
  initialValues: Application;
  onFinish: (obj: Application) => void;
  disabled?: boolean;
}

function ApplicationForm(props: IProps) {
  const onFinish = (values: Application.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let app = new Application();

    app.setId(v.id);
    app.setTenantId(v.tenantId);
    app.setName(v.name);
    app.setDescription(v.description);

    props.onFinish(app);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
        <Input disabled={props.disabled} />
      </Form.Item>
      <Form.Item label="Description" name="description">
        <Input.TextArea disabled={props.disabled} />
      </Form.Item>
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default ApplicationForm;
