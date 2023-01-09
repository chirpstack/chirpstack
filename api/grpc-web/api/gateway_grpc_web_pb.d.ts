import * as grpcWeb from 'grpc-web';

import * as api_gateway_pb from '../api/gateway_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class GatewayServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_gateway_pb.CreateGatewayRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  get(
    request: api_gateway_pb.GetGatewayRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_gateway_pb.GetGatewayResponse) => void
  ): grpcWeb.ClientReadableStream<api_gateway_pb.GetGatewayResponse>;

  update(
    request: api_gateway_pb.UpdateGatewayRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_gateway_pb.DeleteGatewayRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  list(
    request: api_gateway_pb.ListGatewaysRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_gateway_pb.ListGatewaysResponse) => void
  ): grpcWeb.ClientReadableStream<api_gateway_pb.ListGatewaysResponse>;

  generateClientCertificate(
    request: api_gateway_pb.GenerateGatewayClientCertificateRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_gateway_pb.GenerateGatewayClientCertificateResponse) => void
  ): grpcWeb.ClientReadableStream<api_gateway_pb.GenerateGatewayClientCertificateResponse>;

  getMetrics(
    request: api_gateway_pb.GetGatewayMetricsRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_gateway_pb.GetGatewayMetricsResponse) => void
  ): grpcWeb.ClientReadableStream<api_gateway_pb.GetGatewayMetricsResponse>;

}

export class GatewayServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_gateway_pb.CreateGatewayRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  get(
    request: api_gateway_pb.GetGatewayRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_gateway_pb.GetGatewayResponse>;

  update(
    request: api_gateway_pb.UpdateGatewayRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_gateway_pb.DeleteGatewayRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  list(
    request: api_gateway_pb.ListGatewaysRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_gateway_pb.ListGatewaysResponse>;

  generateClientCertificate(
    request: api_gateway_pb.GenerateGatewayClientCertificateRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_gateway_pb.GenerateGatewayClientCertificateResponse>;

  getMetrics(
    request: api_gateway_pb.GetGatewayMetricsRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_gateway_pb.GetGatewayMetricsResponse>;

}

