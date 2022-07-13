// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_device_profile_pb = require('../api/device_profile_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
var common_common_pb = require('../common/common_pb.js');

function serialize_api_CreateDeviceProfileRequest(arg) {
  if (!(arg instanceof api_device_profile_pb.CreateDeviceProfileRequest)) {
    throw new Error('Expected argument of type api.CreateDeviceProfileRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateDeviceProfileRequest(buffer_arg) {
  return api_device_profile_pb.CreateDeviceProfileRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateDeviceProfileResponse(arg) {
  if (!(arg instanceof api_device_profile_pb.CreateDeviceProfileResponse)) {
    throw new Error('Expected argument of type api.CreateDeviceProfileResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateDeviceProfileResponse(buffer_arg) {
  return api_device_profile_pb.CreateDeviceProfileResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteDeviceProfileRequest(arg) {
  if (!(arg instanceof api_device_profile_pb.DeleteDeviceProfileRequest)) {
    throw new Error('Expected argument of type api.DeleteDeviceProfileRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteDeviceProfileRequest(buffer_arg) {
  return api_device_profile_pb.DeleteDeviceProfileRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceProfileRequest(arg) {
  if (!(arg instanceof api_device_profile_pb.GetDeviceProfileRequest)) {
    throw new Error('Expected argument of type api.GetDeviceProfileRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceProfileRequest(buffer_arg) {
  return api_device_profile_pb.GetDeviceProfileRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceProfileResponse(arg) {
  if (!(arg instanceof api_device_profile_pb.GetDeviceProfileResponse)) {
    throw new Error('Expected argument of type api.GetDeviceProfileResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceProfileResponse(buffer_arg) {
  return api_device_profile_pb.GetDeviceProfileResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListDeviceProfileAdrAlgorithmsResponse(arg) {
  if (!(arg instanceof api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse)) {
    throw new Error('Expected argument of type api.ListDeviceProfileAdrAlgorithmsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListDeviceProfileAdrAlgorithmsResponse(buffer_arg) {
  return api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListDeviceProfilesRequest(arg) {
  if (!(arg instanceof api_device_profile_pb.ListDeviceProfilesRequest)) {
    throw new Error('Expected argument of type api.ListDeviceProfilesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListDeviceProfilesRequest(buffer_arg) {
  return api_device_profile_pb.ListDeviceProfilesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListDeviceProfilesResponse(arg) {
  if (!(arg instanceof api_device_profile_pb.ListDeviceProfilesResponse)) {
    throw new Error('Expected argument of type api.ListDeviceProfilesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListDeviceProfilesResponse(buffer_arg) {
  return api_device_profile_pb.ListDeviceProfilesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateDeviceProfileRequest(arg) {
  if (!(arg instanceof api_device_profile_pb.UpdateDeviceProfileRequest)) {
    throw new Error('Expected argument of type api.UpdateDeviceProfileRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateDeviceProfileRequest(buffer_arg) {
  return api_device_profile_pb.UpdateDeviceProfileRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// DeviceProfileService is the service providing API methods for managing device-profiles.
var DeviceProfileServiceService = exports.DeviceProfileServiceService = {
  // Create the given device-profile.
create: {
    path: '/api.DeviceProfileService/Create',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_pb.CreateDeviceProfileRequest,
    responseType: api_device_profile_pb.CreateDeviceProfileResponse,
    requestSerialize: serialize_api_CreateDeviceProfileRequest,
    requestDeserialize: deserialize_api_CreateDeviceProfileRequest,
    responseSerialize: serialize_api_CreateDeviceProfileResponse,
    responseDeserialize: deserialize_api_CreateDeviceProfileResponse,
  },
  // Get the device-profile for the given ID.
get: {
    path: '/api.DeviceProfileService/Get',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_pb.GetDeviceProfileRequest,
    responseType: api_device_profile_pb.GetDeviceProfileResponse,
    requestSerialize: serialize_api_GetDeviceProfileRequest,
    requestDeserialize: deserialize_api_GetDeviceProfileRequest,
    responseSerialize: serialize_api_GetDeviceProfileResponse,
    responseDeserialize: deserialize_api_GetDeviceProfileResponse,
  },
  // Update the given device-profile.
update: {
    path: '/api.DeviceProfileService/Update',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_pb.UpdateDeviceProfileRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateDeviceProfileRequest,
    requestDeserialize: deserialize_api_UpdateDeviceProfileRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the device-profile with the given ID.
delete: {
    path: '/api.DeviceProfileService/Delete',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_pb.DeleteDeviceProfileRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteDeviceProfileRequest,
    requestDeserialize: deserialize_api_DeleteDeviceProfileRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // List the available device-profiles.
list: {
    path: '/api.DeviceProfileService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_pb.ListDeviceProfilesRequest,
    responseType: api_device_profile_pb.ListDeviceProfilesResponse,
    requestSerialize: serialize_api_ListDeviceProfilesRequest,
    requestDeserialize: deserialize_api_ListDeviceProfilesRequest,
    responseSerialize: serialize_api_ListDeviceProfilesResponse,
    responseDeserialize: deserialize_api_ListDeviceProfilesResponse,
  },
  // List available ADR algorithms.
listAdrAlgorithms: {
    path: '/api.DeviceProfileService/ListAdrAlgorithms',
    requestStream: false,
    responseStream: false,
    requestType: google_protobuf_empty_pb.Empty,
    responseType: api_device_profile_pb.ListDeviceProfileAdrAlgorithmsResponse,
    requestSerialize: serialize_google_protobuf_Empty,
    requestDeserialize: deserialize_google_protobuf_Empty,
    responseSerialize: serialize_api_ListDeviceProfileAdrAlgorithmsResponse,
    responseDeserialize: deserialize_api_ListDeviceProfileAdrAlgorithmsResponse,
  },
};

exports.DeviceProfileServiceClient = grpc.makeGenericClientConstructor(DeviceProfileServiceService);
