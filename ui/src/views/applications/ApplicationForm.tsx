import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Form, Input, Button, Tabs, Row, Col } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

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

    // tags
    for (const elm of v.tagsMap) {
      app.getTagsMap().set(elm[0], elm[1]);
    }

    props.onFinish(app);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Tabs>
        <Tabs.TabPane tab="General" key="1">
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input disabled={props.disabled} />
          </Form.Item>
          <Form.Item label="Description" name="description">
            <Input.TextArea disabled={props.disabled} />
          </Form.Item>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Tags" key="2">
          <Form.List name="tagsMap">
            {(fields, { add, remove }) => (
              <>
                {fields.map(({ key, name, ...restField }) => (
                  <Row gutter={24}>
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 0]}
                        fieldKey={[name, 0]}
                        rules={[{ required: true, message: "Please enter a key!" }]}
                      >
                        <Input placeholder="Key" />
                      </Form.Item>
                    </Col>
                    <Col span={16}>
                      <Form.Item
                        {...restField}
                        name={[name, 1]}
                        fieldKey={[name, 1]}
                        rules={[{ required: true, message: "Please enter a value!" }]}
                      >
                        <Input placeholder="Value" />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    Add tag
                  </Button>
                </Form.Item>
              </>
            )}
          </Form.List>
        </Tabs.TabPane>
      </Tabs>
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default ApplicationForm;
