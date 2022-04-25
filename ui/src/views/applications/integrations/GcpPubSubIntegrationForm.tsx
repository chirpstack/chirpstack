import React, { Component } from "react";

import { Form, Input, Button, Select } from "antd";

import { GcpPubSubIntegration, Encoding } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  initialValues: GcpPubSubIntegration;
  onFinish: (obj: GcpPubSubIntegration) => void;
}

class GcpPubSubIntegrationForm extends Component<IProps> {
  onFinish = (values: GcpPubSubIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let i = new GcpPubSubIntegration();

    i.setApplicationId(v.applicationId);
    i.setEncoding(v.encoding);
    i.setProjectId(v.projectId);
    i.setTopicName(v.topicName);
    i.setCredentialsFile(v.credentialsFile);

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
        <Form.Item
          label="GCP project ID"
          name="projectId"
          rules={[{ required: true, message: "Please enter a GCP project ID!" }]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          label="GCP Pub/Sub topic name"
          name="topicName"
          rules={[{ required: true, message: "Please enter a GCP Pub/Sub topic name!" }]}
        >
          <Input />
        </Form.Item>
        <Form.Item
          label="GCP Service account credentials file"
          name="credentialsFile"
          tooltip="Under IAM create a Service account with 'Pub/Sub Publisher' role, then put the content of the JSON key in this field."
          rules={[{ required: true, message: "Please enter a GCP Service account credentials file!" }]}
        >
          <Input.TextArea rows={10} />
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

export default GcpPubSubIntegrationForm;
