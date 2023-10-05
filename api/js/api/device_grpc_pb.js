// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_device_pb = require('../api/device_pb.js');
var common_common_pb = require('../common/common_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_struct_pb = require('google-protobuf/google/protobuf/struct_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_api_ActivateDeviceRequest(arg) {
  if (!(arg instanceof api_device_pb.ActivateDeviceRequest)) {
    throw new Error('Expected argument of type api.ActivateDeviceRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ActivateDeviceRequest(buffer_arg) {
  return api_device_pb.ActivateDeviceRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateDeviceKeysRequest(arg) {
  if (!(arg instanceof api_device_pb.CreateDeviceKeysRequest)) {
    throw new Error('Expected argument of type api.CreateDeviceKeysRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateDeviceKeysRequest(buffer_arg) {
  return api_device_pb.CreateDeviceKeysRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateDeviceRequest(arg) {
  if (!(arg instanceof api_device_pb.CreateDeviceRequest)) {
    throw new Error('Expected argument of type api.CreateDeviceRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateDeviceRequest(buffer_arg) {
  return api_device_pb.CreateDeviceRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeactivateDeviceRequest(arg) {
  if (!(arg instanceof api_device_pb.DeactivateDeviceRequest)) {
    throw new Error('Expected argument of type api.DeactivateDeviceRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeactivateDeviceRequest(buffer_arg) {
  return api_device_pb.DeactivateDeviceRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteDeviceKeysRequest(arg) {
  if (!(arg instanceof api_device_pb.DeleteDeviceKeysRequest)) {
    throw new Error('Expected argument of type api.DeleteDeviceKeysRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteDeviceKeysRequest(buffer_arg) {
  return api_device_pb.DeleteDeviceKeysRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteDeviceRequest(arg) {
  if (!(arg instanceof api_device_pb.DeleteDeviceRequest)) {
    throw new Error('Expected argument of type api.DeleteDeviceRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteDeviceRequest(buffer_arg) {
  return api_device_pb.DeleteDeviceRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_EnqueueDeviceQueueItemRequest(arg) {
  if (!(arg instanceof api_device_pb.EnqueueDeviceQueueItemRequest)) {
    throw new Error('Expected argument of type api.EnqueueDeviceQueueItemRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_EnqueueDeviceQueueItemRequest(buffer_arg) {
  return api_device_pb.EnqueueDeviceQueueItemRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_EnqueueDeviceQueueItemResponse(arg) {
  if (!(arg instanceof api_device_pb.EnqueueDeviceQueueItemResponse)) {
    throw new Error('Expected argument of type api.EnqueueDeviceQueueItemResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_EnqueueDeviceQueueItemResponse(buffer_arg) {
  return api_device_pb.EnqueueDeviceQueueItemResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_FlushDevNoncesRequest(arg) {
  if (!(arg instanceof api_device_pb.FlushDevNoncesRequest)) {
    throw new Error('Expected argument of type api.FlushDevNoncesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_FlushDevNoncesRequest(buffer_arg) {
  return api_device_pb.FlushDevNoncesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_FlushDeviceQueueRequest(arg) {
  if (!(arg instanceof api_device_pb.FlushDeviceQueueRequest)) {
    throw new Error('Expected argument of type api.FlushDeviceQueueRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_FlushDeviceQueueRequest(buffer_arg) {
  return api_device_pb.FlushDeviceQueueRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceActivationRequest(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceActivationRequest)) {
    throw new Error('Expected argument of type api.GetDeviceActivationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceActivationRequest(buffer_arg) {
  return api_device_pb.GetDeviceActivationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceActivationResponse(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceActivationResponse)) {
    throw new Error('Expected argument of type api.GetDeviceActivationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceActivationResponse(buffer_arg) {
  return api_device_pb.GetDeviceActivationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceKeysRequest(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceKeysRequest)) {
    throw new Error('Expected argument of type api.GetDeviceKeysRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceKeysRequest(buffer_arg) {
  return api_device_pb.GetDeviceKeysRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceKeysResponse(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceKeysResponse)) {
    throw new Error('Expected argument of type api.GetDeviceKeysResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceKeysResponse(buffer_arg) {
  return api_device_pb.GetDeviceKeysResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceLinkMetricsRequest(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceLinkMetricsRequest)) {
    throw new Error('Expected argument of type api.GetDeviceLinkMetricsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceLinkMetricsRequest(buffer_arg) {
  return api_device_pb.GetDeviceLinkMetricsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceLinkMetricsResponse(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceLinkMetricsResponse)) {
    throw new Error('Expected argument of type api.GetDeviceLinkMetricsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceLinkMetricsResponse(buffer_arg) {
  return api_device_pb.GetDeviceLinkMetricsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceMetricsRequest(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceMetricsRequest)) {
    throw new Error('Expected argument of type api.GetDeviceMetricsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceMetricsRequest(buffer_arg) {
  return api_device_pb.GetDeviceMetricsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceMetricsResponse(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceMetricsResponse)) {
    throw new Error('Expected argument of type api.GetDeviceMetricsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceMetricsResponse(buffer_arg) {
  return api_device_pb.GetDeviceMetricsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceNextFCntDownRequest(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceNextFCntDownRequest)) {
    throw new Error('Expected argument of type api.GetDeviceNextFCntDownRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceNextFCntDownRequest(buffer_arg) {
  return api_device_pb.GetDeviceNextFCntDownRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceNextFCntDownResponse(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceNextFCntDownResponse)) {
    throw new Error('Expected argument of type api.GetDeviceNextFCntDownResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceNextFCntDownResponse(buffer_arg) {
  return api_device_pb.GetDeviceNextFCntDownResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceQueueItemsRequest(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceQueueItemsRequest)) {
    throw new Error('Expected argument of type api.GetDeviceQueueItemsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceQueueItemsRequest(buffer_arg) {
  return api_device_pb.GetDeviceQueueItemsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceQueueItemsResponse(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceQueueItemsResponse)) {
    throw new Error('Expected argument of type api.GetDeviceQueueItemsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceQueueItemsResponse(buffer_arg) {
  return api_device_pb.GetDeviceQueueItemsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceRequest(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceRequest)) {
    throw new Error('Expected argument of type api.GetDeviceRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceRequest(buffer_arg) {
  return api_device_pb.GetDeviceRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDeviceResponse(arg) {
  if (!(arg instanceof api_device_pb.GetDeviceResponse)) {
    throw new Error('Expected argument of type api.GetDeviceResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDeviceResponse(buffer_arg) {
  return api_device_pb.GetDeviceResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetRandomDevAddrRequest(arg) {
  if (!(arg instanceof api_device_pb.GetRandomDevAddrRequest)) {
    throw new Error('Expected argument of type api.GetRandomDevAddrRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetRandomDevAddrRequest(buffer_arg) {
  return api_device_pb.GetRandomDevAddrRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetRandomDevAddrResponse(arg) {
  if (!(arg instanceof api_device_pb.GetRandomDevAddrResponse)) {
    throw new Error('Expected argument of type api.GetRandomDevAddrResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetRandomDevAddrResponse(buffer_arg) {
  return api_device_pb.GetRandomDevAddrResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListDevicesRequest(arg) {
  if (!(arg instanceof api_device_pb.ListDevicesRequest)) {
    throw new Error('Expected argument of type api.ListDevicesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListDevicesRequest(buffer_arg) {
  return api_device_pb.ListDevicesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListDevicesResponse(arg) {
  if (!(arg instanceof api_device_pb.ListDevicesResponse)) {
    throw new Error('Expected argument of type api.ListDevicesResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListDevicesResponse(buffer_arg) {
  return api_device_pb.ListDevicesResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateDeviceKeysRequest(arg) {
  if (!(arg instanceof api_device_pb.UpdateDeviceKeysRequest)) {
    throw new Error('Expected argument of type api.UpdateDeviceKeysRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateDeviceKeysRequest(buffer_arg) {
  return api_device_pb.UpdateDeviceKeysRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateDeviceRequest(arg) {
  if (!(arg instanceof api_device_pb.UpdateDeviceRequest)) {
    throw new Error('Expected argument of type api.UpdateDeviceRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateDeviceRequest(buffer_arg) {
  return api_device_pb.UpdateDeviceRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// DeviceService is the service providing API methods for managing devices.
var DeviceServiceService = exports.DeviceServiceService = {
  // Create the given device.
create: {
    path: '/api.DeviceService/Create',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.CreateDeviceRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateDeviceRequest,
    requestDeserialize: deserialize_api_CreateDeviceRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get returns the device for the given DevEUI.
get: {
    path: '/api.DeviceService/Get',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.GetDeviceRequest,
    responseType: api_device_pb.GetDeviceResponse,
    requestSerialize: serialize_api_GetDeviceRequest,
    requestDeserialize: deserialize_api_GetDeviceRequest,
    responseSerialize: serialize_api_GetDeviceResponse,
    responseDeserialize: deserialize_api_GetDeviceResponse,
  },
  // Update the given device.
update: {
    path: '/api.DeviceService/Update',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.UpdateDeviceRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateDeviceRequest,
    requestDeserialize: deserialize_api_UpdateDeviceRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the device with the given DevEUI.
delete: {
    path: '/api.DeviceService/Delete',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.DeleteDeviceRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteDeviceRequest,
    requestDeserialize: deserialize_api_DeleteDeviceRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the list of devices.
list: {
    path: '/api.DeviceService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.ListDevicesRequest,
    responseType: api_device_pb.ListDevicesResponse,
    requestSerialize: serialize_api_ListDevicesRequest,
    requestDeserialize: deserialize_api_ListDevicesRequest,
    responseSerialize: serialize_api_ListDevicesResponse,
    responseDeserialize: deserialize_api_ListDevicesResponse,
  },
  // Create the given device-keys.
createKeys: {
    path: '/api.DeviceService/CreateKeys',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.CreateDeviceKeysRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateDeviceKeysRequest,
    requestDeserialize: deserialize_api_CreateDeviceKeysRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the device-keys for the given DevEUI.
getKeys: {
    path: '/api.DeviceService/GetKeys',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.GetDeviceKeysRequest,
    responseType: api_device_pb.GetDeviceKeysResponse,
    requestSerialize: serialize_api_GetDeviceKeysRequest,
    requestDeserialize: deserialize_api_GetDeviceKeysRequest,
    responseSerialize: serialize_api_GetDeviceKeysResponse,
    responseDeserialize: deserialize_api_GetDeviceKeysResponse,
  },
  // Update the given device-keys.
updateKeys: {
    path: '/api.DeviceService/UpdateKeys',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.UpdateDeviceKeysRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateDeviceKeysRequest,
    requestDeserialize: deserialize_api_UpdateDeviceKeysRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the device-keys for the given DevEUI.
deleteKeys: {
    path: '/api.DeviceService/DeleteKeys',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.DeleteDeviceKeysRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteDeviceKeysRequest,
    requestDeserialize: deserialize_api_DeleteDeviceKeysRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // FlushDevNonces flushes the OTAA device nonces.
flushDevNonces: {
    path: '/api.DeviceService/FlushDevNonces',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.FlushDevNoncesRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_FlushDevNoncesRequest,
    requestDeserialize: deserialize_api_FlushDevNoncesRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Activate (re)activates the device with the given parameters (for ABP or for
// importing OTAA activations).
activate: {
    path: '/api.DeviceService/Activate',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.ActivateDeviceRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_ActivateDeviceRequest,
    requestDeserialize: deserialize_api_ActivateDeviceRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Deactivate de-activates the device.
deactivate: {
    path: '/api.DeviceService/Deactivate',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.DeactivateDeviceRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeactivateDeviceRequest,
    requestDeserialize: deserialize_api_DeactivateDeviceRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // GetActivation returns the current activation details of the device (OTAA or
// ABP).
getActivation: {
    path: '/api.DeviceService/GetActivation',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.GetDeviceActivationRequest,
    responseType: api_device_pb.GetDeviceActivationResponse,
    requestSerialize: serialize_api_GetDeviceActivationRequest,
    requestDeserialize: deserialize_api_GetDeviceActivationRequest,
    responseSerialize: serialize_api_GetDeviceActivationResponse,
    responseDeserialize: deserialize_api_GetDeviceActivationResponse,
  },
  // GetRandomDevAddr returns a random DevAddr taking the NwkID prefix into
// account.
getRandomDevAddr: {
    path: '/api.DeviceService/GetRandomDevAddr',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.GetRandomDevAddrRequest,
    responseType: api_device_pb.GetRandomDevAddrResponse,
    requestSerialize: serialize_api_GetRandomDevAddrRequest,
    requestDeserialize: deserialize_api_GetRandomDevAddrRequest,
    responseSerialize: serialize_api_GetRandomDevAddrResponse,
    responseDeserialize: deserialize_api_GetRandomDevAddrResponse,
  },
  // GetMetrics returns the device metrics.
// Note that this requires a device-profile with codec and measurements
// configured.
getMetrics: {
    path: '/api.DeviceService/GetMetrics',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.GetDeviceMetricsRequest,
    responseType: api_device_pb.GetDeviceMetricsResponse,
    requestSerialize: serialize_api_GetDeviceMetricsRequest,
    requestDeserialize: deserialize_api_GetDeviceMetricsRequest,
    responseSerialize: serialize_api_GetDeviceMetricsResponse,
    responseDeserialize: deserialize_api_GetDeviceMetricsResponse,
  },
  // GetLinkMetrics returns the device link metrics.
// This includes uplinks, downlinks, RSSI, SNR, etc...
getLinkMetrics: {
    path: '/api.DeviceService/GetLinkMetrics',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.GetDeviceLinkMetricsRequest,
    responseType: api_device_pb.GetDeviceLinkMetricsResponse,
    requestSerialize: serialize_api_GetDeviceLinkMetricsRequest,
    requestDeserialize: deserialize_api_GetDeviceLinkMetricsRequest,
    responseSerialize: serialize_api_GetDeviceLinkMetricsResponse,
    responseDeserialize: deserialize_api_GetDeviceLinkMetricsResponse,
  },
  // Enqueue adds the given item to the downlink queue.
enqueue: {
    path: '/api.DeviceService/Enqueue',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.EnqueueDeviceQueueItemRequest,
    responseType: api_device_pb.EnqueueDeviceQueueItemResponse,
    requestSerialize: serialize_api_EnqueueDeviceQueueItemRequest,
    requestDeserialize: deserialize_api_EnqueueDeviceQueueItemRequest,
    responseSerialize: serialize_api_EnqueueDeviceQueueItemResponse,
    responseDeserialize: deserialize_api_EnqueueDeviceQueueItemResponse,
  },
  // FlushQueue flushes the downlink device-queue.
flushQueue: {
    path: '/api.DeviceService/FlushQueue',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.FlushDeviceQueueRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_FlushDeviceQueueRequest,
    requestDeserialize: deserialize_api_FlushDeviceQueueRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // GetQueue returns the downlink device-queue.
getQueue: {
    path: '/api.DeviceService/GetQueue',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.GetDeviceQueueItemsRequest,
    responseType: api_device_pb.GetDeviceQueueItemsResponse,
    requestSerialize: serialize_api_GetDeviceQueueItemsRequest,
    requestDeserialize: deserialize_api_GetDeviceQueueItemsRequest,
    responseSerialize: serialize_api_GetDeviceQueueItemsResponse,
    responseDeserialize: deserialize_api_GetDeviceQueueItemsResponse,
  },
  // GetNextFCntDown returns the next FCntDown to use for enqueing encrypted
// downlinks. The difference with the DeviceActivation f_cont_down is that
// this method takes potential existing queue-items into account.
getNextFCntDown: {
    path: '/api.DeviceService/GetNextFCntDown',
    requestStream: false,
    responseStream: false,
    requestType: api_device_pb.GetDeviceNextFCntDownRequest,
    responseType: api_device_pb.GetDeviceNextFCntDownResponse,
    requestSerialize: serialize_api_GetDeviceNextFCntDownRequest,
    requestDeserialize: deserialize_api_GetDeviceNextFCntDownRequest,
    responseSerialize: serialize_api_GetDeviceNextFCntDownResponse,
    responseDeserialize: deserialize_api_GetDeviceNextFCntDownResponse,
  },
};

exports.DeviceServiceClient = grpc.makeGenericClientConstructor(DeviceServiceService);
