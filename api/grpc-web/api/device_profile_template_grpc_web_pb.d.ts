import * as grpcWeb from 'grpc-web';

import * as api_device_profile_template_pb from '../api/device_profile_template_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class DeviceProfileTemplateServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_device_profile_template_pb.CreateDeviceProfileTemplateRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  get(
    request: api_device_profile_template_pb.GetDeviceProfileTemplateRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_profile_template_pb.GetDeviceProfileTemplateResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_profile_template_pb.GetDeviceProfileTemplateResponse>;

  update(
    request: api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  list(
    request: api_device_profile_template_pb.ListDeviceProfileTemplatesRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_device_profile_template_pb.ListDeviceProfileTemplatesResponse) => void
  ): grpcWeb.ClientReadableStream<api_device_profile_template_pb.ListDeviceProfileTemplatesResponse>;

}

export class DeviceProfileTemplateServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_device_profile_template_pb.CreateDeviceProfileTemplateRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  get(
    request: api_device_profile_template_pb.GetDeviceProfileTemplateRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_profile_template_pb.GetDeviceProfileTemplateResponse>;

  update(
    request: api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  list(
    request: api_device_profile_template_pb.ListDeviceProfileTemplatesRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_device_profile_template_pb.ListDeviceProfileTemplatesResponse>;

}

