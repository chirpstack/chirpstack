import { useState, useEffect } from "react";
import { Form, Input, Switch, Row, Col, Button, Tabs, Select, Space, Card } from "antd";
import type { TabsProps, SelectProps } from "antd/lib";

import { ListApplicationsRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { ListDeviceProfilesRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import type {
  DeviceProfileListItem,
  ListDeviceProfilesResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import type {
  ListApplicationsResponse,
  ApplicationListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  TenantUser,
  TenantUserDeviceProfile,
  TenantUserApplication,
} from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import deviceProfileStore from "../../stores/DeviceProfileStore";
import { onFinishFailed } from "../helpers";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";
import applicationStore from "../../stores/ApplicationStore";

interface IProps {
  initialValues: TenantUser;
  tenant: Tenant;
  onFinish: (obj: TenantUser) => void;
  disableEmail?: boolean;
  disabled?: boolean;
}

function TenantUserForm(props: IProps) {
  const [isAdmin, setIsAdmin] = useState<boolean>(false);
  const [deviceProfiles, setDeviceProfiles] = useState<DeviceProfileListItem[]>([]);
  const [applications, setApplications] = useState<ApplicationListItem[]>([]);

  useEffect(() => {
    setIsAdmin(props.initialValues.getIsAdmin());

    const dpReq = new ListDeviceProfilesRequest();
    dpReq.setTenantOnly(true);
    dpReq.setTenantId(props.tenant.getId());
    dpReq.setLimit(9999);

    deviceProfileStore.list(dpReq, (resp: ListDeviceProfilesResponse) => {
      setDeviceProfiles(resp.getResultList());
    });

    const appReq = new ListApplicationsRequest();
    appReq.setTenantId(props.tenant.getId());
    appReq.setLimit(9999);

    applicationStore.list(appReq, (resp: ListApplicationsResponse) => {
      setApplications(resp.getResultList());
    });
  }, [props]);

  const onFinish = (values: TenantUser.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);

    const tu = new TenantUser();
    tu.setEmail(v.email);
    tu.setIsAdmin(v.isAdmin);
    tu.setIsGatewayAdmin(v.isGatewayAdmin);
    tu.setIsDeviceAdmin(v.isDeviceAdmin);
    tu.setUserId(v.userId);
    tu.setTenantId(v.tenantId);

    // device-profiles
    for (const d of v.deviceProfilesList) {
      const dp = new TenantUserDeviceProfile();
      dp.setDeviceProfileId(d.deviceProfileId);
      tu.addDeviceProfiles(dp);
    }

    // applications
    for (const a of v.applicationsList) {
      const app = new TenantUserApplication();
      app.setApplicationId(a.applicationId);
      app.setIsReadOnly(a.isReadOnly);
      tu.addApplications(app);
    }

    props.onFinish(tu);
  };

  const onIsAdminChange = (checked: boolean) => {
    setIsAdmin(checked);
  };

  const deviceProfileOptions: SelectProps["options"] = deviceProfiles.map(v => {
    return {
      label: v.getName(),
      value: v.getId(),
    };
  });

  const applicationOptions: SelectProps["options"] = applications.map(v => {
    return {
      label: v.getName(),
      value: v.getId(),
    };
  });

  const tabItems: TabsProps["items"] = [
    {
      key: "1",
      label: "General",
      children: (
        <>
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
                  tooltip="A gateway admin user is able to add and modify gateways part of the tenant. Gateways are readable to all users."
                  name="isGatewayAdmin"
                  valuePropName="checked"
                >
                  <Switch disabled={props.disabled} />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item
                  label="User is device admin"
                  tooltip="A device admin user is able to read, add and modify device-profiles, applications and everything under an application. Device-profiles are readable by all users. Applications and eveything under it are not readable unless explicity given access to non device-admins."
                  name="isDeviceAdmin"
                  valuePropName="checked"
                >
                  <Switch disabled={props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          )}
        </>
      ),
    },
    {
      key: "2",
      label: "Device-profiles",
      children: (
        <Space orientation="vertical" size="large">
          <Card variant="borderless">
            <p>
              By default, tenant-users are able to access device-profiles, without the option to edit / delete. To
              assign edit permissions, add one or multiple device-profiles below to the tenant-user. Please note that
              unless a tenant-user is a tenant-admin or device-admin, it can not delete device-profiles.
            </p>
          </Card>
          <Form.List name="deviceProfilesList">
            {(fields, { add, remove }) => (
              <>
                {fields.map(({ key, name, ...restField }) => (
                  <Row gutter={24} key={key}>
                    <Col span={22}>
                      <Form.Item
                        name={[name, "deviceProfileId"]}
                        {...restField}
                        rules={[
                          {
                            required: true,
                            message: "Please select a device profile",
                          },
                        ]}
                      >
                        <Select options={deviceProfileOptions} />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    Add device-profile
                  </Button>
                </Form.Item>
              </>
            )}
          </Form.List>
        </Space>
      ),
    },
    {
      key: "3",
      label: "Applications",
      children: (
        <Space orientation="vertical" size="large">
          <Card variant="borderless">
            <p>
              Unless a tenant-user is a tenant-admin or device-admin, it will not have access to any applications and
              resources part of an application. To assign access, add one or multiple applications to the tenant-user.
              In case of read-only, the tenant-user will not be able to modify the application or resources part of it.
              Please note that unless a tenant-user is a tenant-admin or device-admin, it can not delete applications.
            </p>
          </Card>
          <Form.List name="applicationsList">
            {(fields, { add, remove }) => (
              <>
                {fields.map(({ key, name, ...restField }) => (
                  <Row gutter={24} key={key}>
                    <Col span={16}>
                      <Form.Item
                        name={[name, "applicationId"]}
                        {...restField}
                        rules={[
                          {
                            required: true,
                            message: "Please select an application",
                          },
                        ]}
                      >
                        <Select options={applicationOptions} />
                      </Form.Item>
                    </Col>
                    <Col span={6}>
                      <Form.Item layout="horizontal" name={[name, "isReadOnly"]} label="Read-only" {...restField}>
                        <Switch />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    Add application
                  </Button>
                </Form.Item>
              </>
            )}
          </Form.List>
        </Space>
      ),
    },
  ];

  return (
    <Form
      layout="vertical"
      initialValues={props.initialValues.toObject()}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
    >
      <Tabs items={tabItems} />
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default TenantUserForm;
