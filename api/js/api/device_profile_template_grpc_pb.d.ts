// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/device_profile_template.proto

import * as api_device_profile_template_pb from "../api/device_profile_template_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface IDeviceProfileTemplateServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  create: grpc.MethodDefinition<api_device_profile_template_pb.CreateDeviceProfileTemplateRequest, google_protobuf_empty_pb.Empty>;
  get: grpc.MethodDefinition<api_device_profile_template_pb.GetDeviceProfileTemplateRequest, api_device_profile_template_pb.GetDeviceProfileTemplateResponse>;
  update: grpc.MethodDefinition<api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.MethodDefinition<api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.MethodDefinition<api_device_profile_template_pb.ListDeviceProfileTemplatesRequest, api_device_profile_template_pb.ListDeviceProfileTemplatesResponse>;
}

export const DeviceProfileTemplateServiceService: IDeviceProfileTemplateServiceService;

export interface IDeviceProfileTemplateServiceServer extends grpc.UntypedServiceImplementation {
  create: grpc.handleUnaryCall<api_device_profile_template_pb.CreateDeviceProfileTemplateRequest, google_protobuf_empty_pb.Empty>;
  get: grpc.handleUnaryCall<api_device_profile_template_pb.GetDeviceProfileTemplateRequest, api_device_profile_template_pb.GetDeviceProfileTemplateResponse>;
  update: grpc.handleUnaryCall<api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.handleUnaryCall<api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.handleUnaryCall<api_device_profile_template_pb.ListDeviceProfileTemplatesRequest, api_device_profile_template_pb.ListDeviceProfileTemplatesResponse>;
}

export class DeviceProfileTemplateServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  create(argument: api_device_profile_template_pb.CreateDeviceProfileTemplateRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  create(argument: api_device_profile_template_pb.CreateDeviceProfileTemplateRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  create(argument: api_device_profile_template_pb.CreateDeviceProfileTemplateRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  get(argument: api_device_profile_template_pb.GetDeviceProfileTemplateRequest, callback: grpc.requestCallback<api_device_profile_template_pb.GetDeviceProfileTemplateResponse>): grpc.ClientUnaryCall;
  get(argument: api_device_profile_template_pb.GetDeviceProfileTemplateRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_template_pb.GetDeviceProfileTemplateResponse>): grpc.ClientUnaryCall;
  get(argument: api_device_profile_template_pb.GetDeviceProfileTemplateRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_template_pb.GetDeviceProfileTemplateResponse>): grpc.ClientUnaryCall;
  update(argument: api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  list(argument: api_device_profile_template_pb.ListDeviceProfileTemplatesRequest, callback: grpc.requestCallback<api_device_profile_template_pb.ListDeviceProfileTemplatesResponse>): grpc.ClientUnaryCall;
  list(argument: api_device_profile_template_pb.ListDeviceProfileTemplatesRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_template_pb.ListDeviceProfileTemplatesResponse>): grpc.ClientUnaryCall;
  list(argument: api_device_profile_template_pb.ListDeviceProfileTemplatesRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_profile_template_pb.ListDeviceProfileTemplatesResponse>): grpc.ClientUnaryCall;
}
