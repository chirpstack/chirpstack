import React, { Component } from "react";

import { Form, Input, Button, Select } from "antd";

import {
  InfluxDbIntegration,
  InfluxDbPrecision,
  InfluxDbVersion,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  initialValues: InfluxDbIntegration;
  onFinish: (obj: InfluxDbIntegration) => void;
}

interface IState {
  selectedVersion: InfluxDbVersion;
}

class InfluxDbIntegrationForm extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      selectedVersion: InfluxDbVersion.INFLUXDB_1,
    };
  }

  onFinish = (values: InfluxDbIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
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

    this.props.onFinish(i);
  };

  onVersionChange = (version: InfluxDbVersion) => {
    this.setState({
      selectedVersion: version,
    });
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item
          label="InfluxDB version"
          name="version"
          rules={[{ required: true, message: "Please select an InfluxDB version!" }]}
        >
          <Select onChange={this.onVersionChange}>
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
        {this.state.selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
          <Form.Item label="Username" name="username">
            <Input />
          </Form.Item>
        )}
        {this.state.selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
          <Form.Item label="Password" name="password">
            <Input.Password />
          </Form.Item>
        )}
        {this.state.selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
          <Form.Item
            label="Database name"
            name="db"
            rules={[{ required: true, message: "Please enter database name!" }]}
          >
            <Input />
          </Form.Item>
        )}
        {this.state.selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
          <Form.Item
            label="Retention policy name"
            name="retentionPolicyName"
            tooltip="Sets the target retention policy for the write. InfluxDB writes to the DEFAULT retention policy if you do not specify a retention policy."
          >
            <Input />
          </Form.Item>
        )}
        {this.state.selectedVersion === InfluxDbVersion.INFLUXDB_1 && (
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
        {this.state.selectedVersion === InfluxDbVersion.INFLUXDB_2 && (
          <Form.Item label="Organization" name="organization">
            <Input />
          </Form.Item>
        )}
        {this.state.selectedVersion === InfluxDbVersion.INFLUXDB_2 && (
          <Form.Item label="Bucket" name="bucket">
            <Input />
          </Form.Item>
        )}
        {this.state.selectedVersion === InfluxDbVersion.INFLUXDB_2 && (
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
}

export default InfluxDbIntegrationForm;
