import * as grpcWeb from 'grpc-web';

import * as api_tenant_pb from '../api/tenant_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class TenantServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_tenant_pb.CreateTenantRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_tenant_pb.CreateTenantResponse) => void
  ): grpcWeb.ClientReadableStream<api_tenant_pb.CreateTenantResponse>;

  get(
    request: api_tenant_pb.GetTenantRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_tenant_pb.GetTenantResponse) => void
  ): grpcWeb.ClientReadableStream<api_tenant_pb.GetTenantResponse>;

  update(
    request: api_tenant_pb.UpdateTenantRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_tenant_pb.DeleteTenantRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  list(
    request: api_tenant_pb.ListTenantsRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_tenant_pb.ListTenantsResponse) => void
  ): grpcWeb.ClientReadableStream<api_tenant_pb.ListTenantsResponse>;

  addUser(
    request: api_tenant_pb.AddTenantUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getUser(
    request: api_tenant_pb.GetTenantUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_tenant_pb.GetTenantUserResponse) => void
  ): grpcWeb.ClientReadableStream<api_tenant_pb.GetTenantUserResponse>;

  updateUser(
    request: api_tenant_pb.UpdateTenantUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteUser(
    request: api_tenant_pb.DeleteTenantUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  listUsers(
    request: api_tenant_pb.ListTenantUsersRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_tenant_pb.ListTenantUsersResponse) => void
  ): grpcWeb.ClientReadableStream<api_tenant_pb.ListTenantUsersResponse>;

}

export class TenantServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_tenant_pb.CreateTenantRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_tenant_pb.CreateTenantResponse>;

  get(
    request: api_tenant_pb.GetTenantRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_tenant_pb.GetTenantResponse>;

  update(
    request: api_tenant_pb.UpdateTenantRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_tenant_pb.DeleteTenantRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  list(
    request: api_tenant_pb.ListTenantsRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_tenant_pb.ListTenantsResponse>;

  addUser(
    request: api_tenant_pb.AddTenantUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getUser(
    request: api_tenant_pb.GetTenantUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_tenant_pb.GetTenantUserResponse>;

  updateUser(
    request: api_tenant_pb.UpdateTenantUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteUser(
    request: api_tenant_pb.DeleteTenantUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  listUsers(
    request: api_tenant_pb.ListTenantUsersRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_tenant_pb.ListTenantUsersResponse>;

}

