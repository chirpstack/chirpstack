import React, { Component } from "react";

import { Form, Input, InputNumber, Switch, Row, Col, Button } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

interface IProps {
  initialValues: Tenant;
  onFinish: (obj: Tenant) => void;
  disabled?: boolean;
}

interface IState {}

class TenantForm extends Component<IProps, IState> {
  onFinish = (v: Tenant.AsObject) => {
    const values = Object.assign(this.props.initialValues.toObject(), v);

    let tenant = new Tenant();
    tenant.setId(values.id);
    tenant.setName(values.name);
    tenant.setDescription(values.description);
    tenant.setCanHaveGateways(values.canHaveGateways);
    tenant.setMaxGatewayCount(values.maxGatewayCount);
    tenant.setMaxDeviceCount(values.maxDeviceCount);
    tenant.setPrivateGatewaysUp(values.privateGatewaysUp);
    tenant.setPrivateGatewaysDown(values.privateGatewaysDown);

    this.props.onFinish(tenant);
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
          <Input disabled={this.props.disabled} />
        </Form.Item>
        <Form.Item label="Description" name="description">
          <Input.TextArea disabled={this.props.disabled} />
        </Form.Item>
        <Row>
          <Col span={8}>
            <Form.Item
              label="Tenant can have gateways"
              name="canHaveGateways"
              tooltip="When checked, the tenant can add gateways. Note that the usage of the gateways is not limited to this tenant only unless these are marked private."
              valuePropName="checked"
            >
              <Switch disabled={this.props.disabled} />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="Gateways are private (uplink)"
              name="privateGatewaysUp"
              tooltip="Uplink received by gateways of this tenant can only be used by the devices of this tenant."
              valuePropName="checked"
            >
              <Switch disabled={this.props.disabled} />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="Gateways are private (downlink)"
              name="privateGatewaysDown"
              tooltip="Other tenants can not use the gateways of this tenant for downlinks. This can be useful in case uplinks are shared with other tenants, but you want to avoid other tenants using downlink airtime of your gateways."
              valuePropName="checked"
            >
              <Switch disabled={this.props.disabled} />
            </Form.Item>
          </Col>
        </Row>
        <Row>
          <Col span={12}>
            <Form.Item
              label="Max. gateway count"
              name="maxGatewayCount"
              tooltip="The maximum number of gateways that can be added by this tenant (0 = unlimited)."
            >
              <InputNumber min={0} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item
              label="Max. device count"
              name="maxDeviceCount"
              required={true}
              tooltip="The maximum number of devices that can be added by this tenant (0 = unlimited)."
            >
              <InputNumber min={0} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
        </Row>
        <Form.Item>
          <Button type="primary" htmlType="submit" disabled={this.props.disabled}>
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default TenantForm;
