import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Application, UpdateApplicationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../stores/ApplicationStore";
import ApplicationForm from "./ApplicationForm";
import SessionStore from "../../stores/SessionStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class EditApplication extends Component<IProps> {
  onFinish = (obj: Application) => {
    let req = new UpdateApplicationRequest();
    req.setApplication(obj);

    ApplicationStore.update(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}`,
      );
    });
  };

  render() {
    const disabled = !(
      SessionStore.isAdmin() ||
      SessionStore.isTenantAdmin(this.props.application.getTenantId()) ||
      SessionStore.isTenantDeviceAdmin(this.props.application.getTenantId())
    );

    return <ApplicationForm initialValues={this.props.application} disabled={disabled} onFinish={this.onFinish} />;
  }
}

export default EditApplication;
