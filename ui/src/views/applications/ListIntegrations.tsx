import React, { Component } from "react";

import { Row } from "antd";

import {
  Application,
  ListIntegrationsRequest,
  ListIntegrationsResponse,
  IntegrationListItem,
  IntegrationKind,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../stores/ApplicationStore";
import HttpCard from "./integrations/HttpCard";
import MyDevicesCard from "./integrations/MyDevicesCard";
import MqttCard from "./integrations/MqttCard";
import AwsSnsCard from "./integrations/AwsSnsCard";
import AzureServiceBusCard from "./integrations/AzureServiceBusCard";
import GcpPubSubCard from "./integrations/GcpPubSubCard";
import InfluxdbCard from "./integrations/InfluxdbCard";
import PilotThingsCard from "./integrations/PilotThingsCard";
import LoRaCloudCard from "./integrations/LoRaCloudCard";
import ThingsBoardCard from "./integrations/ThingsBoardCard";
import IftttCard from "./integrations/IftttCard";

interface IProps {
  application: Application;
}

interface IState {
  configured: any[];
  available: any[];
}

class ListIntegrations extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      configured: [],
      available: [],
    };
  }

  componentDidMount() {
    ApplicationStore.on("integration.delete", this.loadIntegrations);
    this.loadIntegrations();
  }

  componentWillUnmount() {
    ApplicationStore.removeAllListeners("integration.delete");
  }

  loadIntegrations = () => {
    let req = new ListIntegrationsRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.listIntegrations(req, (resp: ListIntegrationsResponse) => {
      let configured: any[] = [];
      let available: any[] = [];

      const includes = (integrations: IntegrationListItem[], kind: IntegrationKind) => {
        for (let x of integrations) {
          if (x.getKind() === kind) {
            return true;
          }
        }

        return false;
      };

      // AWS SNS
      if (includes(resp.getResultList(), IntegrationKind.AWS_SNS)) {
        configured.push(<AwsSnsCard application={this.props.application} />);
      } else {
        available.push(<AwsSnsCard application={this.props.application} add />);
      }

      // Azure Service-Bus
      if (includes(resp.getResultList(), IntegrationKind.AZURE_SERVICE_BUS)) {
        configured.push(<AzureServiceBusCard application={this.props.application} />);
      } else {
        available.push(<AzureServiceBusCard application={this.props.application} add />);
      }

      // GCP Pub/Sub
      if (includes(resp.getResultList(), IntegrationKind.GCP_PUB_SUB)) {
        configured.push(<GcpPubSubCard application={this.props.application} />);
      } else {
        available.push(<GcpPubSubCard application={this.props.application} add />);
      }

      // HTTP
      if (includes(resp.getResultList(), IntegrationKind.HTTP)) {
        configured.push(<HttpCard application={this.props.application} />);
      } else {
        available.push(<HttpCard application={this.props.application} add />);
      }

      // IFTTT
      if (includes(resp.getResultList(), IntegrationKind.IFTTT)) {
        configured.push(<IftttCard application={this.props.application} />);
      } else {
        available.push(<IftttCard application={this.props.application} add />);
      }

      // InfluxDB
      if (includes(resp.getResultList(), IntegrationKind.INFLUX_DB)) {
        configured.push(<InfluxdbCard application={this.props.application} />);
      } else {
        available.push(<InfluxdbCard application={this.props.application} add />);
      }

      // MQTT
      if (includes(resp.getResultList(), IntegrationKind.MQTT_GLOBAL)) {
        configured.push(<MqttCard application={this.props.application} />);
      }

      // myDevices
      if (includes(resp.getResultList(), IntegrationKind.MY_DEVICES)) {
        configured.push(<MyDevicesCard application={this.props.application} />);
      } else {
        available.push(<MyDevicesCard application={this.props.application} add />);
      }

      // Pilot Things
      if (includes(resp.getResultList(), IntegrationKind.PILOT_THINGS)) {
        configured.push(<PilotThingsCard application={this.props.application} />);
      } else {
        available.push(<PilotThingsCard application={this.props.application} add />);
      }

      // Semtech LoRa Cloud
      if (includes(resp.getResultList(), IntegrationKind.LORA_CLOUD)) {
        configured.push(<LoRaCloudCard application={this.props.application} />);
      } else {
        available.push(<LoRaCloudCard application={this.props.application} add />);
      }

      // ThingsBoard
      if (includes(resp.getResultList(), IntegrationKind.THINGS_BOARD)) {
        configured.push(<ThingsBoardCard application={this.props.application} />);
      } else {
        available.push(<ThingsBoardCard application={this.props.application} add />);
      }

      this.setState({
        configured: configured,
        available: available,
      });
    });
  };

  render() {
    return (
      <Row gutter={24}>
        {this.state.configured}
        {this.state.available}
      </Row>
    );
  }
}

export default ListIntegrations;
