import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  InfluxDbIntegration,
  CreateInfluxDbIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import InfluxDbIntegrationForm from "./InfluxDbIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreateInfluxDbIntegration extends Component<IProps> {
  onFinish = (obj: InfluxDbIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateInfluxDbIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createInfluxDbIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new InfluxDbIntegration();

    return (
      <Card title="Add InfluxDB integration">
        <InfluxDbIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateInfluxDbIntegration;
