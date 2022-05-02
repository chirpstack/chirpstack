import React, { Component } from "react";

import { Form, Input, Button, Select } from "antd";

import { MyDevicesIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  initialValues: MyDevicesIntegration;
  onFinish: (obj: MyDevicesIntegration) => void;
}

interface IState {
  selectedEndpoint: string;
  customEndpoint: string;
}

class MyDevicesIntegrationForm extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      selectedEndpoint: "",
      customEndpoint: "",
    };
  }

  onFinish = (values: MyDevicesIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let i = new MyDevicesIntegration();

    i.setApplicationId(v.applicationId);
    if (v.endpoint === "custom") {
      i.setEndpoint(this.state.customEndpoint);
    } else {
      i.setEndpoint(v.endpoint);
    }

    this.props.onFinish(i);
  };

  onEndpointChange = (v: string) => {
    this.setState({
      selectedEndpoint: v,
    });
  };

  onCustomEndpointChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({
      customEndpoint: e.target.value,
    });
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item
          label="Select myDevices endpoint"
          name="endpoint"
          rules={[{ required: true, message: "Please select a myDevices endpoint!" }]}
        >
          <Select onChange={this.onEndpointChange}>
            <Select.Option value="https://lora.mydevices.com/v1/networks/chirpstackio/uplink">Cayenne</Select.Option>
            <Select.Option value="https://lora.iotinabox.com/v1/networks/iotinabox.chirpstackio/uplink">
              IoT in a Box
            </Select.Option>
            <Select.Option value="custom">Custom endpoint URL</Select.Option>
          </Select>
        </Form.Item>
        {this.state.selectedEndpoint === "custom" && (
          <Form.Item
            label="myDevices API endpoint"
            name="customEndpoint"
            rules={[{ required: true, message: "Please enter an API endpoint!" }]}
          >
            <Input onChange={this.onCustomEndpointChange} />
          </Form.Item>
        )}
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default MyDevicesIntegrationForm;
