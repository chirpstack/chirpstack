import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteAwsSnsIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

class AwsSns extends Component<IProps> {
  onDelete = () => {
    let req = new DeleteAwsSnsIntegrationRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.deleteAwsSnsIntegration(req, () => {});
  };

  render() {
    let actions: any[] = [];

    if (!!this.props.add) {
      actions = [
        <Link to="integrations/aws-sns/create">
          <PlusOutlined />
        </Link>,
      ];
    } else {
      actions = [
        <Link to="integrations/aws-sns/edit">
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
          title="AWS SNS"
          className="integration-card"
          cover={<img alt="AWS SNS" src="/integrations/aws_sns.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The AWS SNS integration forwards events to an AWS SNS topic." />
        </Card>
      </Col>
    );
  }
}

export default AwsSns;
