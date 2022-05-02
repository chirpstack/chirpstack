import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Col, Card } from "antd";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  application: Application;
}

class HttpCard extends Component<IProps> {
  render() {
    let actions: any[] = [<Link to="integrations/mqtt/certificate">Get certificate</Link>];

    return (
      <Col span={8}>
        <Card
          title="MQTT"
          className="integration-card"
          cover={<img alt="MQTT" src="/integrations/mqtt.png" style={{ padding: 1 }} />}
          actions={actions}
        >
          <Card.Meta description="The MQTT integration forwards events to a MQTT broker." />
        </Card>
      </Col>
    );
  }
}

export default HttpCard;
