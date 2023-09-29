import React, { useState } from "react";

import { Form, Input, Button, Select } from "antd";

import { MyDevicesIntegration } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import { onFinishFailed } from "../../helpers";

interface IProps {
  initialValues: MyDevicesIntegration;
  onFinish: (obj: MyDevicesIntegration) => void;
}

function MyDevicesIntegrationForm(props: IProps) {
  const [selectedEndpoint, setSelectedEndpoint] = useState<string>("");
  const [customEndpoint, setCustomEndpoint] = useState<string>("");

  const onFinish = (values: MyDevicesIntegration.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let i = new MyDevicesIntegration();

    i.setApplicationId(v.applicationId);
    if (v.endpoint === "custom") {
      i.setEndpoint(customEndpoint);
    } else {
      i.setEndpoint(v.endpoint);
    }

    props.onFinish(i);
  };

  const onEndpointChange = (v: string) => {
    setSelectedEndpoint(v);
  };

  const onCustomEndpointChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setCustomEndpoint(e.target.value);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Form.Item
        label="Select myDevices endpoint"
        name="endpoint"
        rules={[{ required: true, message: "Please select a myDevices endpoint!" }]}
      >
        <Select onChange={onEndpointChange}>
          <Select.Option value="https://lora.mydevices.com/v1/networks/chirpstackio/uplink">Cayenne</Select.Option>
          <Select.Option value="https://lora.iotinabox.com/v1/networks/iotinabox.chirpstackio/uplink">
            IoT in a Box
          </Select.Option>
          <Select.Option value="custom">Custom endpoint URL</Select.Option>
        </Select>
      </Form.Item>
      {selectedEndpoint === "custom" && (
        <Form.Item
          label="myDevices API endpoint"
          name="customEndpoint"
          rules={[{ required: true, message: "Please enter an API endpoint!" }]}
        >
          <Input onChange={onCustomEndpointChange} />
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

export default MyDevicesIntegrationForm;
