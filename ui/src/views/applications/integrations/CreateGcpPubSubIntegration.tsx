import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  GcpPubSubIntegration,
  CreateGcpPubSubIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import GcpPubSubIntegrationForm from "./GcpPubSubIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreateGcpPubSubIntegration extends Component<IProps> {
  onFinish = (obj: GcpPubSubIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateGcpPubSubIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createGcpPubSubIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new GcpPubSubIntegration();

    return (
      <Card title="Add GCP Pub/Sub integration">
        <GcpPubSubIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateGcpPubSubIntegration;
