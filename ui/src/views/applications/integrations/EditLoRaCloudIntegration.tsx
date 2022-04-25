import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  LoraCloudIntegration,
  GetLoraCloudIntegrationRequest,
  GetLoraCloudIntegrationResponse,
  UpdateLoraCloudIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import LoRaCloudIntegrationForm from "./LoRaCloudIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: LoraCloudIntegration;
}

class EditLoRaCloudIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetLoraCloudIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getLoraCloudIntegration(req, (resp: GetLoraCloudIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: LoraCloudIntegration) => {
    let req = new UpdateLoraCloudIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateLoraCloudIntegration(req, () => {
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
      <Card title="Update Semtech LoRa Cloud&trade; integration">
        <LoRaCloudIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditLoRaCloudIntegration;
