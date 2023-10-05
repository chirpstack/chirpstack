// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/device.proto

import * as api_device_pb from "../api/device_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface IDeviceServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  create: grpc.MethodDefinition<api_device_pb.CreateDeviceRequest, google_protobuf_empty_pb.Empty>;
  get: grpc.MethodDefinition<api_device_pb.GetDeviceRequest, api_device_pb.GetDeviceResponse>;
  update: grpc.MethodDefinition<api_device_pb.UpdateDeviceRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.MethodDefinition<api_device_pb.DeleteDeviceRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.MethodDefinition<api_device_pb.ListDevicesRequest, api_device_pb.ListDevicesResponse>;
  createKeys: grpc.MethodDefinition<api_device_pb.CreateDeviceKeysRequest, google_protobuf_empty_pb.Empty>;
  getKeys: grpc.MethodDefinition<api_device_pb.GetDeviceKeysRequest, api_device_pb.GetDeviceKeysResponse>;
  updateKeys: grpc.MethodDefinition<api_device_pb.UpdateDeviceKeysRequest, google_protobuf_empty_pb.Empty>;
  deleteKeys: grpc.MethodDefinition<api_device_pb.DeleteDeviceKeysRequest, google_protobuf_empty_pb.Empty>;
  flushDevNonces: grpc.MethodDefinition<api_device_pb.FlushDevNoncesRequest, google_protobuf_empty_pb.Empty>;
  activate: grpc.MethodDefinition<api_device_pb.ActivateDeviceRequest, google_protobuf_empty_pb.Empty>;
  deactivate: grpc.MethodDefinition<api_device_pb.DeactivateDeviceRequest, google_protobuf_empty_pb.Empty>;
  getActivation: grpc.MethodDefinition<api_device_pb.GetDeviceActivationRequest, api_device_pb.GetDeviceActivationResponse>;
  getRandomDevAddr: grpc.MethodDefinition<api_device_pb.GetRandomDevAddrRequest, api_device_pb.GetRandomDevAddrResponse>;
  getMetrics: grpc.MethodDefinition<api_device_pb.GetDeviceMetricsRequest, api_device_pb.GetDeviceMetricsResponse>;
  getLinkMetrics: grpc.MethodDefinition<api_device_pb.GetDeviceLinkMetricsRequest, api_device_pb.GetDeviceLinkMetricsResponse>;
  enqueue: grpc.MethodDefinition<api_device_pb.EnqueueDeviceQueueItemRequest, api_device_pb.EnqueueDeviceQueueItemResponse>;
  flushQueue: grpc.MethodDefinition<api_device_pb.FlushDeviceQueueRequest, google_protobuf_empty_pb.Empty>;
  getQueue: grpc.MethodDefinition<api_device_pb.GetDeviceQueueItemsRequest, api_device_pb.GetDeviceQueueItemsResponse>;
  getNextFCntDown: grpc.MethodDefinition<api_device_pb.GetDeviceNextFCntDownRequest, api_device_pb.GetDeviceNextFCntDownResponse>;
}

export const DeviceServiceService: IDeviceServiceService;

