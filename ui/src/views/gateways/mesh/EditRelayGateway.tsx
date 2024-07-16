import { useNavigate } from "react-router-dom";

import type { RelayGateway } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { UpdateRelayGatewayRequest } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import RelayGatewayForm from "./RelayGatewayForm";
import GatewayStore from "../../../stores/GatewayStore";
import SessionStore from "../../../stores/SessionStore";

interface IProps {
  relayGateway: RelayGateway;
}

function EditRelayGateway(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: RelayGateway) => {
    const req = new UpdateRelayGatewayRequest();
    req.setRelayGateway(obj);

    GatewayStore.updateRelayGateway(req, () => {
      navigate(`/tenants/${obj.getTenantId()}/gateways/mesh/relays`);
    });
  };

  const disabled = !(
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.relayGateway.getTenantId()) ||
    SessionStore.isTenantGatewayAdmin(props.relayGateway.getTenantId())
  );

  return <RelayGatewayForm initialValues={props.relayGateway} onFinish={onFinish} disabled={disabled} update />;
}

export default EditRelayGateway;
