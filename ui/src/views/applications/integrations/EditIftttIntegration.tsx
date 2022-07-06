import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  IftttIntegration,
  GetIftttIntegrationRequest,
  GetIftttIntegrationResponse,
  UpdateIftttIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import IftttIntegrationForm from "./IftttIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
  measurementKeys: string[];
}

interface IState {
  integration?: IftttIntegration;
}

class EditIftttIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetIftttIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getIftttIntegration(req, (resp: GetIftttIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: IftttIntegration) => {
    let req = new UpdateIftttIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateIftttIntegration(req, () => {
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
      <Card title="Update IFTTT integration">
        <IftttIntegrationForm
          measurementKeys={this.props.measurementKeys}
          initialValues={this.state.integration}
          onFinish={this.onFinish}
        />
      </Card>
    );
  }
}

export default EditIftttIntegration;
