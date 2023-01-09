import * as grpcWeb from 'grpc-web';

import * as api_user_pb from '../api/user_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class UserServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_user_pb.CreateUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_user_pb.CreateUserResponse) => void
  ): grpcWeb.ClientReadableStream<api_user_pb.CreateUserResponse>;

  get(
    request: api_user_pb.GetUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_user_pb.GetUserResponse) => void
  ): grpcWeb.ClientReadableStream<api_user_pb.GetUserResponse>;

  update(
    request: api_user_pb.UpdateUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_user_pb.DeleteUserRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  list(
    request: api_user_pb.ListUsersRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_user_pb.ListUsersResponse) => void
  ): grpcWeb.ClientReadableStream<api_user_pb.ListUsersResponse>;

  updatePassword(
    request: api_user_pb.UpdateUserPasswordRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

}

export class UserServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_user_pb.CreateUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_user_pb.CreateUserResponse>;

  get(
    request: api_user_pb.GetUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_user_pb.GetUserResponse>;

  update(
    request: api_user_pb.UpdateUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_user_pb.DeleteUserRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  list(
    request: api_user_pb.ListUsersRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_user_pb.ListUsersResponse>;

  updatePassword(
    request: api_user_pb.UpdateUserPasswordRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

}

