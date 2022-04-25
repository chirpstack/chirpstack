import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import {
  Application,
  DeleteAzureServiceBusIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

class AzureServiceBusCard extends Component<IProps> {
  onDelete = () => {
    let req = new DeleteAzureServiceBusIntegrationRequest();
    req.setApplicationId(this.props.application.getId());
    ApplicationStore.deleteAzureServiceBusIntegration(req, () => {});
  };

  render() {
    let actions: any[] = [];

    if (!!this.props.add) {
      actions = [
        <Link to="integrations/azure-service-bus/create">
          <PlusOutlined />
        </Link>,
      ];
    } else {
      actions = [
        <Link to="integrations/azure-service-bus/edit">
          <EditOutlined />
        </Link>,
        <Popconfirm title="Are you sure you want to delete this integration?" onConfirm={this.onDelete}>
          <DeleteOutlined />
        </Popconfirm>,
      ];
    }

    return (
      <Col span={8}>
        <Card
          title="Azure Service-Bus"
          className="integration-card"
          cover={<img alt="Azure Service-Bus" src="/integrations/azure_service_bus.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The Azure Service-Bus integration forwards events to an Azure Service-Bus topic or queue." />
        </Card>
      </Col>
    );
  }
}

export default AzureServiceBusCard;
