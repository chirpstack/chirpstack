import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  ThingsBoardIntegration,
  GetThingsBoardIntegrationRequest,
  GetThingsBoardIntegrationResponse,
  UpdateThingsBoardIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ThingsBoardIntegrationForm from "./ThingsBoardIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: ThingsBoardIntegration;
}

class EditThingsBoardIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetThingsBoardIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getThingsBoardIntegration(req, (resp: GetThingsBoardIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: ThingsBoardIntegration) => {
    let req = new UpdateThingsBoardIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateThingsBoardIntegration(req, () => {
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
      <Card title="Update ThingsBoard integration">
        <ThingsBoardIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditThingsBoardIntegration;
