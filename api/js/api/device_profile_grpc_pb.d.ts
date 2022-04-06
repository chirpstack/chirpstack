// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/device_profile.proto

import * as api_device_profile_pb from "../api/device_profile_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface IDeviceProfileServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  create: grpc.MethodDefinition<api_device_profile_pb.CreateDeviceProfileRequest, api_device_profile_pb.CreateDeviceProfileResponse>;
  get: grpc.MethodDefinition<api_device_profile_pb.GetDeviceProfileRequest, api_device_profile_pb.GetDeviceProfileResponse>;
  update: grpc.MethodDefinition<api_device_profile_pb.UpdateDeviceProfileRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.MethodDefinition<api_device_profile_pb.DeleteDeviceProfileRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.MethodDefinition<api_device_profile_pb.ListDeviceProfilesRequest, api_device_profile_pb.ListDeviceProfilesResponse>;
  listAdrAlgorithms: grpc.MethodDefinition<google_protobuf_empty_pb.Empty, api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse>;
}

export const DeviceProfileServiceService: IDeviceProfileServiceService;

export interface IDeviceProfileServiceServer extends grpc.UntypedServiceImplementation {
  create: grpc.handleUnaryCall<api_device_profile_pb.CreateDeviceProfileRequest, api_device_profile_pb.CreateDeviceProfileResponse>;
  get: grpc.handleUnaryCall<api_device_profile_pb.GetDeviceProfileRequest, api_device_profile_pb.GetDeviceProfileResponse>;
  update: grpc.handleUnaryCall<api_device_profile_pb.UpdateDeviceProfileRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.handleUnaryCall<api_device_profile_pb.DeleteDeviceProfileRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.handleUnaryCall<api_device_profile_pb.ListDeviceProfilesRequest, api_device_profile_pb.ListDeviceProfilesResponse>;
  listAdrAlgorithms: grpc.handleUnaryCall<google_protobuf_empty_pb.Empty, api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse>;
}

export class DeviceProfileServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  create(argument: api_device_profile_pb.CreateDeviceProfileRequest, callback: grpc.requestCallback<api_device_profile_pb.CreateDeviceProfileResponse>): grpc.ClientUnaryCall;
  create(argument: api_device_profile_pb.CreateDeviceProfileRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_pb.CreateDeviceProfileResponse>): grpc.ClientUnaryCall;
  create(argument: api_device_profile_pb.CreateDeviceProfileRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_pb.CreateDeviceProfileResponse>): grpc.ClientUnaryCall;
  get(argument: api_device_profile_pb.GetDeviceProfileRequest, callback: grpc.requestCallback<api_device_profile_pb.GetDeviceProfileResponse>): grpc.ClientUnaryCall;
  get(argument: api_device_profile_pb.GetDeviceProfileRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_pb.GetDeviceProfileResponse>): grpc.ClientUnaryCall;
  get(argument: api_device_profile_pb.GetDeviceProfileRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_pb.GetDeviceProfileResponse>): grpc.ClientUnaryCall;
  update(argument: api_device_profile_pb.UpdateDeviceProfileRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_device_profile_pb.UpdateDeviceProfileRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_device_profile_pb.UpdateDeviceProfileRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_profile_pb.DeleteDeviceProfileRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_profile_pb.DeleteDeviceProfileRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_profile_pb.DeleteDeviceProfileRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  list(argument: api_device_profile_pb.ListDeviceProfilesRequest, callback: grpc.requestCallback<api_device_profile_pb.ListDeviceProfilesResponse>): grpc.ClientUnaryCall;
  list(argument: api_device_profile_pb.ListDeviceProfilesRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_pb.ListDeviceProfilesResponse>): grpc.ClientUnaryCall;
  list(argument: api_device_profile_pb.ListDeviceProfilesRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_pb.ListDeviceProfilesResponse>): grpc.ClientUnaryCall;
  listAdrAlgorithms(argument: google_protobuf_empty_pb.Empty, callback: grpc.requestCallback<api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse>): grpc.ClientUnaryCall;
  listAdrAlgorithms(argument: google_protobuf_empty_pb.Empty, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse>): grpc.ClientUnaryCall;
  listAdrAlgorithms(argument: google_protobuf_empty_pb.Empty, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse>): grpc.ClientUnaryCall;
}
