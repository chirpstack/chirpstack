import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  PilotThingsIntegration,
  GetPilotThingsIntegrationRequest,
  GetPilotThingsIntegrationResponse,
  UpdatePilotThingsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import PilotThingsIntegrationForm from "./PilotThingsIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: PilotThingsIntegration;
}

class EditPilotThingsIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetPilotThingsIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getPilotThingsIntegration(req, (resp: GetPilotThingsIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: PilotThingsIntegration) => {
    let req = new UpdatePilotThingsIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updatePilotThingsIntegration(req, () => {
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
      <Card title="Update Pilot Things integration">
        <PilotThingsIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditPilotThingsIntegration;
