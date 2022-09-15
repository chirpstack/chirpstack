// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_internal_pb = require('../api/internal_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');
var common_common_pb = require('../common/common_pb.js');
var api_user_pb = require('../api/user_pb.js');

function serialize_api_CreateApiKeyRequest(arg) {
  if (!(arg instanceof api_internal_pb.CreateApiKeyRequest)) {
    throw new Error('Expected argument of type api.CreateApiKeyRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateApiKeyRequest(buffer_arg) {
  return api_internal_pb.CreateApiKeyRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateApiKeyResponse(arg) {
  if (!(arg instanceof api_internal_pb.CreateApiKeyResponse)) {
    throw new Error('Expected argument of type api.CreateApiKeyResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateApiKeyResponse(buffer_arg) {
  return api_internal_pb.CreateApiKeyResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteApiKeyRequest(arg) {
  if (!(arg instanceof api_internal_pb.DeleteApiKeyRequest)) {
    throw new Error('Expected argument of type api.DeleteApiKeyRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteApiKeyRequest(buffer_arg) {
  return api_internal_pb.DeleteApiKeyRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDevicesSummaryRequest(arg) {
  if (!(arg instanceof api_internal_pb.GetDevicesSummaryRequest)) {
    throw new Error('Expected argument of type api.GetDevicesSummaryRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDevicesSummaryRequest(buffer_arg) {
  return api_internal_pb.GetDevicesSummaryRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetDevicesSummaryResponse(arg) {
  if (!(arg instanceof api_internal_pb.GetDevicesSummaryResponse)) {
    throw new Error('Expected argument of type api.GetDevicesSummaryResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetDevicesSummaryResponse(buffer_arg) {
  return api_internal_pb.GetDevicesSummaryResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetGatewaysSummaryRequest(arg) {
  if (!(arg instanceof api_internal_pb.GetGatewaysSummaryRequest)) {
    throw new Error('Expected argument of type api.GetGatewaysSummaryRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetGatewaysSummaryRequest(buffer_arg) {
  return api_internal_pb.GetGatewaysSummaryRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetGatewaysSummaryResponse(arg) {
  if (!(arg instanceof api_internal_pb.GetGatewaysSummaryResponse)) {
    throw new Error('Expected argument of type api.GetGatewaysSummaryResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetGatewaysSummaryResponse(buffer_arg) {
  return api_internal_pb.GetGatewaysSummaryResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetRegionRequest(arg) {
  if (!(arg instanceof api_internal_pb.GetRegionRequest)) {
    throw new Error('Expected argument of type api.GetRegionRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetRegionRequest(buffer_arg) {
  return api_internal_pb.GetRegionRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetRegionResponse(arg) {
  if (!(arg instanceof api_internal_pb.GetRegionResponse)) {
    throw new Error('Expected argument of type api.GetRegionResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetRegionResponse(buffer_arg) {
  return api_internal_pb.GetRegionResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GlobalSearchRequest(arg) {
  if (!(arg instanceof api_internal_pb.GlobalSearchRequest)) {
    throw new Error('Expected argument of type api.GlobalSearchRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GlobalSearchRequest(buffer_arg) {
  return api_internal_pb.GlobalSearchRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GlobalSearchResponse(arg) {
  if (!(arg instanceof api_internal_pb.GlobalSearchResponse)) {
    throw new Error('Expected argument of type api.GlobalSearchResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GlobalSearchResponse(buffer_arg) {
  return api_internal_pb.GlobalSearchResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListApiKeysRequest(arg) {
  if (!(arg instanceof api_internal_pb.ListApiKeysRequest)) {
    throw new Error('Expected argument of type api.ListApiKeysRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListApiKeysRequest(buffer_arg) {
  return api_internal_pb.ListApiKeysRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListApiKeysResponse(arg) {
  if (!(arg instanceof api_internal_pb.ListApiKeysResponse)) {
    throw new Error('Expected argument of type api.ListApiKeysResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListApiKeysResponse(buffer_arg) {
  return api_internal_pb.ListApiKeysResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListRegionsResponse(arg) {
  if (!(arg instanceof api_internal_pb.ListRegionsResponse)) {
    throw new Error('Expected argument of type api.ListRegionsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListRegionsResponse(buffer_arg) {
  return api_internal_pb.ListRegionsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_LogItem(arg) {
  if (!(arg instanceof api_internal_pb.LogItem)) {
    throw new Error('Expected argument of type api.LogItem');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_LogItem(buffer_arg) {
  return api_internal_pb.LogItem.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_LoginRequest(arg) {
  if (!(arg instanceof api_internal_pb.LoginRequest)) {
    throw new Error('Expected argument of type api.LoginRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_LoginRequest(buffer_arg) {
  return api_internal_pb.LoginRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_LoginResponse(arg) {
  if (!(arg instanceof api_internal_pb.LoginResponse)) {
    throw new Error('Expected argument of type api.LoginResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_LoginResponse(buffer_arg) {
  return api_internal_pb.LoginResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_OpenIdConnectLoginRequest(arg) {
  if (!(arg instanceof api_internal_pb.OpenIdConnectLoginRequest)) {
    throw new Error('Expected argument of type api.OpenIdConnectLoginRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_OpenIdConnectLoginRequest(buffer_arg) {
  return api_internal_pb.OpenIdConnectLoginRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_OpenIdConnectLoginResponse(arg) {
  if (!(arg instanceof api_internal_pb.OpenIdConnectLoginResponse)) {
    throw new Error('Expected argument of type api.OpenIdConnectLoginResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_OpenIdConnectLoginResponse(buffer_arg) {
  return api_internal_pb.OpenIdConnectLoginResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ProfileResponse(arg) {
  if (!(arg instanceof api_internal_pb.ProfileResponse)) {
    throw new Error('Expected argument of type api.ProfileResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ProfileResponse(buffer_arg) {
  return api_internal_pb.ProfileResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_SettingsResponse(arg) {
  if (!(arg instanceof api_internal_pb.SettingsResponse)) {
    throw new Error('Expected argument of type api.SettingsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_SettingsResponse(buffer_arg) {
  return api_internal_pb.SettingsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_StreamDeviceEventsRequest(arg) {
  if (!(arg instanceof api_internal_pb.StreamDeviceEventsRequest)) {
    throw new Error('Expected argument of type api.StreamDeviceEventsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_StreamDeviceEventsRequest(buffer_arg) {
  return api_internal_pb.StreamDeviceEventsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_StreamDeviceFramesRequest(arg) {
  if (!(arg instanceof api_internal_pb.StreamDeviceFramesRequest)) {
    throw new Error('Expected argument of type api.StreamDeviceFramesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_StreamDeviceFramesRequest(buffer_arg) {
  return api_internal_pb.StreamDeviceFramesRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_StreamGatewayFramesRequest(arg) {
  if (!(arg instanceof api_internal_pb.StreamGatewayFramesRequest)) {
    throw new Error('Expected argument of type api.StreamGatewayFramesRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_StreamGatewayFramesRequest(buffer_arg) {
  return api_internal_pb.StreamGatewayFramesRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// InternalService is the service providing API endpoints for internal usage.
var InternalServiceService = exports.InternalServiceService = {
  // Log in a user
login: {
    path: '/api.InternalService/Login',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.LoginRequest,
    responseType: api_internal_pb.LoginResponse,
    requestSerialize: serialize_api_LoginRequest,
    requestDeserialize: deserialize_api_LoginRequest,
    responseSerialize: serialize_api_LoginResponse,
    responseDeserialize: deserialize_api_LoginResponse,
  },
  // Get the current user's profile
profile: {
    path: '/api.InternalService/Profile',
    requestStream: false,
    responseStream: false,
    requestType: google_protobuf_empty_pb.Empty,
    responseType: api_internal_pb.ProfileResponse,
    requestSerialize: serialize_google_protobuf_Empty,
    requestDeserialize: deserialize_google_protobuf_Empty,
    responseSerialize: serialize_api_ProfileResponse,
    responseDeserialize: deserialize_api_ProfileResponse,
  },
  // Perform a global search.
globalSearch: {
    path: '/api.InternalService/GlobalSearch',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.GlobalSearchRequest,
    responseType: api_internal_pb.GlobalSearchResponse,
    requestSerialize: serialize_api_GlobalSearchRequest,
    requestDeserialize: deserialize_api_GlobalSearchRequest,
    responseSerialize: serialize_api_GlobalSearchResponse,
    responseDeserialize: deserialize_api_GlobalSearchResponse,
  },
  // CreateApiKey creates the given API key.
createApiKey: {
    path: '/api.InternalService/CreateApiKey',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.CreateApiKeyRequest,
    responseType: api_internal_pb.CreateApiKeyResponse,
    requestSerialize: serialize_api_CreateApiKeyRequest,
    requestDeserialize: deserialize_api_CreateApiKeyRequest,
    responseSerialize: serialize_api_CreateApiKeyResponse,
    responseDeserialize: deserialize_api_CreateApiKeyResponse,
  },
  // DeleteApiKey deletes the API key.
deleteApiKey: {
    path: '/api.InternalService/DeleteApiKey',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.DeleteApiKeyRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteApiKeyRequest,
    requestDeserialize: deserialize_api_DeleteApiKeyRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // ListApiKeys lists the available API keys.
listApiKeys: {
    path: '/api.InternalService/ListApiKeys',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.ListApiKeysRequest,
    responseType: api_internal_pb.ListApiKeysResponse,
    requestSerialize: serialize_api_ListApiKeysRequest,
    requestDeserialize: deserialize_api_ListApiKeysRequest,
    responseSerialize: serialize_api_ListApiKeysResponse,
    responseDeserialize: deserialize_api_ListApiKeysResponse,
  },
  // Get the global settings.
settings: {
    path: '/api.InternalService/Settings',
    requestStream: false,
    responseStream: false,
    requestType: google_protobuf_empty_pb.Empty,
    responseType: api_internal_pb.SettingsResponse,
    requestSerialize: serialize_google_protobuf_Empty,
    requestDeserialize: deserialize_google_protobuf_Empty,
    responseSerialize: serialize_api_SettingsResponse,
    responseDeserialize: deserialize_api_SettingsResponse,
  },
  // OpenId Connect login.
openIdConnectLogin: {
    path: '/api.InternalService/OpenIdConnectLogin',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.OpenIdConnectLoginRequest,
    responseType: api_internal_pb.OpenIdConnectLoginResponse,
    requestSerialize: serialize_api_OpenIdConnectLoginRequest,
    requestDeserialize: deserialize_api_OpenIdConnectLoginRequest,
    responseSerialize: serialize_api_OpenIdConnectLoginResponse,
    responseDeserialize: deserialize_api_OpenIdConnectLoginResponse,
  },
  // GetDevicesSummary returns an aggregated summary of the devices.
getDevicesSummary: {
    path: '/api.InternalService/GetDevicesSummary',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.GetDevicesSummaryRequest,
    responseType: api_internal_pb.GetDevicesSummaryResponse,
    requestSerialize: serialize_api_GetDevicesSummaryRequest,
    requestDeserialize: deserialize_api_GetDevicesSummaryRequest,
    responseSerialize: serialize_api_GetDevicesSummaryResponse,
    responseDeserialize: deserialize_api_GetDevicesSummaryResponse,
  },
  // GetGatewaysSummary returns an aggregated summary of the gateways.
getGatewaysSummary: {
    path: '/api.InternalService/GetGatewaysSummary',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.GetGatewaysSummaryRequest,
    responseType: api_internal_pb.GetGatewaysSummaryResponse,
    requestSerialize: serialize_api_GetGatewaysSummaryRequest,
    requestDeserialize: deserialize_api_GetGatewaysSummaryRequest,
    responseSerialize: serialize_api_GetGatewaysSummaryResponse,
    responseDeserialize: deserialize_api_GetGatewaysSummaryResponse,
  },
  // Stream frame for the given Gateway ID.
streamGatewayFrames: {
    path: '/api.InternalService/StreamGatewayFrames',
    requestStream: false,
    responseStream: true,
    requestType: api_internal_pb.StreamGatewayFramesRequest,
    responseType: api_internal_pb.LogItem,
    requestSerialize: serialize_api_StreamGatewayFramesRequest,
    requestDeserialize: deserialize_api_StreamGatewayFramesRequest,
    responseSerialize: serialize_api_LogItem,
    responseDeserialize: deserialize_api_LogItem,
  },
  // Stream frames for the given Device EUI.
streamDeviceFrames: {
    path: '/api.InternalService/StreamDeviceFrames',
    requestStream: false,
    responseStream: true,
    requestType: api_internal_pb.StreamDeviceFramesRequest,
    responseType: api_internal_pb.LogItem,
    requestSerialize: serialize_api_StreamDeviceFramesRequest,
    requestDeserialize: deserialize_api_StreamDeviceFramesRequest,
    responseSerialize: serialize_api_LogItem,
    responseDeserialize: deserialize_api_LogItem,
  },
  // Stream events for the given Device EUI.
streamDeviceEvents: {
    path: '/api.InternalService/StreamDeviceEvents',
    requestStream: false,
    responseStream: true,
    requestType: api_internal_pb.StreamDeviceEventsRequest,
    responseType: api_internal_pb.LogItem,
    requestSerialize: serialize_api_StreamDeviceEventsRequest,
    requestDeserialize: deserialize_api_StreamDeviceEventsRequest,
    responseSerialize: serialize_api_LogItem,
    responseDeserialize: deserialize_api_LogItem,
  },
  // ListRegions lists the available (configured) regions.
listRegions: {
    path: '/api.InternalService/ListRegions',
    requestStream: false,
    responseStream: false,
    requestType: google_protobuf_empty_pb.Empty,
    responseType: api_internal_pb.ListRegionsResponse,
    requestSerialize: serialize_google_protobuf_Empty,
    requestDeserialize: deserialize_google_protobuf_Empty,
    responseSerialize: serialize_api_ListRegionsResponse,
    responseDeserialize: deserialize_api_ListRegionsResponse,
  },
  // GetRegion returns the region details for the given region.
getRegion: {
    path: '/api.InternalService/GetRegion',
    requestStream: false,
    responseStream: false,
    requestType: api_internal_pb.GetRegionRequest,
    responseType: api_internal_pb.GetRegionResponse,
    requestSerialize: serialize_api_GetRegionRequest,
    requestDeserialize: deserialize_api_GetRegionRequest,
    responseSerialize: serialize_api_GetRegionResponse,
    responseDeserialize: deserialize_api_GetRegionResponse,
  },
};

exports.InternalServiceClient = grpc.makeGenericClientConstructor(InternalServiceService);
