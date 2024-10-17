import { useState, useEffect } from "react";
import { Route, Routes, useNavigate, useParams, useLocation, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, Menu } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type {
  GetMulticastGroupResponse,
  MulticastGroup,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  GetMulticastGroupRequest,
  DeleteMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import MulticastGroupStore from "../../stores/MulticastGroupStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import ListMulticastGroupDevices from "./ListMulticastGroupDevices";
import ListMulticastGroupGateways from "./ListMulticastGroupGateways";
import EditMulticastGroup from "./EditMulticastGroup";
import Admin from "../../components/Admin";
import MulticastGroupQueue from "./MulticastGroupQueue";
import { useTitle } from "../helpers";

interface IProps {
  tenant: Tenant;
  application: Application;
}

function MulticastGroupLayout(props: IProps) {
  const { multicastGroupId } = useParams();
  const navigate = useNavigate();
  const location = useLocation();
  const [multicastGroup, setMulticastGroup] = useState<MulticastGroup | undefined>(undefined);
  useTitle(
    "Tenants",
    props.tenant.getName(),
    "Applications",
    props.application.getName(),
    "Multicast-groups",
    multicastGroup?.getName(),
  );

  useEffect(() => {
    const req = new GetMulticastGroupRequest();
    req.setId(multicastGroupId!);

    MulticastGroupStore.get(req, (resp: GetMulticastGroupResponse) => {
      setMulticastGroup(resp.getMulticastGroup());
    });
  }, [multicastGroupId]);

  const deleteMulticastGroup = () => {
    const req = new DeleteMulticastGroupRequest();
    req.setId(multicastGroupId!);

    MulticastGroupStore.delete(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/multicast-groups`);
    });
  };

  const tenant = props.tenant;
  const app = props.application;
  const mg = multicastGroup;

  if (!mg) {
    return null;
  }

  let tab = "devices";

  const path = location.pathname;
  if (path.endsWith("gateways")) {
    tab = "gateways";
  }
  if (path.endsWith("edit")) {
    tab = "edit";
  }
  if (path.endsWith("queue")) {
    tab = "queue";
  }

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
                <Link to={`/tenants/${props.tenant.getId()}/applications/${app.getId()}`}>{app.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications/${app.getId()}/multicast-groups`}>
                  Multicast-groups
                </Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>{mg.getName()}</Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={mg.getName()}
        subTitle={`multicast-group id: ${mg.getId()}`}
        extra={[
          <Admin tenantId={tenant.getId()} isDeviceAdmin>
            <DeleteConfirm typ="multicast-group" confirm={mg.getName()} onConfirm={deleteMulticastGroup}>
              <Button danger type="primary">
                Delete multicast-group
              </Button>
            </DeleteConfirm>
          </Admin>,
        ]}
      />
      <Card>
        <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
          <Menu.Item key="devices">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}`}>
              Devices
            </Link>
          </Menu.Item>
          <Menu.Item key="gateways">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}/gateways`}>
              Gateways
            </Link>
          </Menu.Item>
          <Menu.Item key="edit">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}/edit`}>
              Configuration
            </Link>
          </Menu.Item>
          <Menu.Item key="queue">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}/queue`}>
              Queue
            </Link>
          </Menu.Item>
        </Menu>
        <Routes>
          <Route path="/" element={<ListMulticastGroupDevices multicastGroup={mg} />} />
          <Route path="/gateways" element={<ListMulticastGroupGateways multicastGroup={mg} application={app} />} />
          <Route path="/edit" element={<EditMulticastGroup application={app} multicastGroup={mg} />} />
          <Route path="/queue" element={<MulticastGroupQueue multicastGroup={mg} />} />
        </Routes>
      </Card>
    </Space>
  );
}

export default MulticastGroupLayout;
