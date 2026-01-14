import { notification } from "antd";
import { EventEmitter } from "events";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import { DeviceProfileServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_grpc_web_pb";
import type {
  CreateDeviceProfileRequest,
  CreateDeviceProfileResponse,
  GetDeviceProfileRequest,
  GetDeviceProfileResponse,
  UpdateDeviceProfileRequest,
  DeleteDeviceProfileRequest,
  ListDeviceProfilesRequest,
  ListDeviceProfilesResponse,
  ListDeviceProfileAdrAlgorithmsResponse,
  ListDeviceProfileVendorsRequest,
  ListDeviceProfileVendorsResponse,
  ListDeviceProfileDevicesResponse,
  ListDeviceProfileDevicesRequest,
  GetDeviceProfileDeviceRequest,
  GetDeviceProfileDeviceResponse,
  GetDeviceProfileVendorRequest,
  GetDeviceProfileVendorResponse,
  DeleteDeviceProfileVendorRequest,
  DeleteDeviceProfileDeviceRequest,
  GetDeviceProfileByProfileIdResponse,
  GetDeviceProfileByProfileIdRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class DeviceProfileStore extends EventEmitter {
  client: DeviceProfileServiceClient;

  constructor() {
    super();
    this.client = new DeviceProfileServiceClient("");
  }

  create = (req: CreateDeviceProfileRequest, callbackFunc: (resp: CreateDeviceProfileResponse) => void) => {
    this.client.create(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device profile created",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };

  get = (req: GetDeviceProfileRequest, callbackFunc: (resp: GetDeviceProfileResponse) => void) => {
    this.client.get(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getByProfileId = (
    req: GetDeviceProfileByProfileIdRequest,
    callbackFunc: (resp: GetDeviceProfileByProfileIdResponse) => void,
  ) => {
    this.client.getByProfileId(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  update = (req: UpdateDeviceProfileRequest, callbackFunc: () => void) => {
    this.client.update(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device profile updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  delete = (req: DeleteDeviceProfileRequest, callbackFunc: () => void) => {
    this.client.delete(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Device profile deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  list = (req: ListDeviceProfilesRequest, callbackFunc: (resp: ListDeviceProfilesResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  listVendors = (
    req: ListDeviceProfileVendorsRequest,
    callbackFunc: (resp: ListDeviceProfileVendorsResponse) => void,
  ) => {
    this.client.listVendors(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getVendor = (req: GetDeviceProfileVendorRequest, callbackFunc: (resp: GetDeviceProfileVendorResponse) => void) => {
    this.client.getVendor(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  deleteVendor = (req: DeleteDeviceProfileVendorRequest, callbackFunc: () => void) => {
    this.client.deleteVendor(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Vendor deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteDevice = (req: DeleteDeviceProfileDeviceRequest, callbackFunc: () => void) => {
    this.client.deleteDevice(req, SessionStore.getMetadata(), err => {
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

  getDevice = (req: GetDeviceProfileDeviceRequest, callbackFunc: (resp: GetDeviceProfileDeviceResponse) => void) => {
    this.client.getDevice(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  listDevices = (
    req: ListDeviceProfileDevicesRequest,
    callbackFunc: (resp: ListDeviceProfileDevicesResponse) => void,
  ) => {
    this.client.listDevices(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  listAdrAlgorithms = (callbackFunc: (resp: ListDeviceProfileAdrAlgorithmsResponse) => void) => {
    this.client.listAdrAlgorithms(new google_protobuf_empty_pb.Empty(), SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const deviceProfileStore = new DeviceProfileStore();
export default deviceProfileStore;
