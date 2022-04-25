import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  AwsSnsIntegration,
  CreateAwsSnsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import AwsSnsIntegrationForm from "./AwsSnsIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreateAwsSnsIntegration extends Component<IProps> {
  onFinish = (obj: AwsSnsIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateAwsSnsIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createAwsSnsIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new AwsSnsIntegration();

    return (
      <Card title="Add AWS SNS integration">
        <AwsSnsIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateAwsSnsIntegration;
