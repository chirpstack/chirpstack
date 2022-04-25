import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  HttpIntegration,
  GetHttpIntegrationRequest,
  GetHttpIntegrationResponse,
  UpdateHttpIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import HttpIntegrationForm from "./HttpIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: HttpIntegration;
}

class EditHttpIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetHttpIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getHttpIntegration(req, (resp: GetHttpIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: HttpIntegration) => {
    let req = new UpdateHttpIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateHttpIntegration(req, () => {
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
      <Card title="Update HTTP integration">
        <HttpIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditHttpIntegration;
