import { useState, useEffect } from "react";

import { Route, Routes, Link, useParams, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { RelayGateway, GetRelayGatewayResponse } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { GetRelayGatewayRequest, DeleteRelayGatewayRequest } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import Admin from "../../../components/Admin";
import SessionStore from "../../../stores/SessionStore";
import GatewayStore from "../../../stores/GatewayStore";
import DeleteConfirm from "../../../components/DeleteConfirm";

import EditRelayGateway from "./EditRelayGateway";
import { useTitle } from "../../helpers";

interface IProps {
  tenant: Tenant;
}

function RelayGatewayLayout(props: IProps) {
  const { relayId } = useParams();
  const navigate = useNavigate();
  const [relayGateway, setRelayGateway] = useState<RelayGateway | undefined>(undefined);
  useTitle("Tenants", props.tenant.getName(), "Gateway Mesh", "Relay Gateways", relayGateway?.getName());

  useEffect(() => {
    const req = new GetRelayGatewayRequest();
    req.setTenantId(props.tenant.getId());
    req.setRelayId(relayId!);

    GatewayStore.getRelayGateway(req, (resp: GetRelayGatewayResponse) => {
      setRelayGateway(resp.getRelayGateway());
    });
  }, [props, relayId]);

  const deleteRelayGateway = () => {
    const req = new DeleteRelayGatewayRequest();
    req.setTenantId(props.tenant.getId());
    req.setRelayId(relayId!);

    GatewayStore.deleteRelayGateway(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/gateways/mesh/relays`);
    });
  };

  if (!relayGateway) {
    return null;
  }

  const isGatewayAdmin =
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.tenant.getId()) ||
    SessionStore.isTenantGatewayAdmin(props.tenant.getId());

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb
            items={[
              { title: "Tenants" },
              { title: <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link> },
              { title: "Gateway Mesh" },
              { title: <Link to={`/tenants/${props.tenant.getId()}/gateways/mesh/relays`}>Relay Gateways</Link> },
              { title: relayGateway.getName() },
            ]}
          />
        )}
        title={relayGateway.getName()}
        subTitle={`relay id: ${relayGateway.getRelayId()}`}
        extra={[
          <Admin tenantId={props.tenant.getId()} isGatewayAdmin={isGatewayAdmin} key="delete-relay-gateway">
            <DeleteConfirm confirm={relayGateway.getName()} typ="relay gateway" onConfirm={deleteRelayGateway}>
              <Button danger type="primary">
                Delete Relay Gateway
              </Button>
            </DeleteConfirm>
          </Admin>,
        ]}
      />
      <Card>
        <Routes>
          <Route path="/edit" element={<EditRelayGateway relayGateway={relayGateway} />} />
        </Routes>
      </Card>
    </Space>
  );
}

export default RelayGatewayLayout;
