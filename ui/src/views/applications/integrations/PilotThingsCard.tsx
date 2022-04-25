import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import {
  Application,
  DeletePilotThingsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

class PilotThingsCard extends Component<IProps> {
  onDelete = () => {
    let req = new DeletePilotThingsIntegrationRequest();
    req.setApplicationId(this.props.application.getId());
    ApplicationStore.deletePilotThingsIntegration(req, () => {});
  };

  render() {
    let actions: any[] = [];

    if (!!this.props.add) {
      actions = [
        <Link to="integrations/pilot-things/create">
          <PlusOutlined />
        </Link>,
      ];
    } else {
      actions = [
        <Link to="integrations/pilot-things/edit">
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
          title="Pilot Things"
          className="integration-card"
          cover={<img alt="Pilot Things" src="/integrations/pilot_things.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The Pilot Things integration forwards messages to a Pilot Things instance." />
        </Card>
      </Col>
    );
  }
}

export default PilotThingsCard;
