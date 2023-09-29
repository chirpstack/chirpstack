import React, { useState, useEffect } from "react";
import { Form, Input, Switch, Row, Col, Button } from "antd";

import { TenantUser } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import { onFinishFailed } from "../helpers";

interface IProps {
  initialValues: TenantUser;
  onFinish: (obj: TenantUser) => void;
  disableEmail?: boolean;
  disabled?: boolean;
}

function TenantUserForm(props: IProps) {
  const [isAdmin, setIsAdmin] = useState<boolean>(false);

  useEffect(() => {
    setIsAdmin(props.initialValues.getIsAdmin());
  }, [props]);

  const onFinish = (values: TenantUser.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);

    let tu = new TenantUser();
    tu.setEmail(v.email);
    tu.setIsAdmin(v.isAdmin);
    tu.setIsGatewayAdmin(v.isGatewayAdmin);
    tu.setIsDeviceAdmin(v.isDeviceAdmin);
    tu.setUserId(v.userId);
    tu.setTenantId(v.tenantId);

    props.onFinish(tu);
  };

  const onIsAdminChange = (checked: boolean) => {
    setIsAdmin(checked);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Form.Item
        label="Email (of existing user)"
        tooltip="An user without additional permissions will be able to see all resources under this tenant and will be able to send and receive device payloads."
        name="email"
        rules={[{ required: true, message: "Please enter an email!" }]}
      >
        <Input disabled={props.disableEmail || props.disabled} />
      </Form.Item>
      <Form.Item
        label="User is tenant admin"
        tooltip="A tenant admin user is able to add and modify resources part of the tenant."
        name="isAdmin"
        valuePropName="checked"
      >
        <Switch onChange={onIsAdminChange} disabled={props.disabled} />
      </Form.Item>
      {!isAdmin && (
        <Row>
          <Col span={12}>
            <Form.Item
              label="User is gateway admin"
              tooltip="A gateway admin user is able to add and modify gateways part of the tenant."
              name="isGatewayAdmin"
              valuePropName="checked"
            >
              <Switch disabled={props.disabled} />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item
              label="User is device admin"
              tooltip="A device admin user is able to add and modify resources part of the tenant that are related to devices."
              name="isDeviceAdmin"
              valuePropName="checked"
            >
              <Switch disabled={props.disabled} />
            </Form.Item>
          </Col>
        </Row>
      )}
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default TenantUserForm;
