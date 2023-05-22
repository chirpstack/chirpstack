// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/relay.proto

import * as api_relay_pb from "../api/relay_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface IRelayServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  list: grpc.MethodDefinition<api_relay_pb.ListRelaysRequest, api_relay_pb.ListRelaysResponse>;
  addDevice: grpc.MethodDefinition<api_relay_pb.AddRelayDeviceRequest, google_protobuf_empty_pb.Empty>;
  removeDevice: grpc.MethodDefinition<api_relay_pb.RemoveRelayDeviceRequest, google_protobuf_empty_pb.Empty>;
  listDevices: grpc.MethodDefinition<api_relay_pb.ListRelayDevicesRequest, api_relay_pb.ListRelayDevicesResponse>;
}

export const RelayServiceService: IRelayServiceService;

export interface IRelayServiceServer extends grpc.UntypedServiceImplementation {
  list: grpc.handleUnaryCall<api_relay_pb.ListRelaysRequest, api_relay_pb.ListRelaysResponse>;
  addDevice: grpc.handleUnaryCall<api_relay_pb.AddRelayDeviceRequest, google_protobuf_empty_pb.Empty>;
  removeDevice: grpc.handleUnaryCall<api_relay_pb.RemoveRelayDeviceRequest, google_protobuf_empty_pb.Empty>;
  listDevices: grpc.handleUnaryCall<api_relay_pb.ListRelayDevicesRequest, api_relay_pb.ListRelayDevicesResponse>;
}

export class RelayServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  list(argument: api_relay_pb.ListRelaysRequest, callback: grpc.requestCallback<api_relay_pb.ListRelaysResponse>): grpc.ClientUnaryCall;
  list(argument: api_relay_pb.ListRelaysRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_relay_pb.ListRelaysResponse>): grpc.ClientUnaryCall;
  list(argument: api_relay_pb.ListRelaysRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_relay_pb.ListRelaysResponse>): grpc.ClientUnaryCall;
  addDevice(argument: api_relay_pb.AddRelayDeviceRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addDevice(argument: api_relay_pb.AddRelayDeviceRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addDevice(argument: api_relay_pb.AddRelayDeviceRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeDevice(argument: api_relay_pb.RemoveRelayDeviceRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeDevice(argument: api_relay_pb.RemoveRelayDeviceRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeDevice(argument: api_relay_pb.RemoveRelayDeviceRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  listDevices(argument: api_relay_pb.ListRelayDevicesRequest, callback: grpc.requestCallback<api_relay_pb.ListRelayDevicesResponse>): grpc.ClientUnaryCall;
  listDevices(argument: api_relay_pb.ListRelayDevicesRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_relay_pb.ListRelayDevicesResponse>): grpc.ClientUnaryCall;
  listDevices(argument: api_relay_pb.ListRelayDevicesRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_relay_pb.ListRelayDevicesResponse>): grpc.ClientUnaryCall;
}
