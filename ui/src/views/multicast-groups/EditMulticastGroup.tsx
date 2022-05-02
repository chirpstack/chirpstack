import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  MulticastGroup,
  UpdateMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import MulticastGroupStore from "../../stores/MulticastGroupStore";
import MulticastGroupForm from "./MulticastGroupForm";
import SessionStore from "../../stores/SessionStore";

interface IProps extends RouteComponentProps {
  multicastGroup: MulticastGroup;
  application: Application;
}

class EditMulticastGroup extends Component<IProps> {
  onFinish = (obj: MulticastGroup) => {
    let req = new UpdateMulticastGroupRequest();
    req.setMulticastGroup(obj);

    MulticastGroupStore.update(req, () => {
      this.props.history.push(`../${this.props.multicastGroup.getId()}`);
    });
  };

  render() {
    let disabled = !(
      SessionStore.isAdmin() ||
      SessionStore.isTenantAdmin(this.props.application.getTenantId()) ||
      SessionStore.isTenantDeviceAdmin(this.props.application.getTenantId())
    );

    return (
      <MulticastGroupForm initialValues={this.props.multicastGroup} disabled={disabled} onFinish={this.onFinish} />
    );
  }
}

export default EditMulticastGroup;
