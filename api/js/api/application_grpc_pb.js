// GENERATED CODE -- DO NOT EDIT!

'use strict';
var grpc = require('@grpc/grpc-js');
var api_application_pb = require('../api/application_pb.js');
var google_api_annotations_pb = require('../google/api/annotations_pb.js');
var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
var google_protobuf_empty_pb = require('google-protobuf/google/protobuf/empty_pb.js');

function serialize_api_CreateApplicationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateApplicationRequest)) {
    throw new Error('Expected argument of type api.CreateApplicationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateApplicationRequest(buffer_arg) {
  return api_application_pb.CreateApplicationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateApplicationResponse(arg) {
  if (!(arg instanceof api_application_pb.CreateApplicationResponse)) {
    throw new Error('Expected argument of type api.CreateApplicationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateApplicationResponse(buffer_arg) {
  return api_application_pb.CreateApplicationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateAwsSnsIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateAwsSnsIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateAwsSnsIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateAwsSnsIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateAwsSnsIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateAzureServiceBusIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateAzureServiceBusIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateAzureServiceBusIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateAzureServiceBusIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateAzureServiceBusIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateGcpPubSubIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateGcpPubSubIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateGcpPubSubIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateGcpPubSubIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateGcpPubSubIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateHttpIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateHttpIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateHttpIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateHttpIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateHttpIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateIftttIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateIftttIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateIftttIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateIftttIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateIftttIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateInfluxDbIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateInfluxDbIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateInfluxDbIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateInfluxDbIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateInfluxDbIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateLoraCloudIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateLoraCloudIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateLoraCloudIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateLoraCloudIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateLoraCloudIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateMyDevicesIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateMyDevicesIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateMyDevicesIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateMyDevicesIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateMyDevicesIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreatePilotThingsIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreatePilotThingsIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreatePilotThingsIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreatePilotThingsIntegrationRequest(buffer_arg) {
  return api_application_pb.CreatePilotThingsIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_CreateThingsBoardIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.CreateThingsBoardIntegrationRequest)) {
    throw new Error('Expected argument of type api.CreateThingsBoardIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_CreateThingsBoardIntegrationRequest(buffer_arg) {
  return api_application_pb.CreateThingsBoardIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteApplicationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteApplicationRequest)) {
    throw new Error('Expected argument of type api.DeleteApplicationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteApplicationRequest(buffer_arg) {
  return api_application_pb.DeleteApplicationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteAwsSnsIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteAwsSnsIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteAwsSnsIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteAwsSnsIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteAwsSnsIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteAzureServiceBusIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteAzureServiceBusIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteAzureServiceBusIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteAzureServiceBusIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteAzureServiceBusIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteGcpPubSubIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteGcpPubSubIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteGcpPubSubIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteGcpPubSubIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteGcpPubSubIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteHttpIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteHttpIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteHttpIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteHttpIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteHttpIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteIftttIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteIftttIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteIftttIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteIftttIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteIftttIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteInfluxDbIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteInfluxDbIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteInfluxDbIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteInfluxDbIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteInfluxDbIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteLoraCloudIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteLoraCloudIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteLoraCloudIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteLoraCloudIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteLoraCloudIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteMyDevicesIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteMyDevicesIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteMyDevicesIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteMyDevicesIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteMyDevicesIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeletePilotThingsIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeletePilotThingsIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeletePilotThingsIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeletePilotThingsIntegrationRequest(buffer_arg) {
  return api_application_pb.DeletePilotThingsIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_DeleteThingsBoardIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.DeleteThingsBoardIntegrationRequest)) {
    throw new Error('Expected argument of type api.DeleteThingsBoardIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_DeleteThingsBoardIntegrationRequest(buffer_arg) {
  return api_application_pb.DeleteThingsBoardIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GenerateMqttIntegrationClientCertificateRequest(arg) {
  if (!(arg instanceof api_application_pb.GenerateMqttIntegrationClientCertificateRequest)) {
    throw new Error('Expected argument of type api.GenerateMqttIntegrationClientCertificateRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GenerateMqttIntegrationClientCertificateRequest(buffer_arg) {
  return api_application_pb.GenerateMqttIntegrationClientCertificateRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GenerateMqttIntegrationClientCertificateResponse(arg) {
  if (!(arg instanceof api_application_pb.GenerateMqttIntegrationClientCertificateResponse)) {
    throw new Error('Expected argument of type api.GenerateMqttIntegrationClientCertificateResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GenerateMqttIntegrationClientCertificateResponse(buffer_arg) {
  return api_application_pb.GenerateMqttIntegrationClientCertificateResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetApplicationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetApplicationRequest)) {
    throw new Error('Expected argument of type api.GetApplicationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetApplicationRequest(buffer_arg) {
  return api_application_pb.GetApplicationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetApplicationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetApplicationResponse)) {
    throw new Error('Expected argument of type api.GetApplicationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetApplicationResponse(buffer_arg) {
  return api_application_pb.GetApplicationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetAwsSnsIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetAwsSnsIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetAwsSnsIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetAwsSnsIntegrationRequest(buffer_arg) {
  return api_application_pb.GetAwsSnsIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetAwsSnsIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetAwsSnsIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetAwsSnsIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetAwsSnsIntegrationResponse(buffer_arg) {
  return api_application_pb.GetAwsSnsIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetAzureServiceBusIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetAzureServiceBusIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetAzureServiceBusIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetAzureServiceBusIntegrationRequest(buffer_arg) {
  return api_application_pb.GetAzureServiceBusIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetAzureServiceBusIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetAzureServiceBusIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetAzureServiceBusIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetAzureServiceBusIntegrationResponse(buffer_arg) {
  return api_application_pb.GetAzureServiceBusIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetGcpPubSubIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetGcpPubSubIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetGcpPubSubIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetGcpPubSubIntegrationRequest(buffer_arg) {
  return api_application_pb.GetGcpPubSubIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetGcpPubSubIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetGcpPubSubIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetGcpPubSubIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetGcpPubSubIntegrationResponse(buffer_arg) {
  return api_application_pb.GetGcpPubSubIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetHttpIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetHttpIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetHttpIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetHttpIntegrationRequest(buffer_arg) {
  return api_application_pb.GetHttpIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetHttpIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetHttpIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetHttpIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetHttpIntegrationResponse(buffer_arg) {
  return api_application_pb.GetHttpIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetIftttIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetIftttIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetIftttIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetIftttIntegrationRequest(buffer_arg) {
  return api_application_pb.GetIftttIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetIftttIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetIftttIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetIftttIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetIftttIntegrationResponse(buffer_arg) {
  return api_application_pb.GetIftttIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetInfluxDbIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetInfluxDbIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetInfluxDbIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetInfluxDbIntegrationRequest(buffer_arg) {
  return api_application_pb.GetInfluxDbIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetInfluxDbIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetInfluxDbIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetInfluxDbIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetInfluxDbIntegrationResponse(buffer_arg) {
  return api_application_pb.GetInfluxDbIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetLoraCloudIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetLoraCloudIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetLoraCloudIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetLoraCloudIntegrationRequest(buffer_arg) {
  return api_application_pb.GetLoraCloudIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetLoraCloudIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetLoraCloudIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetLoraCloudIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetLoraCloudIntegrationResponse(buffer_arg) {
  return api_application_pb.GetLoraCloudIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetMyDevicesIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetMyDevicesIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetMyDevicesIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetMyDevicesIntegrationRequest(buffer_arg) {
  return api_application_pb.GetMyDevicesIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetMyDevicesIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetMyDevicesIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetMyDevicesIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetMyDevicesIntegrationResponse(buffer_arg) {
  return api_application_pb.GetMyDevicesIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetPilotThingsIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetPilotThingsIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetPilotThingsIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetPilotThingsIntegrationRequest(buffer_arg) {
  return api_application_pb.GetPilotThingsIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetPilotThingsIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetPilotThingsIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetPilotThingsIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetPilotThingsIntegrationResponse(buffer_arg) {
  return api_application_pb.GetPilotThingsIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetThingsBoardIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.GetThingsBoardIntegrationRequest)) {
    throw new Error('Expected argument of type api.GetThingsBoardIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetThingsBoardIntegrationRequest(buffer_arg) {
  return api_application_pb.GetThingsBoardIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_GetThingsBoardIntegrationResponse(arg) {
  if (!(arg instanceof api_application_pb.GetThingsBoardIntegrationResponse)) {
    throw new Error('Expected argument of type api.GetThingsBoardIntegrationResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_GetThingsBoardIntegrationResponse(buffer_arg) {
  return api_application_pb.GetThingsBoardIntegrationResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListApplicationsRequest(arg) {
  if (!(arg instanceof api_application_pb.ListApplicationsRequest)) {
    throw new Error('Expected argument of type api.ListApplicationsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListApplicationsRequest(buffer_arg) {
  return api_application_pb.ListApplicationsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListApplicationsResponse(arg) {
  if (!(arg instanceof api_application_pb.ListApplicationsResponse)) {
    throw new Error('Expected argument of type api.ListApplicationsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListApplicationsResponse(buffer_arg) {
  return api_application_pb.ListApplicationsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListIntegrationsRequest(arg) {
  if (!(arg instanceof api_application_pb.ListIntegrationsRequest)) {
    throw new Error('Expected argument of type api.ListIntegrationsRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListIntegrationsRequest(buffer_arg) {
  return api_application_pb.ListIntegrationsRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_ListIntegrationsResponse(arg) {
  if (!(arg instanceof api_application_pb.ListIntegrationsResponse)) {
    throw new Error('Expected argument of type api.ListIntegrationsResponse');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_ListIntegrationsResponse(buffer_arg) {
  return api_application_pb.ListIntegrationsResponse.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateApplicationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateApplicationRequest)) {
    throw new Error('Expected argument of type api.UpdateApplicationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateApplicationRequest(buffer_arg) {
  return api_application_pb.UpdateApplicationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateAwsSnsIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateAwsSnsIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateAwsSnsIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateAwsSnsIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateAwsSnsIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateAzureServiceBusIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateAzureServiceBusIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateAzureServiceBusIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateAzureServiceBusIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateAzureServiceBusIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateGcpPubSubIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateGcpPubSubIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateGcpPubSubIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateGcpPubSubIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateGcpPubSubIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateHttpIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateHttpIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateHttpIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateHttpIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateHttpIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateIftttIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateIftttIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateIftttIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateIftttIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateIftttIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateInfluxDbIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateInfluxDbIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateInfluxDbIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateInfluxDbIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateInfluxDbIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateLoraCloudIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateLoraCloudIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateLoraCloudIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateLoraCloudIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateLoraCloudIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateMyDevicesIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateMyDevicesIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateMyDevicesIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateMyDevicesIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateMyDevicesIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdatePilotThingsIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdatePilotThingsIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdatePilotThingsIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdatePilotThingsIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdatePilotThingsIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
}

function serialize_api_UpdateThingsBoardIntegrationRequest(arg) {
  if (!(arg instanceof api_application_pb.UpdateThingsBoardIntegrationRequest)) {
    throw new Error('Expected argument of type api.UpdateThingsBoardIntegrationRequest');
  }
  return Buffer.from(arg.serializeBinary());
}

function deserialize_api_UpdateThingsBoardIntegrationRequest(buffer_arg) {
  return api_application_pb.UpdateThingsBoardIntegrationRequest.deserializeBinary(new Uint8Array(buffer_arg));
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


// ApplicationService is the service providing API methods for managing
// applications.
var ApplicationServiceService = exports.ApplicationServiceService = {
  // Create creates the given application.
create: {
    path: '/api.ApplicationService/Create',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateApplicationRequest,
    responseType: api_application_pb.CreateApplicationResponse,
    requestSerialize: serialize_api_CreateApplicationRequest,
    requestDeserialize: deserialize_api_CreateApplicationRequest,
    responseSerialize: serialize_api_CreateApplicationResponse,
    responseDeserialize: deserialize_api_CreateApplicationResponse,
  },
  // Get the application for the given ID.
get: {
    path: '/api.ApplicationService/Get',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetApplicationRequest,
    responseType: api_application_pb.GetApplicationResponse,
    requestSerialize: serialize_api_GetApplicationRequest,
    requestDeserialize: deserialize_api_GetApplicationRequest,
    responseSerialize: serialize_api_GetApplicationResponse,
    responseDeserialize: deserialize_api_GetApplicationResponse,
  },
  // Update updates the given application.
update: {
    path: '/api.ApplicationService/Update',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateApplicationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateApplicationRequest,
    requestDeserialize: deserialize_api_UpdateApplicationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the application for the given ID.
delete: {
    path: '/api.ApplicationService/Delete',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteApplicationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteApplicationRequest,
    requestDeserialize: deserialize_api_DeleteApplicationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the list of applications.
list: {
    path: '/api.ApplicationService/List',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.ListApplicationsRequest,
    responseType: api_application_pb.ListApplicationsResponse,
    requestSerialize: serialize_api_ListApplicationsRequest,
    requestDeserialize: deserialize_api_ListApplicationsRequest,
    responseSerialize: serialize_api_ListApplicationsResponse,
    responseDeserialize: deserialize_api_ListApplicationsResponse,
  },
  // List all configured integrations.
listIntegrations: {
    path: '/api.ApplicationService/ListIntegrations',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.ListIntegrationsRequest,
    responseType: api_application_pb.ListIntegrationsResponse,
    requestSerialize: serialize_api_ListIntegrationsRequest,
    requestDeserialize: deserialize_api_ListIntegrationsRequest,
    responseSerialize: serialize_api_ListIntegrationsResponse,
    responseDeserialize: deserialize_api_ListIntegrationsResponse,
  },
  // Create HTTP integration.
createHttpIntegration: {
    path: '/api.ApplicationService/CreateHttpIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateHttpIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateHttpIntegrationRequest,
    requestDeserialize: deserialize_api_CreateHttpIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get the configured HTTP integration.
getHttpIntegration: {
    path: '/api.ApplicationService/GetHttpIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetHttpIntegrationRequest,
    responseType: api_application_pb.GetHttpIntegrationResponse,
    requestSerialize: serialize_api_GetHttpIntegrationRequest,
    requestDeserialize: deserialize_api_GetHttpIntegrationRequest,
    responseSerialize: serialize_api_GetHttpIntegrationResponse,
    responseDeserialize: deserialize_api_GetHttpIntegrationResponse,
  },
  // Update the HTTP integration.
updateHttpIntegration: {
    path: '/api.ApplicationService/UpdateHttpIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateHttpIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateHttpIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateHttpIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete the HTTP integration.
deleteHttpIntegration: {
    path: '/api.ApplicationService/DeleteHttpIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteHttpIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteHttpIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteHttpIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create InfluxDb integration.
createInfluxDbIntegration: {
    path: '/api.ApplicationService/CreateInfluxDbIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateInfluxDbIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateInfluxDbIntegrationRequest,
    requestDeserialize: deserialize_api_CreateInfluxDbIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get InfluxDb integration.
getInfluxDbIntegration: {
    path: '/api.ApplicationService/GetInfluxDbIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetInfluxDbIntegrationRequest,
    responseType: api_application_pb.GetInfluxDbIntegrationResponse,
    requestSerialize: serialize_api_GetInfluxDbIntegrationRequest,
    requestDeserialize: deserialize_api_GetInfluxDbIntegrationRequest,
    responseSerialize: serialize_api_GetInfluxDbIntegrationResponse,
    responseDeserialize: deserialize_api_GetInfluxDbIntegrationResponse,
  },
  // Update InfluxDb integration.
updateInfluxDbIntegration: {
    path: '/api.ApplicationService/UpdateInfluxDbIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateInfluxDbIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateInfluxDbIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateInfluxDbIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete InfluxDb integration.
deleteInfluxDbIntegration: {
    path: '/api.ApplicationService/DeleteInfluxDbIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteInfluxDbIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteInfluxDbIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteInfluxDbIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create ThingsBoard integration.
createThingsBoardIntegration: {
    path: '/api.ApplicationService/CreateThingsBoardIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateThingsBoardIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateThingsBoardIntegrationRequest,
    requestDeserialize: deserialize_api_CreateThingsBoardIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get ThingsBoard integration.
getThingsBoardIntegration: {
    path: '/api.ApplicationService/GetThingsBoardIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetThingsBoardIntegrationRequest,
    responseType: api_application_pb.GetThingsBoardIntegrationResponse,
    requestSerialize: serialize_api_GetThingsBoardIntegrationRequest,
    requestDeserialize: deserialize_api_GetThingsBoardIntegrationRequest,
    responseSerialize: serialize_api_GetThingsBoardIntegrationResponse,
    responseDeserialize: deserialize_api_GetThingsBoardIntegrationResponse,
  },
  // Update ThingsBoard integration.
updateThingsBoardIntegration: {
    path: '/api.ApplicationService/UpdateThingsBoardIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateThingsBoardIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateThingsBoardIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateThingsBoardIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete ThingsBoard integration.
deleteThingsBoardIntegration: {
    path: '/api.ApplicationService/DeleteThingsBoardIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteThingsBoardIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteThingsBoardIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteThingsBoardIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create myDevices integration.
createMyDevicesIntegration: {
    path: '/api.ApplicationService/CreateMyDevicesIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateMyDevicesIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateMyDevicesIntegrationRequest,
    requestDeserialize: deserialize_api_CreateMyDevicesIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get myDevices integration.
getMyDevicesIntegration: {
    path: '/api.ApplicationService/GetMyDevicesIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetMyDevicesIntegrationRequest,
    responseType: api_application_pb.GetMyDevicesIntegrationResponse,
    requestSerialize: serialize_api_GetMyDevicesIntegrationRequest,
    requestDeserialize: deserialize_api_GetMyDevicesIntegrationRequest,
    responseSerialize: serialize_api_GetMyDevicesIntegrationResponse,
    responseDeserialize: deserialize_api_GetMyDevicesIntegrationResponse,
  },
  // Update myDevices integration.
updateMyDevicesIntegration: {
    path: '/api.ApplicationService/UpdateMyDevicesIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateMyDevicesIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateMyDevicesIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateMyDevicesIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete myDevices integration.
deleteMyDevicesIntegration: {
    path: '/api.ApplicationService/DeleteMyDevicesIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteMyDevicesIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteMyDevicesIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteMyDevicesIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create LoRaCloud integration.
createLoraCloudIntegration: {
    path: '/api.ApplicationService/CreateLoraCloudIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateLoraCloudIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateLoraCloudIntegrationRequest,
    requestDeserialize: deserialize_api_CreateLoraCloudIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get LoRaCloud integration.
getLoraCloudIntegration: {
    path: '/api.ApplicationService/GetLoraCloudIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetLoraCloudIntegrationRequest,
    responseType: api_application_pb.GetLoraCloudIntegrationResponse,
    requestSerialize: serialize_api_GetLoraCloudIntegrationRequest,
    requestDeserialize: deserialize_api_GetLoraCloudIntegrationRequest,
    responseSerialize: serialize_api_GetLoraCloudIntegrationResponse,
    responseDeserialize: deserialize_api_GetLoraCloudIntegrationResponse,
  },
  // Update LoRaCloud integration.
updateLoraCloudIntegration: {
    path: '/api.ApplicationService/UpdateLoraCloudIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateLoraCloudIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateLoraCloudIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateLoraCloudIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete LoRaCloud integration.
deleteLoraCloudIntegration: {
    path: '/api.ApplicationService/DeleteLoraCloudIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteLoraCloudIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteLoraCloudIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteLoraCloudIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create GCP Pub/Sub integration.
createGcpPubSubIntegration: {
    path: '/api.ApplicationService/CreateGcpPubSubIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateGcpPubSubIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateGcpPubSubIntegrationRequest,
    requestDeserialize: deserialize_api_CreateGcpPubSubIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get GCP Pub/Sub integration.
getGcpPubSubIntegration: {
    path: '/api.ApplicationService/GetGcpPubSubIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetGcpPubSubIntegrationRequest,
    responseType: api_application_pb.GetGcpPubSubIntegrationResponse,
    requestSerialize: serialize_api_GetGcpPubSubIntegrationRequest,
    requestDeserialize: deserialize_api_GetGcpPubSubIntegrationRequest,
    responseSerialize: serialize_api_GetGcpPubSubIntegrationResponse,
    responseDeserialize: deserialize_api_GetGcpPubSubIntegrationResponse,
  },
  // Update GCP Pub/Sub integration.
updateGcpPubSubIntegration: {
    path: '/api.ApplicationService/UpdateGcpPubSubIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateGcpPubSubIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateGcpPubSubIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateGcpPubSubIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete GCP Pub/Sub integration.
deleteGcpPubSubIntegration: {
    path: '/api.ApplicationService/DeleteGcpPubSubIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteGcpPubSubIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteGcpPubSubIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteGcpPubSubIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create AWS SNS integration.
createAwsSnsIntegration: {
    path: '/api.ApplicationService/CreateAwsSnsIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateAwsSnsIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateAwsSnsIntegrationRequest,
    requestDeserialize: deserialize_api_CreateAwsSnsIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get AWS SNS integration.
getAwsSnsIntegration: {
    path: '/api.ApplicationService/GetAwsSnsIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetAwsSnsIntegrationRequest,
    responseType: api_application_pb.GetAwsSnsIntegrationResponse,
    requestSerialize: serialize_api_GetAwsSnsIntegrationRequest,
    requestDeserialize: deserialize_api_GetAwsSnsIntegrationRequest,
    responseSerialize: serialize_api_GetAwsSnsIntegrationResponse,
    responseDeserialize: deserialize_api_GetAwsSnsIntegrationResponse,
  },
  // Update AWS SNS integration.
updateAwsSnsIntegration: {
    path: '/api.ApplicationService/UpdateAwsSnsIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateAwsSnsIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateAwsSnsIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateAwsSnsIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete AWS SNS integration.
deleteAwsSnsIntegration: {
    path: '/api.ApplicationService/DeleteAwsSnsIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteAwsSnsIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteAwsSnsIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteAwsSnsIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create Azure Service-Bus integration.
createAzureServiceBusIntegration: {
    path: '/api.ApplicationService/CreateAzureServiceBusIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateAzureServiceBusIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateAzureServiceBusIntegrationRequest,
    requestDeserialize: deserialize_api_CreateAzureServiceBusIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get Azure Service-Bus integration.
getAzureServiceBusIntegration: {
    path: '/api.ApplicationService/GetAzureServiceBusIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetAzureServiceBusIntegrationRequest,
    responseType: api_application_pb.GetAzureServiceBusIntegrationResponse,
    requestSerialize: serialize_api_GetAzureServiceBusIntegrationRequest,
    requestDeserialize: deserialize_api_GetAzureServiceBusIntegrationRequest,
    responseSerialize: serialize_api_GetAzureServiceBusIntegrationResponse,
    responseDeserialize: deserialize_api_GetAzureServiceBusIntegrationResponse,
  },
  // Update Azure Service-Bus integration.
updateAzureServiceBusIntegration: {
    path: '/api.ApplicationService/UpdateAzureServiceBusIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateAzureServiceBusIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateAzureServiceBusIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateAzureServiceBusIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete Azure Service-Bus integration.
deleteAzureServiceBusIntegration: {
    path: '/api.ApplicationService/DeleteAzureServiceBusIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteAzureServiceBusIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteAzureServiceBusIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteAzureServiceBusIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create Pilot Things integration.
createPilotThingsIntegration: {
    path: '/api.ApplicationService/CreatePilotThingsIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreatePilotThingsIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreatePilotThingsIntegrationRequest,
    requestDeserialize: deserialize_api_CreatePilotThingsIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get Pilot Things integration.
getPilotThingsIntegration: {
    path: '/api.ApplicationService/GetPilotThingsIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetPilotThingsIntegrationRequest,
    responseType: api_application_pb.GetPilotThingsIntegrationResponse,
    requestSerialize: serialize_api_GetPilotThingsIntegrationRequest,
    requestDeserialize: deserialize_api_GetPilotThingsIntegrationRequest,
    responseSerialize: serialize_api_GetPilotThingsIntegrationResponse,
    responseDeserialize: deserialize_api_GetPilotThingsIntegrationResponse,
  },
  // Update Pilot Things integration.
updatePilotThingsIntegration: {
    path: '/api.ApplicationService/UpdatePilotThingsIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdatePilotThingsIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdatePilotThingsIntegrationRequest,
    requestDeserialize: deserialize_api_UpdatePilotThingsIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete Pilot Things integration.
deletePilotThingsIntegration: {
    path: '/api.ApplicationService/DeletePilotThingsIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeletePilotThingsIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeletePilotThingsIntegrationRequest,
    requestDeserialize: deserialize_api_DeletePilotThingsIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Create IFTTT integration.
createIftttIntegration: {
    path: '/api.ApplicationService/CreateIftttIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.CreateIftttIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_CreateIftttIntegrationRequest,
    requestDeserialize: deserialize_api_CreateIftttIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Get IFTTT integration.
getIftttIntegration: {
    path: '/api.ApplicationService/GetIftttIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GetIftttIntegrationRequest,
    responseType: api_application_pb.GetIftttIntegrationResponse,
    requestSerialize: serialize_api_GetIftttIntegrationRequest,
    requestDeserialize: deserialize_api_GetIftttIntegrationRequest,
    responseSerialize: serialize_api_GetIftttIntegrationResponse,
    responseDeserialize: deserialize_api_GetIftttIntegrationResponse,
  },
  // Update IFTTT integration.
updateIftttIntegration: {
    path: '/api.ApplicationService/UpdateIftttIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.UpdateIftttIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_UpdateIftttIntegrationRequest,
    requestDeserialize: deserialize_api_UpdateIftttIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Delete IFTTT integration.
deleteIftttIntegration: {
    path: '/api.ApplicationService/DeleteIftttIntegration',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.DeleteIftttIntegrationRequest,
    responseType: google_protobuf_empty_pb.Empty,
    requestSerialize: serialize_api_DeleteIftttIntegrationRequest,
    requestDeserialize: deserialize_api_DeleteIftttIntegrationRequest,
    responseSerialize: serialize_google_protobuf_Empty,
    responseDeserialize: deserialize_google_protobuf_Empty,
  },
  // Generates application ID specific client-certificate.
generateMqttIntegrationClientCertificate: {
    path: '/api.ApplicationService/GenerateMqttIntegrationClientCertificate',
    requestStream: false,
    responseStream: false,
    requestType: api_application_pb.GenerateMqttIntegrationClientCertificateRequest,
    responseType: api_application_pb.GenerateMqttIntegrationClientCertificateResponse,
    requestSerialize: serialize_api_GenerateMqttIntegrationClientCertificateRequest,
    requestDeserialize: deserialize_api_GenerateMqttIntegrationClientCertificateRequest,
    responseSerialize: serialize_api_GenerateMqttIntegrationClientCertificateResponse,
    responseDeserialize: deserialize_api_GenerateMqttIntegrationClientCertificateResponse,
  },
};

exports.ApplicationServiceClient = grpc.makeGenericClientConstructor(ApplicationServiceService);
