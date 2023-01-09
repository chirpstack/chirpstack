import * as grpcWeb from 'grpc-web';

import * as api_device_profile_pb from '../api/device_profile_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class DeviceProfileServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_device_profile_pb.CreateDeviceProfileRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_profile_pb.CreateDeviceProfileResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_profile_pb.CreateDeviceProfileResponse>;

  get(
    request: api_device_profile_pb.GetDeviceProfileRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_profile_pb.GetDeviceProfileResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_profile_pb.GetDeviceProfileResponse>;

  update(
    request: api_device_profile_pb.UpdateDeviceProfileRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_device_profile_pb.DeleteDeviceProfileRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  list(
    request: api_device_profile_pb.ListDeviceProfilesRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_profile_pb.ListDeviceProfilesResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_profile_pb.ListDeviceProfilesResponse>;

  listAdrAlgorithms(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse>;

}

export class DeviceProfileServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_device_profile_pb.CreateDeviceProfileRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_profile_pb.CreateDeviceProfileResponse>;

  get(
    request: api_device_profile_pb.GetDeviceProfileRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_profile_pb.GetDeviceProfileResponse>;

  update(
    request: api_device_profile_pb.UpdateDeviceProfileRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_device_profile_pb.DeleteDeviceProfileRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  list(
    request: api_device_profile_pb.ListDeviceProfilesRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_profile_pb.ListDeviceProfilesResponse>;

  listAdrAlgorithms(
    request: google_protobuf_empty_pb.Empty,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse>;

}

