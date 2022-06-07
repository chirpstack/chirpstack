import { notification } from "antd";
import { EventEmitter } from "events";
import { DeviceProfileTemplateServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_grpc_web_pb";
import {
  CreateDeviceProfileTemplateRequest,
  GetDeviceProfileTemplateRequest,
  GetDeviceProfileTemplateResponse,
  UpdateDeviceProfileTemplateRequest,
  DeleteDeviceProfileTemplateRequest,
  ListDeviceProfileTemplatesRequest,
  ListDeviceProfileTemplatesResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class DeviceProfileTemplateStore extends EventEmitter {
  client: DeviceProfileTemplateServiceClient;

  constructor() {
    super();
    this.client = new DeviceProfileTemplateServiceClient("");
  }

  create = (req: CreateDeviceProfileTemplateRequest, callbackFunc: () => void) => {
    this.client.create(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device-profile template created",
        duration: 3,
      });

      callbackFunc();
    });
  };

  get = (req: GetDeviceProfileTemplateRequest, callbackFunc: (resp: GetDeviceProfileTemplateResponse) => void) => {
    this.client.get(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  update = (req: UpdateDeviceProfileTemplateRequest, callbackFunc: () => void) => {
    this.client.update(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device-profile template updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  delete = (req: DeleteDeviceProfileTemplateRequest, callbackFunc: () => void) => {
    this.client.delete(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device-profile template deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  list = (req: ListDeviceProfileTemplatesRequest, callbackFunc: (resp: ListDeviceProfileTemplatesResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const deviceProfileTemplateStore = new DeviceProfileTemplateStore();
export default deviceProfileTemplateStore;
