// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_gateway_pb = require('../api/gateway_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
var common_common_pb = require('../common/common_pb.js');

function serialize_api_CreateGatewayRequest(arg) {
  if (!(arg instanceof api_gateway_pb.CreateGatewayRequest)) {
    throw new Error('Expected argument of type api.CreateGatewayRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateGatewayRequest(buffer_arg) {
  return api_gateway_pb.CreateGatewayRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteGatewayRequest(arg) {
  if (!(arg instanceof api_gateway_pb.DeleteGatewayRequest)) {
    throw new Error('Expected argument of type api.DeleteGatewayRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteGatewayRequest(buffer_arg) {
  return api_gateway_pb.DeleteGatewayRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GenerateGatewayClientCertificateRequest(arg) {
  if (!(arg instanceof api_gateway_pb.GenerateGatewayClientCertificateRequest)) {
    throw new Error('Expected argument of type api.GenerateGatewayClientCertificateRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GenerateGatewayClientCertificateRequest(buffer_arg) {
  return api_gateway_pb.GenerateGatewayClientCertificateRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GenerateGatewayClientCertificateResponse(arg) {
  if (!(arg instanceof api_gateway_pb.GenerateGatewayClientCertificateResponse)) {
    throw new Error('Expected argument of type api.GenerateGatewayClientCertificateResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GenerateGatewayClientCertificateResponse(buffer_arg) {
  return api_gateway_pb.GenerateGatewayClientCertificateResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetGatewayMetricsRequest(arg) {
  if (!(arg instanceof api_gateway_pb.GetGatewayMetricsRequest)) {
    throw new Error('Expected argument of type api.GetGatewayMetricsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetGatewayMetricsRequest(buffer_arg) {
  return api_gateway_pb.GetGatewayMetricsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetGatewayMetricsResponse(arg) {
  if (!(arg instanceof api_gateway_pb.GetGatewayMetricsResponse)) {
    throw new Error('Expected argument of type api.GetGatewayMetricsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetGatewayMetricsResponse(buffer_arg) {
  return api_gateway_pb.GetGatewayMetricsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetGatewayRequest(arg) {
  if (!(arg instanceof api_gateway_pb.GetGatewayRequest)) {
    throw new Error('Expected argument of type api.GetGatewayRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetGatewayRequest(buffer_arg) {
  return api_gateway_pb.GetGatewayRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetGatewayResponse(arg) {
  if (!(arg instanceof api_gateway_pb.GetGatewayResponse)) {
    throw new Error('Expected argument of type api.GetGatewayResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetGatewayResponse(buffer_arg) {
  return api_gateway_pb.GetGatewayResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListGatewaysRequest(arg) {
  if (!(arg instanceof api_gateway_pb.ListGatewaysRequest)) {
    throw new Error('Expected argument of type api.ListGatewaysRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListGatewaysRequest(buffer_arg) {
  return api_gateway_pb.ListGatewaysRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListGatewaysResponse(arg) {
  if (!(arg instanceof api_gateway_pb.ListGatewaysResponse)) {
    throw new Error('Expected argument of type api.ListGatewaysResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListGatewaysResponse(buffer_arg) {
  return api_gateway_pb.ListGatewaysResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateGatewayRequest(arg) {
  if (!(arg instanceof api_gateway_pb.UpdateGatewayRequest)) {
    throw new Error('Expected argument of type api.UpdateGatewayRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateGatewayRequest(buffer_arg) {
  return api_gateway_pb.UpdateGatewayRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// GatewayService is the service providing API methods for managing gateways.
var GatewayServiceService = exports.GatewayServiceService = {
  // Create creates the given gateway.
create: {
    path: '/api.GatewayService/Create',
    requestStream: false,
    responseStream: false,
    requestType: api_gateway_pb.CreateGatewayRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateGatewayRequest,
    requestDeserialize: deserialize_api_CreateGatewayRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get returns the gateway for the given Gateway ID.
get: {
    path: '/api.GatewayService/Get',
    requestStream: false,
    responseStream: false,
    requestType: api_gateway_pb.GetGatewayRequest,
    responseType: api_gateway_pb.GetGatewayResponse,
    requestSerialize: serialize_api_GetGatewayRequest,
    requestDeserialize: deserialize_api_GetGatewayRequest,
    responseSerialize: serialize_api_GetGatewayResponse,
    responseDeserialize: deserialize_api_GetGatewayResponse,
  },
  // Update updates the given gateway.
update: {
    path: '/api.GatewayService/Update',
    requestStream: false,
    responseStream: false,
    requestType: api_gateway_pb.UpdateGatewayRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateGatewayRequest,
    requestDeserialize: deserialize_api_UpdateGatewayRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete deletes the gateway matching the given Gateway ID.
delete: {
    path: '/api.GatewayService/Delete',
    requestStream: false,
    responseStream: false,
    requestType: api_gateway_pb.DeleteGatewayRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteGatewayRequest,
    requestDeserialize: deserialize_api_DeleteGatewayRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the list of gateways.
list: {
    path: '/api.GatewayService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_gateway_pb.ListGatewaysRequest,
    responseType: api_gateway_pb.ListGatewaysResponse,
    requestSerialize: serialize_api_ListGatewaysRequest,
    requestDeserialize: deserialize_api_ListGatewaysRequest,
    responseSerialize: serialize_api_ListGatewaysResponse,
    responseDeserialize: deserialize_api_ListGatewaysResponse,
  },
  // Generate client-certificate for the gateway.
generateClientCertificate: {
    path: '/api.GatewayService/GenerateClientCertificate',
    requestStream: false,
    responseStream: false,
    requestType: api_gateway_pb.GenerateGatewayClientCertificateRequest,
    responseType: api_gateway_pb.GenerateGatewayClientCertificateResponse,
    requestSerialize: serialize_api_GenerateGatewayClientCertificateRequest,
    requestDeserialize: deserialize_api_GenerateGatewayClientCertificateRequest,
    responseSerialize: serialize_api_GenerateGatewayClientCertificateResponse,
    responseDeserialize: deserialize_api_GenerateGatewayClientCertificateResponse,
  },
  // GetMetrics returns the gateway metrics.
getMetrics: {
    path: '/api.GatewayService/GetMetrics',
    requestStream: false,
    responseStream: false,
    requestType: api_gateway_pb.GetGatewayMetricsRequest,
    responseType: api_gateway_pb.GetGatewayMetricsResponse,
    requestSerialize: serialize_api_GetGatewayMetricsRequest,
    requestDeserialize: deserialize_api_GetGatewayMetricsRequest,
    responseSerialize: serialize_api_GetGatewayMetricsResponse,
    responseDeserialize: deserialize_api_GetGatewayMetricsResponse,
  },
};

exports.GatewayServiceClient = grpc.makeGenericClientConstructor(GatewayServiceService);
