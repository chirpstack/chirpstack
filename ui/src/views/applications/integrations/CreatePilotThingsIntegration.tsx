import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  PilotThingsIntegration,
  CreatePilotThingsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import PilotThingsIntegrationForm from "./PilotThingsIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreatePilotThingsIntegration extends Component<IProps> {
  onFinish = (obj: PilotThingsIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreatePilotThingsIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createPilotThingsIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new PilotThingsIntegration();

    return (
      <Card title="Add Pilot Things integration">
        <PilotThingsIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreatePilotThingsIntegration;
