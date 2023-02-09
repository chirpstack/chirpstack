import * as grpcWeb from 'grpc-web';

import * as api_multicast_group_pb from '../api/multicast_group_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class MulticastGroupServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_multicast_group_pb.CreateMulticastGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_multicast_group_pb.CreateMulticastGroupResponse) => void
  ): grpcWeb.ClientReadableStream<api_multicast_group_pb.CreateMulticastGroupResponse>;

  get(
    request: api_multicast_group_pb.GetMulticastGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_multicast_group_pb.GetMulticastGroupResponse) => void
  ): grpcWeb.ClientReadableStream<api_multicast_group_pb.GetMulticastGroupResponse>;

  update(
    request: api_multicast_group_pb.UpdateMulticastGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_multicast_group_pb.DeleteMulticastGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  list(
    request: api_multicast_group_pb.ListMulticastGroupsRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_multicast_group_pb.ListMulticastGroupsResponse) => void
  ): grpcWeb.ClientReadableStream<api_multicast_group_pb.ListMulticastGroupsResponse>;

  addDevice(
    request: api_multicast_group_pb.AddDeviceToMulticastGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  removeDevice(
    request: api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  addGateway(
    request: api_multicast_group_pb.AddGatewayToMulticastGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  removeGateway(
    request: api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  enqueue(
    request: api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse) => void
  ): grpcWeb.ClientReadableStream<api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse>;

  flushQueue(
    request: api_multicast_group_pb.FlushMulticastGroupQueueRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  listQueue(
    request: api_multicast_group_pb.ListMulticastGroupQueueRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_multicast_group_pb.ListMulticastGroupQueueResponse) => void
  ): grpcWeb.ClientReadableStream<api_multicast_group_pb.ListMulticastGroupQueueResponse>;

}

export class MulticastGroupServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_multicast_group_pb.CreateMulticastGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_multicast_group_pb.CreateMulticastGroupResponse>;

  get(
    request: api_multicast_group_pb.GetMulticastGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_multicast_group_pb.GetMulticastGroupResponse>;

  update(
    request: api_multicast_group_pb.UpdateMulticastGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_multicast_group_pb.DeleteMulticastGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  list(
    request: api_multicast_group_pb.ListMulticastGroupsRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_multicast_group_pb.ListMulticastGroupsResponse>;

  addDevice(
    request: api_multicast_group_pb.AddDeviceToMulticastGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  removeDevice(
    request: api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  addGateway(
    request: api_multicast_group_pb.AddGatewayToMulticastGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  removeGateway(
    request: api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  enqueue(
    request: api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse>;

  flushQueue(
    request: api_multicast_group_pb.FlushMulticastGroupQueueRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  listQueue(
    request: api_multicast_group_pb.ListMulticastGroupQueueRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_multicast_group_pb.ListMulticastGroupQueueResponse>;

}

