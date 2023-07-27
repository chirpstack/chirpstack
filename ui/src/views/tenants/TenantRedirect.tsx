import { useNavigate } from "react-router";

import { ListTenantsRequest, ListTenantsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantStore from "../../stores/TenantStore";
import SessionStore from "../../stores/SessionStore";

function TenantRedirect() {
  const navigate = useNavigate();

  const tenantId = SessionStore.getTenantId();

  if (tenantId !== "") {
    navigate(`/tenants/${tenantId}`);
  } else {
    let req = new ListTenantsRequest();
    req.setLimit(1);

    TenantStore.list(req, (resp: ListTenantsResponse) => {
      if (resp.getResultList().length !== 0) {
        navigate(`/tenants/${resp.getResultList()[0].getId()}`);
      }
    });
  }

  return null;
}

export default TenantRedirect;
