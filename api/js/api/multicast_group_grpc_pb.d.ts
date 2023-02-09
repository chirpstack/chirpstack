// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/multicast_group.proto

import * as api_multicast_group_pb from "../api/multicast_group_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface IMulticastGroupServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  create: grpc.MethodDefinition<api_multicast_group_pb.CreateMulticastGroupRequest, api_multicast_group_pb.CreateMulticastGroupResponse>;
  get: grpc.MethodDefinition<api_multicast_group_pb.GetMulticastGroupRequest, api_multicast_group_pb.GetMulticastGroupResponse>;
  update: grpc.MethodDefinition<api_multicast_group_pb.UpdateMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.MethodDefinition<api_multicast_group_pb.DeleteMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.MethodDefinition<api_multicast_group_pb.ListMulticastGroupsRequest, api_multicast_group_pb.ListMulticastGroupsResponse>;
  addDevice: grpc.MethodDefinition<api_multicast_group_pb.AddDeviceToMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  removeDevice: grpc.MethodDefinition<api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  addGateway: grpc.MethodDefinition<api_multicast_group_pb.AddGatewayToMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  removeGateway: grpc.MethodDefinition<api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  enqueue: grpc.MethodDefinition<api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest, api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse>;
  flushQueue: grpc.MethodDefinition<api_multicast_group_pb.FlushMulticastGroupQueueRequest, google_protobuf_empty_pb.Empty>;
  listQueue: grpc.MethodDefinition<api_multicast_group_pb.ListMulticastGroupQueueRequest, api_multicast_group_pb.ListMulticastGroupQueueResponse>;
}

export const MulticastGroupServiceService: IMulticastGroupServiceService;

export interface IMulticastGroupServiceServer extends grpc.UntypedServiceImplementation {
  create: grpc.handleUnaryCall<api_multicast_group_pb.CreateMulticastGroupRequest, api_multicast_group_pb.CreateMulticastGroupResponse>;
  get: grpc.handleUnaryCall<api_multicast_group_pb.GetMulticastGroupRequest, api_multicast_group_pb.GetMulticastGroupResponse>;
  update: grpc.handleUnaryCall<api_multicast_group_pb.UpdateMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.handleUnaryCall<api_multicast_group_pb.DeleteMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.handleUnaryCall<api_multicast_group_pb.ListMulticastGroupsRequest, api_multicast_group_pb.ListMulticastGroupsResponse>;
  addDevice: grpc.handleUnaryCall<api_multicast_group_pb.AddDeviceToMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  removeDevice: grpc.handleUnaryCall<api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  addGateway: grpc.handleUnaryCall<api_multicast_group_pb.AddGatewayToMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  removeGateway: grpc.handleUnaryCall<api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest, google_protobuf_empty_pb.Empty>;
  enqueue: grpc.handleUnaryCall<api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest, api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse>;
  flushQueue: grpc.handleUnaryCall<api_multicast_group_pb.FlushMulticastGroupQueueRequest, google_protobuf_empty_pb.Empty>;
  listQueue: grpc.handleUnaryCall<api_multicast_group_pb.ListMulticastGroupQueueRequest, api_multicast_group_pb.ListMulticastGroupQueueResponse>;
}

export class MulticastGroupServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  create(argument: api_multicast_group_pb.CreateMulticastGroupRequest, callback: grpc.requestCallback<api_multicast_group_pb.CreateMulticastGroupResponse>): grpc.ClientUnaryCall;
  create(argument: api_multicast_group_pb.CreateMulticastGroupRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.CreateMulticastGroupResponse>): grpc.ClientUnaryCall;
  create(argument: api_multicast_group_pb.CreateMulticastGroupRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.CreateMulticastGroupResponse>): grpc.ClientUnaryCall;
  get(argument: api_multicast_group_pb.GetMulticastGroupRequest, callback: grpc.requestCallback<api_multicast_group_pb.GetMulticastGroupResponse>): grpc.ClientUnaryCall;
  get(argument: api_multicast_group_pb.GetMulticastGroupRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.GetMulticastGroupResponse>): grpc.ClientUnaryCall;
  get(argument: api_multicast_group_pb.GetMulticastGroupRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.GetMulticastGroupResponse>): grpc.ClientUnaryCall;
  update(argument: api_multicast_group_pb.UpdateMulticastGroupRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_multicast_group_pb.UpdateMulticastGroupRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_multicast_group_pb.UpdateMulticastGroupRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_multicast_group_pb.DeleteMulticastGroupRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_multicast_group_pb.DeleteMulticastGroupRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_multicast_group_pb.DeleteMulticastGroupRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  list(argument: api_multicast_group_pb.ListMulticastGroupsRequest, callback: grpc.requestCallback<api_multicast_group_pb.ListMulticastGroupsResponse>): grpc.ClientUnaryCall;
  list(argument: api_multicast_group_pb.ListMulticastGroupsRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.ListMulticastGroupsResponse>): grpc.ClientUnaryCall;
  list(argument: api_multicast_group_pb.ListMulticastGroupsRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.ListMulticastGroupsResponse>): grpc.ClientUnaryCall;
  addDevice(argument: api_multicast_group_pb.AddDeviceToMulticastGroupRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addDevice(argument: api_multicast_group_pb.AddDeviceToMulticastGroupRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addDevice(argument: api_multicast_group_pb.AddDeviceToMulticastGroupRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeDevice(argument: api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeDevice(argument: api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeDevice(argument: api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addGateway(argument: api_multicast_group_pb.AddGatewayToMulticastGroupRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addGateway(argument: api_multicast_group_pb.AddGatewayToMulticastGroupRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addGateway(argument: api_multicast_group_pb.AddGatewayToMulticastGroupRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeGateway(argument: api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeGateway(argument: api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  removeGateway(argument: api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  enqueue(argument: api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest, callback: grpc.requestCallback<api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse>): grpc.ClientUnaryCall;
  enqueue(argument: api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse>): grpc.ClientUnaryCall;
  enqueue(argument: api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse>): grpc.ClientUnaryCall;
  flushQueue(argument: api_multicast_group_pb.FlushMulticastGroupQueueRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  flushQueue(argument: api_multicast_group_pb.FlushMulticastGroupQueueRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  flushQueue(argument: api_multicast_group_pb.FlushMulticastGroupQueueRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  listQueue(argument: api_multicast_group_pb.ListMulticastGroupQueueRequest, callback: grpc.requestCallback<api_multicast_group_pb.ListMulticastGroupQueueResponse>): grpc.ClientUnaryCall;
  listQueue(argument: api_multicast_group_pb.ListMulticastGroupQueueRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.ListMulticastGroupQueueResponse>): grpc.ClientUnaryCall;
  listQueue(argument: api_multicast_group_pb.ListMulticastGroupQueueRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_multicast_group_pb.ListMulticastGroupQueueResponse>): grpc.ClientUnaryCall;
}
