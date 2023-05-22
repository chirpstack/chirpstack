import * as grpcWeb from 'grpc-web';

import * as api_relay_pb from '../api/relay_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class RelayServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  list(
    request: api_relay_pb.ListRelaysRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_relay_pb.ListRelaysResponse) => void
  ): grpcWeb.ClientReadableStream<api_relay_pb.ListRelaysResponse>;

  addDevice(
    request: api_relay_pb.AddRelayDeviceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  removeDevice(
    request: api_relay_pb.RemoveRelayDeviceRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  listDevices(
    request: api_relay_pb.ListRelayDevicesRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_relay_pb.ListRelayDevicesResponse) => void
  ): grpcWeb.ClientReadableStream<api_relay_pb.ListRelayDevicesResponse>;

}

export class RelayServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  list(
    request: api_relay_pb.ListRelaysRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_relay_pb.ListRelaysResponse>;

  addDevice(
    request: api_relay_pb.AddRelayDeviceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  removeDevice(
    request: api_relay_pb.RemoveRelayDeviceRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  listDevices(
    request: api_relay_pb.ListRelayDevicesRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_relay_pb.ListRelayDevicesResponse>;

}

