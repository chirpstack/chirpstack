import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  GcpPubSubIntegration,
  GetGcpPubSubIntegrationRequest,
  GetGcpPubSubIntegrationResponse,
  UpdateGcpPubSubIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import GcpPubSubIntegrationForm from "./GcpPubSubIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: GcpPubSubIntegration;
}

class EditGcpPubSubIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetGcpPubSubIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getGcpPubSubIntegration(req, (resp: GetGcpPubSubIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: GcpPubSubIntegration) => {
    let req = new UpdateGcpPubSubIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateGcpPubSubIntegration(req, () => {
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
      <Card title="Update GCP Pub/Sub integration">
        <GcpPubSubIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditGcpPubSubIntegration;
