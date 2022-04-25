import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  MyDevicesIntegration,
  GetMyDevicesIntegrationRequest,
  GetMyDevicesIntegrationResponse,
  UpdateMyDevicesIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import MyDevicesIntegrationForm from "./MyDevicesIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

interface IState {
  integration?: MyDevicesIntegration;
}

class EditMyDevicesIntegration extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetMyDevicesIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.getMyDevicesIntegration(req, (resp: GetMyDevicesIntegrationResponse) => {
      this.setState({
        integration: resp.getIntegration(),
      });
    });
  }

  onFinish = (obj: MyDevicesIntegration) => {
    let req = new UpdateMyDevicesIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateMyDevicesIntegration(req, () => {
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
      <Card title="Update myDevices integration">
        <MyDevicesIntegrationForm initialValues={this.state.integration} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default EditMyDevicesIntegration;
