import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteIftttIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

class IftttCard extends Component<IProps> {
  onDelete = () => {
    let req = new DeleteIftttIntegrationRequest();
    req.setApplicationId(this.props.application.getId());
    ApplicationStore.deleteIftttIntegration(req, () => {});
  };

  render() {
    let actions: any[] = [];

    if (!!this.props.add) {
      actions = [
        <Link to="integrations/ifttt/create">
          <PlusOutlined />
        </Link>,
      ];
    } else {
      actions = [
        <Link to="integrations/ifttt/edit">
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
          title="IFTTT"
          className="integration-card"
          cover={<img alt="IFTTT" src="/integrations/ifttt.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The IFTTT integration forwards events to the IFTTT Webhooks integration." />
        </Card>
      </Col>
    );
  }
}

export default IftttCard;
