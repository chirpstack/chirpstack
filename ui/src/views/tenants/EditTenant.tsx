import { useNavigate } from "react-router";

import { Tenant, UpdateTenantRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantStore from "../../stores/TenantStore";
import TenantForm from "./TenantForm";
import SessionStore from "../../stores/SessionStore";

function EditTenant({ tenant }: { tenant: Tenant }) {
  const navigate = useNavigate();

  const onFinish = (obj: Tenant) => {
    let req = new UpdateTenantRequest();
    req.setTenant(obj);

    TenantStore.update(req, () => {
      navigate("/tenants/" + obj.getId());
    });
  };

  const disabled = !SessionStore.isAdmin();

  return <TenantForm initialValues={tenant} onFinish={onFinish} disabled={disabled} />;
}

export default EditTenant;