export interface IDeviceServiceServer extends grpc.UntypedServiceImplementation {
  create: grpc.handleUnaryCall<api_device_pb.CreateDeviceRequest, google_protobuf_empty_pb.Empty>;
  get: grpc.handleUnaryCall<api_device_pb.GetDeviceRequest, api_device_pb.GetDeviceResponse>;
  update: grpc.handleUnaryCall<api_device_pb.UpdateDeviceRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.handleUnaryCall<api_device_pb.DeleteDeviceRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.handleUnaryCall<api_device_pb.ListDevicesRequest, api_device_pb.ListDevicesResponse>;
  createKeys: grpc.handleUnaryCall<api_device_pb.CreateDeviceKeysRequest, google_protobuf_empty_pb.Empty>;
  getKeys: grpc.handleUnaryCall<api_device_pb.GetDeviceKeysRequest, api_device_pb.GetDeviceKeysResponse>;
  updateKeys: grpc.handleUnaryCall<api_device_pb.UpdateDeviceKeysRequest, google_protobuf_empty_pb.Empty>;
  deleteKeys: grpc.handleUnaryCall<api_device_pb.DeleteDeviceKeysRequest, google_protobuf_empty_pb.Empty>;
  flushDevNonces: grpc.handleUnaryCall<api_device_pb.FlushDevNoncesRequest, google_protobuf_empty_pb.Empty>;
  activate: grpc.handleUnaryCall<api_device_pb.ActivateDeviceRequest, google_protobuf_empty_pb.Empty>;
  deactivate: grpc.handleUnaryCall<api_device_pb.DeactivateDeviceRequest, google_protobuf_empty_pb.Empty>;
  getActivation: grpc.handleUnaryCall<api_device_pb.GetDeviceActivationRequest, api_device_pb.GetDeviceActivationResponse>;
  getRandomDevAddr: grpc.handleUnaryCall<api_device_pb.GetRandomDevAddrRequest, api_device_pb.GetRandomDevAddrResponse>;
  getMetrics: grpc.handleUnaryCall<api_device_pb.GetDeviceMetricsRequest, api_device_pb.GetDeviceMetricsResponse>;
  getLinkMetrics: grpc.handleUnaryCall<api_device_pb.GetDeviceLinkMetricsRequest, api_device_pb.GetDeviceLinkMetricsResponse>;
  enqueue: grpc.handleUnaryCall<api_device_pb.EnqueueDeviceQueueItemRequest, api_device_pb.EnqueueDeviceQueueItemResponse>;
  flushQueue: grpc.handleUnaryCall<api_device_pb.FlushDeviceQueueRequest, google_protobuf_empty_pb.Empty>;
  getQueue: grpc.handleUnaryCall<api_device_pb.GetDeviceQueueItemsRequest, api_device_pb.GetDeviceQueueItemsResponse>;
  getNextFCntDown: grpc.handleUnaryCall<api_device_pb.GetDeviceNextFCntDownRequest, api_device_pb.GetDeviceNextFCntDownResponse>;
}

