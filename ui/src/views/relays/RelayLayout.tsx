import { useState, useEffect } from "react";
import { Route, Routes, useParams, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Menu } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Device, GetDeviceResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { GetDeviceRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import DeviceStore from "../../stores/DeviceStore";
import ListRelayDevices from "./ListRelayDevices";
import { useTitle } from "../helpers";

interface IProps {
  tenant: Tenant;
  application: Application;
}

function RelayLayout(props: IProps) {
  const [relayDevice, setRelayDevice] = useState<Device | undefined>(undefined);
  const { relayDevEui } = useParams();
  useTitle(
    "Tenants",
    props.tenant.getName(),
    "Applications",
    props.application.getName(),
    "Relays",
    relayDevice?.getName(),
  );

  useEffect(() => {
    const req = new GetDeviceRequest();
    req.setDevEui(relayDevEui!);

    DeviceStore.get(req, (resp: GetDeviceResponse) => {
      setRelayDevice(resp.getDevice());
    });
  }, [relayDevEui]);

  const tenant = props.tenant;
  const app = props.application;
  const rd = relayDevice;

  if (!rd) {
    return null;
  }

  const tab = "devices";

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Building</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${tenant.getId()}`}>{tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${tenant.getId()}/applications`}>Applications</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}`}>{app.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/relays`}>Relays</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>{rd.getName()}</Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={rd.getName()}
        subTitle={`relay DevEUI: ${rd.getDevEui()}`}
      />
      <Card>
        <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
          <Menu.Item key="devices">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/relays/${rd.getDevEui()}`}>Devices</Link>
          </Menu.Item>
        </Menu>
        <Routes>
          <Route path="/" element={<ListRelayDevices relayDevice={rd} />} />
        </Routes>
      </Card>
    </Space>
  );
}

export default RelayLayout;
