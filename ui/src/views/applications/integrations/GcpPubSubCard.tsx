import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteGcpPubSubIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

class GcpPubSubCard extends Component<IProps> {
  onDelete = () => {
    let req = new DeleteGcpPubSubIntegrationRequest();
    req.setApplicationId(this.props.application.getId());
    ApplicationStore.deleteGcpPubSubIntegration(req, () => {});
  };

  render() {
    let actions: any[] = [];

    if (!!this.props.add) {
      actions = [
        <Link to="integrations/gcp-pub-sub/create">
          <PlusOutlined />
        </Link>,
      ];
    } else {
      actions = [
        <Link to="integrations/gcp-pub-sub/edit">
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
          title="GCP Pub/Sub"
          className="integration-card"
          cover={<img alt="GCP Pub/Sub" src="/integrations/gcp_pubsub.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The Google Cloud Pub/Sub integration forwards events to a GCP Pub/Sub topic." />
        </Card>
      </Col>
    );
  }
}

export default GcpPubSubCard;
