import { useNavigate } from "react-router-dom";

import { Gateway, UpdateGatewayRequest } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import GatewayForm from "./GatewayForm";
import GatewayStore from "../../stores/GatewayStore";
import SessionStore from "../../stores/SessionStore";

interface IProps {
  gateway: Gateway;
}

function EditGateway(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: Gateway) => {
    let req = new UpdateGatewayRequest();
    req.setGateway(obj);

    GatewayStore.update(req, () => {
      navigate(`/tenants/${obj.getTenantId()}/gateways/${obj.getGatewayId()}`);
    });
  };

  const disabled = !(
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.gateway.getTenantId()) ||
    SessionStore.isTenantGatewayAdmin(props.gateway.getTenantId())
  );
  return <GatewayForm initialValues={props.gateway} onFinish={onFinish} disabled={disabled} update />;
}

export default EditGateway;
