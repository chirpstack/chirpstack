import { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { ListTenantsRequest, ListTenantsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantStore from "../../stores/TenantStore";
import SessionStore from "../../stores/SessionStore";

class TenantRedirect extends Component<RouteComponentProps> {
  componentDidMount() {
    const tenantId = SessionStore.getTenantId();
    if (tenantId !== "") {
      this.props.history.push(`/tenants/${tenantId}`);
    } else {
      let req = new ListTenantsRequest();
      req.setLimit(1);

      TenantStore.list(req, (resp: ListTenantsResponse) => {
        if (resp.getResultList().length !== 0) {
          this.props.history.push(`/tenants/${resp.getResultList()[0].getId()}`);
        }
      });
    }
  }

  render() {
    return null;
  }
}

export default TenantRedirect;
