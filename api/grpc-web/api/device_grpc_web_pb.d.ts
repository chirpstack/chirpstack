import * as grpcWeb from 'grpc-web';

import * as api_device_pb from '../api/device_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class DeviceServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_device_pb.CreateDeviceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  get(
    request: api_device_pb.GetDeviceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.GetDeviceResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.GetDeviceResponse>;

  update(
    request: api_device_pb.UpdateDeviceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_device_pb.DeleteDeviceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  list(
    request: api_device_pb.ListDevicesRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.ListDevicesResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.ListDevicesResponse>;

  createKeys(
    request: api_device_pb.CreateDeviceKeysRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getKeys(
    request: api_device_pb.GetDeviceKeysRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.GetDeviceKeysResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.GetDeviceKeysResponse>;

  updateKeys(
    request: api_device_pb.UpdateDeviceKeysRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteKeys(
    request: api_device_pb.DeleteDeviceKeysRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  flushDevNonces(
    request: api_device_pb.FlushDevNoncesRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  activate(
    request: api_device_pb.ActivateDeviceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deactivate(
    request: api_device_pb.DeactivateDeviceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getActivation(
    request: api_device_pb.GetDeviceActivationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.GetDeviceActivationResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.GetDeviceActivationResponse>;

  getRandomDevAddr(
    request: api_device_pb.GetRandomDevAddrRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.GetRandomDevAddrResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.GetRandomDevAddrResponse>;

  getMetrics(
    request: api_device_pb.GetDeviceMetricsRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.GetDeviceMetricsResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.GetDeviceMetricsResponse>;

  getLinkMetrics(
    request: api_device_pb.GetDeviceLinkMetricsRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.GetDeviceLinkMetricsResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.GetDeviceLinkMetricsResponse>;

  enqueue(
    request: api_device_pb.EnqueueDeviceQueueItemRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.EnqueueDeviceQueueItemResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.EnqueueDeviceQueueItemResponse>;

  flushQueue(
    request: api_device_pb.FlushDeviceQueueRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getQueue(
    request: api_device_pb.GetDeviceQueueItemsRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.GetDeviceQueueItemsResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.GetDeviceQueueItemsResponse>;

  getNextFCntDown(
    request: api_device_pb.GetDeviceNextFCntDownRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_pb.GetDeviceNextFCntDownResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_pb.GetDeviceNextFCntDownResponse>;

}

export class DeviceServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_device_pb.CreateDeviceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  get(
    request: api_device_pb.GetDeviceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.GetDeviceResponse>;

  update(
    request: api_device_pb.UpdateDeviceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_device_pb.DeleteDeviceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  list(
    request: api_device_pb.ListDevicesRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.ListDevicesResponse>;

  createKeys(
    request: api_device_pb.CreateDeviceKeysRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getKeys(
    request: api_device_pb.GetDeviceKeysRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.GetDeviceKeysResponse>;

  updateKeys(
    request: api_device_pb.UpdateDeviceKeysRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteKeys(
    request: api_device_pb.DeleteDeviceKeysRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  flushDevNonces(
    request: api_device_pb.FlushDevNoncesRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  activate(
    request: api_device_pb.ActivateDeviceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deactivate(
    request: api_device_pb.DeactivateDeviceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getActivation(
    request: api_device_pb.GetDeviceActivationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.GetDeviceActivationResponse>;

  getRandomDevAddr(
    request: api_device_pb.GetRandomDevAddrRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.GetRandomDevAddrResponse>;

  getMetrics(
    request: api_device_pb.GetDeviceMetricsRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.GetDeviceMetricsResponse>;

  getLinkMetrics(
    request: api_device_pb.GetDeviceLinkMetricsRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.GetDeviceLinkMetricsResponse>;

  enqueue(
    request: api_device_pb.EnqueueDeviceQueueItemRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.EnqueueDeviceQueueItemResponse>;

  flushQueue(
    request: api_device_pb.FlushDeviceQueueRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getQueue(
    request: api_device_pb.GetDeviceQueueItemsRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.GetDeviceQueueItemsResponse>;

  getNextFCntDown(
    request: api_device_pb.GetDeviceNextFCntDownRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_pb.GetDeviceNextFCntDownResponse>;

}

