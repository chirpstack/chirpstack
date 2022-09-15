// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/internal.proto

import * as api_internal_pb from "../api/internal_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface IInternalServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  login: grpc.MethodDefinition<api_internal_pb.LoginRequest, api_internal_pb.LoginResponse>;
  profile: grpc.MethodDefinition<google_protobuf_empty_pb.Empty, api_internal_pb.ProfileResponse>;
  globalSearch: grpc.MethodDefinition<api_internal_pb.GlobalSearchRequest, api_internal_pb.GlobalSearchResponse>;
  createApiKey: grpc.MethodDefinition<api_internal_pb.CreateApiKeyRequest, api_internal_pb.CreateApiKeyResponse>;
  deleteApiKey: grpc.MethodDefinition<api_internal_pb.DeleteApiKeyRequest, google_protobuf_empty_pb.Empty>;
  listApiKeys: grpc.MethodDefinition<api_internal_pb.ListApiKeysRequest, api_internal_pb.ListApiKeysResponse>;
  settings: grpc.MethodDefinition<google_protobuf_empty_pb.Empty, api_internal_pb.SettingsResponse>;
  openIdConnectLogin: grpc.MethodDefinition<api_internal_pb.OpenIdConnectLoginRequest, api_internal_pb.OpenIdConnectLoginResponse>;
  getDevicesSummary: grpc.MethodDefinition<api_internal_pb.GetDevicesSummaryRequest, api_internal_pb.GetDevicesSummaryResponse>;
  getGatewaysSummary: grpc.MethodDefinition<api_internal_pb.GetGatewaysSummaryRequest, api_internal_pb.GetGatewaysSummaryResponse>;
  streamGatewayFrames: grpc.MethodDefinition<api_internal_pb.StreamGatewayFramesRequest, api_internal_pb.LogItem>;
  streamDeviceFrames: grpc.MethodDefinition<api_internal_pb.StreamDeviceFramesRequest, api_internal_pb.LogItem>;
  streamDeviceEvents: grpc.MethodDefinition<api_internal_pb.StreamDeviceEventsRequest, api_internal_pb.LogItem>;
  listRegions: grpc.MethodDefinition<google_protobuf_empty_pb.Empty, api_internal_pb.ListRegionsResponse>;
  getRegion: grpc.MethodDefinition<api_internal_pb.GetRegionRequest, api_internal_pb.GetRegionResponse>;
}

export const InternalServiceService: IInternalServiceService;

export interface IInternalServiceServer extends grpc.UntypedServiceImplementation {
  login: grpc.handleUnaryCall<api_internal_pb.LoginRequest, api_internal_pb.LoginResponse>;
  profile: grpc.handleUnaryCall<google_protobuf_empty_pb.Empty, api_internal_pb.ProfileResponse>;
  globalSearch: grpc.handleUnaryCall<api_internal_pb.GlobalSearchRequest, api_internal_pb.GlobalSearchResponse>;
  createApiKey: grpc.handleUnaryCall<api_internal_pb.CreateApiKeyRequest, api_internal_pb.CreateApiKeyResponse>;
  deleteApiKey: grpc.handleUnaryCall<api_internal_pb.DeleteApiKeyRequest, google_protobuf_empty_pb.Empty>;
  listApiKeys: grpc.handleUnaryCall<api_internal_pb.ListApiKeysRequest, api_internal_pb.ListApiKeysResponse>;
  settings: grpc.handleUnaryCall<google_protobuf_empty_pb.Empty, api_internal_pb.SettingsResponse>;
  openIdConnectLogin: grpc.handleUnaryCall<api_internal_pb.OpenIdConnectLoginRequest, api_internal_pb.OpenIdConnectLoginResponse>;
  getDevicesSummary: grpc.handleUnaryCall<api_internal_pb.GetDevicesSummaryRequest, api_internal_pb.GetDevicesSummaryResponse>;
  getGatewaysSummary: grpc.handleUnaryCall<api_internal_pb.GetGatewaysSummaryRequest, api_internal_pb.GetGatewaysSummaryResponse>;
  streamGatewayFrames: grpc.handleServerStreamingCall<api_internal_pb.StreamGatewayFramesRequest, api_internal_pb.LogItem>;
  streamDeviceFrames: grpc.handleServerStreamingCall<api_internal_pb.StreamDeviceFramesRequest, api_internal_pb.LogItem>;
  streamDeviceEvents: grpc.handleServerStreamingCall<api_internal_pb.StreamDeviceEventsRequest, api_internal_pb.LogItem>;
  listRegions: grpc.handleUnaryCall<google_protobuf_empty_pb.Empty, api_internal_pb.ListRegionsResponse>;
  getRegion: grpc.handleUnaryCall<api_internal_pb.GetRegionRequest, api_internal_pb.GetRegionResponse>;
}

