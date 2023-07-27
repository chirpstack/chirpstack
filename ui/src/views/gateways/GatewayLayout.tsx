import React, { useState, useEffect } from "react";

import { Route, Routes, Link, useParams, useNavigate, useLocation } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, Menu } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import {
  Gateway,
  GetGatewayRequest,
  GetGatewayResponse,
  DeleteGatewayRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import GatewayStore from "../../stores/GatewayStore";
import DeleteConfirm from "../../components/DeleteConfirm";

import GatewayDashboard from "./GatewayDashboard";
import EditGateway from "./EditGateway";
import GatewayFrames from "./GatewayFrames";
import GatewayCertificate from "./GatewayCertificate";
import Admin from "../../components/Admin";
import SessionStore from "../../stores/SessionStore";

interface IProps {
  tenant: Tenant;
}

function GatewayLayout(props: IProps) {
  const { gatewayId } = useParams();
  const navigate = useNavigate();
  const location = useLocation();
  const [gateway, setGateway] = useState<Gateway | undefined>(undefined);
  const [lastSeenAt, setLastSeenAt] = useState<Date | undefined>(undefined);

  useEffect(() => {
    let req = new GetGatewayRequest();
    req.setGatewayId(gatewayId!);

    GatewayStore.get(req, (resp: GetGatewayResponse) => {
      setGateway(resp.getGateway());

      if (resp.getLastSeenAt() !== undefined) {
        setLastSeenAt(resp.getLastSeenAt()!.toDate());
      }
    });
  }, [props, gatewayId]);

  const deleteGateway = () => {
    let req = new DeleteGatewayRequest();
    req.setGatewayId(gatewayId!);

    GatewayStore.delete(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/gateways`);
    });
  };

  const tenant = props.tenant;
  const gw = gateway;
  if (!gw) {
    return null;
  }

  const path = location.pathname;
  let tab = "dashboard";

  if (path.endsWith("/edit")) {
    tab = "edit";
  }
  if (path.endsWith("/certificate")) {
    tab = "cert";
  }
  if (path.endsWith("/frames")) {
    tab = "frames";
  }

  let isGatewayAdmin =
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.tenant.getId()) ||
    SessionStore.isTenantGatewayAdmin(props.tenant.getId());

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
                <Link to={`/tenants/${props.tenant.getId()}/gateways`}>Gateways</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>{gw.getName()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={gw.getName()}
        subTitle={`gateway id: ${gw.getGatewayId()}`}
        extra={[
          <Admin tenantId={props.tenant.getId()} isGatewayAdmin>
            <DeleteConfirm confirm={gw.getName()} typ="gateway" onConfirm={deleteGateway}>
              <Button danger type="primary">
                Delete gateway
              </Button>
            </DeleteConfirm>
          </Admin>,
        ]}
      />
      <Card>
        <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
          <Menu.Item key="dashboard">
            <Link to={`/tenants/${tenant.getId()}/gateways/${gw.getGatewayId()}`}>Dashboard</Link>
          </Menu.Item>
          <Menu.Item key="edit">
            <Link to={`/tenants/${tenant.getId()}/gateways/${gw.getGatewayId()}/edit`}>Configuration</Link>
          </Menu.Item>
          {isGatewayAdmin && (
            <Menu.Item key="cert">
              <Link to={`/tenants/${tenant.getId()}/gateways/${gw.getGatewayId()}/certificate`}>TLS certificate</Link>
            </Menu.Item>
          )}
          <Menu.Item key="frames">
            <Link to={`/tenants/${tenant.getId()}/gateways/${gw.getGatewayId()}/frames`}>LoRaWAN frames</Link>
          </Menu.Item>
        </Menu>
        <Routes>
          <Route path="/" element={<GatewayDashboard gateway={gw} lastSeenAt={lastSeenAt} />} />
          <Route path="/edit" element={<EditGateway gateway={gw} />} />
          <Route path="/certificate" element={<GatewayCertificate gateway={gw} />} />
          <Route path="/frames" element={<GatewayFrames gateway={gw} />} />
        </Routes>
      </Card>
    </Space>
  );
}

export default GatewayLayout;
