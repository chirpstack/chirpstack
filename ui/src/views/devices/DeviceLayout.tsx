import React, { useState, useEffect } from "react";
import { Route, Routes, useParams, Link, useNavigate, useLocation } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, Menu } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  DeviceProfile,
  GetDeviceProfileRequest,
  GetDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  Device,
  GetDeviceRequest,
  GetDeviceResponse,
  DeleteDeviceRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import DeviceStore from "../../stores/DeviceStore";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import Admin from "../../components/Admin";

import DeviceDashboard from "./DeviceDashboard";
import EditDevice from "./EditDevice";
import SetDeviceKeys from "./SetDeviceKeys";
import DeviceFrames from "./DeviceFrames";
import DeviceEvents from "./DeviceEvents";
import DeviceQueue from "./DeviceQueue";
import DeviceActivation from "./DeviceActivation";

interface IProps {
  tenant: Tenant;
  application: Application;
}

function DeviceLayout(props: IProps) {
  const { devEui } = useParams();
  const navigate = useNavigate();
  const location = useLocation();

  const [device, setDevice] = useState<Device | undefined>(undefined);
  const [deviceProfile, setDeviceProfile] = useState<DeviceProfile | undefined>(undefined);
  const [lastSeenAt, setLastSeenAt] = useState<Date | undefined>(undefined);

  useEffect(() => {
    let req = new GetDeviceRequest();
    req.setDevEui(devEui!);

    DeviceStore.get(req, (resp: GetDeviceResponse) => {
      setDevice(resp.getDevice());

      if (resp.getLastSeenAt() !== undefined) {
        setLastSeenAt(resp.getLastSeenAt()!.toDate());
      }

      let req = new GetDeviceProfileRequest();
      req.setId(resp.getDevice()!.getDeviceProfileId());
      DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
        setDeviceProfile(resp.getDeviceProfile());
      });
    });
  }, [devEui]);

  const deleteDevice = () => {
    let req = new DeleteDeviceRequest();
    req.setDevEui(devEui!);

    DeviceStore.delete(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}`);
    });
  };

  const dp = deviceProfile;
  if (!device || !dp) {
    return null;
  }

  const tenant = props.tenant;
  const app = props.application;

  const path = location.pathname;
  let tab = "dashboard";

  if (path.endsWith("edit")) {
    tab = "edit";
  }
  if (path.endsWith("queue")) {
    tab = "queue";
  }
  if (path.endsWith("keys")) {
    tab = "keys";
  }
  if (path.endsWith("activation")) {
    tab = "activation";
  }
  if (path.endsWith("events")) {
    tab = "events";
  }
  if (path.endsWith("frames")) {
    tab = "frames";
  }

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Tenants</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications`}>Applications</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}`}>
                  {props.application.getName()}
                </Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}`}>Devices</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>{device.getName()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={device.getName()}
        subTitle={`device eui: ${device.getDevEui()}`}
        extra={[
          <Admin tenantId={props.tenant.getId()} isDeviceAdmin>
            <DeleteConfirm typ="device" confirm={device.getName()} onConfirm={deleteDevice}>
              <Button danger type="primary">
                Delete device
              </Button>
            </DeleteConfirm>
          </Admin>,
        ]}
      />
      <Card>
        <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
          <Menu.Item key="dashboard">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}`}>
              Dashboard
            </Link>
          </Menu.Item>
          <Menu.Item key="edit">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/edit`}>
              Configuration
            </Link>
          </Menu.Item>
          <Menu.Item key="keys" disabled={!dp.getSupportsOtaa()}>
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/keys`}>
              OTAA keys
            </Link>
          </Menu.Item>
          <Menu.Item key="activation">
            <Link
              to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/activation`}
            >
              Activation
            </Link>
          </Menu.Item>
          <Menu.Item key="queue">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/queue`}>
              Queue
            </Link>
          </Menu.Item>
          <Menu.Item key="events">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/events`}>
              Events
            </Link>
          </Menu.Item>
          <Menu.Item key="frames">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/devices/${device.getDevEui()}/frames`}>
              LoRaWAN frames
            </Link>
          </Menu.Item>
        </Menu>
        <Routes>
          <Route path="/" element={<DeviceDashboard device={device} lastSeenAt={lastSeenAt} deviceProfile={dp} />} />
          <Route path="/edit" element={<EditDevice device={device} application={app} tenant={tenant} />} />
          <Route
            path="/keys"
            element={<SetDeviceKeys device={device} application={app} tenant={tenant} deviceProfile={dp} />}
          />
          <Route path="/frames" element={<DeviceFrames device={device} />} />
          <Route path="/events" element={<DeviceEvents device={device} />} />
          <Route path="/queue" element={<DeviceQueue device={device} />} />
          <Route
            path="/activation"
            element={<DeviceActivation device={device} deviceProfile={dp} tenant={tenant} application={app} />}
          />
        </Routes>
      </Card>
    </Space>
  );
}

export default DeviceLayout;
