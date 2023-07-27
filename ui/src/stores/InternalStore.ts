import * as grpcWeb from "grpc-web";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

import { notification } from "antd";
import { EventEmitter } from "events";
import { InternalServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/internal_grpc_web_pb";
import {
  CreateApiKeyRequest,
  CreateApiKeyResponse,
  DeleteApiKeyRequest,
  ListApiKeysRequest,
  ListApiKeysResponse,
  StreamGatewayFramesRequest,
  StreamDeviceFramesRequest,
  StreamDeviceEventsRequest,
  LogItem,
  GetGatewaysSummaryRequest,
  GetGatewaysSummaryResponse,
  GetDevicesSummaryRequest,
  GetDevicesSummaryResponse,
  SettingsResponse,
  GlobalSearchRequest,
  GlobalSearchResponse,
  ListRegionsResponse,
  GetRegionRequest,
  GetRegionResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class InternalStore extends EventEmitter {
  client: InternalServiceClient;

  constructor() {
    super();
    this.client = new InternalServiceClient("");
  }

  createApiKey = (req: CreateApiKeyRequest, callbackFunc: (resp: CreateApiKeyResponse) => void) => {
    this.client.createApiKey(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "API key created",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };

  deleteApiKey = (req: DeleteApiKeyRequest, callbackFunc: () => void) => {
    this.client.deleteApiKey(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "API key deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  listApiKeys = (req: ListApiKeysRequest, callbackFunc: (resp: ListApiKeysResponse) => void) => {
    this.client.listApiKeys(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  streamGatewayFrames = (req: StreamGatewayFramesRequest, callbackFunc: (resp: LogItem) => void): (() => void) => {
    var stream: grpcWeb.ClientReadableStream<LogItem> | undefined = undefined;

    let setup = () => {
      console.log("Setting up gRPC stream");
      stream = this.client.streamGatewayFrames(req, SessionStore.getMetadata());

      stream = stream.on("data", resp => {
        callbackFunc(resp);
      });

      stream = stream.on("end", function () {
        console.log("gRPC stream end, reconnecting");
        setTimeout(setup, 1000);
      });
    };

    setup();

    return () => {
      if (stream) {
        console.log("Cancelling gRPC stream");
        stream.cancel();
      }
    };
  };

  streamDeviceFrames = (req: StreamDeviceFramesRequest, callbackFunc: (resp: LogItem) => void): (() => void) => {
    var stream: grpcWeb.ClientReadableStream<LogItem> | undefined = undefined;

    let setup = () => {
      console.log("Setting up gRPC stream");
      stream = this.client.streamDeviceFrames(req, SessionStore.getMetadata());

      stream = stream.on("data", resp => {
        callbackFunc(resp);
      });

      stream = stream.on("end", function () {
        console.log("gRPC stream end, reconnecting");
        setTimeout(setup, 1000);
      });
    };

    setup();

    return () => {
      if (stream) {
        stream.cancel();
      }
    };
  };

  streamDeviceEvents = (req: StreamDeviceEventsRequest, callbackFunc: (resp: LogItem) => void): (() => void) => {
    var stream: grpcWeb.ClientReadableStream<LogItem> | undefined = undefined;

    let setup = () => {
      console.log("Setting up gRPC stream");
      stream = this.client.streamDeviceEvents(req, SessionStore.getMetadata());

      stream = stream.on("data", resp => {
        callbackFunc(resp);
      });

      stream = stream.on("end", function () {
        console.log("gRPC stream end, reconnecting");
        setTimeout(setup, 1000);
      });
    };

    setup();

    return () => {
      if (stream) {
        stream.cancel();
      }
    };
  };

  getGatewaysSummary = (req: GetGatewaysSummaryRequest, callbackFunc: (resp: GetGatewaysSummaryResponse) => void) => {
    this.client.getGatewaysSummary(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getDevicesSummary = (req: GetDevicesSummaryRequest, callbackFunc: (resp: GetDevicesSummaryResponse) => void) => {
    this.client.getDevicesSummary(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  settings = (callbackFunc: (resp: SettingsResponse) => void) => {
    this.client.settings(new google_protobuf_empty_pb.Empty(), {}, (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  globalSearch = (req: GlobalSearchRequest, callbackFunc: (resp: GlobalSearchResponse) => void) => {
    this.client.globalSearch(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  listRegions = (callbackFunc: (resp: ListRegionsResponse) => void) => {
    this.client.listRegions(new google_protobuf_empty_pb.Empty(), SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  getRegion = (req: GetRegionRequest, callbackFunc: (resp: GetRegionResponse) => void) => {
    this.client.getRegion(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const internalStore = new InternalStore();
export default internalStore;
