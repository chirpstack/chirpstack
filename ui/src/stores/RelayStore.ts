import { notification } from "antd";
import { EventEmitter } from "events";

import { RelayServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/relay_grpc_web_pb";
import {
  ListRelaysRequest,
  ListRelaysResponse,
  AddRelayDeviceRequest,
  RemoveRelayDeviceRequest,
  ListRelayDevicesRequest,
  ListRelayDevicesResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class RelayStore extends EventEmitter {
  client: RelayServiceClient;

  constructor() {
    super();
    this.client = new RelayServiceClient("");
  }

  list = (req: ListRelaysRequest, callbackFunc: (resp: ListRelaysResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  addDevice = (req: AddRelayDeviceRequest, callbackFunc: () => void) => {
    this.client.addDevice(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device added to relay",
        duration: 3,
      });

      callbackFunc();
    });
  };

  removeDevice = (req: RemoveRelayDeviceRequest, callbackFunc: () => void) => {
    this.client.removeDevice(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device removed from relay",
        duration: 3,
      });

      callbackFunc();
    });
  };

  listDevices = (req: ListRelayDevicesRequest, callbackFunc: (resp: ListRelayDevicesResponse) => void) => {
    this.client.listDevices(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const relayStore = new RelayStore();
export default relayStore;
