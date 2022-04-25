import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  ThingsBoardIntegration,
  CreateThingsBoardIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ThingsBoardIntegrationForm from "./ThingsBoardIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreateThingsBoardIntegration extends Component<IProps> {
  onFinish = (obj: ThingsBoardIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateThingsBoardIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createThingsBoardIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new ThingsBoardIntegration();

    return (
      <Card title="Add ThingsBoard integration">
        <ThingsBoardIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateThingsBoardIntegration;
