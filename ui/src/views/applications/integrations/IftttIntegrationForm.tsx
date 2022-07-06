import React, { Component } from "react";

import { Form, Input, AutoComplete, Button } from "antd";

import { IftttIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  measurementKeys: string[];
  initialValues: IftttIntegration;
  onFinish: (obj: IftttIntegration) => void;
}

class IftttIntegrationForm extends Component<IProps> {
  onFinish = (values: IftttIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let i = new IftttIntegration();

    i.setApplicationId(v.applicationId);
    i.setKey(v.key);
    i.setUplinkValuesList(v.uplinkValuesList);

    this.props.onFinish(i);
  };

  render() {
    const options: {
      value: string;
    }[] = this.props.measurementKeys.map(v => {
      return { value: v };
    });

    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item
          label="Key"
          name="key"
          rules={[{ required: true, message: "Please enter a key!" }]}
          tooltip="This key can be obtained from the IFTTT Webhooks integrations documentation"
        >
          <Input.Password />
        </Form.Item>
        <Form.List name="uplinkValuesList">
          {fields => (
            <div>
              {fields.map((field, i) => (
                <Form.Item
                  label={`Value ${i + 1} key`}
                  {...field}
                  tooltip="This must match the key in the decoded uplink payload. Nested keys are joined with a '_', array elements are indexed (starting at zero), e.g. 'sensor_0', 'sensor_1'. Auto-completion is based on measurements configuration in the device-profile(s) used within this application."
                >
                  <AutoComplete options={options} />
                </Form.Item>
              ))}
            </div>
          )}
        </Form.List>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default IftttIntegrationForm;
