import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import {
  Application,
  DeleteThingsBoardIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

class ThingsBoardCard extends Component<IProps> {
  onDelete = () => {
    let req = new DeleteThingsBoardIntegrationRequest();
    req.setApplicationId(this.props.application.getId());
    ApplicationStore.deleteThingsBoardIntegration(req, () => {});
  };

  render() {
    let actions: any[] = [];

    if (!!this.props.add) {
      actions = [
        <Link to="integrations/thingsboard/create">
          <PlusOutlined />
        </Link>,
      ];
    } else {
      actions = [
        <Link to="integrations/thingsboard/edit">
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
          title="ThingsBoard"
          className="integration-card"
          cover={<img alt="ThingsBoard" src="/integrations/thingsboard.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The ThingsBoard integration forwards events to a ThingsBoard instance." />
        </Card>
      </Col>
    );
  }
}

export default ThingsBoardCard;
