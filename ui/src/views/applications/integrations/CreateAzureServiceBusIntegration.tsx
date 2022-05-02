import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  AzureServiceBusIntegration,
  CreateAzureServiceBusIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import AzureServiceBusIntegrationForm from "./AzureServiceBusIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreateAzureServiceBusIntegration extends Component<IProps> {
  onFinish = (obj: AzureServiceBusIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateAzureServiceBusIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createAzureServiceBusIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new AzureServiceBusIntegration();

    return (
      <Card title="Add Azure Service-Bus integration">
        <AzureServiceBusIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateAzureServiceBusIntegration;
