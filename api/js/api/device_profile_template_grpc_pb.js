// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_device_profile_template_pb = require('../api/device_profile_template_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
var common_common_pb = require('../common/common_pb.js');
var api_device_profile_pb = require('../api/device_profile_pb.js');

function serialize_api_CreateDeviceProfileTemplateRequest(arg) {
  if (!(arg instanceof api_device_profile_template_pb.CreateDeviceProfileTemplateRequest)) {
    throw new Error('Expected argument of type api.CreateDeviceProfileTemplateRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateDeviceProfileTemplateRequest(buffer_arg) {
  return api_device_profile_template_pb.CreateDeviceProfileTemplateRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteDeviceProfileTemplateRequest(arg) {
  if (!(arg instanceof api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest)) {
    throw new Error('Expected argument of type api.DeleteDeviceProfileTemplateRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteDeviceProfileTemplateRequest(buffer_arg) {
  return api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceProfileTemplateRequest(arg) {
  if (!(arg instanceof api_device_profile_template_pb.GetDeviceProfileTemplateRequest)) {
    throw new Error('Expected argument of type api.GetDeviceProfileTemplateRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceProfileTemplateRequest(buffer_arg) {
  return api_device_profile_template_pb.GetDeviceProfileTemplateRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceProfileTemplateResponse(arg) {
  if (!(arg instanceof api_device_profile_template_pb.GetDeviceProfileTemplateResponse)) {
    throw new Error('Expected argument of type api.GetDeviceProfileTemplateResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceProfileTemplateResponse(buffer_arg) {
  return api_device_profile_template_pb.GetDeviceProfileTemplateResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListDeviceProfileTemplatesRequest(arg) {
  if (!(arg instanceof api_device_profile_template_pb.ListDeviceProfileTemplatesRequest)) {
    throw new Error('Expected argument of type api.ListDeviceProfileTemplatesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListDeviceProfileTemplatesRequest(buffer_arg) {
  return api_device_profile_template_pb.ListDeviceProfileTemplatesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListDeviceProfileTemplatesResponse(arg) {
  if (!(arg instanceof api_device_profile_template_pb.ListDeviceProfileTemplatesResponse)) {
    throw new Error('Expected argument of type api.ListDeviceProfileTemplatesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListDeviceProfileTemplatesResponse(buffer_arg) {
  return api_device_profile_template_pb.ListDeviceProfileTemplatesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateDeviceProfileTemplateRequest(arg) {
  if (!(arg instanceof api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest)) {
    throw new Error('Expected argument of type api.UpdateDeviceProfileTemplateRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateDeviceProfileTemplateRequest(buffer_arg) {
  return api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// DeviceProfileTemplateService is the service providing API methods for managing device-profile templates.
var DeviceProfileTemplateServiceService = exports.DeviceProfileTemplateServiceService = {
  // Create the given device-profile template.
create: {
    path: '/api.DeviceProfileTemplateService/Create',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_template_pb.CreateDeviceProfileTemplateRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateDeviceProfileTemplateRequest,
    requestDeserialize: deserialize_api_CreateDeviceProfileTemplateRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the device-profile template for the given ID.
get: {
    path: '/api.DeviceProfileTemplateService/Get',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_template_pb.GetDeviceProfileTemplateRequest,
    responseType: api_device_profile_template_pb.GetDeviceProfileTemplateResponse,
    requestSerialize: serialize_api_GetDeviceProfileTemplateRequest,
    requestDeserialize: deserialize_api_GetDeviceProfileTemplateRequest,
    responseSerialize: serialize_api_GetDeviceProfileTemplateResponse,
    responseDeserialize: deserialize_api_GetDeviceProfileTemplateResponse,
  },
  // Update the given device-profile template.
update: {
    path: '/api.DeviceProfileTemplateService/Update',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_template_pb.UpdateDeviceProfileTemplateRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateDeviceProfileTemplateRequest,
    requestDeserialize: deserialize_api_UpdateDeviceProfileTemplateRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the device-profile template with the given ID.
delete: {
    path: '/api.DeviceProfileTemplateService/Delete',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_template_pb.DeleteDeviceProfileTemplateRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteDeviceProfileTemplateRequest,
    requestDeserialize: deserialize_api_DeleteDeviceProfileTemplateRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // List the available device-profile templates.
list: {
    path: '/api.DeviceProfileTemplateService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_device_profile_template_pb.ListDeviceProfileTemplatesRequest,
    responseType: api_device_profile_template_pb.ListDeviceProfileTemplatesResponse,
    requestSerialize: serialize_api_ListDeviceProfileTemplatesRequest,
    requestDeserialize: deserialize_api_ListDeviceProfileTemplatesRequest,
    responseSerialize: serialize_api_ListDeviceProfileTemplatesResponse,
    responseDeserialize: deserialize_api_ListDeviceProfileTemplatesResponse,
  },
};

exports.DeviceProfileTemplateServiceClient = grpc.makeGenericClientConstructor(DeviceProfileTemplateServiceService);