export class DeviceServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  create(argument: api_device_pb.CreateDeviceRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  create(argument: api_device_pb.CreateDeviceRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  create(argument: api_device_pb.CreateDeviceRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  get(argument: api_device_pb.GetDeviceRequest, callback: grpc.requestCallback<api_device_pb.GetDeviceResponse>): grpc.ClientUnaryCall;
  get(argument: api_device_pb.GetDeviceRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceResponse>): grpc.ClientUnaryCall;
  get(argument: api_device_pb.GetDeviceRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceResponse>): grpc.ClientUnaryCall;
  update(argument: api_device_pb.UpdateDeviceRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_device_pb.UpdateDeviceRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_device_pb.UpdateDeviceRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_pb.DeleteDeviceRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_pb.DeleteDeviceRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_device_pb.DeleteDeviceRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  list(argument: api_device_pb.ListDevicesRequest, callback: grpc.requestCallback<api_device_pb.ListDevicesResponse>): grpc.ClientUnaryCall;
  list(argument: api_device_pb.ListDevicesRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.ListDevicesResponse>): grpc.ClientUnaryCall;
  list(argument: api_device_pb.ListDevicesRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.ListDevicesResponse>): grpc.ClientUnaryCall;
  createKeys(argument: api_device_pb.CreateDeviceKeysRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  createKeys(argument: api_device_pb.CreateDeviceKeysRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  createKeys(argument: api_device_pb.CreateDeviceKeysRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  getKeys(argument: api_device_pb.GetDeviceKeysRequest, callback: grpc.requestCallback<api_device_pb.GetDeviceKeysResponse>): grpc.ClientUnaryCall;
  getKeys(argument: api_device_pb.GetDeviceKeysRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceKeysResponse>): grpc.ClientUnaryCall;
  getKeys(argument: api_device_pb.GetDeviceKeysRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceKeysResponse>): grpc.ClientUnaryCall;
  updateKeys(argument: api_device_pb.UpdateDeviceKeysRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  updateKeys(argument: api_device_pb.UpdateDeviceKeysRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  updateKeys(argument: api_device_pb.UpdateDeviceKeysRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deleteKeys(argument: api_device_pb.DeleteDeviceKeysRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deleteKeys(argument: api_device_pb.DeleteDeviceKeysRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deleteKeys(argument: api_device_pb.DeleteDeviceKeysRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  flushDevNonces(argument: api_device_pb.FlushDevNoncesRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  flushDevNonces(argument: api_device_pb.FlushDevNoncesRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  flushDevNonces(argument: api_device_pb.FlushDevNoncesRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  activate(argument: api_device_pb.ActivateDeviceRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  activate(argument: api_device_pb.ActivateDeviceRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  activate(argument: api_device_pb.ActivateDeviceRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deactivate(argument: api_device_pb.DeactivateDeviceRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deactivate(argument: api_device_pb.DeactivateDeviceRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deactivate(argument: api_device_pb.DeactivateDeviceRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  getActivation(argument: api_device_pb.GetDeviceActivationRequest, callback: grpc.requestCallback<api_device_pb.GetDeviceActivationResponse>): grpc.ClientUnaryCall;
  getActivation(argument: api_device_pb.GetDeviceActivationRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceActivationResponse>): grpc.ClientUnaryCall;
  getActivation(argument: api_device_pb.GetDeviceActivationRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceActivationResponse>): grpc.ClientUnaryCall;
  getRandomDevAddr(argument: api_device_pb.GetRandomDevAddrRequest, callback: grpc.requestCallback<api_device_pb.GetRandomDevAddrResponse>): grpc.ClientUnaryCall;
  getRandomDevAddr(argument: api_device_pb.GetRandomDevAddrRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetRandomDevAddrResponse>): grpc.ClientUnaryCall;
  getRandomDevAddr(argument: api_device_pb.GetRandomDevAddrRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetRandomDevAddrResponse>): grpc.ClientUnaryCall;
  getMetrics(argument: api_device_pb.GetDeviceMetricsRequest, callback: grpc.requestCallback<api_device_pb.GetDeviceMetricsResponse>): grpc.ClientUnaryCall;
  getMetrics(argument: api_device_pb.GetDeviceMetricsRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceMetricsResponse>): grpc.ClientUnaryCall;
  getMetrics(argument: api_device_pb.GetDeviceMetricsRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceMetricsResponse>): grpc.ClientUnaryCall;
  getLinkMetrics(argument: api_device_pb.GetDeviceLinkMetricsRequest, callback: grpc.requestCallback<api_device_pb.GetDeviceLinkMetricsResponse>): grpc.ClientUnaryCall;
  getLinkMetrics(argument: api_device_pb.GetDeviceLinkMetricsRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceLinkMetricsResponse>): grpc.ClientUnaryCall;
  getLinkMetrics(argument: api_device_pb.GetDeviceLinkMetricsRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceLinkMetricsResponse>): grpc.ClientUnaryCall;
  enqueue(argument: api_device_pb.EnqueueDeviceQueueItemRequest, callback: grpc.requestCallback<api_device_pb.EnqueueDeviceQueueItemResponse>): grpc.ClientUnaryCall;
  enqueue(argument: api_device_pb.EnqueueDeviceQueueItemRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.EnqueueDeviceQueueItemResponse>): grpc.ClientUnaryCall;
  enqueue(argument: api_device_pb.EnqueueDeviceQueueItemRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.EnqueueDeviceQueueItemResponse>): grpc.ClientUnaryCall;
  flushQueue(argument: api_device_pb.FlushDeviceQueueRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  flushQueue(argument: api_device_pb.FlushDeviceQueueRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  flushQueue(argument: api_device_pb.FlushDeviceQueueRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  getQueue(argument: api_device_pb.GetDeviceQueueItemsRequest, callback: grpc.requestCallback<api_device_pb.GetDeviceQueueItemsResponse>): grpc.ClientUnaryCall;
  getQueue(argument: api_device_pb.GetDeviceQueueItemsRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceQueueItemsResponse>): grpc.ClientUnaryCall;
  getQueue(argument: api_device_pb.GetDeviceQueueItemsRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceQueueItemsResponse>): grpc.ClientUnaryCall;
  getNextFCntDown(argument: api_device_pb.GetDeviceNextFCntDownRequest, callback: grpc.requestCallback<api_device_pb.GetDeviceNextFCntDownResponse>): grpc.ClientUnaryCall;
  getNextFCntDown(argument: api_device_pb.GetDeviceNextFCntDownRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceNextFCntDownResponse>): grpc.ClientUnaryCall;
  getNextFCntDown(argument: api_device_pb.GetDeviceNextFCntDownRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_device_pb.GetDeviceNextFCntDownResponse>): grpc.ClientUnaryCall;
}
