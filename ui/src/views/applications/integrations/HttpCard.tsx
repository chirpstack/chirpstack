import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteHttpIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

class HttpCard extends Component<IProps> {
  onDelete = () => {
    let req = new DeleteHttpIntegrationRequest();
    req.setApplicationId(this.props.application.getId());
    ApplicationStore.deleteHttpIntegration(req, () => {});
  };

  render() {
    let actions: any[] = [];

    if (!!this.props.add) {
      actions = [
        <Link to="integrations/http/create">
          <PlusOutlined />
        </Link>,
      ];
    } else {
      actions = [
        <Link to="integrations/http/edit">
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
          title="HTTP"
          className="integration-card"
          cover={<img alt="HTTP" src="/integrations/http.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The HTTP integration forwards events to a user-configurable endpoint as POST requests." />
        </Card>
      </Col>
    );
  }
}

export default HttpCard;
