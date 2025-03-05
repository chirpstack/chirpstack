import { notification } from "antd";
import { EventEmitter } from "events";

import { FuotaServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_grpc_web_pb";

import type {
  CreateFuotaDeploymentRequest,
  CreateFuotaDeploymentResponse,
  GetFuotaDeploymentRequest,
  GetFuotaDeploymentResponse,
  UpdateFuotaDeploymentRequest,
  DeleteFuotaDeploymentRequest,
  ListFuotaDeploymentsRequest,
  ListFuotaDeploymentsResponse,
  AddDevicesToFuotaDeploymentRequest,
  RemoveDevicesFromFuotaDeploymentRequest,
  ListFuotaDeploymentDevicesRequest,
  ListFuotaDeploymentDevicesResponse,
  AddGatewaysToFuotaDeploymentRequest,
  RemoveGatewaysFromFuotaDeploymentRequest,
  ListFuotaDeploymentGatewaysRequest,
  ListFuotaDeploymentGatewaysResponse,
  StartFuotaDeploymentRequest,
  ListFuotaDeploymentJobsRequest,
  ListFuotaDeploymentJobsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";
import { NotificationOutlined } from "@ant-design/icons";

class FuotaStore extends EventEmitter {
  client: FuotaServiceClient;

  constructor() {
    super();
    this.client = new FuotaServiceClient("");
  }

  createDeployment = (
    req: CreateFuotaDeploymentRequest,
    callbackFunc: (resp: CreateFuotaDeploymentResponse) => void,
  ) => {
    this.client.createDeployment(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "FUOTA deployment created",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };

  getDeployment = (req: GetFuotaDeploymentRequest, callbackFunc: (resp: GetFuotaDeploymentResponse) => void) => {
    this.client.getDeployment(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateDeployment = (req: UpdateFuotaDeploymentRequest, callbackFunc: () => void) => {
    this.client.updateDeployment(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "FUOTA deployment updated",
        duration: 3,
      });

      this.emit("updateDeployment");

      callbackFunc();
    });
  };

  deleteDeployment = (req: DeleteFuotaDeploymentRequest, callbackFunc: () => void) => {
    this.client.deleteDeployment(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "FUOTA deployment deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  startDeployment = (req: StartFuotaDeploymentRequest, callbackFunc: () => void) => {
    this.client.startDeployment(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "FUOTA deployment started",
        duration: 3,
      });

      this.emit("updateDeployment");

      callbackFunc();
    });
  };

  listDeployments = (req: ListFuotaDeploymentsRequest, callbackFunc: (resp: ListFuotaDeploymentsResponse) => void) => {
    this.client.listDeployments(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  addDevices = (req: AddDevicesToFuotaDeploymentRequest, callbackFunc: () => void) => {
    this.client.addDevices(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device(s) added to FUOTA deployment",
        duration: 3,
      });

      callbackFunc();
    });
  };

  removeDevices = (req: RemoveDevicesFromFuotaDeploymentRequest, callbackFunc: () => void) => {
    this.client.removeDevices(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device(s) removed from FUOTA deployment",
        duration: 3,
      });

      callbackFunc();
    });
  };

  listDevices = (
    req: ListFuotaDeploymentDevicesRequest,
    callbackFunc: (resp: ListFuotaDeploymentDevicesResponse) => void,
  ) => {
    this.client.listDevices(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  addGateways = (req: AddGatewaysToFuotaDeploymentRequest, callbackFunc: () => void) => {
    this.client.addGateways(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Gateway(s) added to FUOTA deployment",
        duration: 3,
      });

      callbackFunc();
    });
  };

  removeGateways = (req: RemoveGatewaysFromFuotaDeploymentRequest, callbackFunc: () => void) => {
    this.client.removeGateways(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Gateway(s) removed from FUOTA deployment",
        duration: 3,
      });

      callbackFunc();
    });
  };

  listGateways = (
    req: ListFuotaDeploymentGatewaysRequest,
    callbackFunc: (resp: ListFuotaDeploymentGatewaysResponse) => void,
  ) => {
    this.client.listGateways(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  listJobs = (req: ListFuotaDeploymentJobsRequest, callbackFunc: (resp: ListFuotaDeploymentJobsResponse) => void) => {
    this.client.listJobs(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const fuotaStore = new FuotaStore();
export default fuotaStore;
