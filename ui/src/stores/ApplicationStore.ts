import { notification } from "antd";
import { EventEmitter } from "events";
import { ApplicationServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/application_grpc_web_pb";
import {
  CreateApplicationRequest,
  CreateApplicationResponse,
  GetApplicationRequest,
  GetApplicationResponse,
  UpdateApplicationRequest,
  DeleteApplicationRequest,
  ListApplicationsRequest,
  ListApplicationsResponse,
  ListIntegrationsRequest,
  ListIntegrationsResponse,
  CreateHttpIntegrationRequest,
  GetHttpIntegrationRequest,
  GetHttpIntegrationResponse,
  UpdateHttpIntegrationRequest,
  DeleteHttpIntegrationRequest,
  CreateAwsSnsIntegrationRequest,
  GetAwsSnsIntegrationRequest,
  GetAwsSnsIntegrationResponse,
  UpdateAwsSnsIntegrationRequest,
  DeleteAwsSnsIntegrationRequest,
  CreateAzureServiceBusIntegrationRequest,
  GetAzureServiceBusIntegrationRequest,
  GetAzureServiceBusIntegrationResponse,
  UpdateAzureServiceBusIntegrationRequest,
  DeleteAzureServiceBusIntegrationRequest,
  CreateGcpPubSubIntegrationRequest,
  GetGcpPubSubIntegrationRequest,
  GetGcpPubSubIntegrationResponse,
  UpdateGcpPubSubIntegrationRequest,
  DeleteGcpPubSubIntegrationRequest,
  CreateInfluxDbIntegrationRequest,
  GetInfluxDbIntegrationRequest,
  GetInfluxDbIntegrationResponse,
  UpdateInfluxDbIntegrationRequest,
  DeleteInfluxDbIntegrationRequest,
  CreateMyDevicesIntegrationRequest,
  GetMyDevicesIntegrationRequest,
  GetMyDevicesIntegrationResponse,
  UpdateMyDevicesIntegrationRequest,
  DeleteMyDevicesIntegrationRequest,
  CreatePilotThingsIntegrationRequest,
  GetPilotThingsIntegrationRequest,
  GetPilotThingsIntegrationResponse,
  UpdatePilotThingsIntegrationRequest,
  DeletePilotThingsIntegrationRequest,
  CreateLoraCloudIntegrationRequest,
  GetLoraCloudIntegrationRequest,
  GetLoraCloudIntegrationResponse,
  UpdateLoraCloudIntegrationRequest,
  DeleteLoraCloudIntegrationRequest,
  CreateThingsBoardIntegrationRequest,
  GetThingsBoardIntegrationRequest,
  GetThingsBoardIntegrationResponse,
  UpdateThingsBoardIntegrationRequest,
  DeleteThingsBoardIntegrationRequest,
  CreateIftttIntegrationRequest,
  GetIftttIntegrationRequest,
  GetIftttIntegrationResponse,
  UpdateIftttIntegrationRequest,
  DeleteIftttIntegrationRequest,
  GenerateMqttIntegrationClientCertificateRequest,
  GenerateMqttIntegrationClientCertificateResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class ApplicationStore extends EventEmitter {
  client: ApplicationServiceClient;

  constructor() {
    super();
    this.client = new ApplicationServiceClient("");
  }

  create = (req: CreateApplicationRequest, callbackFunc: (resp: CreateApplicationResponse) => void) => {
    this.client.create(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Application created",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };

  get = (req: GetApplicationRequest, callbackFunc: (resp: GetApplicationResponse) => void) => {
    this.client.get(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  update = (req: UpdateApplicationRequest, callbackFunc: () => void) => {
    this.client.update(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      this.emit("change");

      notification.success({
        message: "Application updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  delete = (req: DeleteApplicationRequest, callbackFunc: () => void) => {
    this.client.delete(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Application deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  list = (req: ListApplicationsRequest, callbackFunc: (resp: ListApplicationsResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  listIntegrations = (req: ListIntegrationsRequest, callbackFunc: (resp: ListIntegrationsResponse) => void) => {
    this.client.listIntegrations(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  createHttpIntegration = (req: CreateHttpIntegrationRequest, callbackFunc: () => void) => {
    this.client.createHttpIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "HTTP integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getHttpIntegration = (req: GetHttpIntegrationRequest, callbackFunc: (resp: GetHttpIntegrationResponse) => void) => {
    this.client.getHttpIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateHttpIntegration = (req: UpdateHttpIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateHttpIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "HTTP integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteHttpIntegration = (req: DeleteHttpIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteHttpIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "HTTP integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createAwsSnsIntegration = (req: CreateAwsSnsIntegrationRequest, callbackFunc: () => void) => {
    this.client.createAwsSnsIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "AWS SNS integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getAwsSnsIntegration = (
    req: GetAwsSnsIntegrationRequest,
    callbackFunc: (resp: GetAwsSnsIntegrationResponse) => void,
  ) => {
    this.client.getAwsSnsIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateAwsSnsIntegration = (req: UpdateAwsSnsIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateAwsSnsIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "AWS SNS integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteAwsSnsIntegration = (req: DeleteAwsSnsIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteAwsSnsIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "AWS SNS integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createAzureServiceBusIntegration = (req: CreateAzureServiceBusIntegrationRequest, callbackFunc: () => void) => {
    this.client.createAzureServiceBusIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Azure Service-Bus integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getAzureServiceBusIntegration = (
    req: GetAzureServiceBusIntegrationRequest,
    callbackFunc: (resp: GetAzureServiceBusIntegrationResponse) => void,
  ) => {
    this.client.getAzureServiceBusIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateAzureServiceBusIntegration = (req: UpdateAzureServiceBusIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateAzureServiceBusIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Azure Service-Bus integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteAzureServiceBusIntegration = (req: DeleteAzureServiceBusIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteAzureServiceBusIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Azure Service-Bus integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createGcpPubSubIntegration = (req: CreateGcpPubSubIntegrationRequest, callbackFunc: () => void) => {
    this.client.createGcpPubSubIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "GCP Pub/Sub integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getGcpPubSubIntegration = (
    req: GetGcpPubSubIntegrationRequest,
    callbackFunc: (resp: GetGcpPubSubIntegrationResponse) => void,
  ) => {
    this.client.getGcpPubSubIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateGcpPubSubIntegration = (req: UpdateGcpPubSubIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateGcpPubSubIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "GCP Pub/Sub integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteGcpPubSubIntegration = (req: DeleteGcpPubSubIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteGcpPubSubIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "GCP Pub/Sub integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createInfluxDbIntegration = (req: CreateInfluxDbIntegrationRequest, callbackFunc: () => void) => {
    this.client.createInfluxDbIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "InfluxDB integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getInfluxDbIntegration = (
    req: GetInfluxDbIntegrationRequest,
    callbackFunc: (resp: GetInfluxDbIntegrationResponse) => void,
  ) => {
    this.client.getInfluxDbIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateInfluxDbIntegration = (req: UpdateInfluxDbIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateInfluxDbIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "InfluxDB integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteInfluxDbIntegration = (req: DeleteInfluxDbIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteInfluxDbIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "InfluxDB integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createMyDevicesIntegration = (req: CreateMyDevicesIntegrationRequest, callbackFunc: () => void) => {
    this.client.createMyDevicesIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "myDevices integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getMyDevicesIntegration = (
    req: GetMyDevicesIntegrationRequest,
    callbackFunc: (resp: GetMyDevicesIntegrationResponse) => void,
  ) => {
    this.client.getMyDevicesIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateMyDevicesIntegration = (req: UpdateMyDevicesIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateMyDevicesIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "myDevices integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteMyDevicesIntegration = (req: DeleteMyDevicesIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteMyDevicesIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "myDevices integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createPilotThingsIntegration = (req: CreatePilotThingsIntegrationRequest, callbackFunc: () => void) => {
    this.client.createPilotThingsIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Pilot Things integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getPilotThingsIntegration = (
    req: GetPilotThingsIntegrationRequest,
    callbackFunc: (resp: GetPilotThingsIntegrationResponse) => void,
  ) => {
    this.client.getPilotThingsIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updatePilotThingsIntegration = (req: UpdatePilotThingsIntegrationRequest, callbackFunc: () => void) => {
    this.client.updatePilotThingsIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Pilot Things interation updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deletePilotThingsIntegration = (req: DeletePilotThingsIntegrationRequest, callbackFunc: () => void) => {
    this.client.deletePilotThingsIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Pilot Things interation deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createLoraCloudIntegration = (req: CreateLoraCloudIntegrationRequest, callbackFunc: () => void) => {
    this.client.createLoraCloudIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "LoRa Cloud interation created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getLoraCloudIntegration = (
    req: GetLoraCloudIntegrationRequest,
    callbackFunc: (resp: GetLoraCloudIntegrationResponse) => void,
  ) => {
    this.client.getLoraCloudIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateLoraCloudIntegration = (req: UpdateLoraCloudIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateLoraCloudIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "LoRa Cloud integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteLoraCloudIntegration = (req: DeleteLoraCloudIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteLoraCloudIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "LoRa Cloud integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createThingsBoardIntegration = (req: CreateThingsBoardIntegrationRequest, callbackFunc: () => void) => {
    this.client.createThingsBoardIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "ThingsBoard integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getThingsBoardIntegration = (
    req: GetThingsBoardIntegrationRequest,
    callbackFunc: (resp: GetThingsBoardIntegrationResponse) => void,
  ) => {
    this.client.getThingsBoardIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateThingsBoardIntegration = (req: UpdateThingsBoardIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateThingsBoardIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "ThingsBoard integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteThingsBoardIntegration = (req: DeleteThingsBoardIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteThingsBoardIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "ThingsBoard integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  createIftttIntegration = (req: CreateIftttIntegrationRequest, callbackFunc: () => void) => {
    this.client.createIftttIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "IFTTT integration created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getIftttIntegration = (
    req: GetIftttIntegrationRequest,
    callbackFunc: (resp: GetIftttIntegrationResponse) => void,
  ) => {
    this.client.getIftttIntegration(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateIftttIntegration = (req: UpdateIftttIntegrationRequest, callbackFunc: () => void) => {
    this.client.updateIftttIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "IFTTT integration updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteIftttIntegration = (req: DeleteIftttIntegrationRequest, callbackFunc: () => void) => {
    this.client.deleteIftttIntegration(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "IFTTT integration deleted",
        duration: 3,
      });

      this.emit("integration.delete");
      callbackFunc();
    });
  };

  generateMqttIntegrationClientCertificate = (
    req: GenerateMqttIntegrationClientCertificateRequest,
    callbackFunc: (resp: GenerateMqttIntegrationClientCertificateResponse) => void,
  ) => {
    this.client.generateMqttIntegrationClientCertificate(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const applicationStore = new ApplicationStore();
export default applicationStore;
