import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  IftttIntegration,
  CreateIftttIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import IftttIntegrationForm from "./IftttIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
  measurementKeys: string[];
}

class CreateIftttIntegration extends Component<IProps> {
  onFinish = (obj: IftttIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateIftttIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createIftttIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new IftttIntegration();
    i.setUplinkValuesList(["", ""]);

    return (
      <Card title="Add IFTTT integration">
        <IftttIntegrationForm measurementKeys={this.props.measurementKeys} initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateIftttIntegration;
