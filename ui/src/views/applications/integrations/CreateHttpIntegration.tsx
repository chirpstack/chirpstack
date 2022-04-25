import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  HttpIntegration,
  CreateHttpIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import HttpIntegrationForm from "./HttpIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreateHttpIntegration extends Component<IProps> {
  onFinish = (obj: HttpIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateHttpIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createHttpIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new HttpIntegration();

    return (
      <Card title="Add HTTP integration">
        <HttpIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateHttpIntegration;
