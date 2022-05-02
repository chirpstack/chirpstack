import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  MyDevicesIntegration,
  CreateMyDevicesIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import MyDevicesIntegrationForm from "./MyDevicesIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreateMyDevicesIntegration extends Component<IProps> {
  onFinish = (obj: MyDevicesIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateMyDevicesIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createMyDevicesIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    const i = new MyDevicesIntegration();

    return (
      <Card title="Add myDevices integration">
        <MyDevicesIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateMyDevicesIntegration;
