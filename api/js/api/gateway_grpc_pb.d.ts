// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/gateway.proto

import * as api_gateway_pb from "../api/gateway_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface IGatewayServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  create: grpc.MethodDefinition<api_gateway_pb.CreateGatewayRequest, google_protobuf_empty_pb.Empty>;
  get: grpc.MethodDefinition<api_gateway_pb.GetGatewayRequest, api_gateway_pb.GetGatewayResponse>;
  update: grpc.MethodDefinition<api_gateway_pb.UpdateGatewayRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.MethodDefinition<api_gateway_pb.DeleteGatewayRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.MethodDefinition<api_gateway_pb.ListGatewaysRequest, api_gateway_pb.ListGatewaysResponse>;
  generateClientCertificate: grpc.MethodDefinition<api_gateway_pb.GenerateGatewayClientCertificateRequest, api_gateway_pb.GenerateGatewayClientCertificateResponse>;
  getMetrics: grpc.MethodDefinition<api_gateway_pb.GetGatewayMetricsRequest, api_gateway_pb.GetGatewayMetricsResponse>;
}

export const GatewayServiceService: IGatewayServiceService;

export interface IGatewayServiceServer extends grpc.UntypedServiceImplementation {
  create: grpc.handleUnaryCall<api_gateway_pb.CreateGatewayRequest, google_protobuf_empty_pb.Empty>;
  get: grpc.handleUnaryCall<api_gateway_pb.GetGatewayRequest, api_gateway_pb.GetGatewayResponse>;
  update: grpc.handleUnaryCall<api_gateway_pb.UpdateGatewayRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.handleUnaryCall<api_gateway_pb.DeleteGatewayRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.handleUnaryCall<api_gateway_pb.ListGatewaysRequest, api_gateway_pb.ListGatewaysResponse>;
  generateClientCertificate: grpc.handleUnaryCall<api_gateway_pb.GenerateGatewayClientCertificateRequest, api_gateway_pb.GenerateGatewayClientCertificateResponse>;
  getMetrics: grpc.handleUnaryCall<api_gateway_pb.GetGatewayMetricsRequest, api_gateway_pb.GetGatewayMetricsResponse>;
}

export class GatewayServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  create(argument: api_gateway_pb.CreateGatewayRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  create(argument: api_gateway_pb.CreateGatewayRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  create(argument: api_gateway_pb.CreateGatewayRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  get(argument: api_gateway_pb.GetGatewayRequest, callback: grpc.requestCallback<api_gateway_pb.GetGatewayResponse>): grpc.ClientUnaryCall;
  get(argument: api_gateway_pb.GetGatewayRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_gateway_pb.GetGatewayResponse>): grpc.ClientUnaryCall;
  get(argument: api_gateway_pb.GetGatewayRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_gateway_pb.GetGatewayResponse>): grpc.ClientUnaryCall;
  update(argument: api_gateway_pb.UpdateGatewayRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_gateway_pb.UpdateGatewayRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_gateway_pb.UpdateGatewayRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_gateway_pb.DeleteGatewayRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_gateway_pb.DeleteGatewayRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_gateway_pb.DeleteGatewayRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  list(argument: api_gateway_pb.ListGatewaysRequest, callback: grpc.requestCallback<api_gateway_pb.ListGatewaysResponse>): grpc.ClientUnaryCall;
  list(argument: api_gateway_pb.ListGatewaysRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_gateway_pb.ListGatewaysResponse>): grpc.ClientUnaryCall;
  list(argument: api_gateway_pb.ListGatewaysRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_gateway_pb.ListGatewaysResponse>): grpc.ClientUnaryCall;
  generateClientCertificate(argument: api_gateway_pb.GenerateGatewayClientCertificateRequest, callback: grpc.requestCallback<api_gateway_pb.GenerateGatewayClientCertificateResponse>): grpc.ClientUnaryCall;
  generateClientCertificate(argument: api_gateway_pb.GenerateGatewayClientCertificateRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_gateway_pb.GenerateGatewayClientCertificateResponse>): grpc.ClientUnaryCall;
  generateClientCertificate(argument: api_gateway_pb.GenerateGatewayClientCertificateRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_gateway_pb.GenerateGatewayClientCertificateResponse>): grpc.ClientUnaryCall;
  getMetrics(argument: api_gateway_pb.GetGatewayMetricsRequest, callback: grpc.requestCallback<api_gateway_pb.GetGatewayMetricsResponse>): grpc.ClientUnaryCall;
  getMetrics(argument: api_gateway_pb.GetGatewayMetricsRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_gateway_pb.GetGatewayMetricsResponse>): grpc.ClientUnaryCall;
  getMetrics(argument: api_gateway_pb.GetGatewayMetricsRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_gateway_pb.GetGatewayMetricsResponse>): grpc.ClientUnaryCall;
}
