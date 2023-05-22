// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_relay_pb = require('../api/relay_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_api_AddRelayDeviceRequest(arg) {
  if (!(arg instanceof api_relay_pb.AddRelayDeviceRequest)) {
    throw new Error('Expected argument of type api.AddRelayDeviceRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_AddRelayDeviceRequest(buffer_arg) {
  return api_relay_pb.AddRelayDeviceRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListRelayDevicesRequest(arg) {
  if (!(arg instanceof api_relay_pb.ListRelayDevicesRequest)) {
    throw new Error('Expected argument of type api.ListRelayDevicesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListRelayDevicesRequest(buffer_arg) {
  return api_relay_pb.ListRelayDevicesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListRelayDevicesResponse(arg) {
  if (!(arg instanceof api_relay_pb.ListRelayDevicesResponse)) {
    throw new Error('Expected argument of type api.ListRelayDevicesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListRelayDevicesResponse(buffer_arg) {
  return api_relay_pb.ListRelayDevicesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListRelaysRequest(arg) {
  if (!(arg instanceof api_relay_pb.ListRelaysRequest)) {
    throw new Error('Expected argument of type api.ListRelaysRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListRelaysRequest(buffer_arg) {
  return api_relay_pb.ListRelaysRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListRelaysResponse(arg) {
  if (!(arg instanceof api_relay_pb.ListRelaysResponse)) {
    throw new Error('Expected argument of type api.ListRelaysResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListRelaysResponse(buffer_arg) {
  return api_relay_pb.ListRelaysResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_RemoveRelayDeviceRequest(arg) {
  if (!(arg instanceof api_relay_pb.RemoveRelayDeviceRequest)) {
    throw new Error('Expected argument of type api.RemoveRelayDeviceRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_RemoveRelayDeviceRequest(buffer_arg) {
  return api_relay_pb.RemoveRelayDeviceRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// RelayService is the service providing API methos for managing relays.
var RelayServiceService = exports.RelayServiceService = {
  // List lists the relays for the given application id.
list: {
    path: '/api.RelayService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_relay_pb.ListRelaysRequest,
    responseType: api_relay_pb.ListRelaysResponse,
    requestSerialize: serialize_api_ListRelaysRequest,
    requestDeserialize: deserialize_api_ListRelaysRequest,
    responseSerialize: serialize_api_ListRelaysResponse,
    responseDeserialize: deserialize_api_ListRelaysResponse,
  },
  // AddDevice adds the given device to the relay.
addDevice: {
    path: '/api.RelayService/AddDevice',
    requestStream: false,
    responseStream: false,
    requestType: api_relay_pb.AddRelayDeviceRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_AddRelayDeviceRequest,
    requestDeserialize: deserialize_api_AddRelayDeviceRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // RemoveDevice removes the given device from the relay.
removeDevice: {
    path: '/api.RelayService/RemoveDevice',
    requestStream: false,
    responseStream: false,
    requestType: api_relay_pb.RemoveRelayDeviceRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_RemoveRelayDeviceRequest,
    requestDeserialize: deserialize_api_RemoveRelayDeviceRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // ListDevices lists the devices for the given relay.
listDevices: {
    path: '/api.RelayService/ListDevices',
    requestStream: false,
    responseStream: false,
    requestType: api_relay_pb.ListRelayDevicesRequest,
    responseType: api_relay_pb.ListRelayDevicesResponse,
    requestSerialize: serialize_api_ListRelayDevicesRequest,
    requestDeserialize: deserialize_api_ListRelayDevicesRequest,
    responseSerialize: serialize_api_ListRelayDevicesResponse,
    responseDeserialize: deserialize_api_ListRelayDevicesResponse,
  },
};

exports.RelayServiceClient = grpc.makeGenericClientConstructor(RelayServiceService);
