import { notification } from "antd";
import { EventEmitter } from "events";

import { DeviceServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/device_grpc_web_pb";
import {
  CreateDeviceRequest,
  GetDeviceRequest,
  GetDeviceResponse,
  UpdateDeviceRequest,
  DeleteDeviceRequest,
  ListDevicesRequest,
  ListDevicesResponse,
  CreateDeviceKeysRequest,
  GetDeviceKeysRequest,
  GetDeviceKeysResponse,
  UpdateDeviceKeysRequest,
  DeleteDeviceKeysRequest,
  EnqueueDeviceQueueItemRequest,
  EnqueueDeviceQueueItemResponse,
  FlushDeviceQueueRequest,
  GetDeviceQueueItemsRequest,
  GetDeviceQueueItemsResponse,
  FlushDevNoncesRequest,
  GetDeviceActivationRequest,
  GetDeviceActivationResponse,
  ActivateDeviceRequest,
  GetRandomDevAddrRequest,
  GetRandomDevAddrResponse,
  GetDeviceMetricsRequest,
  GetDeviceMetricsResponse,
  GetDeviceLinkMetricsRequest,
  GetDeviceLinkMetricsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class DeviceStore extends EventEmitter {
  client: DeviceServiceClient;

  constructor() {
    super();
    this.client = new DeviceServiceClient("");
  }

  create = (req: CreateDeviceRequest, callbackFunc: () => void) => {
    this.client.create(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  get = (req: GetDeviceRequest, callbackFunc: (resp: GetDeviceResponse) => void) => {
    this.client.get(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  update = (req: UpdateDeviceRequest, callbackFunc: () => void) => {
    this.client.update(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  delete = (req: DeleteDeviceRequest, callbackFunc: () => void) => {
    this.client.delete(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  list = (req: ListDevicesRequest, callbackFunc: (resp: ListDevicesResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  createKeys = (req: CreateDeviceKeysRequest, callbackFunc: () => void) => {
    this.client.createKeys(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device-key created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getKeys = (req: GetDeviceKeysRequest, callbackFunc: (resp?: GetDeviceKeysResponse) => void) => {
    this.client.getKeys(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        if (err.code !== 5) {
          HandleError(err);
          return;
        }
      }

      callbackFunc(resp);
    });
  };

  updateKeys = (req: UpdateDeviceKeysRequest, callbackFunc: () => void) => {
    this.client.updateKeys(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device-keys updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteKeys = (req: DeleteDeviceKeysRequest, callbackFunc: () => void) => {
    this.client.deleteKeys(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device-keys deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getMetrics = (req: GetDeviceMetricsRequest, callbackFunc: (resp: GetDeviceMetricsResponse) => void) => {
    this.client.getMetrics(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getLinkMetrics = (req: GetDeviceLinkMetricsRequest, callbackFunc: (resp: GetDeviceLinkMetricsResponse) => void) => {
    this.client.getLinkMetrics(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  enqueue = (req: EnqueueDeviceQueueItemRequest, callbackFunc: (resp: EnqueueDeviceQueueItemResponse) => void) => {
    this.client.enqueue(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  flushQueue = (req: FlushDeviceQueueRequest, callbackFunc: () => void) => {
    this.client.flushQueue(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc();
    });
  };

  flushDevNonces = (req: FlushDevNoncesRequest, callbackFunc: () => void) => {
    this.client.flushDevNonces(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "OTAA device-nonces flushed",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getQueue = (req: GetDeviceQueueItemsRequest, callbackFunc: (resp: GetDeviceQueueItemsResponse) => void) => {
    this.client.getQueue(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  activate = (req: ActivateDeviceRequest, callbackFunc: () => void) => {
    this.client.activate(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device (re)activated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getActivation = (req: GetDeviceActivationRequest, callbackFunc: (resp: GetDeviceActivationResponse) => void) => {
    this.client.getActivation(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getRandomDevAddr = (req: GetRandomDevAddrRequest, callbackFunc: (resp: GetRandomDevAddrResponse) => void) => {
    this.client.getRandomDevAddr(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const deviceStore = new DeviceStore();
export default deviceStore;
