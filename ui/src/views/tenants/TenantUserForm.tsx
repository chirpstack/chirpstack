import React, { Component } from "react";

import { Form, Input, Switch, Row, Col, Button } from "antd";

import { TenantUser } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

interface IProps {
  initialValues: TenantUser;
  onFinish: (obj: TenantUser) => void;
  disableEmail?: boolean;
  disabled?: boolean;
}

interface IState {
  isAdmin: boolean;
}

class TenantUserForm extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      isAdmin: false,
    };
  }

  componentDidMount() {
    const v = this.props.initialValues;

    this.setState({
      isAdmin: v.getIsAdmin(),
    });
  }

  onFinish = (values: TenantUser.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);

    let tu = new TenantUser();
    tu.setEmail(v.email);
    tu.setIsAdmin(v.isAdmin);
    tu.setIsGatewayAdmin(v.isGatewayAdmin);
    tu.setIsDeviceAdmin(v.isDeviceAdmin);
    tu.setUserId(v.userId);
    tu.setTenantId(v.tenantId);

    this.props.onFinish(tu);
  };

  onIsAdminChange = (checked: boolean) => {
    this.setState({
      isAdmin: checked,
    });
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
        <Form.Item
          label="Email (of existing user)"
          tooltip="An user without additional permissions will be able to see all resources under this tenant and will be able to send and receive device payloads."
          name="email"
          rules={[{ required: true, message: "Please enter an email!" }]}
        >
          <Input disabled={this.props.disableEmail || this.props.disabled} />
        </Form.Item>
        <Form.Item
          label="User is tenant admin"
          tooltip="A tenant admin user is able to add and modify resources part of the tenant."
          name="isAdmin"
          valuePropName="checked"
        >
          <Switch onChange={this.onIsAdminChange} disabled={this.props.disabled} />
        </Form.Item>
        {!this.state.isAdmin && (
          <Row>
            <Col span={12}>
              <Form.Item
                label="User is gateway admin"
                tooltip="A gateway admin user is able to add and modify gateways part of the tenant."
                name="isGatewayAdmin"
                valuePropName="checked"
              >
                <Switch disabled={this.props.disabled} />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item
                label="User is device admin"
                tooltip="A device admin user is able to add and modify resources part of the tenant that are related to devices."
                name="isDeviceAdmin"
                valuePropName="checked"
              >
                <Switch disabled={this.props.disabled} />
              </Form.Item>
            </Col>
          </Row>
        )}
        <Form.Item>
          <Button type="primary" htmlType="submit" disabled={this.props.disabled}>
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default TenantUserForm;
