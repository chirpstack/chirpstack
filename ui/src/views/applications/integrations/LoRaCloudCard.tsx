import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteLoraCloudIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

class LoRaCloudCard extends Component<IProps> {
  onDelete = () => {
    let req = new DeleteLoraCloudIntegrationRequest();
    req.setApplicationId(this.props.application.getId());
    ApplicationStore.deleteLoraCloudIntegration(req, () => {});
  };

  render() {
    let actions: any[] = [];

    if (!!this.props.add) {
      actions = [
        <Link to="integrations/loracloud/create">
          <PlusOutlined />
        </Link>,
      ];
    } else {
      actions = [
        <Link to="integrations/loracloud/edit">
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
          title="Semtech LoRa Cloud&trade;"
          className="integration-card"
          cover={<img alt="Semtech LoRa Cloud" src="/integrations/loracloud.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The Semtech LoRa Cloud integration provides Modem & Geolocation Services." />
        </Card>
      </Col>
    );
  }
}

export default LoRaCloudCard;
