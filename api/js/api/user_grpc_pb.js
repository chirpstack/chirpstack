// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_user_pb = require('../api/user_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_api_CreateUserRequest(arg) {
  if (!(arg instanceof api_user_pb.CreateUserRequest)) {
    throw new Error('Expected argument of type api.CreateUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateUserRequest(buffer_arg) {
  return api_user_pb.CreateUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateUserResponse(arg) {
  if (!(arg instanceof api_user_pb.CreateUserResponse)) {
    throw new Error('Expected argument of type api.CreateUserResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateUserResponse(buffer_arg) {
  return api_user_pb.CreateUserResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteUserRequest(arg) {
  if (!(arg instanceof api_user_pb.DeleteUserRequest)) {
    throw new Error('Expected argument of type api.DeleteUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteUserRequest(buffer_arg) {
  return api_user_pb.DeleteUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetUserRequest(arg) {
  if (!(arg instanceof api_user_pb.GetUserRequest)) {
    throw new Error('Expected argument of type api.GetUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetUserRequest(buffer_arg) {
  return api_user_pb.GetUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetUserResponse(arg) {
  if (!(arg instanceof api_user_pb.GetUserResponse)) {
    throw new Error('Expected argument of type api.GetUserResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetUserResponse(buffer_arg) {
  return api_user_pb.GetUserResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListUsersRequest(arg) {
  if (!(arg instanceof api_user_pb.ListUsersRequest)) {
    throw new Error('Expected argument of type api.ListUsersRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListUsersRequest(buffer_arg) {
  return api_user_pb.ListUsersRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListUsersResponse(arg) {
  if (!(arg instanceof api_user_pb.ListUsersResponse)) {
    throw new Error('Expected argument of type api.ListUsersResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListUsersResponse(buffer_arg) {
  return api_user_pb.ListUsersResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateUserPasswordRequest(arg) {
  if (!(arg instanceof api_user_pb.UpdateUserPasswordRequest)) {
    throw new Error('Expected argument of type api.UpdateUserPasswordRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateUserPasswordRequest(buffer_arg) {
  return api_user_pb.UpdateUserPasswordRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateUserRequest(arg) {
  if (!(arg instanceof api_user_pb.UpdateUserRequest)) {
    throw new Error('Expected argument of type api.UpdateUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateUserRequest(buffer_arg) {
  return api_user_pb.UpdateUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_google_protobuf_Empty(arg) {
  if (!(arg instanceof google_protobuf_empty_pb.Empty)) {
    throw new Error('Expected argument of type google.protobuf.Empty');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_google_protobuf_Empty(buffer_arg) {
  return google_protobuf_empty_pb.Empty.deserializeBinary(new Uint8Array(buffer_arg));
}


// UserService is the service providing API methods for managing users.
var UserServiceService = exports.UserServiceService = {
  // Create a new user.
create: {
    path: '/api.UserService/Create',
    requestStream: false,
    responseStream: false,
    requestType: api_user_pb.CreateUserRequest,
    responseType: api_user_pb.CreateUserResponse,
    requestSerialize: serialize_api_CreateUserRequest,
    requestDeserialize: deserialize_api_CreateUserRequest,
    responseSerialize: serialize_api_CreateUserResponse,
    responseDeserialize: deserialize_api_CreateUserResponse,
  },
  // Get the user for the given ID.
get: {
    path: '/api.UserService/Get',
    requestStream: false,
    responseStream: false,
    requestType: api_user_pb.GetUserRequest,
    responseType: api_user_pb.GetUserResponse,
    requestSerialize: serialize_api_GetUserRequest,
    requestDeserialize: deserialize_api_GetUserRequest,
    responseSerialize: serialize_api_GetUserResponse,
    responseDeserialize: deserialize_api_GetUserResponse,
  },
  // Update the given user.
update: {
    path: '/api.UserService/Update',
    requestStream: false,
    responseStream: false,
    requestType: api_user_pb.UpdateUserRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateUserRequest,
    requestDeserialize: deserialize_api_UpdateUserRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the user with the given ID.
delete: {
    path: '/api.UserService/Delete',
    requestStream: false,
    responseStream: false,
    requestType: api_user_pb.DeleteUserRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteUserRequest,
    requestDeserialize: deserialize_api_DeleteUserRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the list of users.
list: {
    path: '/api.UserService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_user_pb.ListUsersRequest,
    responseType: api_user_pb.ListUsersResponse,
    requestSerialize: serialize_api_ListUsersRequest,
    requestDeserialize: deserialize_api_ListUsersRequest,
    responseSerialize: serialize_api_ListUsersResponse,
    responseDeserialize: deserialize_api_ListUsersResponse,
  },
  // Update the password for the given user.
updatePassword: {
    path: '/api.UserService/UpdatePassword',
    requestStream: false,
    responseStream: false,
    requestType: api_user_pb.UpdateUserPasswordRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateUserPasswordRequest,
    requestDeserialize: deserialize_api_UpdateUserPasswordRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
};

exports.UserServiceClient = grpc.makeGenericClientConstructor(UserServiceService);
