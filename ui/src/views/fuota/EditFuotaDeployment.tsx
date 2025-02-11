import { useNavigate } from "react-router-dom";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { FuotaDeployment } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import { UpdateFuotaDeploymentRequest } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import FuotaStore from "../../stores/FuotaStore";
import FuotaDeploymentForm from "./FuotaDeploymentForm";
import SessionStore from "../../stores/SessionStore";

interface IProps {
  fuotaDeployment: FuotaDeployment;
  application: Application;
  tenant: Tenant;
  disabled?: boolean;
}

function EditFuotaDeployment(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: FuotaDeployment) => {
    const req = new UpdateFuotaDeploymentRequest();
    req.setDeployment(obj);

    FuotaStore.updateDeployment(req, () => {
      navigate(`../${props.fuotaDeployment.getId()}`);
    });
  };

  const disabled =
    props.disabled ||
    !(
      SessionStore.isAdmin() ||
      SessionStore.isTenantAdmin(props.application.getTenantId()) ||
      SessionStore.isTenantDeviceAdmin(props.application.getTenantId())
    );

  return (
    <FuotaDeploymentForm
      initialValues={props.fuotaDeployment}
      disabled={disabled}
      onFinish={onFinish}
      tenant={props.tenant}
      update
    />
  );
}

export default EditFuotaDeployment;
