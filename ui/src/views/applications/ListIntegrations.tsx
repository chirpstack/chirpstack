import { useState, useEffect, ReactElement } from "react";

import { Row } from "antd";

import type {
  Application,
  ListIntegrationsResponse,
  IntegrationListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { ListIntegrationsRequest, IntegrationKind } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../stores/ApplicationStore";
import HttpCard from "./integrations/HttpCard";
import MyDevicesCard from "./integrations/MyDevicesCard";
import MqttCard from "./integrations/MqttCard";
import AwsSnsCard from "./integrations/AwsSnsCard";
import AzureServiceBusCard from "./integrations/AzureServiceBusCard";
import GcpPubSubCard from "./integrations/GcpPubSubCard";
import InfluxdbCard from "./integrations/InfluxdbCard";
import PilotThingsCard from "./integrations/PilotThingsCard";
import ThingsBoardCard from "./integrations/ThingsBoardCard";
import IftttCard from "./integrations/IftttCard";
import BlynkCard from "./integrations/BlynkCard";

interface IProps {
  application: Application;
}

function ListIntegrations(props: IProps) {
  const [configured, setConfigured] = useState<ReactElement[]>([]);
  const [available, setAvailable] = useState<ReactElement[]>([]);

  useEffect(() => {
    const loadIntegrations = () => {
      const req = new ListIntegrationsRequest();
      req.setApplicationId(props.application.getId());

      ApplicationStore.listIntegrations(req, (resp: ListIntegrationsResponse) => {
        const configured: ReactElement[] = [];
        const available: ReactElement[] = [];

        const includes = (integrations: IntegrationListItem[], kind: IntegrationKind) => {
          for (const x of integrations) {
            if (x.getKind() === kind) {
              return true;
            }
          }

          return false;
        };

        // AWS SNS
        if (includes(resp.getResultList(), IntegrationKind.AWS_SNS)) {
          configured.push(<AwsSnsCard application={props.application} key={IntegrationKind.AWS_SNS} />);
        } else {
          available.push(<AwsSnsCard application={props.application} add key={IntegrationKind.AWS_SNS} />);
        }

        // Azure Service-Bus
        if (includes(resp.getResultList(), IntegrationKind.AZURE_SERVICE_BUS)) {
          configured.push(
            <AzureServiceBusCard application={props.application} key={IntegrationKind.AZURE_SERVICE_BUS} />,
          );
        } else {
          available.push(
            <AzureServiceBusCard application={props.application} add key={IntegrationKind.AZURE_SERVICE_BUS} />,
          );
        }

        // Blynk
        if (includes(resp.getResultList(), IntegrationKind.BLYNK)) {
          configured.push(<BlynkCard application={props.application} key={IntegrationKind.BLYNK} />);
        } else {
          available.push(<BlynkCard application={props.application} add key={IntegrationKind.BLYNK} />);
        }

        // GCP Pub/Sub
        if (includes(resp.getResultList(), IntegrationKind.GCP_PUB_SUB)) {
          configured.push(<GcpPubSubCard application={props.application} key={IntegrationKind.GCP_PUB_SUB} />);
        } else {
          available.push(<GcpPubSubCard application={props.application} add key={IntegrationKind.GCP_PUB_SUB} />);
        }

        // HTTP
        if (includes(resp.getResultList(), IntegrationKind.HTTP)) {
          configured.push(<HttpCard application={props.application} key={IntegrationKind.HTTP} />);
        } else {
          available.push(<HttpCard application={props.application} add key={IntegrationKind.HTTP} />);
        }

        // IFTTT
        if (includes(resp.getResultList(), IntegrationKind.IFTTT)) {
          configured.push(<IftttCard application={props.application} key={IntegrationKind.IFTTT} />);
        } else {
          available.push(<IftttCard application={props.application} add key={IntegrationKind.IFTTT} />);
        }

        // InfluxDB
        if (includes(resp.getResultList(), IntegrationKind.INFLUX_DB)) {
          configured.push(<InfluxdbCard application={props.application} key={IntegrationKind.INFLUX_DB} />);
        } else {
          available.push(<InfluxdbCard application={props.application} add key={IntegrationKind.INFLUX_DB} />);
        }

        // MQTT
        if (includes(resp.getResultList(), IntegrationKind.MQTT_GLOBAL)) {
          configured.push(<MqttCard application={props.application} key={IntegrationKind.MQTT_GLOBAL} />);
        }

        // myDevices
        if (includes(resp.getResultList(), IntegrationKind.MY_DEVICES)) {
          configured.push(<MyDevicesCard application={props.application} key={IntegrationKind.MY_DEVICES} />);
        } else {
          available.push(<MyDevicesCard application={props.application} add key={IntegrationKind.MY_DEVICES} />);
        }

        // Pilot Things
        if (includes(resp.getResultList(), IntegrationKind.PILOT_THINGS)) {
          configured.push(<PilotThingsCard application={props.application} key={IntegrationKind.PILOT_THINGS} />);
        } else {
          available.push(<PilotThingsCard application={props.application} add key={IntegrationKind.PILOT_THINGS} />);
        }

        // ThingsBoard
        if (includes(resp.getResultList(), IntegrationKind.THINGS_BOARD)) {
          configured.push(<ThingsBoardCard application={props.application} key={IntegrationKind.THINGS_BOARD} />);
        } else {
          available.push(<ThingsBoardCard application={props.application} add key={IntegrationKind.THINGS_BOARD} />);
        }

        setConfigured(configured);
        setAvailable(available);
      });
    };

    ApplicationStore.on("integration.delete", loadIntegrations);
    loadIntegrations();

    return () => {
      ApplicationStore.removeAllListeners("integration.delete");
    };
  }, [props.application]);

  return (
    <Row gutter={24}>
      {configured}
      {available}
    </Row>
  );
}

export default ListIntegrations;