export class InternalServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  login(argument: api_internal_pb.LoginRequest, callback: grpc.requestCallback<api_internal_pb.LoginResponse>): grpc.ClientUnaryCall;
  login(argument: api_internal_pb.LoginRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.LoginResponse>): grpc.ClientUnaryCall;
  login(argument: api_internal_pb.LoginRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.LoginResponse>): grpc.ClientUnaryCall;
  profile(argument: google_protobuf_empty_pb.Empty, callback: grpc.requestCallback<api_internal_pb.ProfileResponse>): grpc.ClientUnaryCall;
  profile(argument: google_protobuf_empty_pb.Empty, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.ProfileResponse>): grpc.ClientUnaryCall;
  profile(argument: google_protobuf_empty_pb.Empty, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.ProfileResponse>): grpc.ClientUnaryCall;
  globalSearch(argument: api_internal_pb.GlobalSearchRequest, callback: grpc.requestCallback<api_internal_pb.GlobalSearchResponse>): grpc.ClientUnaryCall;
  globalSearch(argument: api_internal_pb.GlobalSearchRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.GlobalSearchResponse>): grpc.ClientUnaryCall;
  globalSearch(argument: api_internal_pb.GlobalSearchRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.GlobalSearchResponse>): grpc.ClientUnaryCall;
  createApiKey(argument: api_internal_pb.CreateApiKeyRequest, callback: grpc.requestCallback<api_internal_pb.CreateApiKeyResponse>): grpc.ClientUnaryCall;
  createApiKey(argument: api_internal_pb.CreateApiKeyRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.CreateApiKeyResponse>): grpc.ClientUnaryCall;
  createApiKey(argument: api_internal_pb.CreateApiKeyRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.CreateApiKeyResponse>): grpc.ClientUnaryCall;
  deleteApiKey(argument: api_internal_pb.DeleteApiKeyRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deleteApiKey(argument: api_internal_pb.DeleteApiKeyRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deleteApiKey(argument: api_internal_pb.DeleteApiKeyRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  listApiKeys(argument: api_internal_pb.ListApiKeysRequest, callback: grpc.requestCallback<api_internal_pb.ListApiKeysResponse>): grpc.ClientUnaryCall;
  listApiKeys(argument: api_internal_pb.ListApiKeysRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.ListApiKeysResponse>): grpc.ClientUnaryCall;
  listApiKeys(argument: api_internal_pb.ListApiKeysRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.ListApiKeysResponse>): grpc.ClientUnaryCall;
  settings(argument: google_protobuf_empty_pb.Empty, callback: grpc.requestCallback<api_internal_pb.SettingsResponse>): grpc.ClientUnaryCall;
  settings(argument: google_protobuf_empty_pb.Empty, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.SettingsResponse>): grpc.ClientUnaryCall;
  settings(argument: google_protobuf_empty_pb.Empty, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.SettingsResponse>): grpc.ClientUnaryCall;
  openIdConnectLogin(argument: api_internal_pb.OpenIdConnectLoginRequest, callback: grpc.requestCallback<api_internal_pb.OpenIdConnectLoginResponse>): grpc.ClientUnaryCall;
  openIdConnectLogin(argument: api_internal_pb.OpenIdConnectLoginRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.OpenIdConnectLoginResponse>): grpc.ClientUnaryCall;
  openIdConnectLogin(argument: api_internal_pb.OpenIdConnectLoginRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.OpenIdConnectLoginResponse>): grpc.ClientUnaryCall;
  getDevicesSummary(argument: api_internal_pb.GetDevicesSummaryRequest, callback: grpc.requestCallback<api_internal_pb.GetDevicesSummaryResponse>): grpc.ClientUnaryCall;
  getDevicesSummary(argument: api_internal_pb.GetDevicesSummaryRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.GetDevicesSummaryResponse>): grpc.ClientUnaryCall;
  getDevicesSummary(argument: api_internal_pb.GetDevicesSummaryRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.GetDevicesSummaryResponse>): grpc.ClientUnaryCall;
  getGatewaysSummary(argument: api_internal_pb.GetGatewaysSummaryRequest, callback: grpc.requestCallback<api_internal_pb.GetGatewaysSummaryResponse>): grpc.ClientUnaryCall;
  getGatewaysSummary(argument: api_internal_pb.GetGatewaysSummaryRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.GetGatewaysSummaryResponse>): grpc.ClientUnaryCall;
  getGatewaysSummary(argument: api_internal_pb.GetGatewaysSummaryRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.GetGatewaysSummaryResponse>): grpc.ClientUnaryCall;
  streamGatewayFrames(argument: api_internal_pb.StreamGatewayFramesRequest, metadataOrOptions?: grpc.Metadata | grpc.CallOptions | null): grpc.ClientReadableStream<api_internal_pb.LogItem>;
  streamGatewayFrames(argument: api_internal_pb.StreamGatewayFramesRequest, metadata?: grpc.Metadata | null, options?: grpc.CallOptions | null): grpc.ClientReadableStream<api_internal_pb.LogItem>;
  streamDeviceFrames(argument: api_internal_pb.StreamDeviceFramesRequest, metadataOrOptions?: grpc.Metadata | grpc.CallOptions | null): grpc.ClientReadableStream<api_internal_pb.LogItem>;
  streamDeviceFrames(argument: api_internal_pb.StreamDeviceFramesRequest, metadata?: grpc.Metadata | null, options?: grpc.CallOptions | null): grpc.ClientReadableStream<api_internal_pb.LogItem>;
  streamDeviceEvents(argument: api_internal_pb.StreamDeviceEventsRequest, metadataOrOptions?: grpc.Metadata | grpc.CallOptions | null): grpc.ClientReadableStream<api_internal_pb.LogItem>;
  streamDeviceEvents(argument: api_internal_pb.StreamDeviceEventsRequest, metadata?: grpc.Metadata | null, options?: grpc.CallOptions | null): grpc.ClientReadableStream<api_internal_pb.LogItem>;
  listRegions(argument: google_protobuf_empty_pb.Empty, callback: grpc.requestCallback<api_internal_pb.ListRegionsResponse>): grpc.ClientUnaryCall;
  listRegions(argument: google_protobuf_empty_pb.Empty, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.ListRegionsResponse>): grpc.ClientUnaryCall;
  listRegions(argument: google_protobuf_empty_pb.Empty, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.ListRegionsResponse>): grpc.ClientUnaryCall;
  getRegion(argument: api_internal_pb.GetRegionRequest, callback: grpc.requestCallback<api_internal_pb.GetRegionResponse>): grpc.ClientUnaryCall;
  getRegion(argument: api_internal_pb.GetRegionRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.GetRegionResponse>): grpc.ClientUnaryCall;
  getRegion(argument: api_internal_pb.GetRegionRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_internal_pb.GetRegionResponse>): grpc.ClientUnaryCall;
}
