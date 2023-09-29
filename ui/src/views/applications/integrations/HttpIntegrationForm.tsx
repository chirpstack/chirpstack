import { Form, Input, Button, Select, Row, Col, Typography, Space } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import { HttpIntegration, Encoding } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import { onFinishFailed } from "../../helpers";

interface IProps {
  initialValues: HttpIntegration;
  onFinish: (obj: HttpIntegration) => void;
}

function HttpIntegrationForm(props: IProps) {
  const onFinish = (values: HttpIntegration.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let i = new HttpIntegration();

    i.setApplicationId(v.applicationId);
    i.setEncoding(v.encoding);
    i.setEventEndpointUrl(v.eventEndpointUrl);

    // headers
    for (const elm of v.headersMap) {
      i.getHeadersMap().set(elm[0], elm[1]);
    }

    props.onFinish(i);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Form.Item
        label="Payload encoding"
        name="encoding"
        rules={[{ required: true, message: "Please select an encoding!" }]}
      >
        <Select>
          <Select.Option value={Encoding.JSON}>JSON</Select.Option>
          <Select.Option value={Encoding.PROTOBUF}>Protobuf (binary)</Select.Option>
        </Select>
      </Form.Item>
      <Form.Item
        label="Event endpoint URL(s)"
        name="eventEndpointUrl"
        tooltip="ChirpStack will make a POST request to this URL(s) with 'event' as query parameter. Multiple URLs can be defined as a comma separated list. Whitespace will be automatically removed."
        rules={[{ required: true, message: "Please enter an event endpoint URL!" }]}
      >
        <Input />
      </Form.Item>
      <Space direction="vertical" style={{ width: "100%" }}>
        <Typography.Text>Headers</Typography.Text>
        <Form.List name="headersMap">
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
                  Add header
                </Button>
              </Form.Item>
            </>
          )}
        </Form.List>
      </Space>
      <Form.Item>
        <Button type="primary" htmlType="submit">
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default HttpIntegrationForm;
