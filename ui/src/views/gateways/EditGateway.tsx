import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Gateway, UpdateGatewayRequest } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import GatewayForm from "./GatewayForm";
import GatewayStore from "../../stores/GatewayStore";
import SessionStore from "../../stores/SessionStore";

interface IProps extends RouteComponentProps {
  gateway: Gateway;
}

interface IState {}

class EditGateway extends Component<IProps, IState> {
  onFinish = (obj: Gateway) => {
    let req = new UpdateGatewayRequest();
    req.setGateway(obj);

    GatewayStore.update(req, () => {
      this.props.history.push(`/tenants/${obj.getTenantId()}/gateways/${obj.getGatewayId()}`);
    });
  };

  render() {
    const disabled = !(
      SessionStore.isAdmin() ||
      SessionStore.isTenantAdmin(this.props.gateway.getTenantId()) ||
      SessionStore.isTenantGatewayAdmin(this.props.gateway.getTenantId())
    );
    return <GatewayForm initialValues={this.props.gateway} onFinish={this.onFinish} disabled={disabled} update />;
  }
}

export default EditGateway;
