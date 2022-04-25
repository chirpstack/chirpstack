import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Tenant, UpdateTenantRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantStore from "../../stores/TenantStore";
import TenantForm from "./TenantForm";
import SessionStore from "../../stores/SessionStore";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
}

interface IState {}

class EditTenant extends Component<IProps, IState> {
  onFinish = (obj: Tenant) => {
    let req = new UpdateTenantRequest();
    req.setTenant(obj);

    TenantStore.update(req, () => {
      this.props.history.push("/tenants/" + obj.getId());
    });
  };

  render() {
    const disabled = !SessionStore.isAdmin();

    return <TenantForm initialValues={this.props.tenant} onFinish={this.onFinish} disabled={disabled} />;
  }
}

export default EditTenant;
