import React, { Component } from "react";

import { Form, Input, AutoComplete, Button, Row, Col, Switch } from "antd";

import { IftttIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  measurementKeys: string[];
  initialValues: IftttIntegration;
  onFinish: (obj: IftttIntegration) => void;
}

interface IState {
  arbitraryJson: boolean;
}

class IftttIntegrationForm extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      arbitraryJson: false,
    };
  }

  componentDidMount() {
    const v = this.props.initialValues;

    this.setState({
      arbitraryJson: v.getArbitraryJson(),
    });
  }

  onFinish = (values: IftttIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let i = new IftttIntegration();

    i.setApplicationId(v.applicationId);
    i.setKey(v.key);
    i.setEventPrefix(v.eventPrefix);
    i.setArbitraryJson(v.arbitraryJson);
    i.setUplinkValuesList(v.uplinkValuesList);

    this.props.onFinish(i);
  };

  onArbitraryJsonChange = (checked: boolean) => {
    this.setState({
      arbitraryJson: checked,
    });
  }

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
        <Row gutter={24}>
          <Col span={12}>
            <Form.Item
              label="Event prefix"
              name="eventPrefix"
              rules={[{ pattern: /[A-Za-z0-9]+/, message: "Only use A-Z, a-z and 0-9 characters" }]}
              tooltip="The prefix will be added to the Webhook event, e.g. if set an uplink will be published as PREFIX_up instead of up."
            >
              <Input />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item
              label="Publish as arbitrary JSON"
              name="arbitraryJson"
              valuePropName="checked"
              tooltip="If enabled, the event payload will be published as-is (arbitrary JSON payload instead of 3 JSON values format)."
            >
              <Switch onChange={this.onArbitraryJsonChange} />
            </Form.Item>
          </Col>
        </Row>
        {!this.state.arbitraryJson && <Form.List name="uplinkValuesList">
          {fields => (
            <Row gutter={24}>
              {fields.map((field, i) => (
                <Col span={12}>
                  <Form.Item
                    label={`Value ${i + 1} key`}
                    {...field}
                    tooltip="This must match the key in the decoded uplink payload. Nested keys are joined with a '_', array elements are indexed (starting at zero), e.g. 'sensor_0', 'sensor_1'. Auto-completion is based on measurements configuration in the device-profile(s) used within this application."
                  >
                    <AutoComplete options={options} />
                  </Form.Item>
                </Col>
              ))}
            </Row>
          )}
        </Form.List>}
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
