import * as grpcWeb from 'grpc-web';

import * as api_application_pb from '../api/application_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class ApplicationServiceClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_application_pb.CreateApplicationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.CreateApplicationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.CreateApplicationResponse>;

  get(
    request: api_application_pb.GetApplicationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetApplicationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetApplicationResponse>;

  update(
    request: api_application_pb.UpdateApplicationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_application_pb.DeleteApplicationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  list(
    request: api_application_pb.ListApplicationsRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.ListApplicationsResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.ListApplicationsResponse>;

  listIntegrations(
    request: api_application_pb.ListIntegrationsRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.ListIntegrationsResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.ListIntegrationsResponse>;

  createHttpIntegration(
    request: api_application_pb.CreateHttpIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getHttpIntegration(
    request: api_application_pb.GetHttpIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetHttpIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetHttpIntegrationResponse>;

  updateHttpIntegration(
    request: api_application_pb.UpdateHttpIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteHttpIntegration(
    request: api_application_pb.DeleteHttpIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createInfluxDbIntegration(
    request: api_application_pb.CreateInfluxDbIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getInfluxDbIntegration(
    request: api_application_pb.GetInfluxDbIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetInfluxDbIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetInfluxDbIntegrationResponse>;

  updateInfluxDbIntegration(
    request: api_application_pb.UpdateInfluxDbIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteInfluxDbIntegration(
    request: api_application_pb.DeleteInfluxDbIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createThingsBoardIntegration(
    request: api_application_pb.CreateThingsBoardIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getThingsBoardIntegration(
    request: api_application_pb.GetThingsBoardIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetThingsBoardIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetThingsBoardIntegrationResponse>;

  updateThingsBoardIntegration(
    request: api_application_pb.UpdateThingsBoardIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteThingsBoardIntegration(
    request: api_application_pb.DeleteThingsBoardIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createMyDevicesIntegration(
    request: api_application_pb.CreateMyDevicesIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getMyDevicesIntegration(
    request: api_application_pb.GetMyDevicesIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetMyDevicesIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetMyDevicesIntegrationResponse>;

  updateMyDevicesIntegration(
    request: api_application_pb.UpdateMyDevicesIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteMyDevicesIntegration(
    request: api_application_pb.DeleteMyDevicesIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createLoraCloudIntegration(
    request: api_application_pb.CreateLoraCloudIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getLoraCloudIntegration(
    request: api_application_pb.GetLoraCloudIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetLoraCloudIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetLoraCloudIntegrationResponse>;

  updateLoraCloudIntegration(
    request: api_application_pb.UpdateLoraCloudIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteLoraCloudIntegration(
    request: api_application_pb.DeleteLoraCloudIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createGcpPubSubIntegration(
    request: api_application_pb.CreateGcpPubSubIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getGcpPubSubIntegration(
    request: api_application_pb.GetGcpPubSubIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetGcpPubSubIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetGcpPubSubIntegrationResponse>;

  updateGcpPubSubIntegration(
    request: api_application_pb.UpdateGcpPubSubIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteGcpPubSubIntegration(
    request: api_application_pb.DeleteGcpPubSubIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createAwsSnsIntegration(
    request: api_application_pb.CreateAwsSnsIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getAwsSnsIntegration(
    request: api_application_pb.GetAwsSnsIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetAwsSnsIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetAwsSnsIntegrationResponse>;

  updateAwsSnsIntegration(
    request: api_application_pb.UpdateAwsSnsIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteAwsSnsIntegration(
    request: api_application_pb.DeleteAwsSnsIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createAzureServiceBusIntegration(
    request: api_application_pb.CreateAzureServiceBusIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getAzureServiceBusIntegration(
    request: api_application_pb.GetAzureServiceBusIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetAzureServiceBusIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetAzureServiceBusIntegrationResponse>;

  updateAzureServiceBusIntegration(
    request: api_application_pb.UpdateAzureServiceBusIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteAzureServiceBusIntegration(
    request: api_application_pb.DeleteAzureServiceBusIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createPilotThingsIntegration(
    request: api_application_pb.CreatePilotThingsIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getPilotThingsIntegration(
    request: api_application_pb.GetPilotThingsIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetPilotThingsIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetPilotThingsIntegrationResponse>;

  updatePilotThingsIntegration(
    request: api_application_pb.UpdatePilotThingsIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deletePilotThingsIntegration(
    request: api_application_pb.DeletePilotThingsIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  createIftttIntegration(
    request: api_application_pb.CreateIftttIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  getIftttIntegration(
    request: api_application_pb.GetIftttIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GetIftttIntegrationResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GetIftttIntegrationResponse>;

  updateIftttIntegration(
    request: api_application_pb.UpdateIftttIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  deleteIftttIntegration(
    request: api_application_pb.DeleteIftttIntegrationRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: google_protobuf_empty_pb.Empty) => void
  ): grpcWeb.ClientReadableStream<google_protobuf_empty_pb.Empty>;

  generateMqttIntegrationClientCertificate(
    request: api_application_pb.GenerateMqttIntegrationClientCertificateRequest,
    metadata: grpcWeb.Metadata | undefined,
    callback: (err: grpcWeb.RpcError,
               response: api_application_pb.GenerateMqttIntegrationClientCertificateResponse) => void
  ): grpcWeb.ClientReadableStream<api_application_pb.GenerateMqttIntegrationClientCertificateResponse>;

}

export class ApplicationServicePromiseClient {
  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; });

  create(
    request: api_application_pb.CreateApplicationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.CreateApplicationResponse>;

  get(
    request: api_application_pb.GetApplicationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetApplicationResponse>;

  update(
    request: api_application_pb.UpdateApplicationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  delete(
    request: api_application_pb.DeleteApplicationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  list(
    request: api_application_pb.ListApplicationsRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.ListApplicationsResponse>;

  listIntegrations(
    request: api_application_pb.ListIntegrationsRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.ListIntegrationsResponse>;

  createHttpIntegration(
    request: api_application_pb.CreateHttpIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getHttpIntegration(
    request: api_application_pb.GetHttpIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetHttpIntegrationResponse>;

  updateHttpIntegration(
    request: api_application_pb.UpdateHttpIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteHttpIntegration(
    request: api_application_pb.DeleteHttpIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createInfluxDbIntegration(
    request: api_application_pb.CreateInfluxDbIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getInfluxDbIntegration(
    request: api_application_pb.GetInfluxDbIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetInfluxDbIntegrationResponse>;

  updateInfluxDbIntegration(
    request: api_application_pb.UpdateInfluxDbIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteInfluxDbIntegration(
    request: api_application_pb.DeleteInfluxDbIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createThingsBoardIntegration(
    request: api_application_pb.CreateThingsBoardIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getThingsBoardIntegration(
    request: api_application_pb.GetThingsBoardIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetThingsBoardIntegrationResponse>;

  updateThingsBoardIntegration(
    request: api_application_pb.UpdateThingsBoardIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteThingsBoardIntegration(
    request: api_application_pb.DeleteThingsBoardIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createMyDevicesIntegration(
    request: api_application_pb.CreateMyDevicesIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getMyDevicesIntegration(
    request: api_application_pb.GetMyDevicesIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetMyDevicesIntegrationResponse>;

  updateMyDevicesIntegration(
    request: api_application_pb.UpdateMyDevicesIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteMyDevicesIntegration(
    request: api_application_pb.DeleteMyDevicesIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createLoraCloudIntegration(
    request: api_application_pb.CreateLoraCloudIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getLoraCloudIntegration(
    request: api_application_pb.GetLoraCloudIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetLoraCloudIntegrationResponse>;

  updateLoraCloudIntegration(
    request: api_application_pb.UpdateLoraCloudIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteLoraCloudIntegration(
    request: api_application_pb.DeleteLoraCloudIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createGcpPubSubIntegration(
    request: api_application_pb.CreateGcpPubSubIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getGcpPubSubIntegration(
    request: api_application_pb.GetGcpPubSubIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetGcpPubSubIntegrationResponse>;

  updateGcpPubSubIntegration(
    request: api_application_pb.UpdateGcpPubSubIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteGcpPubSubIntegration(
    request: api_application_pb.DeleteGcpPubSubIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createAwsSnsIntegration(
    request: api_application_pb.CreateAwsSnsIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getAwsSnsIntegration(
    request: api_application_pb.GetAwsSnsIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetAwsSnsIntegrationResponse>;

  updateAwsSnsIntegration(
    request: api_application_pb.UpdateAwsSnsIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteAwsSnsIntegration(
    request: api_application_pb.DeleteAwsSnsIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createAzureServiceBusIntegration(
    request: api_application_pb.CreateAzureServiceBusIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getAzureServiceBusIntegration(
    request: api_application_pb.GetAzureServiceBusIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetAzureServiceBusIntegrationResponse>;

  updateAzureServiceBusIntegration(
    request: api_application_pb.UpdateAzureServiceBusIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteAzureServiceBusIntegration(
    request: api_application_pb.DeleteAzureServiceBusIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createPilotThingsIntegration(
    request: api_application_pb.CreatePilotThingsIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getPilotThingsIntegration(
    request: api_application_pb.GetPilotThingsIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetPilotThingsIntegrationResponse>;

  updatePilotThingsIntegration(
    request: api_application_pb.UpdatePilotThingsIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deletePilotThingsIntegration(
    request: api_application_pb.DeletePilotThingsIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  createIftttIntegration(
    request: api_application_pb.CreateIftttIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  getIftttIntegration(
    request: api_application_pb.GetIftttIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GetIftttIntegrationResponse>;

  updateIftttIntegration(
    request: api_application_pb.UpdateIftttIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  deleteIftttIntegration(
    request: api_application_pb.DeleteIftttIntegrationRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<google_protobuf_empty_pb.Empty>;

  generateMqttIntegrationClientCertificate(
    request: api_application_pb.GenerateMqttIntegrationClientCertificateRequest,
    metadata?: grpcWeb.Metadata
  ): Promise<api_application_pb.GenerateMqttIntegrationClientCertificateResponse>;

}

