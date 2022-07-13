/**
 * @fileoverview gRPC-Web generated client stub for api
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');


var google_api_annotations_pb = require('../google/api/annotations_pb.js')

var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js')

var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js')
const proto = {};
proto.api = require('./tenant_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.api.TenantServiceClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.api.TenantServicePromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.CreateTenantRequest,
 *   !proto.api.CreateTenantResponse>}
 */
const methodDescriptor_TenantService_Create = new grpc.web.MethodDescriptor(
  '/api.TenantService/Create',
  grpc.web.MethodType.UNARY,
  proto.api.CreateTenantRequest,
  proto.api.CreateTenantResponse,
  /**
   * @param {!proto.api.CreateTenantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.CreateTenantResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.CreateTenantRequest,
 *   !proto.api.CreateTenantResponse>}
 */
const methodInfo_TenantService_Create = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.CreateTenantResponse,
  /**
   * @param {!proto.api.CreateTenantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.CreateTenantResponse.deserializeBinary
);


/**
 * @param {!proto.api.CreateTenantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.CreateTenantResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.CreateTenantResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.create =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/Create',
      request,
      metadata || {},
      methodDescriptor_TenantService_Create,
      callback);
};


/**
 * @param {!proto.api.CreateTenantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.CreateTenantResponse>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.create =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/Create',
      request,
      metadata || {},
      methodDescriptor_TenantService_Create);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.GetTenantRequest,
 *   !proto.api.GetTenantResponse>}
 */
const methodDescriptor_TenantService_Get = new grpc.web.MethodDescriptor(
  '/api.TenantService/Get',
  grpc.web.MethodType.UNARY,
  proto.api.GetTenantRequest,
  proto.api.GetTenantResponse,
  /**
   * @param {!proto.api.GetTenantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetTenantResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.GetTenantRequest,
 *   !proto.api.GetTenantResponse>}
 */
const methodInfo_TenantService_Get = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.GetTenantResponse,
  /**
   * @param {!proto.api.GetTenantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetTenantResponse.deserializeBinary
);


/**
 * @param {!proto.api.GetTenantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.GetTenantResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.GetTenantResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.get =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/Get',
      request,
      metadata || {},
      methodDescriptor_TenantService_Get,
      callback);
};


/**
 * @param {!proto.api.GetTenantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.GetTenantResponse>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.get =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/Get',
      request,
      metadata || {},
      methodDescriptor_TenantService_Get);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.UpdateTenantRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_TenantService_Update = new grpc.web.MethodDescriptor(
  '/api.TenantService/Update',
  grpc.web.MethodType.UNARY,
  proto.api.UpdateTenantRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.UpdateTenantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.UpdateTenantRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_TenantService_Update = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.UpdateTenantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.UpdateTenantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.update =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/Update',
      request,
      metadata || {},
      methodDescriptor_TenantService_Update,
      callback);
};


/**
 * @param {!proto.api.UpdateTenantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.update =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/Update',
      request,
      metadata || {},
      methodDescriptor_TenantService_Update);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.DeleteTenantRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_TenantService_Delete = new grpc.web.MethodDescriptor(
  '/api.TenantService/Delete',
  grpc.web.MethodType.UNARY,
  proto.api.DeleteTenantRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.DeleteTenantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.DeleteTenantRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_TenantService_Delete = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.DeleteTenantRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.DeleteTenantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.delete =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/Delete',
      request,
      metadata || {},
      methodDescriptor_TenantService_Delete,
      callback);
};


/**
 * @param {!proto.api.DeleteTenantRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.delete =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/Delete',
      request,
      metadata || {},
      methodDescriptor_TenantService_Delete);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.ListTenantsRequest,
 *   !proto.api.ListTenantsResponse>}
 */
const methodDescriptor_TenantService_List = new grpc.web.MethodDescriptor(
  '/api.TenantService/List',
  grpc.web.MethodType.UNARY,
  proto.api.ListTenantsRequest,
  proto.api.ListTenantsResponse,
  /**
   * @param {!proto.api.ListTenantsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListTenantsResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.ListTenantsRequest,
 *   !proto.api.ListTenantsResponse>}
 */
const methodInfo_TenantService_List = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.ListTenantsResponse,
  /**
   * @param {!proto.api.ListTenantsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListTenantsResponse.deserializeBinary
);


/**
 * @param {!proto.api.ListTenantsRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.ListTenantsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.ListTenantsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.list =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/List',
      request,
      metadata || {},
      methodDescriptor_TenantService_List,
      callback);
};


/**
 * @param {!proto.api.ListTenantsRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.ListTenantsResponse>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.list =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/List',
      request,
      metadata || {},
      methodDescriptor_TenantService_List);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.AddTenantUserRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_TenantService_AddUser = new grpc.web.MethodDescriptor(
  '/api.TenantService/AddUser',
  grpc.web.MethodType.UNARY,
  proto.api.AddTenantUserRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.AddTenantUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.AddTenantUserRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_TenantService_AddUser = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.AddTenantUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.AddTenantUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.addUser =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/AddUser',
      request,
      metadata || {},
      methodDescriptor_TenantService_AddUser,
      callback);
};


/**
 * @param {!proto.api.AddTenantUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.addUser =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/AddUser',
      request,
      metadata || {},
      methodDescriptor_TenantService_AddUser);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.GetTenantUserRequest,
 *   !proto.api.GetTenantUserResponse>}
 */
const methodDescriptor_TenantService_GetUser = new grpc.web.MethodDescriptor(
  '/api.TenantService/GetUser',
  grpc.web.MethodType.UNARY,
  proto.api.GetTenantUserRequest,
  proto.api.GetTenantUserResponse,
  /**
   * @param {!proto.api.GetTenantUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetTenantUserResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.GetTenantUserRequest,
 *   !proto.api.GetTenantUserResponse>}
 */
const methodInfo_TenantService_GetUser = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.GetTenantUserResponse,
  /**
   * @param {!proto.api.GetTenantUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetTenantUserResponse.deserializeBinary
);


/**
 * @param {!proto.api.GetTenantUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.GetTenantUserResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.GetTenantUserResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.getUser =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/GetUser',
      request,
      metadata || {},
      methodDescriptor_TenantService_GetUser,
      callback);
};


/**
 * @param {!proto.api.GetTenantUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.GetTenantUserResponse>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.getUser =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/GetUser',
      request,
      metadata || {},
      methodDescriptor_TenantService_GetUser);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.UpdateTenantUserRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_TenantService_UpdateUser = new grpc.web.MethodDescriptor(
  '/api.TenantService/UpdateUser',
  grpc.web.MethodType.UNARY,
  proto.api.UpdateTenantUserRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.UpdateTenantUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.UpdateTenantUserRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_TenantService_UpdateUser = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.UpdateTenantUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.UpdateTenantUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.updateUser =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/UpdateUser',
      request,
      metadata || {},
      methodDescriptor_TenantService_UpdateUser,
      callback);
};


/**
 * @param {!proto.api.UpdateTenantUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.updateUser =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/UpdateUser',
      request,
      metadata || {},
      methodDescriptor_TenantService_UpdateUser);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.DeleteTenantUserRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_TenantService_DeleteUser = new grpc.web.MethodDescriptor(
  '/api.TenantService/DeleteUser',
  grpc.web.MethodType.UNARY,
  proto.api.DeleteTenantUserRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.DeleteTenantUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.DeleteTenantUserRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_TenantService_DeleteUser = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.DeleteTenantUserRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.DeleteTenantUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.deleteUser =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/DeleteUser',
      request,
      metadata || {},
      methodDescriptor_TenantService_DeleteUser,
      callback);
};


/**
 * @param {!proto.api.DeleteTenantUserRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.deleteUser =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/DeleteUser',
      request,
      metadata || {},
      methodDescriptor_TenantService_DeleteUser);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.ListTenantUsersRequest,
 *   !proto.api.ListTenantUsersResponse>}
 */
const methodDescriptor_TenantService_ListUsers = new grpc.web.MethodDescriptor(
  '/api.TenantService/ListUsers',
  grpc.web.MethodType.UNARY,
  proto.api.ListTenantUsersRequest,
  proto.api.ListTenantUsersResponse,
  /**
   * @param {!proto.api.ListTenantUsersRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListTenantUsersResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.ListTenantUsersRequest,
 *   !proto.api.ListTenantUsersResponse>}
 */
const methodInfo_TenantService_ListUsers = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.ListTenantUsersResponse,
  /**
   * @param {!proto.api.ListTenantUsersRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListTenantUsersResponse.deserializeBinary
);


/**
 * @param {!proto.api.ListTenantUsersRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.ListTenantUsersResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.ListTenantUsersResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.TenantServiceClient.prototype.listUsers =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.TenantService/ListUsers',
      request,
      metadata || {},
      methodDescriptor_TenantService_ListUsers,
      callback);
};


/**
 * @param {!proto.api.ListTenantUsersRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.ListTenantUsersResponse>}
 *     Promise that resolves to the response
 */
proto.api.TenantServicePromiseClient.prototype.listUsers =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.TenantService/ListUsers',
      request,
      metadata || {},
      methodDescriptor_TenantService_ListUsers);
};


module.exports = proto.api;

