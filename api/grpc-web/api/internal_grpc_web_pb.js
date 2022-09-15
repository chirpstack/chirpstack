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


var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js')

var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js')

var common_common_pb = require('../common/common_pb.js')

var api_user_pb = require('../api/user_pb.js')
const proto = {};
proto.api = require('./internal_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.api.InternalServiceClient =
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
proto.api.InternalServicePromiseClient =
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
 *   !proto.api.LoginRequest,
 *   !proto.api.LoginResponse>}
 */
const methodDescriptor_InternalService_Login = new grpc.web.MethodDescriptor(
  '/api.InternalService/Login',
  grpc.web.MethodType.UNARY,
  proto.api.LoginRequest,
  proto.api.LoginResponse,
  /**
   * @param {!proto.api.LoginRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.LoginResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.LoginRequest,
 *   !proto.api.LoginResponse>}
 */
const methodInfo_InternalService_Login = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.LoginResponse,
  /**
   * @param {!proto.api.LoginRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.LoginResponse.deserializeBinary
);


/**
 * @param {!proto.api.LoginRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.LoginResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.LoginResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.login =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/Login',
      request,
      metadata || {},
      methodDescriptor_InternalService_Login,
      callback);
};


/**
 * @param {!proto.api.LoginRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.LoginResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.login =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/Login',
      request,
      metadata || {},
      methodDescriptor_InternalService_Login);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.google.protobuf.Empty,
 *   !proto.api.ProfileResponse>}
 */
const methodDescriptor_InternalService_Profile = new grpc.web.MethodDescriptor(
  '/api.InternalService/Profile',
  grpc.web.MethodType.UNARY,
  google_protobuf_empty_pb.Empty,
  proto.api.ProfileResponse,
  /**
   * @param {!proto.google.protobuf.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ProfileResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.google.protobuf.Empty,
 *   !proto.api.ProfileResponse>}
 */
const methodInfo_InternalService_Profile = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.ProfileResponse,
  /**
   * @param {!proto.google.protobuf.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ProfileResponse.deserializeBinary
);


/**
 * @param {!proto.google.protobuf.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.ProfileResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.ProfileResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.profile =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/Profile',
      request,
      metadata || {},
      methodDescriptor_InternalService_Profile,
      callback);
};


/**
 * @param {!proto.google.protobuf.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.ProfileResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.profile =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/Profile',
      request,
      metadata || {},
      methodDescriptor_InternalService_Profile);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.GlobalSearchRequest,
 *   !proto.api.GlobalSearchResponse>}
 */
const methodDescriptor_InternalService_GlobalSearch = new grpc.web.MethodDescriptor(
  '/api.InternalService/GlobalSearch',
  grpc.web.MethodType.UNARY,
  proto.api.GlobalSearchRequest,
  proto.api.GlobalSearchResponse,
  /**
   * @param {!proto.api.GlobalSearchRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GlobalSearchResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.GlobalSearchRequest,
 *   !proto.api.GlobalSearchResponse>}
 */
const methodInfo_InternalService_GlobalSearch = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.GlobalSearchResponse,
  /**
   * @param {!proto.api.GlobalSearchRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GlobalSearchResponse.deserializeBinary
);


/**
 * @param {!proto.api.GlobalSearchRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.GlobalSearchResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.GlobalSearchResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.globalSearch =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/GlobalSearch',
      request,
      metadata || {},
      methodDescriptor_InternalService_GlobalSearch,
      callback);
};


/**
 * @param {!proto.api.GlobalSearchRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.GlobalSearchResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.globalSearch =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/GlobalSearch',
      request,
      metadata || {},
      methodDescriptor_InternalService_GlobalSearch);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.CreateApiKeyRequest,
 *   !proto.api.CreateApiKeyResponse>}
 */
const methodDescriptor_InternalService_CreateApiKey = new grpc.web.MethodDescriptor(
  '/api.InternalService/CreateApiKey',
  grpc.web.MethodType.UNARY,
  proto.api.CreateApiKeyRequest,
  proto.api.CreateApiKeyResponse,
  /**
   * @param {!proto.api.CreateApiKeyRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.CreateApiKeyResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.CreateApiKeyRequest,
 *   !proto.api.CreateApiKeyResponse>}
 */
const methodInfo_InternalService_CreateApiKey = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.CreateApiKeyResponse,
  /**
   * @param {!proto.api.CreateApiKeyRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.CreateApiKeyResponse.deserializeBinary
);


/**
 * @param {!proto.api.CreateApiKeyRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.CreateApiKeyResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.CreateApiKeyResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.createApiKey =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/CreateApiKey',
      request,
      metadata || {},
      methodDescriptor_InternalService_CreateApiKey,
      callback);
};


/**
 * @param {!proto.api.CreateApiKeyRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.CreateApiKeyResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.createApiKey =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/CreateApiKey',
      request,
      metadata || {},
      methodDescriptor_InternalService_CreateApiKey);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.DeleteApiKeyRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_InternalService_DeleteApiKey = new grpc.web.MethodDescriptor(
  '/api.InternalService/DeleteApiKey',
  grpc.web.MethodType.UNARY,
  proto.api.DeleteApiKeyRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.DeleteApiKeyRequest} request
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
 *   !proto.api.DeleteApiKeyRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_InternalService_DeleteApiKey = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.DeleteApiKeyRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.DeleteApiKeyRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.deleteApiKey =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/DeleteApiKey',
      request,
      metadata || {},
      methodDescriptor_InternalService_DeleteApiKey,
      callback);
};


/**
 * @param {!proto.api.DeleteApiKeyRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.deleteApiKey =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/DeleteApiKey',
      request,
      metadata || {},
      methodDescriptor_InternalService_DeleteApiKey);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.ListApiKeysRequest,
 *   !proto.api.ListApiKeysResponse>}
 */
const methodDescriptor_InternalService_ListApiKeys = new grpc.web.MethodDescriptor(
  '/api.InternalService/ListApiKeys',
  grpc.web.MethodType.UNARY,
  proto.api.ListApiKeysRequest,
  proto.api.ListApiKeysResponse,
  /**
   * @param {!proto.api.ListApiKeysRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListApiKeysResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.ListApiKeysRequest,
 *   !proto.api.ListApiKeysResponse>}
 */
const methodInfo_InternalService_ListApiKeys = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.ListApiKeysResponse,
  /**
   * @param {!proto.api.ListApiKeysRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListApiKeysResponse.deserializeBinary
);


/**
 * @param {!proto.api.ListApiKeysRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.ListApiKeysResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.ListApiKeysResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.listApiKeys =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/ListApiKeys',
      request,
      metadata || {},
      methodDescriptor_InternalService_ListApiKeys,
      callback);
};


/**
 * @param {!proto.api.ListApiKeysRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.ListApiKeysResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.listApiKeys =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/ListApiKeys',
      request,
      metadata || {},
      methodDescriptor_InternalService_ListApiKeys);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.google.protobuf.Empty,
 *   !proto.api.SettingsResponse>}
 */
const methodDescriptor_InternalService_Settings = new grpc.web.MethodDescriptor(
  '/api.InternalService/Settings',
  grpc.web.MethodType.UNARY,
  google_protobuf_empty_pb.Empty,
  proto.api.SettingsResponse,
  /**
   * @param {!proto.google.protobuf.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.SettingsResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.google.protobuf.Empty,
 *   !proto.api.SettingsResponse>}
 */
const methodInfo_InternalService_Settings = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.SettingsResponse,
  /**
   * @param {!proto.google.protobuf.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.SettingsResponse.deserializeBinary
);


/**
 * @param {!proto.google.protobuf.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.SettingsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.SettingsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.settings =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/Settings',
      request,
      metadata || {},
      methodDescriptor_InternalService_Settings,
      callback);
};


/**
 * @param {!proto.google.protobuf.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.SettingsResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.settings =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/Settings',
      request,
      metadata || {},
      methodDescriptor_InternalService_Settings);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.OpenIdConnectLoginRequest,
 *   !proto.api.OpenIdConnectLoginResponse>}
 */
const methodDescriptor_InternalService_OpenIdConnectLogin = new grpc.web.MethodDescriptor(
  '/api.InternalService/OpenIdConnectLogin',
  grpc.web.MethodType.UNARY,
  proto.api.OpenIdConnectLoginRequest,
  proto.api.OpenIdConnectLoginResponse,
  /**
   * @param {!proto.api.OpenIdConnectLoginRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.OpenIdConnectLoginResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.OpenIdConnectLoginRequest,
 *   !proto.api.OpenIdConnectLoginResponse>}
 */
const methodInfo_InternalService_OpenIdConnectLogin = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.OpenIdConnectLoginResponse,
  /**
   * @param {!proto.api.OpenIdConnectLoginRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.OpenIdConnectLoginResponse.deserializeBinary
);


/**
 * @param {!proto.api.OpenIdConnectLoginRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.OpenIdConnectLoginResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.OpenIdConnectLoginResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.openIdConnectLogin =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/OpenIdConnectLogin',
      request,
      metadata || {},
      methodDescriptor_InternalService_OpenIdConnectLogin,
      callback);
};


/**
 * @param {!proto.api.OpenIdConnectLoginRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.OpenIdConnectLoginResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.openIdConnectLogin =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/OpenIdConnectLogin',
      request,
      metadata || {},
      methodDescriptor_InternalService_OpenIdConnectLogin);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.GetDevicesSummaryRequest,
 *   !proto.api.GetDevicesSummaryResponse>}
 */
const methodDescriptor_InternalService_GetDevicesSummary = new grpc.web.MethodDescriptor(
  '/api.InternalService/GetDevicesSummary',
  grpc.web.MethodType.UNARY,
  proto.api.GetDevicesSummaryRequest,
  proto.api.GetDevicesSummaryResponse,
  /**
   * @param {!proto.api.GetDevicesSummaryRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetDevicesSummaryResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.GetDevicesSummaryRequest,
 *   !proto.api.GetDevicesSummaryResponse>}
 */
const methodInfo_InternalService_GetDevicesSummary = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.GetDevicesSummaryResponse,
  /**
   * @param {!proto.api.GetDevicesSummaryRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetDevicesSummaryResponse.deserializeBinary
);


/**
 * @param {!proto.api.GetDevicesSummaryRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.GetDevicesSummaryResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.GetDevicesSummaryResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.getDevicesSummary =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/GetDevicesSummary',
      request,
      metadata || {},
      methodDescriptor_InternalService_GetDevicesSummary,
      callback);
};


/**
 * @param {!proto.api.GetDevicesSummaryRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.GetDevicesSummaryResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.getDevicesSummary =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/GetDevicesSummary',
      request,
      metadata || {},
      methodDescriptor_InternalService_GetDevicesSummary);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.GetGatewaysSummaryRequest,
 *   !proto.api.GetGatewaysSummaryResponse>}
 */
const methodDescriptor_InternalService_GetGatewaysSummary = new grpc.web.MethodDescriptor(
  '/api.InternalService/GetGatewaysSummary',
  grpc.web.MethodType.UNARY,
  proto.api.GetGatewaysSummaryRequest,
  proto.api.GetGatewaysSummaryResponse,
  /**
   * @param {!proto.api.GetGatewaysSummaryRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetGatewaysSummaryResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.GetGatewaysSummaryRequest,
 *   !proto.api.GetGatewaysSummaryResponse>}
 */
const methodInfo_InternalService_GetGatewaysSummary = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.GetGatewaysSummaryResponse,
  /**
   * @param {!proto.api.GetGatewaysSummaryRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetGatewaysSummaryResponse.deserializeBinary
);


/**
 * @param {!proto.api.GetGatewaysSummaryRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.GetGatewaysSummaryResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.GetGatewaysSummaryResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.getGatewaysSummary =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/GetGatewaysSummary',
      request,
      metadata || {},
      methodDescriptor_InternalService_GetGatewaysSummary,
      callback);
};


/**
 * @param {!proto.api.GetGatewaysSummaryRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.GetGatewaysSummaryResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.getGatewaysSummary =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/GetGatewaysSummary',
      request,
      metadata || {},
      methodDescriptor_InternalService_GetGatewaysSummary);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.StreamGatewayFramesRequest,
 *   !proto.api.LogItem>}
 */
const methodDescriptor_InternalService_StreamGatewayFrames = new grpc.web.MethodDescriptor(
  '/api.InternalService/StreamGatewayFrames',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.api.StreamGatewayFramesRequest,
  proto.api.LogItem,
  /**
   * @param {!proto.api.StreamGatewayFramesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.LogItem.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.StreamGatewayFramesRequest,
 *   !proto.api.LogItem>}
 */
const methodInfo_InternalService_StreamGatewayFrames = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.LogItem,
  /**
   * @param {!proto.api.StreamGatewayFramesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.LogItem.deserializeBinary
);


/**
 * @param {!proto.api.StreamGatewayFramesRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.api.LogItem>}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.streamGatewayFrames =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/api.InternalService/StreamGatewayFrames',
      request,
      metadata || {},
      methodDescriptor_InternalService_StreamGatewayFrames);
};


/**
 * @param {!proto.api.StreamGatewayFramesRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.api.LogItem>}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServicePromiseClient.prototype.streamGatewayFrames =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/api.InternalService/StreamGatewayFrames',
      request,
      metadata || {},
      methodDescriptor_InternalService_StreamGatewayFrames);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.StreamDeviceFramesRequest,
 *   !proto.api.LogItem>}
 */
const methodDescriptor_InternalService_StreamDeviceFrames = new grpc.web.MethodDescriptor(
  '/api.InternalService/StreamDeviceFrames',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.api.StreamDeviceFramesRequest,
  proto.api.LogItem,
  /**
   * @param {!proto.api.StreamDeviceFramesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.LogItem.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.StreamDeviceFramesRequest,
 *   !proto.api.LogItem>}
 */
const methodInfo_InternalService_StreamDeviceFrames = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.LogItem,
  /**
   * @param {!proto.api.StreamDeviceFramesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.LogItem.deserializeBinary
);


/**
 * @param {!proto.api.StreamDeviceFramesRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.api.LogItem>}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.streamDeviceFrames =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/api.InternalService/StreamDeviceFrames',
      request,
      metadata || {},
      methodDescriptor_InternalService_StreamDeviceFrames);
};


/**
 * @param {!proto.api.StreamDeviceFramesRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.api.LogItem>}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServicePromiseClient.prototype.streamDeviceFrames =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/api.InternalService/StreamDeviceFrames',
      request,
      metadata || {},
      methodDescriptor_InternalService_StreamDeviceFrames);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.StreamDeviceEventsRequest,
 *   !proto.api.LogItem>}
 */
const methodDescriptor_InternalService_StreamDeviceEvents = new grpc.web.MethodDescriptor(
  '/api.InternalService/StreamDeviceEvents',
  grpc.web.MethodType.SERVER_STREAMING,
  proto.api.StreamDeviceEventsRequest,
  proto.api.LogItem,
  /**
   * @param {!proto.api.StreamDeviceEventsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.LogItem.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.StreamDeviceEventsRequest,
 *   !proto.api.LogItem>}
 */
const methodInfo_InternalService_StreamDeviceEvents = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.LogItem,
  /**
   * @param {!proto.api.StreamDeviceEventsRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.LogItem.deserializeBinary
);


/**
 * @param {!proto.api.StreamDeviceEventsRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.api.LogItem>}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.streamDeviceEvents =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/api.InternalService/StreamDeviceEvents',
      request,
      metadata || {},
      methodDescriptor_InternalService_StreamDeviceEvents);
};


/**
 * @param {!proto.api.StreamDeviceEventsRequest} request The request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!grpc.web.ClientReadableStream<!proto.api.LogItem>}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServicePromiseClient.prototype.streamDeviceEvents =
    function(request, metadata) {
  return this.client_.serverStreaming(this.hostname_ +
      '/api.InternalService/StreamDeviceEvents',
      request,
      metadata || {},
      methodDescriptor_InternalService_StreamDeviceEvents);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.google.protobuf.Empty,
 *   !proto.api.ListRegionsResponse>}
 */
const methodDescriptor_InternalService_ListRegions = new grpc.web.MethodDescriptor(
  '/api.InternalService/ListRegions',
  grpc.web.MethodType.UNARY,
  google_protobuf_empty_pb.Empty,
  proto.api.ListRegionsResponse,
  /**
   * @param {!proto.google.protobuf.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListRegionsResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.google.protobuf.Empty,
 *   !proto.api.ListRegionsResponse>}
 */
const methodInfo_InternalService_ListRegions = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.ListRegionsResponse,
  /**
   * @param {!proto.google.protobuf.Empty} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListRegionsResponse.deserializeBinary
);


/**
 * @param {!proto.google.protobuf.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.ListRegionsResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.ListRegionsResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.listRegions =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/ListRegions',
      request,
      metadata || {},
      methodDescriptor_InternalService_ListRegions,
      callback);
};


/**
 * @param {!proto.google.protobuf.Empty} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.ListRegionsResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.listRegions =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/ListRegions',
      request,
      metadata || {},
      methodDescriptor_InternalService_ListRegions);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.GetRegionRequest,
 *   !proto.api.GetRegionResponse>}
 */
const methodDescriptor_InternalService_GetRegion = new grpc.web.MethodDescriptor(
  '/api.InternalService/GetRegion',
  grpc.web.MethodType.UNARY,
  proto.api.GetRegionRequest,
  proto.api.GetRegionResponse,
  /**
   * @param {!proto.api.GetRegionRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetRegionResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.GetRegionRequest,
 *   !proto.api.GetRegionResponse>}
 */
const methodInfo_InternalService_GetRegion = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.GetRegionResponse,
  /**
   * @param {!proto.api.GetRegionRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetRegionResponse.deserializeBinary
);


/**
 * @param {!proto.api.GetRegionRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.GetRegionResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.GetRegionResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.InternalServiceClient.prototype.getRegion =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.InternalService/GetRegion',
      request,
      metadata || {},
      methodDescriptor_InternalService_GetRegion,
      callback);
};


/**
 * @param {!proto.api.GetRegionRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.GetRegionResponse>}
 *     Promise that resolves to the response
 */
proto.api.InternalServicePromiseClient.prototype.getRegion =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.InternalService/GetRegion',
      request,
      metadata || {},
      methodDescriptor_InternalService_GetRegion);
};


module.exports = proto.api;

