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

var api_device_profile_pb = require('../api/device_profile_pb.js')
const proto = {};
proto.api = require('./device_profile_template_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.api.DeviceProfileTemplateServiceClient =
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
proto.api.DeviceProfileTemplateServicePromiseClient =
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
 *   !proto.api.CreateDeviceProfileTemplateRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_DeviceProfileTemplateService_Create = new grpc.web.MethodDescriptor(
  '/api.DeviceProfileTemplateService/Create',
  grpc.web.MethodType.UNARY,
  proto.api.CreateDeviceProfileTemplateRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.CreateDeviceProfileTemplateRequest} request
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
 *   !proto.api.CreateDeviceProfileTemplateRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_DeviceProfileTemplateService_Create = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.CreateDeviceProfileTemplateRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.CreateDeviceProfileTemplateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.DeviceProfileTemplateServiceClient.prototype.create =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/Create',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_Create,
      callback);
};


/**
 * @param {!proto.api.CreateDeviceProfileTemplateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.DeviceProfileTemplateServicePromiseClient.prototype.create =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/Create',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_Create);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.GetDeviceProfileTemplateRequest,
 *   !proto.api.GetDeviceProfileTemplateResponse>}
 */
const methodDescriptor_DeviceProfileTemplateService_Get = new grpc.web.MethodDescriptor(
  '/api.DeviceProfileTemplateService/Get',
  grpc.web.MethodType.UNARY,
  proto.api.GetDeviceProfileTemplateRequest,
  proto.api.GetDeviceProfileTemplateResponse,
  /**
   * @param {!proto.api.GetDeviceProfileTemplateRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetDeviceProfileTemplateResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.GetDeviceProfileTemplateRequest,
 *   !proto.api.GetDeviceProfileTemplateResponse>}
 */
const methodInfo_DeviceProfileTemplateService_Get = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.GetDeviceProfileTemplateResponse,
  /**
   * @param {!proto.api.GetDeviceProfileTemplateRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.GetDeviceProfileTemplateResponse.deserializeBinary
);


/**
 * @param {!proto.api.GetDeviceProfileTemplateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.GetDeviceProfileTemplateResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.GetDeviceProfileTemplateResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.DeviceProfileTemplateServiceClient.prototype.get =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/Get',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_Get,
      callback);
};


/**
 * @param {!proto.api.GetDeviceProfileTemplateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.GetDeviceProfileTemplateResponse>}
 *     Promise that resolves to the response
 */
proto.api.DeviceProfileTemplateServicePromiseClient.prototype.get =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/Get',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_Get);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.UpdateDeviceProfileTemplateRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_DeviceProfileTemplateService_Update = new grpc.web.MethodDescriptor(
  '/api.DeviceProfileTemplateService/Update',
  grpc.web.MethodType.UNARY,
  proto.api.UpdateDeviceProfileTemplateRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.UpdateDeviceProfileTemplateRequest} request
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
 *   !proto.api.UpdateDeviceProfileTemplateRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_DeviceProfileTemplateService_Update = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.UpdateDeviceProfileTemplateRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.UpdateDeviceProfileTemplateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.DeviceProfileTemplateServiceClient.prototype.update =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/Update',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_Update,
      callback);
};


/**
 * @param {!proto.api.UpdateDeviceProfileTemplateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.DeviceProfileTemplateServicePromiseClient.prototype.update =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/Update',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_Update);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.DeleteDeviceProfileTemplateRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodDescriptor_DeviceProfileTemplateService_Delete = new grpc.web.MethodDescriptor(
  '/api.DeviceProfileTemplateService/Delete',
  grpc.web.MethodType.UNARY,
  proto.api.DeleteDeviceProfileTemplateRequest,
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.DeleteDeviceProfileTemplateRequest} request
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
 *   !proto.api.DeleteDeviceProfileTemplateRequest,
 *   !proto.google.protobuf.Empty>}
 */
const methodInfo_DeviceProfileTemplateService_Delete = new grpc.web.AbstractClientBase.MethodInfo(
  google_protobuf_empty_pb.Empty,
  /**
   * @param {!proto.api.DeleteDeviceProfileTemplateRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  google_protobuf_empty_pb.Empty.deserializeBinary
);


/**
 * @param {!proto.api.DeleteDeviceProfileTemplateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.google.protobuf.Empty)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.google.protobuf.Empty>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.DeviceProfileTemplateServiceClient.prototype.delete =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/Delete',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_Delete,
      callback);
};


/**
 * @param {!proto.api.DeleteDeviceProfileTemplateRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.google.protobuf.Empty>}
 *     Promise that resolves to the response
 */
proto.api.DeviceProfileTemplateServicePromiseClient.prototype.delete =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/Delete',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_Delete);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.api.ListDeviceProfileTemplatesRequest,
 *   !proto.api.ListDeviceProfileTemplatesResponse>}
 */
const methodDescriptor_DeviceProfileTemplateService_List = new grpc.web.MethodDescriptor(
  '/api.DeviceProfileTemplateService/List',
  grpc.web.MethodType.UNARY,
  proto.api.ListDeviceProfileTemplatesRequest,
  proto.api.ListDeviceProfileTemplatesResponse,
  /**
   * @param {!proto.api.ListDeviceProfileTemplatesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListDeviceProfileTemplatesResponse.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.api.ListDeviceProfileTemplatesRequest,
 *   !proto.api.ListDeviceProfileTemplatesResponse>}
 */
const methodInfo_DeviceProfileTemplateService_List = new grpc.web.AbstractClientBase.MethodInfo(
  proto.api.ListDeviceProfileTemplatesResponse,
  /**
   * @param {!proto.api.ListDeviceProfileTemplatesRequest} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.api.ListDeviceProfileTemplatesResponse.deserializeBinary
);


/**
 * @param {!proto.api.ListDeviceProfileTemplatesRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.api.ListDeviceProfileTemplatesResponse)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.api.ListDeviceProfileTemplatesResponse>|undefined}
 *     The XHR Node Readable Stream
 */
proto.api.DeviceProfileTemplateServiceClient.prototype.list =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/List',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_List,
      callback);
};


/**
 * @param {!proto.api.ListDeviceProfileTemplatesRequest} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.api.ListDeviceProfileTemplatesResponse>}
 *     Promise that resolves to the response
 */
proto.api.DeviceProfileTemplateServicePromiseClient.prototype.list =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/api.DeviceProfileTemplateService/List',
      request,
      metadata || {},
      methodDescriptor_DeviceProfileTemplateService_List);
};


module.exports = proto.api;

