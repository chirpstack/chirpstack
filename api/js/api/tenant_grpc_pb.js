// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_tenant_pb = require('../api/tenant_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_api_AddTenantUserRequest(arg) {
  if (!(arg instanceof api_tenant_pb.AddTenantUserRequest)) {
    throw new Error('Expected argument of type api.AddTenantUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_AddTenantUserRequest(buffer_arg) {
  return api_tenant_pb.AddTenantUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateTenantRequest(arg) {
  if (!(arg instanceof api_tenant_pb.CreateTenantRequest)) {
    throw new Error('Expected argument of type api.CreateTenantRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateTenantRequest(buffer_arg) {
  return api_tenant_pb.CreateTenantRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateTenantResponse(arg) {
  if (!(arg instanceof api_tenant_pb.CreateTenantResponse)) {
    throw new Error('Expected argument of type api.CreateTenantResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateTenantResponse(buffer_arg) {
  return api_tenant_pb.CreateTenantResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteTenantRequest(arg) {
  if (!(arg instanceof api_tenant_pb.DeleteTenantRequest)) {
    throw new Error('Expected argument of type api.DeleteTenantRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteTenantRequest(buffer_arg) {
  return api_tenant_pb.DeleteTenantRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteTenantUserRequest(arg) {
  if (!(arg instanceof api_tenant_pb.DeleteTenantUserRequest)) {
    throw new Error('Expected argument of type api.DeleteTenantUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteTenantUserRequest(buffer_arg) {
  return api_tenant_pb.DeleteTenantUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetTenantRequest(arg) {
  if (!(arg instanceof api_tenant_pb.GetTenantRequest)) {
    throw new Error('Expected argument of type api.GetTenantRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetTenantRequest(buffer_arg) {
  return api_tenant_pb.GetTenantRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetTenantResponse(arg) {
  if (!(arg instanceof api_tenant_pb.GetTenantResponse)) {
    throw new Error('Expected argument of type api.GetTenantResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetTenantResponse(buffer_arg) {
  return api_tenant_pb.GetTenantResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetTenantUserRequest(arg) {
  if (!(arg instanceof api_tenant_pb.GetTenantUserRequest)) {
    throw new Error('Expected argument of type api.GetTenantUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetTenantUserRequest(buffer_arg) {
  return api_tenant_pb.GetTenantUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetTenantUserResponse(arg) {
  if (!(arg instanceof api_tenant_pb.GetTenantUserResponse)) {
    throw new Error('Expected argument of type api.GetTenantUserResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetTenantUserResponse(buffer_arg) {
  return api_tenant_pb.GetTenantUserResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListTenantUsersRequest(arg) {
  if (!(arg instanceof api_tenant_pb.ListTenantUsersRequest)) {
    throw new Error('Expected argument of type api.ListTenantUsersRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListTenantUsersRequest(buffer_arg) {
  return api_tenant_pb.ListTenantUsersRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListTenantUsersResponse(arg) {
  if (!(arg instanceof api_tenant_pb.ListTenantUsersResponse)) {
    throw new Error('Expected argument of type api.ListTenantUsersResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListTenantUsersResponse(buffer_arg) {
  return api_tenant_pb.ListTenantUsersResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListTenantsRequest(arg) {
  if (!(arg instanceof api_tenant_pb.ListTenantsRequest)) {
    throw new Error('Expected argument of type api.ListTenantsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListTenantsRequest(buffer_arg) {
  return api_tenant_pb.ListTenantsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListTenantsResponse(arg) {
  if (!(arg instanceof api_tenant_pb.ListTenantsResponse)) {
    throw new Error('Expected argument of type api.ListTenantsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListTenantsResponse(buffer_arg) {
  return api_tenant_pb.ListTenantsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateTenantRequest(arg) {
  if (!(arg instanceof api_tenant_pb.UpdateTenantRequest)) {
    throw new Error('Expected argument of type api.UpdateTenantRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateTenantRequest(buffer_arg) {
  return api_tenant_pb.UpdateTenantRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateTenantUserRequest(arg) {
  if (!(arg instanceof api_tenant_pb.UpdateTenantUserRequest)) {
    throw new Error('Expected argument of type api.UpdateTenantUserRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateTenantUserRequest(buffer_arg) {
  return api_tenant_pb.UpdateTenantUserRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// TenantService is the service providing API methods for managing tenants.
var TenantServiceService = exports.TenantServiceService = {
  // Create a new tenant.
create: {
    path: '/api.TenantService/Create',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.CreateTenantRequest,
    responseType: api_tenant_pb.CreateTenantResponse,
    requestSerialize: serialize_api_CreateTenantRequest,
    requestDeserialize: deserialize_api_CreateTenantRequest,
    responseSerialize: serialize_api_CreateTenantResponse,
    responseDeserialize: deserialize_api_CreateTenantResponse,
  },
  // Get the tenant for the given ID.
get: {
    path: '/api.TenantService/Get',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.GetTenantRequest,
    responseType: api_tenant_pb.GetTenantResponse,
    requestSerialize: serialize_api_GetTenantRequest,
    requestDeserialize: deserialize_api_GetTenantRequest,
    responseSerialize: serialize_api_GetTenantResponse,
    responseDeserialize: deserialize_api_GetTenantResponse,
  },
  // Update the given tenant.
update: {
    path: '/api.TenantService/Update',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.UpdateTenantRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateTenantRequest,
    requestDeserialize: deserialize_api_UpdateTenantRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the tenant with the given ID.
delete: {
    path: '/api.TenantService/Delete',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.DeleteTenantRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteTenantRequest,
    requestDeserialize: deserialize_api_DeleteTenantRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the list of tenants.
list: {
    path: '/api.TenantService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.ListTenantsRequest,
    responseType: api_tenant_pb.ListTenantsResponse,
    requestSerialize: serialize_api_ListTenantsRequest,
    requestDeserialize: deserialize_api_ListTenantsRequest,
    responseSerialize: serialize_api_ListTenantsResponse,
    responseDeserialize: deserialize_api_ListTenantsResponse,
  },
  // Add an user to the tenant.
// Note: the user must already exist.
addUser: {
    path: '/api.TenantService/AddUser',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.AddTenantUserRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_AddTenantUserRequest,
    requestDeserialize: deserialize_api_AddTenantUserRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the the tenant user for the given tenant and user IDs.
getUser: {
    path: '/api.TenantService/GetUser',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.GetTenantUserRequest,
    responseType: api_tenant_pb.GetTenantUserResponse,
    requestSerialize: serialize_api_GetTenantUserRequest,
    requestDeserialize: deserialize_api_GetTenantUserRequest,
    responseSerialize: serialize_api_GetTenantUserResponse,
    responseDeserialize: deserialize_api_GetTenantUserResponse,
  },
  // Update the given tenant user.
updateUser: {
    path: '/api.TenantService/UpdateUser',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.UpdateTenantUserRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateTenantUserRequest,
    requestDeserialize: deserialize_api_UpdateTenantUserRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the given tenant user.
deleteUser: {
    path: '/api.TenantService/DeleteUser',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.DeleteTenantUserRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteTenantUserRequest,
    requestDeserialize: deserialize_api_DeleteTenantUserRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the list of tenant users.
listUsers: {
    path: '/api.TenantService/ListUsers',
    requestStream: false,
    responseStream: false,
    requestType: api_tenant_pb.ListTenantUsersRequest,
    responseType: api_tenant_pb.ListTenantUsersResponse,
    requestSerialize: serialize_api_ListTenantUsersRequest,
    requestDeserialize: deserialize_api_ListTenantUsersRequest,
    responseSerialize: serialize_api_ListTenantUsersResponse,
    responseDeserialize: deserialize_api_ListTenantUsersResponse,
  },
};

exports.TenantServiceClient = grpc.makeGenericClientConstructor(TenantServiceService);
