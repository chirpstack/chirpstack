// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_multicast_group_pb = require('../api/multicast_group_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
var common_common_pb = require('../common/common_pb.js');

function serialize_api_AddDeviceToMulticastGroupRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.AddDeviceToMulticastGroupRequest)) {
    throw new Error('Expected argument of type api.AddDeviceToMulticastGroupRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_AddDeviceToMulticastGroupRequest(buffer_arg) {
  return api_multicast_group_pb.AddDeviceToMulticastGroupRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_AddGatewayToMulticastGroupRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.AddGatewayToMulticastGroupRequest)) {
    throw new Error('Expected argument of type api.AddGatewayToMulticastGroupRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_AddGatewayToMulticastGroupRequest(buffer_arg) {
  return api_multicast_group_pb.AddGatewayToMulticastGroupRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateMulticastGroupRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.CreateMulticastGroupRequest)) {
    throw new Error('Expected argument of type api.CreateMulticastGroupRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateMulticastGroupRequest(buffer_arg) {
  return api_multicast_group_pb.CreateMulticastGroupRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateMulticastGroupResponse(arg) {
  if (!(arg instanceof api_multicast_group_pb.CreateMulticastGroupResponse)) {
    throw new Error('Expected argument of type api.CreateMulticastGroupResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateMulticastGroupResponse(buffer_arg) {
  return api_multicast_group_pb.CreateMulticastGroupResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteMulticastGroupRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.DeleteMulticastGroupRequest)) {
    throw new Error('Expected argument of type api.DeleteMulticastGroupRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteMulticastGroupRequest(buffer_arg) {
  return api_multicast_group_pb.DeleteMulticastGroupRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_EnqueueMulticastGroupQueueItemRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest)) {
    throw new Error('Expected argument of type api.EnqueueMulticastGroupQueueItemRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_EnqueueMulticastGroupQueueItemRequest(buffer_arg) {
  return api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_EnqueueMulticastGroupQueueItemResponse(arg) {
  if (!(arg instanceof api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse)) {
    throw new Error('Expected argument of type api.EnqueueMulticastGroupQueueItemResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_EnqueueMulticastGroupQueueItemResponse(buffer_arg) {
  return api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_FlushMulticastGroupQueueRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.FlushMulticastGroupQueueRequest)) {
    throw new Error('Expected argument of type api.FlushMulticastGroupQueueRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_FlushMulticastGroupQueueRequest(buffer_arg) {
  return api_multicast_group_pb.FlushMulticastGroupQueueRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetMulticastGroupRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.GetMulticastGroupRequest)) {
    throw new Error('Expected argument of type api.GetMulticastGroupRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetMulticastGroupRequest(buffer_arg) {
  return api_multicast_group_pb.GetMulticastGroupRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetMulticastGroupResponse(arg) {
  if (!(arg instanceof api_multicast_group_pb.GetMulticastGroupResponse)) {
    throw new Error('Expected argument of type api.GetMulticastGroupResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetMulticastGroupResponse(buffer_arg) {
  return api_multicast_group_pb.GetMulticastGroupResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListMulticastGroupQueueRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.ListMulticastGroupQueueRequest)) {
    throw new Error('Expected argument of type api.ListMulticastGroupQueueRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListMulticastGroupQueueRequest(buffer_arg) {
  return api_multicast_group_pb.ListMulticastGroupQueueRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListMulticastGroupQueueResponse(arg) {
  if (!(arg instanceof api_multicast_group_pb.ListMulticastGroupQueueResponse)) {
    throw new Error('Expected argument of type api.ListMulticastGroupQueueResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListMulticastGroupQueueResponse(buffer_arg) {
  return api_multicast_group_pb.ListMulticastGroupQueueResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListMulticastGroupsRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.ListMulticastGroupsRequest)) {
    throw new Error('Expected argument of type api.ListMulticastGroupsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListMulticastGroupsRequest(buffer_arg) {
  return api_multicast_group_pb.ListMulticastGroupsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListMulticastGroupsResponse(arg) {
  if (!(arg instanceof api_multicast_group_pb.ListMulticastGroupsResponse)) {
    throw new Error('Expected argument of type api.ListMulticastGroupsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListMulticastGroupsResponse(buffer_arg) {
  return api_multicast_group_pb.ListMulticastGroupsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_RemoveDeviceFromMulticastGroupRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest)) {
    throw new Error('Expected argument of type api.RemoveDeviceFromMulticastGroupRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_RemoveDeviceFromMulticastGroupRequest(buffer_arg) {
  return api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_RemoveGatewayFromMulticastGroupRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest)) {
    throw new Error('Expected argument of type api.RemoveGatewayFromMulticastGroupRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_RemoveGatewayFromMulticastGroupRequest(buffer_arg) {
  return api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateMulticastGroupRequest(arg) {
  if (!(arg instanceof api_multicast_group_pb.UpdateMulticastGroupRequest)) {
    throw new Error('Expected argument of type api.UpdateMulticastGroupRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateMulticastGroupRequest(buffer_arg) {
  return api_multicast_group_pb.UpdateMulticastGroupRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// MulticastGroupService is the service managing multicast-groups.
var MulticastGroupServiceService = exports.MulticastGroupServiceService = {
  // Create the given multicast group.
create: {
    path: '/api.MulticastGroupService/Create',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.CreateMulticastGroupRequest,
    responseType: api_multicast_group_pb.CreateMulticastGroupResponse,
    requestSerialize: serialize_api_CreateMulticastGroupRequest,
    requestDeserialize: deserialize_api_CreateMulticastGroupRequest,
    responseSerialize: serialize_api_CreateMulticastGroupResponse,
    responseDeserialize: deserialize_api_CreateMulticastGroupResponse,
  },
  // Get returns the multicast group for the given ID.
get: {
    path: '/api.MulticastGroupService/Get',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.GetMulticastGroupRequest,
    responseType: api_multicast_group_pb.GetMulticastGroupResponse,
    requestSerialize: serialize_api_GetMulticastGroupRequest,
    requestDeserialize: deserialize_api_GetMulticastGroupRequest,
    responseSerialize: serialize_api_GetMulticastGroupResponse,
    responseDeserialize: deserialize_api_GetMulticastGroupResponse,
  },
  // Update the given multicast group.
update: {
    path: '/api.MulticastGroupService/Update',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.UpdateMulticastGroupRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateMulticastGroupRequest,
    requestDeserialize: deserialize_api_UpdateMulticastGroupRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the multicast-group with the given ID.
delete: {
    path: '/api.MulticastGroupService/Delete',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.DeleteMulticastGroupRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteMulticastGroupRequest,
    requestDeserialize: deserialize_api_DeleteMulticastGroupRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // List the available multicast groups.
list: {
    path: '/api.MulticastGroupService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.ListMulticastGroupsRequest,
    responseType: api_multicast_group_pb.ListMulticastGroupsResponse,
    requestSerialize: serialize_api_ListMulticastGroupsRequest,
    requestDeserialize: deserialize_api_ListMulticastGroupsRequest,
    responseSerialize: serialize_api_ListMulticastGroupsResponse,
    responseDeserialize: deserialize_api_ListMulticastGroupsResponse,
  },
  // Add a device to the multicast group.
addDevice: {
    path: '/api.MulticastGroupService/AddDevice',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.AddDeviceToMulticastGroupRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_AddDeviceToMulticastGroupRequest,
    requestDeserialize: deserialize_api_AddDeviceToMulticastGroupRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Remove a device from the multicast group.
removeDevice: {
    path: '/api.MulticastGroupService/RemoveDevice',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.RemoveDeviceFromMulticastGroupRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_RemoveDeviceFromMulticastGroupRequest,
    requestDeserialize: deserialize_api_RemoveDeviceFromMulticastGroupRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Add a gateway to the multicast group.
addGateway: {
    path: '/api.MulticastGroupService/AddGateway',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.AddGatewayToMulticastGroupRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_AddGatewayToMulticastGroupRequest,
    requestDeserialize: deserialize_api_AddGatewayToMulticastGroupRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Remove a gateway from the multicast group.
removeGateway: {
    path: '/api.MulticastGroupService/RemoveGateway',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.RemoveGatewayFromMulticastGroupRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_RemoveGatewayFromMulticastGroupRequest,
    requestDeserialize: deserialize_api_RemoveGatewayFromMulticastGroupRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Add the given item to the multicast group queue.
enqueue: {
    path: '/api.MulticastGroupService/Enqueue',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.EnqueueMulticastGroupQueueItemRequest,
    responseType: api_multicast_group_pb.EnqueueMulticastGroupQueueItemResponse,
    requestSerialize: serialize_api_EnqueueMulticastGroupQueueItemRequest,
    requestDeserialize: deserialize_api_EnqueueMulticastGroupQueueItemRequest,
    responseSerialize: serialize_api_EnqueueMulticastGroupQueueItemResponse,
    responseDeserialize: deserialize_api_EnqueueMulticastGroupQueueItemResponse,
  },
  // Flush the queue for the given multicast group.
flushQueue: {
    path: '/api.MulticastGroupService/FlushQueue',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.FlushMulticastGroupQueueRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_FlushMulticastGroupQueueRequest,
    requestDeserialize: deserialize_api_FlushMulticastGroupQueueRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // List the items in the multicast group queue.
listQueue: {
    path: '/api.MulticastGroupService/ListQueue',
    requestStream: false,
    responseStream: false,
    requestType: api_multicast_group_pb.ListMulticastGroupQueueRequest,
    responseType: api_multicast_group_pb.ListMulticastGroupQueueResponse,
    requestSerialize: serialize_api_ListMulticastGroupQueueRequest,
    requestDeserialize: deserialize_api_ListMulticastGroupQueueRequest,
    responseSerialize: serialize_api_ListMulticastGroupQueueResponse,
    responseDeserialize: deserialize_api_ListMulticastGroupQueueResponse,
  },
};

exports.MulticastGroupServiceClient = grpc.makeGenericClientConstructor(MulticastGroupServiceService);
