import * as grpcWeb from 'grpc-web';

import * as api_internal_pb from '../api/internal_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class InternalServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  login(
    request: api_internal_pb.LoginRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.LoginResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.LoginResponse>;

  profile(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.ProfileResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.ProfileResponse>;

  globalSearch(
    request: api_internal_pb.GlobalSearchRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.GlobalSearchResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.GlobalSearchResponse>;

  createApiKey(
    request: api_internal_pb.CreateApiKeyRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.CreateApiKeyResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.CreateApiKeyResponse>;

  deleteApiKey(
    request: api_internal_pb.DeleteApiKeyRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  listApiKeys(
    request: api_internal_pb.ListApiKeysRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.ListApiKeysResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.ListApiKeysResponse>;

  settings(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.SettingsResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.SettingsResponse>;

  openIdConnectLogin(
    request: api_internal_pb.OpenIdConnectLoginRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.OpenIdConnectLoginResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.OpenIdConnectLoginResponse>;

  getDevicesSummary(
    request: api_internal_pb.GetDevicesSummaryRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.GetDevicesSummaryResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.GetDevicesSummaryResponse>;

  getGatewaysSummary(
    request: api_internal_pb.GetGatewaysSummaryRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.GetGatewaysSummaryResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.GetGatewaysSummaryResponse>;

  streamGatewayFrames(
    request: api_internal_pb.StreamGatewayFramesRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<api_internal_pb.LogItem>;

  streamDeviceFrames(
    request: api_internal_pb.StreamDeviceFramesRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<api_internal_pb.LogItem>;

  streamDeviceEvents(
    request: api_internal_pb.StreamDeviceEventsRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<api_internal_pb.LogItem>;

  listRegions(
    request: google_protobuf_empty_pb.Empty,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.ListRegionsResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.ListRegionsResponse>;

  getRegion(
    request: api_internal_pb.GetRegionRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_internal_pb.GetRegionResponse) => void
  ): grpcWeb.ClientReadableStream<api_internal_pb.GetRegionResponse>;

}

export class InternalServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  login(
    request: api_internal_pb.LoginRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.LoginResponse>;

  profile(
    request: google_protobuf_empty_pb.Empty,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.ProfileResponse>;

  globalSearch(
    request: api_internal_pb.GlobalSearchRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.GlobalSearchResponse>;

  createApiKey(
    request: api_internal_pb.CreateApiKeyRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.CreateApiKeyResponse>;

  deleteApiKey(
    request: api_internal_pb.DeleteApiKeyRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  listApiKeys(
    request: api_internal_pb.ListApiKeysRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.ListApiKeysResponse>;

  settings(
    request: google_protobuf_empty_pb.Empty,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.SettingsResponse>;

  openIdConnectLogin(
    request: api_internal_pb.OpenIdConnectLoginRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.OpenIdConnectLoginResponse>;

  getDevicesSummary(
    request: api_internal_pb.GetDevicesSummaryRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.GetDevicesSummaryResponse>;

  getGatewaysSummary(
    request: api_internal_pb.GetGatewaysSummaryRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.GetGatewaysSummaryResponse>;

  streamGatewayFrames(
    request: api_internal_pb.StreamGatewayFramesRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<api_internal_pb.LogItem>;

  streamDeviceFrames(
    request: api_internal_pb.StreamDeviceFramesRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<api_internal_pb.LogItem>;

  streamDeviceEvents(
    request: api_internal_pb.StreamDeviceEventsRequest,
    metadata?: grpcWeb.Metadata
  ): grpcWeb.ClientReadableStream<api_internal_pb.LogItem>;

  listRegions(
    request: google_protobuf_empty_pb.Empty,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.ListRegionsResponse>;

  getRegion(
    request: api_internal_pb.GetRegionRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_internal_pb.GetRegionResponse>;

}

