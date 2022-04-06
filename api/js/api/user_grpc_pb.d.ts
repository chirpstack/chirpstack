// GENERATED CODE -- DO NOT EDIT!

// package: api
// file: api/user.proto

import * as api_user_pb from "../api/user_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as grpc from "@grpc/grpc-js";

interface IUserServiceService extends grpc.ServiceDefinition<grpc.UntypedServiceImplementation> {
  create: grpc.MethodDefinition<api_user_pb.CreateUserRequest, api_user_pb.CreateUserResponse>;
  get: grpc.MethodDefinition<api_user_pb.GetUserRequest, api_user_pb.GetUserResponse>;
  update: grpc.MethodDefinition<api_user_pb.UpdateUserRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.MethodDefinition<api_user_pb.DeleteUserRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.MethodDefinition<api_user_pb.ListUsersRequest, api_user_pb.ListUsersResponse>;
  updatePassword: grpc.MethodDefinition<api_user_pb.UpdateUserPasswordRequest, google_protobuf_empty_pb.Empty>;
}

export const UserServiceService: IUserServiceService;

export interface IUserServiceServer extends grpc.UntypedServiceImplementation {
  create: grpc.handleUnaryCall<api_user_pb.CreateUserRequest, api_user_pb.CreateUserResponse>;
  get: grpc.handleUnaryCall<api_user_pb.GetUserRequest, api_user_pb.GetUserResponse>;
  update: grpc.handleUnaryCall<api_user_pb.UpdateUserRequest, google_protobuf_empty_pb.Empty>;
  delete: grpc.handleUnaryCall<api_user_pb.DeleteUserRequest, google_protobuf_empty_pb.Empty>;
  list: grpc.handleUnaryCall<api_user_pb.ListUsersRequest, api_user_pb.ListUsersResponse>;
  updatePassword: grpc.handleUnaryCall<api_user_pb.UpdateUserPasswordRequest, google_protobuf_empty_pb.Empty>;
}

export class UserServiceClient extends grpc.Client {
  constructor(address: string, credentials: grpc.ChannelCredentials, options?: object);
  create(argument: api_user_pb.CreateUserRequest, callback: grpc.requestCallback<api_user_pb.CreateUserResponse>): grpc.ClientUnaryCall;
  create(argument: api_user_pb.CreateUserRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_user_pb.CreateUserResponse>): grpc.ClientUnaryCall;
  create(argument: api_user_pb.CreateUserRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_user_pb.CreateUserResponse>): grpc.ClientUnaryCall;
  get(argument: api_user_pb.GetUserRequest, callback: grpc.requestCallback<api_user_pb.GetUserResponse>): grpc.ClientUnaryCall;
  get(argument: api_user_pb.GetUserRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_user_pb.GetUserResponse>): grpc.ClientUnaryCall;
  get(argument: api_user_pb.GetUserRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_user_pb.GetUserResponse>): grpc.ClientUnaryCall;
  update(argument: api_user_pb.UpdateUserRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_user_pb.UpdateUserRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  update(argument: api_user_pb.UpdateUserRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_user_pb.DeleteUserRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_user_pb.DeleteUserRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  delete(argument: api_user_pb.DeleteUserRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  list(argument: api_user_pb.ListUsersRequest, callback: grpc.requestCallback<api_user_pb.ListUsersResponse>): grpc.ClientUnaryCall;
  list(argument: api_user_pb.ListUsersRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<api_user_pb.ListUsersResponse>): grpc.ClientUnaryCall;
  list(argument: api_user_pb.ListUsersRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<api_user_pb.ListUsersResponse>): grpc.ClientUnaryCall;
  updatePassword(argument: api_user_pb.UpdateUserPasswordRequest, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  updatePassword(argument: api_user_pb.UpdateUserPasswordRequest, metadataOrOptions: grpc.Metadata | grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
  updatePassword(argument: api_user_pb.UpdateUserPasswordRequest, metadata: grpc.Metadata | null, options: grpc.CallOptions | null, callback: grpc.requestCallback<google_protobuf_empty_pb.Empty>): grpc.ClientUnaryCall;
}
