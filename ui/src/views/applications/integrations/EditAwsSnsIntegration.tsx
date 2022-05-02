import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  AwsSnsIntegration,
  GetAwsSnsIntegrationRequest,
  GetAwsSnsIntegrationResponse,
  UpdateAwsSnsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import AwsSnsIntegrationForm from "./AwsSnsIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: AwsSnsIntegration;
}

class EditAwsSnsIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetAwsSnsIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getAwsSnsIntegration(req, (resp: GetAwsSnsIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: AwsSnsIntegration) => {
    let req = new UpdateAwsSnsIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateAwsSnsIntegration(req, () => {
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
      <Card title="Update AWS SNS integration">
        <AwsSnsIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditAwsSnsIntegration;
