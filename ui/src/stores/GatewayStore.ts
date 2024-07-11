import { notification } from "antd";
import { EventEmitter } from "events";
import { GatewayServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_grpc_web_pb";
import type {
  CreateGatewayRequest,
  GetGatewayRequest,
  GetGatewayResponse,
  UpdateGatewayRequest,
  DeleteGatewayRequest,
  ListGatewaysRequest,
  ListGatewaysResponse,
  GetGatewayMetricsRequest,
  GetGatewayMetricsResponse,
  GetGatewayDutyCycleMetricsRequest,
  GetGatewayDutyCycleMetricsResponse,
  GenerateGatewayClientCertificateRequest,
  GenerateGatewayClientCertificateResponse,
  GetRelayGatewayRequest,
  GetRelayGatewayResponse,
  ListRelayGatewaysRequest,
  ListRelayGatewaysResponse,
  UpdateRelayGatewayRequest,
  DeleteRelayGatewayRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class GatewayStore extends EventEmitter {
  client: GatewayServiceClient;

  constructor() {
    super();
    this.client = new GatewayServiceClient("");
  }

  create = (req: CreateGatewayRequest, callbackFunc: () => void) => {
    this.client.create(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Gateway created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  get = (req: GetGatewayRequest, callbackFunc: (resp: GetGatewayResponse) => void) => {
    this.client.get(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  update = (req: UpdateGatewayRequest, callbackFunc: () => void) => {
    this.client.update(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Gateway updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  delete = (req: DeleteGatewayRequest, callbackFunc: () => void) => {
    this.client.delete(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Gateway deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  list = (req: ListGatewaysRequest, callbackFunc: (resp: ListGatewaysResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getMetrics = (req: GetGatewayMetricsRequest, callbackFunc: (resp: GetGatewayMetricsResponse) => void) => {
    this.client.getMetrics(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getDutyCycleMetrics = (
    req: GetGatewayDutyCycleMetricsRequest,
    callbackFunc: (resp: GetGatewayDutyCycleMetricsResponse) => void,
  ) => {
    this.client.getDutyCycleMetrics(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  generateClientCertificate = (
    req: GenerateGatewayClientCertificateRequest,
    callbackFunc: (resp: GenerateGatewayClientCertificateResponse) => void,
  ) => {
    this.client.generateClientCertificate(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getRelayGateway = (req: GetRelayGatewayRequest, callbackFunc: (resp: GetRelayGatewayResponse) => void) => {
    this.client.getRelayGateway(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  listRelayGateways = (req: ListRelayGatewaysRequest, callbackFunc: (resp: ListRelayGatewaysResponse) => void) => {
    this.client.listRelayGateways(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateRelayGateway = (req: UpdateRelayGatewayRequest, callbackFunc: () => void) => {
    this.client.updateRelayGateway(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Relay Gateway updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteRelayGateway = (req: DeleteRelayGatewayRequest, callbackFunc: () => void) => {
    this.client.deleteRelayGateway(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Relay Gateway deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };
}

const gatewayStore = new GatewayStore();
export default gatewayStore;
