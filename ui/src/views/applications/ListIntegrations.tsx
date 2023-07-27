import React, { useState, useEffect } from "react";

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

function ListIntegrations(props: IProps) {
  const [configured, setConfigured] = useState<any[]>([]);
  const [available, setAvailable] = useState<any[]>([]);

  useEffect(() => {
    ApplicationStore.on("integration.delete", loadIntegrations);
    loadIntegrations();

    return () => {
      ApplicationStore.removeAllListeners("integration.delete");
    };
  }, []);

  const loadIntegrations = () => {
    let req = new ListIntegrationsRequest();
    req.setApplicationId(props.application.getId());

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
        configured.push(<AwsSnsCard application={props.application} />);
      } else {
        available.push(<AwsSnsCard application={props.application} add />);
      }

      // Azure Service-Bus
      if (includes(resp.getResultList(), IntegrationKind.AZURE_SERVICE_BUS)) {
        configured.push(<AzureServiceBusCard application={props.application} />);
      } else {
        available.push(<AzureServiceBusCard application={props.application} add />);
      }

      // GCP Pub/Sub
      if (includes(resp.getResultList(), IntegrationKind.GCP_PUB_SUB)) {
        configured.push(<GcpPubSubCard application={props.application} />);
      } else {
        available.push(<GcpPubSubCard application={props.application} add />);
      }

      // HTTP
      if (includes(resp.getResultList(), IntegrationKind.HTTP)) {
        configured.push(<HttpCard application={props.application} />);
      } else {
        available.push(<HttpCard application={props.application} add />);
      }

      // IFTTT
      if (includes(resp.getResultList(), IntegrationKind.IFTTT)) {
        configured.push(<IftttCard application={props.application} />);
      } else {
        available.push(<IftttCard application={props.application} add />);
      }

      // InfluxDB
      if (includes(resp.getResultList(), IntegrationKind.INFLUX_DB)) {
        configured.push(<InfluxdbCard application={props.application} />);
      } else {
        available.push(<InfluxdbCard application={props.application} add />);
      }

      // MQTT
      if (includes(resp.getResultList(), IntegrationKind.MQTT_GLOBAL)) {
        configured.push(<MqttCard application={props.application} />);
      }

      // myDevices
      if (includes(resp.getResultList(), IntegrationKind.MY_DEVICES)) {
        configured.push(<MyDevicesCard application={props.application} />);
      } else {
        available.push(<MyDevicesCard application={props.application} add />);
      }

      // Pilot Things
      if (includes(resp.getResultList(), IntegrationKind.PILOT_THINGS)) {
        configured.push(<PilotThingsCard application={props.application} />);
      } else {
        available.push(<PilotThingsCard application={props.application} add />);
      }

      // Semtech LoRa Cloud
      if (includes(resp.getResultList(), IntegrationKind.LORA_CLOUD)) {
        configured.push(<LoRaCloudCard application={props.application} />);
      } else {
        available.push(<LoRaCloudCard application={props.application} add />);
      }

      // ThingsBoard
      if (includes(resp.getResultList(), IntegrationKind.THINGS_BOARD)) {
        configured.push(<ThingsBoardCard application={props.application} />);
      } else {
        available.push(<ThingsBoardCard application={props.application} add />);
      }

      setConfigured(configured);
      setAvailable(available);
    });
  };

  return (
    <Row gutter={24}>
      {configured}
      {available}
    </Row>
  );
}

export default ListIntegrations;
