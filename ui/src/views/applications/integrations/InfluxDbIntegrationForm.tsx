import React, { useState, useEffect } from "react";

import { Form, Input, Button, Select } from "antd";

import {
  InfluxDbIntegration,
  InfluxDbPrecision,
  InfluxDbVersion,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import { onFinishFailed } from "../../helpers";

interface IProps {
  initialValues: InfluxDbIntegration;
  onFinish: (obj: InfluxDbIntegration) => void;
}

function InfluxDbIntegrationForm(props: IProps) {
  const [selectedVersion, setSelectedVersion] = useState<InfluxDbVersion>(InfluxDbVersion.INFLUXDB_1);

  useEffect(() => {
    setSelectedVersion(props.initialValues.getVersion());
  }, [props]);

  const onFinish = (values: InfluxDbIntegration.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let i = new InfluxDbIntegration();

    i.setApplicationId(v.applicationId);
    i.setVersion(v.version);
    i.setEndpoint(v.endpoint);
    i.setUsername(v.username);
    i.setPassword(v.password);
    i.setDb(v.db);
    i.setRetentionPolicyName(v.retentionPolicyName);
    i.setPrecision(v.precision);
    i.setOrganization(v.organization);
    i.setBucket(v.bucket);
    i.setToken(v.token);

    props.onFinish(i);
  };

  const onVersionChange = (version: InfluxDbVersion) => {
    setSelectedVersion(version);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Form.Item
        label="InfluxDB version"
        name="version"
        rules={[{ required: true, message: "Please select an InfluxDB version!" }]}
      >
        <Select onChange={onVersionChange}>
          <Select.Option value={InfluxDbVersion.INFLUXDB_1}>InfluxDB v1</Select.Option>
          <Select.Option value={InfluxDbVersion.INFLUXDB_2}>InfluxDB v2</Select.Option>
        </Select>
      </Form.Item>
      <Form.Item
        label="API endpoint (write)"
        name="endpoint"
        rules={[{ required: true, message: "Please enter an endpoint!" }]}
      >
        <Input placeholder="http://localhost:8086/api/v2/write" />
      </Form.Item>
      {selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
        <Form.Item label="Username" name="username">
          <Input />
        </Form.Item>
      )}
      {selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
        <Form.Item label="Password" name="password">
          <Input.Password />
        </Form.Item>
      )}
      {selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
        <Form.Item label="Database name" name="db" rules={[{ required: true, message: "Please enter database name!" }]}>
          <Input />
        </Form.Item>
      )}
      {selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
        <Form.Item
          label="Retention policy name"
          name="retentionPolicyName"
          tooltip="Sets the target retention policy for the write. InfluxDB writes to the DEFAULT retention policy if you do not specify a retention policy."
        >
          <Input />
        </Form.Item>
      )}
      {selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
        <Form.Item label="Select timestamp precision" name="precision">
          <Select>
            <Select.Option value={InfluxDbPrecision.NS}>Nanosecond</Select.Option>
            <Select.Option value={InfluxDbPrecision.U}>Microsecond</Select.Option>
            <Select.Option value={InfluxDbPrecision.MS}>Millisecond</Select.Option>
            <Select.Option value={InfluxDbPrecision.S}>Second</Select.Option>
            <Select.Option value={InfluxDbPrecision.M}>Minute</Select.Option>
            <Select.Option value={InfluxDbPrecision.H}>Hour</Select.Option>
          </Select>
        </Form.Item>
      )}
      {selectedVersion === InfluxDbVersion.INFLUXDB_2 && (
        <Form.Item label="Organization" name="organization">
          <Input />
        </Form.Item>
      )}
      {selectedVersion === InfluxDbVersion.INFLUXDB_2 && (
        <Form.Item label="Bucket" name="bucket">
          <Input />
        </Form.Item>
      )}
      {selectedVersion === InfluxDbVersion.INFLUXDB_2 && (
        <Form.Item label="Token" name="token">
          <Input.Password />
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

export default InfluxDbIntegrationForm;
