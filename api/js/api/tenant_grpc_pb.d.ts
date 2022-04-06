// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/tenant.proto

import * as api_tenant_pb from "../api/tenant_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface ITenantServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  create: grpc.MethodDefinition<api_tenant_pb.CreateTenantRequest, api_tenant_pb.CreateTenantResponse>;
  get: grpc.MethodDefinition<api_tenant_pb.GetTenantRequest, api_tenant_pb.GetTenantResponse>;
  update: grpc.MethodDefinition<api_tenant_pb.UpdateTenantRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.MethodDefinition<api_tenant_pb.DeleteTenantRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.MethodDefinition<api_tenant_pb.ListTenantsRequest, api_tenant_pb.ListTenantsResponse>;
  addUser: grpc.MethodDefinition<api_tenant_pb.AddTenantUserRequest, google_protobuf_empty_pb.Empty>;
  getUser: grpc.MethodDefinition<api_tenant_pb.GetTenantUserRequest, api_tenant_pb.GetTenantUserResponse>;
  updateUser: grpc.MethodDefinition<api_tenant_pb.UpdateTenantUserRequest, google_protobuf_empty_pb.Empty>;
  deleteUser: grpc.MethodDefinition<api_tenant_pb.DeleteTenantUserRequest, google_protobuf_empty_pb.Empty>;
  listUsers: grpc.MethodDefinition<api_tenant_pb.ListTenantUsersRequest, api_tenant_pb.ListTenantUsersResponse>;
}

export const TenantServiceService: ITenantServiceService;

export interface ITenantServiceServer extends grpc.UntypedServiceImplementation {
  create: grpc.handleUnaryCall<api_tenant_pb.CreateTenantRequest, api_tenant_pb.CreateTenantResponse>;
  get: grpc.handleUnaryCall<api_tenant_pb.GetTenantRequest, api_tenant_pb.GetTenantResponse>;
  update: grpc.handleUnaryCall<api_tenant_pb.UpdateTenantRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.handleUnaryCall<api_tenant_pb.DeleteTenantRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.handleUnaryCall<api_tenant_pb.ListTenantsRequest, api_tenant_pb.ListTenantsResponse>;
  addUser: grpc.handleUnaryCall<api_tenant_pb.AddTenantUserRequest, google_protobuf_empty_pb.Empty>;
  getUser: grpc.handleUnaryCall<api_tenant_pb.GetTenantUserRequest, api_tenant_pb.GetTenantUserResponse>;
  updateUser: grpc.handleUnaryCall<api_tenant_pb.UpdateTenantUserRequest, google_protobuf_empty_pb.Empty>;
  deleteUser: grpc.handleUnaryCall<api_tenant_pb.DeleteTenantUserRequest, google_protobuf_empty_pb.Empty>;
  listUsers: grpc.handleUnaryCall<api_tenant_pb.ListTenantUsersRequest, api_tenant_pb.ListTenantUsersResponse>;
}

export class TenantServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  create(argument: api_tenant_pb.CreateTenantRequest, callback: grpc.requestCallback<api_tenant_pb.CreateTenantResponse>): grpc.ClientUnaryCall;
  create(argument: api_tenant_pb.CreateTenantRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.CreateTenantResponse>): grpc.ClientUnaryCall;
  create(argument: api_tenant_pb.CreateTenantRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.CreateTenantResponse>): grpc.ClientUnaryCall;
  get(argument: api_tenant_pb.GetTenantRequest, callback: grpc.requestCallback<api_tenant_pb.GetTenantResponse>): grpc.ClientUnaryCall;
  get(argument: api_tenant_pb.GetTenantRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.GetTenantResponse>): grpc.ClientUnaryCall;
  get(argument: api_tenant_pb.GetTenantRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.GetTenantResponse>): grpc.ClientUnaryCall;
  update(argument: api_tenant_pb.UpdateTenantRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_tenant_pb.UpdateTenantRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_tenant_pb.UpdateTenantRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_tenant_pb.DeleteTenantRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_tenant_pb.DeleteTenantRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_tenant_pb.DeleteTenantRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  list(argument: api_tenant_pb.ListTenantsRequest, callback: grpc.requestCallback<api_tenant_pb.ListTenantsResponse>): grpc.ClientUnaryCall;
  list(argument: api_tenant_pb.ListTenantsRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.ListTenantsResponse>): grpc.ClientUnaryCall;
  list(argument: api_tenant_pb.ListTenantsRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.ListTenantsResponse>): grpc.ClientUnaryCall;
  addUser(argument: api_tenant_pb.AddTenantUserRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addUser(argument: api_tenant_pb.AddTenantUserRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  addUser(argument: api_tenant_pb.AddTenantUserRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  getUser(argument: api_tenant_pb.GetTenantUserRequest, callback: grpc.requestCallback<api_tenant_pb.GetTenantUserResponse>): grpc.ClientUnaryCall;
  getUser(argument: api_tenant_pb.GetTenantUserRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.GetTenantUserResponse>): grpc.ClientUnaryCall;
  getUser(argument: api_tenant_pb.GetTenantUserRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.GetTenantUserResponse>): grpc.ClientUnaryCall;
  updateUser(argument: api_tenant_pb.UpdateTenantUserRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  updateUser(argument: api_tenant_pb.UpdateTenantUserRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  updateUser(argument: api_tenant_pb.UpdateTenantUserRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deleteUser(argument: api_tenant_pb.DeleteTenantUserRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deleteUser(argument: api_tenant_pb.DeleteTenantUserRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  deleteUser(argument: api_tenant_pb.DeleteTenantUserRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  listUsers(argument: api_tenant_pb.ListTenantUsersRequest, callback: grpc.requestCallback<api_tenant_pb.ListTenantUsersResponse>): grpc.ClientUnaryCall;
  listUsers(argument: api_tenant_pb.ListTenantUsersRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.ListTenantUsersResponse>): grpc.ClientUnaryCall;
  listUsers(argument: api_tenant_pb.ListTenantUsersRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_tenant_pb.ListTenantUsersResponse>): grpc.ClientUnaryCall;
}
