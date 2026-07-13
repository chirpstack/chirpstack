import { useNavigate } from "react-router-dom";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { UpdateDeviceRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import DeviceStore from "../../stores/DeviceStore";
import DeviceForm from "./DeviceForm";
import SessionStore from "../../stores/SessionStore";

interface IProps {
  tenant: Tenant;
  application: Application;
  device: Device;
}

function EditDevice(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: Device) => {
    const req = new UpdateDeviceRequest();
    req.setDevice(obj);

    DeviceStore.update(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/devices/${obj.getDevEui()}`);
    });
  };

  const disabled = !(
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.tenant.getId()) ||
    SessionStore.isTenantDeviceAdmin(props.tenant.getId()) ||
    SessionStore.isApplicationAdmin(props.device.getApplicationId())
  );

  return (
    <DeviceForm initialValues={props.device} onFinish={onFinish} tenant={props.tenant} disabled={disabled} update />
  );
}

export default EditDevice;
