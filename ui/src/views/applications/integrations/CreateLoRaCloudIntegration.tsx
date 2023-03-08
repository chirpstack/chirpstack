import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  LoraCloudIntegration,
  LoraCloudModemGeolocationServices,
  CreateLoraCloudIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import LoRaCloudIntegrationForm from "./LoRaCloudIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  application: Application;
}

class CreateLoRaCloudIntegration extends Component<IProps> {
  onFinish = (obj: LoraCloudIntegration) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateLoraCloudIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createLoraCloudIntegration(req, () => {
      this.props.history.push(
        `/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/integrations`,
      );
    });
  };

  render() {
    let i = new LoraCloudIntegration();
    let mgs = new LoraCloudModemGeolocationServices();
    mgs.setModemEnabled(true);
    mgs.setForwardFPortsList([192, 197, 198, 199]);

    i.setModemGeolocationServices(mgs);

    return (
      <Card title="Add Semtech LoRa Cloud&trade; integration">
        <LoRaCloudIntegrationForm initialValues={i} onFinish={this.onFinish} />
      </Card>
    );
  }
}

export default CreateLoRaCloudIntegration;
