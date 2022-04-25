import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  InfluxDbIntegration,
  GetInfluxDbIntegrationRequest,
  GetInfluxDbIntegrationResponse,
  UpdateInfluxDbIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import InfluxDbIntegrationForm from "./InfluxDbIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: InfluxDbIntegration;
}

class EditInfluxDbIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetInfluxDbIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getInfluxDbIntegration(req, (resp: GetInfluxDbIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: InfluxDbIntegration) => {
    let req = new UpdateInfluxDbIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateInfluxDbIntegration(req, () => {
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
      <Card title="Update InfluxDB integration">
        <InfluxDbIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditInfluxDbIntegration;
