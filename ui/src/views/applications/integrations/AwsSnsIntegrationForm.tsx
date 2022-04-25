import React, { Component } from "react";

import { Form, Input, Button, Select } from "antd";

import { AwsSnsIntegration, Encoding } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  initialValues: AwsSnsIntegration;
  onFinish: (obj: AwsSnsIntegration) => void;
}

class AwsSnsIntegrationForm extends Component<IProps> {
  onFinish = (values: AwsSnsIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let i = new AwsSnsIntegration();

    i.setApplicationId(v.applicationId);
    i.setEncoding(v.encoding);
    i.setRegion(v.region);
    i.setAccessKeyId(v.accessKeyId);
    i.setSecretAccessKey(v.secretAccessKey);
    i.setTopicArn(v.topicArn);

    this.props.onFinish(i);
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
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
        <Form.Item label="AWS region" name="region" rules={[{ required: true, message: "Please enter a region!" }]}>
          <Input />
        </Form.Item>
        <Form.Item
          label="AWS Access Key ID"
          name="accessKeyId"
          rules={[{ required: true, message: "Please enter an Access Key ID!" }]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          label="AWS Secret Access Key"
          name="secretAccessKey"
          rules={[{ required: true, message: "Please enter a Secret Access Key!" }]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          label="AWS SNS topic ARN"
          name="topicArn"
          rules={[{ required: true, message: "Please enter a SNS topic ARN!" }]}
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
}

export default AwsSnsIntegrationForm;
