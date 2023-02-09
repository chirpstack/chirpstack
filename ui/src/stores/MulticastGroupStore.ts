import { notification } from "antd";
import { EventEmitter } from "events";

import { MulticastGroupServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_grpc_web_pb";
import {
  CreateMulticastGroupRequest,
  CreateMulticastGroupResponse,
  GetMulticastGroupRequest,
  GetMulticastGroupResponse,
  UpdateMulticastGroupRequest,
  DeleteMulticastGroupRequest,
  ListMulticastGroupsRequest,
  ListMulticastGroupsResponse,
  AddDeviceToMulticastGroupRequest,
  RemoveDeviceFromMulticastGroupRequest,
  AddGatewayToMulticastGroupRequest,
  RemoveGatewayFromMulticastGroupRequest,
  ListMulticastGroupQueueRequest,
  ListMulticastGroupQueueResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class MulticastGroupStore extends EventEmitter {
  client: MulticastGroupServiceClient;

  constructor() {
    super();
    this.client = new MulticastGroupServiceClient("");
  }

  create = (req: CreateMulticastGroupRequest, callbackFunc: (resp: CreateMulticastGroupResponse) => void) => {
    this.client.create(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Multicast-group created",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };

  get = (req: GetMulticastGroupRequest, callbackFunc: (resp: GetMulticastGroupResponse) => void) => {
    this.client.get(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  update = (req: UpdateMulticastGroupRequest, callbackFunc: () => void) => {
    this.client.update(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Multicast-group updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  delete = (req: DeleteMulticastGroupRequest, callbackFunc: () => void) => {
    this.client.delete(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Multicast-group delete",
        duration: 3,
      });

      callbackFunc();
    });
  };

  list = (req: ListMulticastGroupsRequest, callbackFunc: (resp: ListMulticastGroupsResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  addDevice = (req: AddDeviceToMulticastGroupRequest, callbackFunc: () => void) => {
    this.client.addDevice(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device has been added to multicast-group",
        duration: 3,
      });

      callbackFunc();
    });
  };

  removeDevice = (req: RemoveDeviceFromMulticastGroupRequest, callbackFunc: () => void) => {
    this.client.removeDevice(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc();
    });
  };

  addGateway = (req: AddGatewayToMulticastGroupRequest, callbackFunc: () => void) => {
    this.client.addGateway(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Gateway has been added to multicast-group",
        duration: 3,
      });

      callbackFunc();
    });
  };

  removeGateway = (req: RemoveGatewayFromMulticastGroupRequest, callbackFunc: () => void) => {
    this.client.removeGateway(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc();
    });
  };

  listQueue = (req: ListMulticastGroupQueueRequest, callbackFunc: (resp: ListMulticastGroupQueueResponse) => void) => {
    this.client.listQueue(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const multicastGroupStore = new MulticastGroupStore();
export default multicastGroupStore;
