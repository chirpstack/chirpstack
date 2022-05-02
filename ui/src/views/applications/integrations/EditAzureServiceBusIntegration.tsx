import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  AzureServiceBusIntegration,
  GetAzureServiceBusIntegrationRequest,
  GetAzureServiceBusIntegrationResponse,
  UpdateAzureServiceBusIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import AzureServiceBusIntegrationForm from "./AzureServiceBusIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: AzureServiceBusIntegration;
}

class EditAzureServiceBusIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetAzureServiceBusIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getAzureServiceBusIntegration(req, (resp: GetAzureServiceBusIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: AzureServiceBusIntegration) => {
    let req = new UpdateAzureServiceBusIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateAzureServiceBusIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    if (this.state.integration === undefined) {
      return null;
    }

    return (
      <Card title="Update Azure Service-Bus integration">
        <AzureServiceBusIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditAzureServiceBusIntegration;
