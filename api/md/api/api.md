# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [api/application.proto](#api_application-proto)
    - [Application](#api-Application)
    - [ApplicationListItem](#api-ApplicationListItem)
    - [AwsSnsIntegration](#api-AwsSnsIntegration)
    - [AzureServiceBusIntegration](#api-AzureServiceBusIntegration)
    - [CreateApplicationRequest](#api-CreateApplicationRequest)
    - [CreateApplicationResponse](#api-CreateApplicationResponse)
    - [CreateAwsSnsIntegrationRequest](#api-CreateAwsSnsIntegrationRequest)
    - [CreateAzureServiceBusIntegrationRequest](#api-CreateAzureServiceBusIntegrationRequest)
    - [CreateGcpPubSubIntegrationRequest](#api-CreateGcpPubSubIntegrationRequest)
    - [CreateHttpIntegrationRequest](#api-CreateHttpIntegrationRequest)
    - [CreateIftttIntegrationRequest](#api-CreateIftttIntegrationRequest)
    - [CreateInfluxDbIntegrationRequest](#api-CreateInfluxDbIntegrationRequest)
    - [CreateLoraCloudIntegrationRequest](#api-CreateLoraCloudIntegrationRequest)
    - [CreateMyDevicesIntegrationRequest](#api-CreateMyDevicesIntegrationRequest)
    - [CreatePilotThingsIntegrationRequest](#api-CreatePilotThingsIntegrationRequest)
    - [CreateThingsBoardIntegrationRequest](#api-CreateThingsBoardIntegrationRequest)
    - [DeleteApplicationRequest](#api-DeleteApplicationRequest)
    - [DeleteAwsSnsIntegrationRequest](#api-DeleteAwsSnsIntegrationRequest)
    - [DeleteAzureServiceBusIntegrationRequest](#api-DeleteAzureServiceBusIntegrationRequest)
    - [DeleteGcpPubSubIntegrationRequest](#api-DeleteGcpPubSubIntegrationRequest)
    - [DeleteHttpIntegrationRequest](#api-DeleteHttpIntegrationRequest)
    - [DeleteIftttIntegrationRequest](#api-DeleteIftttIntegrationRequest)
    - [DeleteInfluxDbIntegrationRequest](#api-DeleteInfluxDbIntegrationRequest)
    - [DeleteLoraCloudIntegrationRequest](#api-DeleteLoraCloudIntegrationRequest)
    - [DeleteMyDevicesIntegrationRequest](#api-DeleteMyDevicesIntegrationRequest)
    - [DeletePilotThingsIntegrationRequest](#api-DeletePilotThingsIntegrationRequest)
    - [DeleteThingsBoardIntegrationRequest](#api-DeleteThingsBoardIntegrationRequest)
    - [GcpPubSubIntegration](#api-GcpPubSubIntegration)
    - [GenerateMqttIntegrationClientCertificateRequest](#api-GenerateMqttIntegrationClientCertificateRequest)
    - [GenerateMqttIntegrationClientCertificateResponse](#api-GenerateMqttIntegrationClientCertificateResponse)
    - [GetApplicationRequest](#api-GetApplicationRequest)
    - [GetApplicationResponse](#api-GetApplicationResponse)
    - [GetAwsSnsIntegrationRequest](#api-GetAwsSnsIntegrationRequest)
    - [GetAwsSnsIntegrationResponse](#api-GetAwsSnsIntegrationResponse)
    - [GetAzureServiceBusIntegrationRequest](#api-GetAzureServiceBusIntegrationRequest)
    - [GetAzureServiceBusIntegrationResponse](#api-GetAzureServiceBusIntegrationResponse)
    - [GetGcpPubSubIntegrationRequest](#api-GetGcpPubSubIntegrationRequest)
    - [GetGcpPubSubIntegrationResponse](#api-GetGcpPubSubIntegrationResponse)
    - [GetHttpIntegrationRequest](#api-GetHttpIntegrationRequest)
    - [GetHttpIntegrationResponse](#api-GetHttpIntegrationResponse)
    - [GetIftttIntegrationRequest](#api-GetIftttIntegrationRequest)
    - [GetIftttIntegrationResponse](#api-GetIftttIntegrationResponse)
    - [GetInfluxDbIntegrationRequest](#api-GetInfluxDbIntegrationRequest)
    - [GetInfluxDbIntegrationResponse](#api-GetInfluxDbIntegrationResponse)
    - [GetLoraCloudIntegrationRequest](#api-GetLoraCloudIntegrationRequest)
    - [GetLoraCloudIntegrationResponse](#api-GetLoraCloudIntegrationResponse)
    - [GetMyDevicesIntegrationRequest](#api-GetMyDevicesIntegrationRequest)
    - [GetMyDevicesIntegrationResponse](#api-GetMyDevicesIntegrationResponse)
    - [GetPilotThingsIntegrationRequest](#api-GetPilotThingsIntegrationRequest)
    - [GetPilotThingsIntegrationResponse](#api-GetPilotThingsIntegrationResponse)
    - [GetThingsBoardIntegrationRequest](#api-GetThingsBoardIntegrationRequest)
    - [GetThingsBoardIntegrationResponse](#api-GetThingsBoardIntegrationResponse)
    - [HttpIntegration](#api-HttpIntegration)
    - [HttpIntegration.HeadersEntry](#api-HttpIntegration-HeadersEntry)
    - [IftttIntegration](#api-IftttIntegration)
    - [InfluxDbIntegration](#api-InfluxDbIntegration)
    - [IntegrationListItem](#api-IntegrationListItem)
    - [ListApplicationsRequest](#api-ListApplicationsRequest)
    - [ListApplicationsResponse](#api-ListApplicationsResponse)
    - [ListIntegrationsRequest](#api-ListIntegrationsRequest)
    - [ListIntegrationsResponse](#api-ListIntegrationsResponse)
    - [LoraCloudIntegration](#api-LoraCloudIntegration)
    - [LoraCloudModemGeolocationServices](#api-LoraCloudModemGeolocationServices)
    - [MyDevicesIntegration](#api-MyDevicesIntegration)
    - [PilotThingsIntegration](#api-PilotThingsIntegration)
    - [ThingsBoardIntegration](#api-ThingsBoardIntegration)
    - [UpdateApplicationRequest](#api-UpdateApplicationRequest)
    - [UpdateAwsSnsIntegrationRequest](#api-UpdateAwsSnsIntegrationRequest)
    - [UpdateAzureServiceBusIntegrationRequest](#api-UpdateAzureServiceBusIntegrationRequest)
    - [UpdateGcpPubSubIntegrationRequest](#api-UpdateGcpPubSubIntegrationRequest)
    - [UpdateHttpIntegrationRequest](#api-UpdateHttpIntegrationRequest)
    - [UpdateIftttIntegrationRequest](#api-UpdateIftttIntegrationRequest)
    - [UpdateInfluxDbIntegrationRequest](#api-UpdateInfluxDbIntegrationRequest)
    - [UpdateLoraCloudIntegrationRequest](#api-UpdateLoraCloudIntegrationRequest)
    - [UpdateMyDevicesIntegrationRequest](#api-UpdateMyDevicesIntegrationRequest)
    - [UpdatePilotThingsIntegrationRequest](#api-UpdatePilotThingsIntegrationRequest)
    - [UpdateThingsBoardIntegrationRequest](#api-UpdateThingsBoardIntegrationRequest)
  
    - [Encoding](#api-Encoding)
    - [InfluxDbPrecision](#api-InfluxDbPrecision)
    - [InfluxDbVersion](#api-InfluxDbVersion)
    - [IntegrationKind](#api-IntegrationKind)
  
    - [ApplicationService](#api-ApplicationService)
  
- [api/device.proto](#api_device-proto)
    - [ActivateDeviceRequest](#api-ActivateDeviceRequest)
    - [CreateDeviceKeysRequest](#api-CreateDeviceKeysRequest)
    - [CreateDeviceRequest](#api-CreateDeviceRequest)
    - [DeactivateDeviceRequest](#api-DeactivateDeviceRequest)
    - [DeleteDeviceKeysRequest](#api-DeleteDeviceKeysRequest)
    - [DeleteDeviceRequest](#api-DeleteDeviceRequest)
    - [Device](#api-Device)
    - [Device.TagsEntry](#api-Device-TagsEntry)
    - [Device.VariablesEntry](#api-Device-VariablesEntry)
    - [DeviceActivation](#api-DeviceActivation)
    - [DeviceKeys](#api-DeviceKeys)
    - [DeviceListItem](#api-DeviceListItem)
    - [DeviceQueueItem](#api-DeviceQueueItem)
    - [DeviceState](#api-DeviceState)
    - [DeviceStatus](#api-DeviceStatus)
    - [EnqueueDeviceQueueItemRequest](#api-EnqueueDeviceQueueItemRequest)
    - [EnqueueDeviceQueueItemResponse](#api-EnqueueDeviceQueueItemResponse)
    - [FlushDevNoncesRequest](#api-FlushDevNoncesRequest)
    - [FlushDeviceQueueRequest](#api-FlushDeviceQueueRequest)
    - [GetDeviceActivationRequest](#api-GetDeviceActivationRequest)
    - [GetDeviceActivationResponse](#api-GetDeviceActivationResponse)
    - [GetDeviceKeysRequest](#api-GetDeviceKeysRequest)
    - [GetDeviceKeysResponse](#api-GetDeviceKeysResponse)
    - [GetDeviceLinkMetricsRequest](#api-GetDeviceLinkMetricsRequest)
    - [GetDeviceLinkMetricsResponse](#api-GetDeviceLinkMetricsResponse)
    - [GetDeviceMetricsRequest](#api-GetDeviceMetricsRequest)
    - [GetDeviceMetricsResponse](#api-GetDeviceMetricsResponse)
    - [GetDeviceMetricsResponse.MetricsEntry](#api-GetDeviceMetricsResponse-MetricsEntry)
    - [GetDeviceMetricsResponse.StatesEntry](#api-GetDeviceMetricsResponse-StatesEntry)
    - [GetDeviceNextFCntDownRequest](#api-GetDeviceNextFCntDownRequest)
    - [GetDeviceNextFCntDownResponse](#api-GetDeviceNextFCntDownResponse)
    - [GetDeviceQueueItemsRequest](#api-GetDeviceQueueItemsRequest)
    - [GetDeviceQueueItemsResponse](#api-GetDeviceQueueItemsResponse)
    - [GetDeviceRequest](#api-GetDeviceRequest)
    - [GetDeviceResponse](#api-GetDeviceResponse)
    - [GetRandomDevAddrRequest](#api-GetRandomDevAddrRequest)
    - [GetRandomDevAddrResponse](#api-GetRandomDevAddrResponse)
    - [ListDevicesRequest](#api-ListDevicesRequest)
    - [ListDevicesResponse](#api-ListDevicesResponse)
    - [UpdateDeviceKeysRequest](#api-UpdateDeviceKeysRequest)
    - [UpdateDeviceRequest](#api-UpdateDeviceRequest)
  
    - [DeviceService](#api-DeviceService)
  
- [api/device_profile.proto](#api_device_profile-proto)
    - [AdrAlgorithmListItem](#api-AdrAlgorithmListItem)
    - [CreateDeviceProfileRequest](#api-CreateDeviceProfileRequest)
    - [CreateDeviceProfileResponse](#api-CreateDeviceProfileResponse)
    - [DeleteDeviceProfileRequest](#api-DeleteDeviceProfileRequest)
    - [DeviceProfile](#api-DeviceProfile)
    - [DeviceProfile.MeasurementsEntry](#api-DeviceProfile-MeasurementsEntry)
    - [DeviceProfile.TagsEntry](#api-DeviceProfile-TagsEntry)
    - [DeviceProfileListItem](#api-DeviceProfileListItem)
    - [GetDeviceProfileRequest](#api-GetDeviceProfileRequest)
    - [GetDeviceProfileResponse](#api-GetDeviceProfileResponse)
    - [ListDeviceProfileAdrAlgorithmsResponse](#api-ListDeviceProfileAdrAlgorithmsResponse)
    - [ListDeviceProfilesRequest](#api-ListDeviceProfilesRequest)
    - [ListDeviceProfilesResponse](#api-ListDeviceProfilesResponse)
    - [Measurement](#api-Measurement)
    - [UpdateDeviceProfileRequest](#api-UpdateDeviceProfileRequest)
  
    - [CadPeriodicity](#api-CadPeriodicity)
    - [CodecRuntime](#api-CodecRuntime)
    - [MeasurementKind](#api-MeasurementKind)
    - [RelayModeActivation](#api-RelayModeActivation)
    - [SecondChAckOffset](#api-SecondChAckOffset)
  
    - [DeviceProfileService](#api-DeviceProfileService)
  
- [api/frame_log.proto](#api_frame_log-proto)
    - [DownlinkFrameLog](#api-DownlinkFrameLog)
    - [UplinkFrameLog](#api-UplinkFrameLog)
  
- [api/gateway.proto](#api_gateway-proto)
    - [CreateGatewayRequest](#api-CreateGatewayRequest)
    - [DeleteGatewayRequest](#api-DeleteGatewayRequest)
    - [Gateway](#api-Gateway)
    - [Gateway.MetadataEntry](#api-Gateway-MetadataEntry)
    - [Gateway.TagsEntry](#api-Gateway-TagsEntry)
    - [GatewayListItem](#api-GatewayListItem)
    - [GatewayListItem.PropertiesEntry](#api-GatewayListItem-PropertiesEntry)
    - [GenerateGatewayClientCertificateRequest](#api-GenerateGatewayClientCertificateRequest)
    - [GenerateGatewayClientCertificateResponse](#api-GenerateGatewayClientCertificateResponse)
    - [GetGatewayMetricsRequest](#api-GetGatewayMetricsRequest)
    - [GetGatewayMetricsResponse](#api-GetGatewayMetricsResponse)
    - [GetGatewayRequest](#api-GetGatewayRequest)
    - [GetGatewayResponse](#api-GetGatewayResponse)
    - [ListGatewaysRequest](#api-ListGatewaysRequest)
    - [ListGatewaysResponse](#api-ListGatewaysResponse)
    - [UpdateGatewayRequest](#api-UpdateGatewayRequest)
  
    - [GatewayState](#api-GatewayState)
  
    - [GatewayService](#api-GatewayService)
  
- [api/multicast_group.proto](#api_multicast_group-proto)
    - [AddDeviceToMulticastGroupRequest](#api-AddDeviceToMulticastGroupRequest)
    - [AddGatewayToMulticastGroupRequest](#api-AddGatewayToMulticastGroupRequest)
    - [CreateMulticastGroupRequest](#api-CreateMulticastGroupRequest)
    - [CreateMulticastGroupResponse](#api-CreateMulticastGroupResponse)
    - [DeleteMulticastGroupRequest](#api-DeleteMulticastGroupRequest)
    - [EnqueueMulticastGroupQueueItemRequest](#api-EnqueueMulticastGroupQueueItemRequest)
    - [EnqueueMulticastGroupQueueItemResponse](#api-EnqueueMulticastGroupQueueItemResponse)
    - [FlushMulticastGroupQueueRequest](#api-FlushMulticastGroupQueueRequest)
    - [GetMulticastGroupRequest](#api-GetMulticastGroupRequest)
    - [GetMulticastGroupResponse](#api-GetMulticastGroupResponse)
    - [ListMulticastGroupQueueRequest](#api-ListMulticastGroupQueueRequest)
    - [ListMulticastGroupQueueResponse](#api-ListMulticastGroupQueueResponse)
    - [ListMulticastGroupsRequest](#api-ListMulticastGroupsRequest)
    - [ListMulticastGroupsResponse](#api-ListMulticastGroupsResponse)
    - [MulticastGroup](#api-MulticastGroup)
    - [MulticastGroupListItem](#api-MulticastGroupListItem)
    - [MulticastGroupQueueItem](#api-MulticastGroupQueueItem)
    - [RemoveDeviceFromMulticastGroupRequest](#api-RemoveDeviceFromMulticastGroupRequest)
    - [RemoveGatewayFromMulticastGroupRequest](#api-RemoveGatewayFromMulticastGroupRequest)
    - [UpdateMulticastGroupRequest](#api-UpdateMulticastGroupRequest)
  
    - [MulticastGroupSchedulingType](#api-MulticastGroupSchedulingType)
    - [MulticastGroupType](#api-MulticastGroupType)
  
    - [MulticastGroupService](#api-MulticastGroupService)
  
- [api/tenant.proto](#api_tenant-proto)
    - [AddTenantUserRequest](#api-AddTenantUserRequest)
    - [CreateTenantRequest](#api-CreateTenantRequest)
    - [CreateTenantResponse](#api-CreateTenantResponse)
    - [DeleteTenantRequest](#api-DeleteTenantRequest)
    - [DeleteTenantUserRequest](#api-DeleteTenantUserRequest)
    - [GetTenantRequest](#api-GetTenantRequest)
    - [GetTenantResponse](#api-GetTenantResponse)
    - [GetTenantUserRequest](#api-GetTenantUserRequest)
    - [GetTenantUserResponse](#api-GetTenantUserResponse)
    - [ListTenantUsersRequest](#api-ListTenantUsersRequest)
    - [ListTenantUsersResponse](#api-ListTenantUsersResponse)
    - [ListTenantsRequest](#api-ListTenantsRequest)
    - [ListTenantsResponse](#api-ListTenantsResponse)
    - [Tenant](#api-Tenant)
    - [TenantListItem](#api-TenantListItem)
    - [TenantUser](#api-TenantUser)
    - [TenantUserListItem](#api-TenantUserListItem)
    - [UpdateTenantRequest](#api-UpdateTenantRequest)
    - [UpdateTenantUserRequest](#api-UpdateTenantUserRequest)
  
    - [TenantService](#api-TenantService)
  
- [api/user.proto](#api_user-proto)
    - [CreateUserRequest](#api-CreateUserRequest)
    - [CreateUserResponse](#api-CreateUserResponse)
    - [DeleteUserRequest](#api-DeleteUserRequest)
    - [GetUserRequest](#api-GetUserRequest)
    - [GetUserResponse](#api-GetUserResponse)
    - [ListUsersRequest](#api-ListUsersRequest)
    - [ListUsersResponse](#api-ListUsersResponse)
    - [UpdateUserPasswordRequest](#api-UpdateUserPasswordRequest)
    - [UpdateUserRequest](#api-UpdateUserRequest)
    - [User](#api-User)
    - [UserListItem](#api-UserListItem)
    - [UserTenant](#api-UserTenant)
  
    - [UserService](#api-UserService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="api_application-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/application.proto



<a name="api-Application"></a>

### Application



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Application ID (UUID). Note: on create this will be automatically generated. |
| name | [string](#string) |  | Application name. |
| description | [string](#string) |  | Application description. |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |






<a name="api-ApplicationListItem"></a>

### ApplicationListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Application ID (UUID). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| name | [string](#string) |  | Application name. |
| description | [string](#string) |  | Application description. |






<a name="api-AwsSnsIntegration"></a>

### AwsSnsIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| encoding | [Encoding](#api-Encoding) |  | Encoding. |
| region | [string](#string) |  | AWS region. |
| access_key_id | [string](#string) |  | AWS Access Key ID. |
| secret_access_key | [string](#string) |  | AWS Secret Access Key. |
| topic_arn | [string](#string) |  | Topic ARN. |






<a name="api-AzureServiceBusIntegration"></a>

### AzureServiceBusIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| encoding | [Encoding](#api-Encoding) |  | Encoding. |
| connection_string | [string](#string) |  | Connection string. |
| publish_name | [string](#string) |  | Publish name. This is the name of the topic or queue. |






<a name="api-CreateApplicationRequest"></a>

### CreateApplicationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application | [Application](#api-Application) |  | Application object to create. |






<a name="api-CreateApplicationResponse"></a>

### CreateApplicationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Application ID (UUID). |






<a name="api-CreateAwsSnsIntegrationRequest"></a>

### CreateAwsSnsIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [AwsSnsIntegration](#api-AwsSnsIntegration) |  | Integration object to create. |






<a name="api-CreateAzureServiceBusIntegrationRequest"></a>

### CreateAzureServiceBusIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [AzureServiceBusIntegration](#api-AzureServiceBusIntegration) |  | Integration object to create. |






<a name="api-CreateGcpPubSubIntegrationRequest"></a>

### CreateGcpPubSubIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [GcpPubSubIntegration](#api-GcpPubSubIntegration) |  | Integration object to create. |






<a name="api-CreateHttpIntegrationRequest"></a>

### CreateHttpIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [HttpIntegration](#api-HttpIntegration) |  | Integration object to create. |






<a name="api-CreateIftttIntegrationRequest"></a>

### CreateIftttIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [IftttIntegration](#api-IftttIntegration) |  | Integration object. |






<a name="api-CreateInfluxDbIntegrationRequest"></a>

### CreateInfluxDbIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [InfluxDbIntegration](#api-InfluxDbIntegration) |  | Integration object to create. |






<a name="api-CreateLoraCloudIntegrationRequest"></a>

### CreateLoraCloudIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [LoraCloudIntegration](#api-LoraCloudIntegration) |  | Integration object to create. |






<a name="api-CreateMyDevicesIntegrationRequest"></a>

### CreateMyDevicesIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [MyDevicesIntegration](#api-MyDevicesIntegration) |  | Integration object to create. |






<a name="api-CreatePilotThingsIntegrationRequest"></a>

### CreatePilotThingsIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [PilotThingsIntegration](#api-PilotThingsIntegration) |  | Integration object to create. |






<a name="api-CreateThingsBoardIntegrationRequest"></a>

### CreateThingsBoardIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [ThingsBoardIntegration](#api-ThingsBoardIntegration) |  | Integration object to create. |






<a name="api-DeleteApplicationRequest"></a>

### DeleteApplicationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteAwsSnsIntegrationRequest"></a>

### DeleteAwsSnsIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteAzureServiceBusIntegrationRequest"></a>

### DeleteAzureServiceBusIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteGcpPubSubIntegrationRequest"></a>

### DeleteGcpPubSubIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteHttpIntegrationRequest"></a>

### DeleteHttpIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteIftttIntegrationRequest"></a>

### DeleteIftttIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteInfluxDbIntegrationRequest"></a>

### DeleteInfluxDbIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteLoraCloudIntegrationRequest"></a>

### DeleteLoraCloudIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteMyDevicesIntegrationRequest"></a>

### DeleteMyDevicesIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeletePilotThingsIntegrationRequest"></a>

### DeletePilotThingsIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-DeleteThingsBoardIntegrationRequest"></a>

### DeleteThingsBoardIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GcpPubSubIntegration"></a>

### GcpPubSubIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| encoding | [Encoding](#api-Encoding) |  | Encoding. |
| credentials_file | [string](#string) |  | Credentials file. This IAM service-account credentials file (JSON) must have the following Pub/Sub roles: * Pub/Sub Publisher |
| project_id | [string](#string) |  | Project ID. |
| topic_name | [string](#string) |  | Topic name. This is the name of the Pub/Sub topic. |






<a name="api-GenerateMqttIntegrationClientCertificateRequest"></a>

### GenerateMqttIntegrationClientCertificateRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GenerateMqttIntegrationClientCertificateResponse"></a>

### GenerateMqttIntegrationClientCertificateResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tls_cert | [string](#string) |  | TLS certificate. |
| tls_key | [string](#string) |  | TLS key. |
| ca_cert | [string](#string) |  | CA certificate. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Expires at defines the expiration date of the certificate. |






<a name="api-GetApplicationRequest"></a>

### GetApplicationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetApplicationResponse"></a>

### GetApplicationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application | [Application](#api-Application) |  | Application object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| measurement_keys | [string](#string) | repeated | Measurement keys. This contains the measurement keys from all the device-profiles that are used by the devices under this application. |






<a name="api-GetAwsSnsIntegrationRequest"></a>

### GetAwsSnsIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetAwsSnsIntegrationResponse"></a>

### GetAwsSnsIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [AwsSnsIntegration](#api-AwsSnsIntegration) |  | Integration object. |






<a name="api-GetAzureServiceBusIntegrationRequest"></a>

### GetAzureServiceBusIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetAzureServiceBusIntegrationResponse"></a>

### GetAzureServiceBusIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [AzureServiceBusIntegration](#api-AzureServiceBusIntegration) |  | Integration object. |






<a name="api-GetGcpPubSubIntegrationRequest"></a>

### GetGcpPubSubIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetGcpPubSubIntegrationResponse"></a>

### GetGcpPubSubIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [GcpPubSubIntegration](#api-GcpPubSubIntegration) |  | Integration object. |






<a name="api-GetHttpIntegrationRequest"></a>

### GetHttpIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetHttpIntegrationResponse"></a>

### GetHttpIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [HttpIntegration](#api-HttpIntegration) |  | Integration object. |






<a name="api-GetIftttIntegrationRequest"></a>

### GetIftttIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetIftttIntegrationResponse"></a>

### GetIftttIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [IftttIntegration](#api-IftttIntegration) |  | Integration object. |






<a name="api-GetInfluxDbIntegrationRequest"></a>

### GetInfluxDbIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetInfluxDbIntegrationResponse"></a>

### GetInfluxDbIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [InfluxDbIntegration](#api-InfluxDbIntegration) |  | Integration object. |






<a name="api-GetLoraCloudIntegrationRequest"></a>

### GetLoraCloudIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetLoraCloudIntegrationResponse"></a>

### GetLoraCloudIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [LoraCloudIntegration](#api-LoraCloudIntegration) |  | Integration object. |






<a name="api-GetMyDevicesIntegrationRequest"></a>

### GetMyDevicesIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetMyDevicesIntegrationResponse"></a>

### GetMyDevicesIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [MyDevicesIntegration](#api-MyDevicesIntegration) |  | Integration object. |






<a name="api-GetPilotThingsIntegrationRequest"></a>

### GetPilotThingsIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetPilotThingsIntegrationResponse"></a>

### GetPilotThingsIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [PilotThingsIntegration](#api-PilotThingsIntegration) |  | Integration object. |






<a name="api-GetThingsBoardIntegrationRequest"></a>

### GetThingsBoardIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-GetThingsBoardIntegrationResponse"></a>

### GetThingsBoardIntegrationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [ThingsBoardIntegration](#api-ThingsBoardIntegration) |  | Integration object. |






<a name="api-HttpIntegration"></a>

### HttpIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| headers | [HttpIntegration.HeadersEntry](#api-HttpIntegration-HeadersEntry) | repeated | HTTP headers to set when making requests. |
| encoding | [Encoding](#api-Encoding) |  | Payload encoding. |
| event_endpoint_url | [string](#string) |  | Event endpoint URL. The HTTP integration will POST all events to this enpoint. The request will contain a query parameters &#34;event&#34; containing the type of the event. |






<a name="api-HttpIntegration-HeadersEntry"></a>

### HttpIntegration.HeadersEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="api-IftttIntegration"></a>

### IftttIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| key | [string](#string) |  | Key. This key can be obtained from the IFTTT Webhooks documentation page. |
| uplink_values | [string](#string) | repeated | Values. Up to 2 values can be forwarded to IFTTT. These values must map to the decoded payload keys. For example: { &#34;batteryLevel&#34;: 75.3, &#34;buttons&#34;: [{&#34;pressed&#34;: false}, {&#34;pressed&#34;: true}] } You would specify the following fields: uplink_values = [&#34;batteryLevel&#34;, &#34;buttons_0_pressed&#34;]

Notes: The first value is always used for the DevEUI. Ignored if arbitrary_json is set to true. |
| arbitrary_json | [bool](#bool) |  | Arbitrary JSON. If set to true, ChirpStack events will be sent as-is as arbitrary JSON payload. If set to false (default), the 3 JSON values format will be used. |
| event_prefix | [string](#string) |  | Event prefix. If set, the event name will be PREFIX_EVENT. For example if event_prefix is set to weatherstation, and uplink event will be sent as weatherstation_up to the IFTTT webhook. Note: Only characters in the A-Z, a-z and 0-9 range are allowed. |






<a name="api-InfluxDbIntegration"></a>

### InfluxDbIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| endpoint | [string](#string) |  | InfluxDb API write endpoint (e.g. http://localhost:8086/write). |
| db | [string](#string) |  | InfluxDb database name. (InfluxDb v1) |
| username | [string](#string) |  | InfluxDb username. (InfluxDb v1) |
| password | [string](#string) |  | InfluxDb password. (InfluxDb v1) |
| retention_policy_name | [string](#string) |  | InfluxDb retention policy name. (InfluxDb v1) |
| precision | [InfluxDbPrecision](#api-InfluxDbPrecision) |  | InfluxDb timestamp precision (InfluxDb v1). |
| version | [InfluxDbVersion](#api-InfluxDbVersion) |  | InfluxDb version. |
| token | [string](#string) |  | Token. (InfluxDb v2) |
| organization | [string](#string) |  | Organization. (InfluxDb v2) |
| bucket | [string](#string) |  | Bucket. (InfluxDb v2) |






<a name="api-IntegrationListItem"></a>

### IntegrationListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| kind | [IntegrationKind](#api-IntegrationKind) |  | Integration kind. |






<a name="api-ListApplicationsRequest"></a>

### ListApplicationsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  | Max number of applications to return in the result-set. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |
| search | [string](#string) |  | If set, the given string will be used to search on name (optional). |
| tenant_id | [string](#string) |  | Tenant ID to list the applications for. |






<a name="api-ListApplicationsResponse"></a>

### ListApplicationsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of applications. |
| result | [ApplicationListItem](#api-ApplicationListItem) | repeated | Result-set. |






<a name="api-ListIntegrationsRequest"></a>

### ListIntegrationsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |






<a name="api-ListIntegrationsResponse"></a>

### ListIntegrationsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of integrations available within the result-set. |
| result | [IntegrationListItem](#api-IntegrationListItem) | repeated | Integrations within result-set. |






<a name="api-LoraCloudIntegration"></a>

### LoraCloudIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| modem_geolocation_services | [LoraCloudModemGeolocationServices](#api-LoraCloudModemGeolocationServices) |  | Modem &amp; Geolocation Services configuration. |






<a name="api-LoraCloudModemGeolocationServices"></a>

### LoraCloudModemGeolocationServices



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | API token. |
| modem_enabled | [bool](#bool) |  | Device implements Modem / Modem-E stack. |
| forward_f_ports | [uint32](#uint32) | repeated | Forward FPorts. Forward uplink messages matching the given FPorts to the MGS. |
| gnss_use_rx_time | [bool](#bool) |  | Use rx time for GNSS resolving. In case this is set to true, the MGS resolver will use the RX time of the network instead of the timestamp included in the LR1110 payload. |
| gnss_use_gateway_location | [bool](#bool) |  | Use gateway location for GNSS resolving. In the case this is set to true, ChirpStack will provide the location of one of the gateways to the MGS resolver to aid the resolving process. Disable this in case the gateway location is not accurate / incorrectly configured as an incorrect location will cause the resolver to return an error. |
| parse_tlv | [bool](#bool) |  | Parse TLV records. If enabled, stream records (expected in TLV format) are scanned for GNSS data (0x06 or 0x07). If found, ChirpStack will make an additional geolocation call to the MGS API for resolving the location of the detected payload. |
| geolocation_buffer_ttl | [uint32](#uint32) |  | Geolocation buffer TTL (in seconds). If &gt; 0, uplink RX meta-data will be stored in a buffer so that the meta-data of multiple uplinks can be used for geolocation. |
| geolocation_min_buffer_size | [uint32](#uint32) |  | Geolocation minimum buffer size. If &gt; 0, geolocation will only be performed when the buffer has at least the given size. |
| geolocation_tdoa | [bool](#bool) |  | TDOA based geolocation is enabled. |
| geolocation_rssi | [bool](#bool) |  | RSSI based geolocation is enabled. |
| geolocation_gnss | [bool](#bool) |  | GNSS based geolocation is enabled (LR1110). |
| geolocation_gnss_payload_field | [string](#string) |  | GNSS payload field. This holds the name of the field in the decoded payload object which contains the GNSS payload bytes (as HEX string). |
| geolocation_gnss_use_rx_time | [bool](#bool) |  | GNSS use RX time. In case this is set to true, the resolver will use the RX time of the network instead of the timestamp included in the LR1110 payload. |
| geolocation_wifi | [bool](#bool) |  | Wifi based geolocation is enabled. |
| geolocation_wifi_payload_field | [string](#string) |  | Wifi payload field. This holds the name of the field in the decoded payload object which contains an array of objects with the following fields: * macAddress - e.g. 01:23:45:67:89:ab * signalStrength - e.g. -51 (optional) |






<a name="api-MyDevicesIntegration"></a>

### MyDevicesIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| endpoint | [string](#string) |  | myDevices API endpoint. |






<a name="api-PilotThingsIntegration"></a>

### PilotThingsIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| server | [string](#string) |  | Server URL. |
| token | [string](#string) |  | Authentication token. |






<a name="api-ThingsBoardIntegration"></a>

### ThingsBoardIntegration



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application_id | [string](#string) |  | Application ID (UUID). |
| server | [string](#string) |  | ThingsBoard server endpoint, e.g. https://example.com |






<a name="api-UpdateApplicationRequest"></a>

### UpdateApplicationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| application | [Application](#api-Application) |  | Application object. |






<a name="api-UpdateAwsSnsIntegrationRequest"></a>

### UpdateAwsSnsIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [AwsSnsIntegration](#api-AwsSnsIntegration) |  | Integration object to update. |






<a name="api-UpdateAzureServiceBusIntegrationRequest"></a>

### UpdateAzureServiceBusIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [AzureServiceBusIntegration](#api-AzureServiceBusIntegration) |  | Integration object to create. |






<a name="api-UpdateGcpPubSubIntegrationRequest"></a>

### UpdateGcpPubSubIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [GcpPubSubIntegration](#api-GcpPubSubIntegration) |  | Integration object to update. |






<a name="api-UpdateHttpIntegrationRequest"></a>

### UpdateHttpIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [HttpIntegration](#api-HttpIntegration) |  | Integration object to update. |






<a name="api-UpdateIftttIntegrationRequest"></a>

### UpdateIftttIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [IftttIntegration](#api-IftttIntegration) |  | Integration object to update. |






<a name="api-UpdateInfluxDbIntegrationRequest"></a>

### UpdateInfluxDbIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [InfluxDbIntegration](#api-InfluxDbIntegration) |  | Integration object to update. |






<a name="api-UpdateLoraCloudIntegrationRequest"></a>

### UpdateLoraCloudIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [LoraCloudIntegration](#api-LoraCloudIntegration) |  | Integration object to update. |






<a name="api-UpdateMyDevicesIntegrationRequest"></a>

### UpdateMyDevicesIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [MyDevicesIntegration](#api-MyDevicesIntegration) |  | Integration object to update. |






<a name="api-UpdatePilotThingsIntegrationRequest"></a>

### UpdatePilotThingsIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [PilotThingsIntegration](#api-PilotThingsIntegration) |  | Integration object to update. |






<a name="api-UpdateThingsBoardIntegrationRequest"></a>

### UpdateThingsBoardIntegrationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| integration | [ThingsBoardIntegration](#api-ThingsBoardIntegration) |  | Integration object to update. |





 


<a name="api-Encoding"></a>

### Encoding


| Name | Number | Description |
| ---- | ------ | ----------- |
| JSON | 0 |  |
| PROTOBUF | 1 |  |



<a name="api-InfluxDbPrecision"></a>

### InfluxDbPrecision


| Name | Number | Description |
| ---- | ------ | ----------- |
| NS | 0 |  |
| U | 1 |  |
| MS | 2 |  |
| S | 3 |  |
| M | 4 |  |
| H | 5 |  |



<a name="api-InfluxDbVersion"></a>

### InfluxDbVersion


| Name | Number | Description |
| ---- | ------ | ----------- |
| INFLUXDB_1 | 0 |  |
| INFLUXDB_2 | 1 |  |



<a name="api-IntegrationKind"></a>

### IntegrationKind


| Name | Number | Description |
| ---- | ------ | ----------- |
| HTTP | 0 |  |
| INFLUX_DB | 1 |  |
| THINGS_BOARD | 2 |  |
| MY_DEVICES | 3 |  |
| LORA_CLOUD | 4 |  |
| GCP_PUB_SUB | 5 |  |
| AWS_SNS | 6 |  |
| AZURE_SERVICE_BUS | 7 |  |
| PILOT_THINGS | 8 |  |
| MQTT_GLOBAL | 9 |  |
| IFTTT | 10 |  |


 

 


<a name="api-ApplicationService"></a>

### ApplicationService
ApplicationService is the service providing API methods for managing
applications.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateApplicationRequest](#api-CreateApplicationRequest) | [CreateApplicationResponse](#api-CreateApplicationResponse) | Create creates the given application. |
| Get | [GetApplicationRequest](#api-GetApplicationRequest) | [GetApplicationResponse](#api-GetApplicationResponse) | Get the application for the given ID. |
| Update | [UpdateApplicationRequest](#api-UpdateApplicationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update updates the given application. |
| Delete | [DeleteApplicationRequest](#api-DeleteApplicationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the application for the given ID. |
| List | [ListApplicationsRequest](#api-ListApplicationsRequest) | [ListApplicationsResponse](#api-ListApplicationsResponse) | Get the list of applications. |
| ListIntegrations | [ListIntegrationsRequest](#api-ListIntegrationsRequest) | [ListIntegrationsResponse](#api-ListIntegrationsResponse) | List all configured integrations. |
| CreateHttpIntegration | [CreateHttpIntegrationRequest](#api-CreateHttpIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create HTTP integration. |
| GetHttpIntegration | [GetHttpIntegrationRequest](#api-GetHttpIntegrationRequest) | [GetHttpIntegrationResponse](#api-GetHttpIntegrationResponse) | Get the configured HTTP integration. |
| UpdateHttpIntegration | [UpdateHttpIntegrationRequest](#api-UpdateHttpIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the HTTP integration. |
| DeleteHttpIntegration | [DeleteHttpIntegrationRequest](#api-DeleteHttpIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the HTTP integration. |
| CreateInfluxDbIntegration | [CreateInfluxDbIntegrationRequest](#api-CreateInfluxDbIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create InfluxDb integration. |
| GetInfluxDbIntegration | [GetInfluxDbIntegrationRequest](#api-GetInfluxDbIntegrationRequest) | [GetInfluxDbIntegrationResponse](#api-GetInfluxDbIntegrationResponse) | Get InfluxDb integration. |
| UpdateInfluxDbIntegration | [UpdateInfluxDbIntegrationRequest](#api-UpdateInfluxDbIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update InfluxDb integration. |
| DeleteInfluxDbIntegration | [DeleteInfluxDbIntegrationRequest](#api-DeleteInfluxDbIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete InfluxDb integration. |
| CreateThingsBoardIntegration | [CreateThingsBoardIntegrationRequest](#api-CreateThingsBoardIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create ThingsBoard integration. |
| GetThingsBoardIntegration | [GetThingsBoardIntegrationRequest](#api-GetThingsBoardIntegrationRequest) | [GetThingsBoardIntegrationResponse](#api-GetThingsBoardIntegrationResponse) | Get ThingsBoard integration. |
| UpdateThingsBoardIntegration | [UpdateThingsBoardIntegrationRequest](#api-UpdateThingsBoardIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update ThingsBoard integration. |
| DeleteThingsBoardIntegration | [DeleteThingsBoardIntegrationRequest](#api-DeleteThingsBoardIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete ThingsBoard integration. |
| CreateMyDevicesIntegration | [CreateMyDevicesIntegrationRequest](#api-CreateMyDevicesIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create myDevices integration. |
| GetMyDevicesIntegration | [GetMyDevicesIntegrationRequest](#api-GetMyDevicesIntegrationRequest) | [GetMyDevicesIntegrationResponse](#api-GetMyDevicesIntegrationResponse) | Get myDevices integration. |
| UpdateMyDevicesIntegration | [UpdateMyDevicesIntegrationRequest](#api-UpdateMyDevicesIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update myDevices integration. |
| DeleteMyDevicesIntegration | [DeleteMyDevicesIntegrationRequest](#api-DeleteMyDevicesIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete myDevices integration. |
| CreateLoraCloudIntegration | [CreateLoraCloudIntegrationRequest](#api-CreateLoraCloudIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create LoRaCloud integration. |
| GetLoraCloudIntegration | [GetLoraCloudIntegrationRequest](#api-GetLoraCloudIntegrationRequest) | [GetLoraCloudIntegrationResponse](#api-GetLoraCloudIntegrationResponse) | Get LoRaCloud integration. |
| UpdateLoraCloudIntegration | [UpdateLoraCloudIntegrationRequest](#api-UpdateLoraCloudIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update LoRaCloud integration. |
| DeleteLoraCloudIntegration | [DeleteLoraCloudIntegrationRequest](#api-DeleteLoraCloudIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete LoRaCloud integration. |
| CreateGcpPubSubIntegration | [CreateGcpPubSubIntegrationRequest](#api-CreateGcpPubSubIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create GCP Pub/Sub integration. |
| GetGcpPubSubIntegration | [GetGcpPubSubIntegrationRequest](#api-GetGcpPubSubIntegrationRequest) | [GetGcpPubSubIntegrationResponse](#api-GetGcpPubSubIntegrationResponse) | Get GCP Pub/Sub integration. |
| UpdateGcpPubSubIntegration | [UpdateGcpPubSubIntegrationRequest](#api-UpdateGcpPubSubIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update GCP Pub/Sub integration. |
| DeleteGcpPubSubIntegration | [DeleteGcpPubSubIntegrationRequest](#api-DeleteGcpPubSubIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete GCP Pub/Sub integration. |
| CreateAwsSnsIntegration | [CreateAwsSnsIntegrationRequest](#api-CreateAwsSnsIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create AWS SNS integration. |
| GetAwsSnsIntegration | [GetAwsSnsIntegrationRequest](#api-GetAwsSnsIntegrationRequest) | [GetAwsSnsIntegrationResponse](#api-GetAwsSnsIntegrationResponse) | Get AWS SNS integration. |
| UpdateAwsSnsIntegration | [UpdateAwsSnsIntegrationRequest](#api-UpdateAwsSnsIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update AWS SNS integration. |
| DeleteAwsSnsIntegration | [DeleteAwsSnsIntegrationRequest](#api-DeleteAwsSnsIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete AWS SNS integration. |
| CreateAzureServiceBusIntegration | [CreateAzureServiceBusIntegrationRequest](#api-CreateAzureServiceBusIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create Azure Service-Bus integration. |
| GetAzureServiceBusIntegration | [GetAzureServiceBusIntegrationRequest](#api-GetAzureServiceBusIntegrationRequest) | [GetAzureServiceBusIntegrationResponse](#api-GetAzureServiceBusIntegrationResponse) | Get Azure Service-Bus integration. |
| UpdateAzureServiceBusIntegration | [UpdateAzureServiceBusIntegrationRequest](#api-UpdateAzureServiceBusIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update Azure Service-Bus integration. |
| DeleteAzureServiceBusIntegration | [DeleteAzureServiceBusIntegrationRequest](#api-DeleteAzureServiceBusIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete Azure Service-Bus integration. |
| CreatePilotThingsIntegration | [CreatePilotThingsIntegrationRequest](#api-CreatePilotThingsIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create Pilot Things integration. |
| GetPilotThingsIntegration | [GetPilotThingsIntegrationRequest](#api-GetPilotThingsIntegrationRequest) | [GetPilotThingsIntegrationResponse](#api-GetPilotThingsIntegrationResponse) | Get Pilot Things integration. |
| UpdatePilotThingsIntegration | [UpdatePilotThingsIntegrationRequest](#api-UpdatePilotThingsIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update Pilot Things integration. |
| DeletePilotThingsIntegration | [DeletePilotThingsIntegrationRequest](#api-DeletePilotThingsIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete Pilot Things integration. |
| CreateIftttIntegration | [CreateIftttIntegrationRequest](#api-CreateIftttIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create IFTTT integration. |
| GetIftttIntegration | [GetIftttIntegrationRequest](#api-GetIftttIntegrationRequest) | [GetIftttIntegrationResponse](#api-GetIftttIntegrationResponse) | Get IFTTT integration. |
| UpdateIftttIntegration | [UpdateIftttIntegrationRequest](#api-UpdateIftttIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update IFTTT integration. |
| DeleteIftttIntegration | [DeleteIftttIntegrationRequest](#api-DeleteIftttIntegrationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete IFTTT integration. |
| GenerateMqttIntegrationClientCertificate | [GenerateMqttIntegrationClientCertificateRequest](#api-GenerateMqttIntegrationClientCertificateRequest) | [GenerateMqttIntegrationClientCertificateResponse](#api-GenerateMqttIntegrationClientCertificateResponse) | Generates application ID specific client-certificate. |

 



<a name="api_device-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/device.proto



<a name="api-ActivateDeviceRequest"></a>

### ActivateDeviceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_activation | [DeviceActivation](#api-DeviceActivation) |  | Device activation object. |






<a name="api-CreateDeviceKeysRequest"></a>

### CreateDeviceKeysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_keys | [DeviceKeys](#api-DeviceKeys) |  | Device-keys object. |






<a name="api-CreateDeviceRequest"></a>

### CreateDeviceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device | [Device](#api-Device) |  | Device object. |






<a name="api-DeactivateDeviceRequest"></a>

### DeactivateDeviceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |






<a name="api-DeleteDeviceKeysRequest"></a>

### DeleteDeviceKeysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |






<a name="api-DeleteDeviceRequest"></a>

### DeleteDeviceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |






<a name="api-Device"></a>

### Device



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |
| name | [string](#string) |  | Name. |
| description | [string](#string) |  | Description. |
| application_id | [string](#string) |  | Application ID (UUID). |
| device_profile_id | [string](#string) |  | Device-profile ID (UUID). |
| skip_fcnt_check | [bool](#bool) |  | Skip frame-counter checks (this is insecure, but could be helpful for debugging). |
| is_disabled | [bool](#bool) |  | Device is disabled. |
| variables | [Device.VariablesEntry](#api-Device-VariablesEntry) | repeated | Variables (user defined). These variables can be used together with integrations to store tokens / secrets that must be configured per device. These variables are not exposed in the event payloads. |
| tags | [Device.TagsEntry](#api-Device-TagsEntry) | repeated | Tags (user defined). These tags are exposed in the event payloads or to integration. Tags are intended for aggregation and filtering. |
| join_eui | [string](#string) |  | JoinEUI (optional, EUI64). This field will be automatically set / updated on OTAA. However, in some cases it must be pre-configured. For example to allow OTAA using a Relay. In this case the Relay needs to know the JoinEUI &#43; DevEUI combinations of the devices for which it needs to forward uplinks. |






<a name="api-Device-TagsEntry"></a>

### Device.TagsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="api-Device-VariablesEntry"></a>

### Device.VariablesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="api-DeviceActivation"></a>

### DeviceActivation



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | Device EUI (EUI64). |
| dev_addr | [string](#string) |  | Device address (HEX encoded). |
| app_s_key | [string](#string) |  | Application session key (HEX encoded). |
| nwk_s_enc_key | [string](#string) |  | Network session encryption key (HEX encoded). |
| s_nwk_s_int_key | [string](#string) |  | Serving network session integrity key (HEX encoded). |
| f_nwk_s_int_key | [string](#string) |  | Forwarding network session integrity key (HEX encoded). |
| f_cnt_up | [uint32](#uint32) |  | Uplink frame-counter. |
| n_f_cnt_down | [uint32](#uint32) |  | Downlink network frame-counter. |
| a_f_cnt_down | [uint32](#uint32) |  | Downlink application frame-counter. |






<a name="api-DeviceKeys"></a>

### DeviceKeys



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |
| nwk_key | [string](#string) |  | Network root key (128 bit). Note: For LoRaWAN 1.0.x, use this field for the LoRaWAN 1.0.x &#39;AppKey`! |
| app_key | [string](#string) |  | Application root key (128 bit). Note: This field only needs to be set for LoRaWAN 1.1.x devices! |






<a name="api-DeviceListItem"></a>

### DeviceListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| last_seen_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last seen at timestamp. |
| name | [string](#string) |  | Name. |
| description | [string](#string) |  | Description. |
| device_profile_id | [string](#string) |  | Device-profile ID (UUID). |
| device_profile_name | [string](#string) |  | Device-profile name. |
| device_status | [DeviceStatus](#api-DeviceStatus) |  | Device status. |






<a name="api-DeviceQueueItem"></a>

### DeviceQueueItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID (UUID). This is automatically generated on enqueue. |
| dev_eui | [string](#string) |  | Device EUI (EUI64). |
| confirmed | [bool](#bool) |  | Confirmed. |
| f_port | [uint32](#uint32) |  | FPort (must be &gt; 0). |
| data | [bytes](#bytes) |  | Data. Or use the json_object field when a codec has been configured. |
| object | [google.protobuf.Struct](#google-protobuf-Struct) |  | Only use this when a codec has been configured that can encode this object to bytes. |
| is_pending | [bool](#bool) |  | Is pending. This is set by ChirpStack to true when the downlink is pending (e.g. it has been sent, but a confirmation is still pending). |
| f_cnt_down | [uint32](#uint32) |  | Downlink frame-counter. Do not set this for plain-text data payloads. It will be automatically set by ChirpStack when the payload has been sent as downlink. |
| is_encrypted | [bool](#bool) |  | Is encrypted. This must be set to true if the end-application has already encrypted the data payload. In this case, the f_cnt_down field must be set to the corresponding frame-counter which has been used during the encryption. |






<a name="api-DeviceState"></a>

### DeviceState



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name. |
| value | [string](#string) |  | Value. |






<a name="api-DeviceStatus"></a>

### DeviceStatus



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| margin | [int32](#int32) |  | The device margin status -32..32: The demodulation SNR ration in dB |
| external_power_source | [bool](#bool) |  | Device is connected to an external power source. |
| battery_level | [float](#float) |  | Device battery level as a percentage. -1 when the battery level is not available. |






<a name="api-EnqueueDeviceQueueItemRequest"></a>

### EnqueueDeviceQueueItemRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| queue_item | [DeviceQueueItem](#api-DeviceQueueItem) |  |  |






<a name="api-EnqueueDeviceQueueItemResponse"></a>

### EnqueueDeviceQueueItemResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID (UUID). |






<a name="api-FlushDevNoncesRequest"></a>

### FlushDevNoncesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | Device EUI (EUI64). |






<a name="api-FlushDeviceQueueRequest"></a>

### FlushDeviceQueueRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | Device EUI (EUI64). |






<a name="api-GetDeviceActivationRequest"></a>

### GetDeviceActivationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |






<a name="api-GetDeviceActivationResponse"></a>

### GetDeviceActivationResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_activation | [DeviceActivation](#api-DeviceActivation) |  | Device activation object. |






<a name="api-GetDeviceKeysRequest"></a>

### GetDeviceKeysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |






<a name="api-GetDeviceKeysResponse"></a>

### GetDeviceKeysResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_keys | [DeviceKeys](#api-DeviceKeys) |  | Device-keys object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |






<a name="api-GetDeviceLinkMetricsRequest"></a>

### GetDeviceLinkMetricsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |
| start | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Interval start timestamp. |
| end | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Interval end timestamp. |
| aggregation | [common.Aggregation](#common-Aggregation) |  | Aggregation. |






<a name="api-GetDeviceLinkMetricsResponse"></a>

### GetDeviceLinkMetricsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| rx_packets | [common.Metric](#common-Metric) |  | Packets received from the device. |
| gw_rssi | [common.Metric](#common-Metric) |  | RSSI (as reported by the gateway(s)). |
| gw_snr | [common.Metric](#common-Metric) |  | SNR (as reported by the gateway(s)). |
| rx_packets_per_freq | [common.Metric](#common-Metric) |  | Packets received by frequency. |
| rx_packets_per_dr | [common.Metric](#common-Metric) |  | Packets received by DR. |
| errors | [common.Metric](#common-Metric) |  | Errors. |






<a name="api-GetDeviceMetricsRequest"></a>

### GetDeviceMetricsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |
| start | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Interval start timestamp. |
| end | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Interval end timestamp. |
| aggregation | [common.Aggregation](#common-Aggregation) |  | Aggregation. |






<a name="api-GetDeviceMetricsResponse"></a>

### GetDeviceMetricsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| metrics | [GetDeviceMetricsResponse.MetricsEntry](#api-GetDeviceMetricsResponse-MetricsEntry) | repeated |  |
| states | [GetDeviceMetricsResponse.StatesEntry](#api-GetDeviceMetricsResponse-StatesEntry) | repeated |  |






<a name="api-GetDeviceMetricsResponse-MetricsEntry"></a>

### GetDeviceMetricsResponse.MetricsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [common.Metric](#common-Metric) |  |  |






<a name="api-GetDeviceMetricsResponse-StatesEntry"></a>

### GetDeviceMetricsResponse.StatesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [DeviceState](#api-DeviceState) |  |  |






<a name="api-GetDeviceNextFCntDownRequest"></a>

### GetDeviceNextFCntDownRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | Device EUI (EUI64). |






<a name="api-GetDeviceNextFCntDownResponse"></a>

### GetDeviceNextFCntDownResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| f_cnt_down | [uint32](#uint32) |  | FCntDown. |






<a name="api-GetDeviceQueueItemsRequest"></a>

### GetDeviceQueueItemsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | Device EUI (EUI64). |
| count_only | [bool](#bool) |  | Return only the count, not the result-set. |






<a name="api-GetDeviceQueueItemsResponse"></a>

### GetDeviceQueueItemsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of queue items. |
| result | [DeviceQueueItem](#api-DeviceQueueItem) | repeated | Result-set. |






<a name="api-GetDeviceRequest"></a>

### GetDeviceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |






<a name="api-GetDeviceResponse"></a>

### GetDeviceResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device | [Device](#api-Device) |  | Device object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| last_seen_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last seen at timestamp. |
| device_status | [DeviceStatus](#api-DeviceStatus) |  | Device status. |
| class_enabled | [common.DeviceClass](#common-DeviceClass) |  | Enabled device class. |






<a name="api-GetRandomDevAddrRequest"></a>

### GetRandomDevAddrRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | DevEUI (EUI64). |






<a name="api-GetRandomDevAddrResponse"></a>

### GetRandomDevAddrResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_addr | [string](#string) |  | DevAddr. |






<a name="api-ListDevicesRequest"></a>

### ListDevicesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  | Max number of devices to return in the result-set. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |
| search | [string](#string) |  | If set, the given string will be used to search on name (optional). |
| application_id | [string](#string) |  | Application ID (UUID) to filter devices on. |
| multicast_group_id | [string](#string) |  | Multicst-group ID (UUID) to filter devices on. |






<a name="api-ListDevicesResponse"></a>

### ListDevicesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of devices. |
| result | [DeviceListItem](#api-DeviceListItem) | repeated | Result-set. |






<a name="api-UpdateDeviceKeysRequest"></a>

### UpdateDeviceKeysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_keys | [DeviceKeys](#api-DeviceKeys) |  | Device-keys object. |






<a name="api-UpdateDeviceRequest"></a>

### UpdateDeviceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device | [Device](#api-Device) |  | Device object. |





 

 

 


<a name="api-DeviceService"></a>

### DeviceService
DeviceService is the service providing API methods for managing devices.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateDeviceRequest](#api-CreateDeviceRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create the given device. |
| Get | [GetDeviceRequest](#api-GetDeviceRequest) | [GetDeviceResponse](#api-GetDeviceResponse) | Get returns the device for the given DevEUI. |
| Update | [UpdateDeviceRequest](#api-UpdateDeviceRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the given device. |
| Delete | [DeleteDeviceRequest](#api-DeleteDeviceRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the device with the given DevEUI. |
| List | [ListDevicesRequest](#api-ListDevicesRequest) | [ListDevicesResponse](#api-ListDevicesResponse) | Get the list of devices. |
| CreateKeys | [CreateDeviceKeysRequest](#api-CreateDeviceKeysRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create the given device-keys. |
| GetKeys | [GetDeviceKeysRequest](#api-GetDeviceKeysRequest) | [GetDeviceKeysResponse](#api-GetDeviceKeysResponse) | Get the device-keys for the given DevEUI. |
| UpdateKeys | [UpdateDeviceKeysRequest](#api-UpdateDeviceKeysRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the given device-keys. |
| DeleteKeys | [DeleteDeviceKeysRequest](#api-DeleteDeviceKeysRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the device-keys for the given DevEUI. |
| FlushDevNonces | [FlushDevNoncesRequest](#api-FlushDevNoncesRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | FlushDevNonces flushes the OTAA device nonces. |
| Activate | [ActivateDeviceRequest](#api-ActivateDeviceRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Activate (re)activates the device with the given parameters (for ABP or for importing OTAA activations). |
| Deactivate | [DeactivateDeviceRequest](#api-DeactivateDeviceRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Deactivate de-activates the device. |
| GetActivation | [GetDeviceActivationRequest](#api-GetDeviceActivationRequest) | [GetDeviceActivationResponse](#api-GetDeviceActivationResponse) | GetActivation returns the current activation details of the device (OTAA or ABP). |
| GetRandomDevAddr | [GetRandomDevAddrRequest](#api-GetRandomDevAddrRequest) | [GetRandomDevAddrResponse](#api-GetRandomDevAddrResponse) | GetRandomDevAddr returns a random DevAddr taking the NwkID prefix into account. |
| GetMetrics | [GetDeviceMetricsRequest](#api-GetDeviceMetricsRequest) | [GetDeviceMetricsResponse](#api-GetDeviceMetricsResponse) | GetMetrics returns the device metrics. Note that this requires a device-profile with codec and measurements configured. |
| GetLinkMetrics | [GetDeviceLinkMetricsRequest](#api-GetDeviceLinkMetricsRequest) | [GetDeviceLinkMetricsResponse](#api-GetDeviceLinkMetricsResponse) | GetLinkMetrics returns the device link metrics. This includes uplinks, downlinks, RSSI, SNR, etc... |
| Enqueue | [EnqueueDeviceQueueItemRequest](#api-EnqueueDeviceQueueItemRequest) | [EnqueueDeviceQueueItemResponse](#api-EnqueueDeviceQueueItemResponse) | Enqueue adds the given item to the downlink queue. |
| FlushQueue | [FlushDeviceQueueRequest](#api-FlushDeviceQueueRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | FlushQueue flushes the downlink device-queue. |
| GetQueue | [GetDeviceQueueItemsRequest](#api-GetDeviceQueueItemsRequest) | [GetDeviceQueueItemsResponse](#api-GetDeviceQueueItemsResponse) | GetQueue returns the downlink device-queue. |
| GetNextFCntDown | [GetDeviceNextFCntDownRequest](#api-GetDeviceNextFCntDownRequest) | [GetDeviceNextFCntDownResponse](#api-GetDeviceNextFCntDownResponse) | GetNextFCntDown returns the next FCntDown to use for enqueing encrypted downlinks. The difference with the DeviceActivation f_cont_down is that this method takes potential existing queue-items into account. |

 



<a name="api_device_profile-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/device_profile.proto



<a name="api-AdrAlgorithmListItem"></a>

### AdrAlgorithmListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Algorithm ID. |
| name | [string](#string) |  | Algorithm name. |






<a name="api-CreateDeviceProfileRequest"></a>

### CreateDeviceProfileRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_profile | [DeviceProfile](#api-DeviceProfile) |  | Object to create. |






<a name="api-CreateDeviceProfileResponse"></a>

### CreateDeviceProfileResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID (UUID). |






<a name="api-DeleteDeviceProfileRequest"></a>

### DeleteDeviceProfileRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID (UUID). |






<a name="api-DeviceProfile"></a>

### DeviceProfile



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Device-profile ID (UUID). Note: on create this will be automatically generated. |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |
| name | [string](#string) |  | Name. |
| description | [string](#string) |  | Description. |
| region | [common.Region](#common-Region) |  | Region. |
| mac_version | [common.MacVersion](#common-MacVersion) |  | LoRaWAN mac-version. |
| reg_params_revision | [common.RegParamsRevision](#common-RegParamsRevision) |  | Regional parameters revision. |
| adr_algorithm_id | [string](#string) |  | ADR algorithm ID. |
| payload_codec_runtime | [CodecRuntime](#api-CodecRuntime) |  | Payload codec runtime. |
| payload_codec_script | [string](#string) |  | Payload codec script. |
| flush_queue_on_activate | [bool](#bool) |  | Flush queue on device activation. |
| uplink_interval | [uint32](#uint32) |  | Uplink interval (seconds). This defines the expected uplink interval which the device uses for communication. If the uplink interval has expired and no uplink has been received, the device is considered inactive. |
| device_status_req_interval | [uint32](#uint32) |  | Device-status request interval (times / day). This defines the times per day that ChirpStack will request the device-status from the device. |
| supports_otaa | [bool](#bool) |  | Supports OTAA. |
| supports_class_b | [bool](#bool) |  | Supports Class B. |
| supports_class_c | [bool](#bool) |  | Supports Class-C. |
| class_b_timeout | [uint32](#uint32) |  | Class-B timeout (seconds). This is the maximum time ChirpStack will wait to receive an acknowledgement from the device (if requested). |
| class_b_ping_slot_nb_k | [uint32](#uint32) |  | Class-B ping-slots per beacon period. Valid options are: 0 - 7.

The actual number of ping-slots per beacon period equals to 2^k. |
| class_b_ping_slot_dr | [uint32](#uint32) |  | Class-B ping-slot DR. |
| class_b_ping_slot_freq | [uint32](#uint32) |  | Class-B ping-slot freq (Hz). |
| class_c_timeout | [uint32](#uint32) |  | Class-C timeout (seconds). This is the maximum time ChirpStack will wait to receive an acknowledgement from the device (if requested). |
| abp_rx1_delay | [uint32](#uint32) |  | RX1 delay (for ABP). |
| abp_rx1_dr_offset | [uint32](#uint32) |  | RX1 DR offset (for ABP). |
| abp_rx2_dr | [uint32](#uint32) |  | RX2 DR (for ABP). |
| abp_rx2_freq | [uint32](#uint32) |  | RX2 frequency (for ABP, Hz). |
| tags | [DeviceProfile.TagsEntry](#api-DeviceProfile-TagsEntry) | repeated | User defined tags. |
| measurements | [DeviceProfile.MeasurementsEntry](#api-DeviceProfile-MeasurementsEntry) | repeated | Measurements. If defined, ChirpStack will visualize these metrics in the web-interface. |
| auto_detect_measurements | [bool](#bool) |  | Auto-detect measurements. If set to true, measurements will be automatically added based on the keys of the decoded payload. In cases where the decoded payload contains random keys in the data, you want to set this to false. |
| region_config_id | [string](#string) |  | Region configuration ID. If set, devices will only use the associated region. If let blank, then devices will use all regions matching the selected common-name. Note that multiple region configurations can exist for the same common-name, e.g. to provide an 8 channel and 16 channel configuration for the US915 band. |
| is_relay | [bool](#bool) |  | Device is a Relay device. Enable this in case the device is a Relay. A Relay device implements TS011 and is able to relay data from relay capable devices. See for more information the TS011 specification. |
| is_relay_ed | [bool](#bool) |  | Device is a Relay end-device. Enable this in case the device is an end-device that can operate under a Relay. Please refer to the TS011 specification for more information. |
| relay_ed_relay_only | [bool](#bool) |  | End-device only accept data through relay. Only accept data for this device through a relay. This setting is useful for testing as in case of a test-setup, the end-device is usually within range of the gateway. |
| relay_enabled | [bool](#bool) |  | Relay must be enabled. |
| relay_cad_periodicity | [CadPeriodicity](#api-CadPeriodicity) |  | Relay CAD periodicity. |
| relay_default_channel_index | [uint32](#uint32) |  | Relay default channel index. Valid values are 0 and 1, please refer to the RP002 specification for the meaning of these values. |
| relay_second_channel_freq | [uint32](#uint32) |  | Relay second channel frequency (Hz). |
| relay_second_channel_dr | [uint32](#uint32) |  | Relay second channel DR. |
| relay_second_channel_ack_offset | [SecondChAckOffset](#api-SecondChAckOffset) |  | Relay second channel ACK offset. |
| relay_ed_activation_mode | [RelayModeActivation](#api-RelayModeActivation) |  | Relay end-device activation mode. |
| relay_ed_smart_enable_level | [uint32](#uint32) |  | Relay end-device smart-enable level. |
| relay_ed_back_off | [uint32](#uint32) |  | Relay end-device back-off (in case it does not receive WOR ACK frame). 0 = Always send a LoRaWAN uplink 1..63 = Send a LoRaWAN uplink after X WOR frames without a WOR ACK |
| relay_ed_uplink_limit_bucket_size | [uint32](#uint32) |  | Relay end-device uplink limit bucket size.

This field indicates the multiplier to determine the bucket size according to the following formula: BucketSize TOKEN = _reload_rate x _bucket_size

Valid values (0 - 3): 0 = 1 1 = 2 2 = 4 3 = 12 |
| relay_ed_uplink_limit_reload_rate | [uint32](#uint32) |  | Relay end-device uplink limit reload rate.

Valid values: * 0 - 62 = X tokens every hour * 63 = no limitation |
| relay_join_req_limit_reload_rate | [uint32](#uint32) |  | Relay join-request limit reload rate.

Valid values: * 0 - 126 = X tokens every hour * 127 = no limitation |
| relay_notify_limit_reload_rate | [uint32](#uint32) |  | Relay notify limit reload rate.

Valid values: * 0 - 126 = X tokens every hour * 127 = no limitation |
| relay_global_uplink_limit_reload_rate | [uint32](#uint32) |  | Relay global uplink limit reload rate.

Valid values: * 0 - 126 = X tokens every hour * 127 = no limitation |
| relay_overall_limit_reload_rate | [uint32](#uint32) |  | Relay overall limit reload rate.

Valid values: * 0 - 126 = X tokens every hour * 127 = no limitation |
| relay_join_req_limit_bucket_size | [uint32](#uint32) |  | Relay join-request limit bucket size.

This field indicates the multiplier to determine the bucket size according to the following formula: BucketSize TOKEN = _reload_rate x _bucket_size

Valid values (0 - 3): 0 = 1 1 = 2 2 = 4 3 = 12 |
| relay_notify_limit_bucket_size | [uint32](#uint32) |  | Relay notify limit bucket size.

This field indicates the multiplier to determine the bucket size according to the following formula: BucketSize TOKEN = _reload_rate x _bucket_size

Valid values (0 - 3): 0 = 1 1 = 2 2 = 4 3 = 12 |
| relay_global_uplink_limit_bucket_size | [uint32](#uint32) |  | Relay globak uplink limit bucket size.

This field indicates the multiplier to determine the bucket size according to the following formula: BucketSize TOKEN = _reload_rate x _bucket_size

Valid values (0 - 3): 0 = 1 1 = 2 2 = 4 3 = 12 |
| relay_overall_limit_bucket_size | [uint32](#uint32) |  | Relay overall limit bucket size.

This field indicates the multiplier to determine the bucket size according to the following formula: BucketSize TOKEN = _reload_rate x _bucket_size

Valid values (0 - 3): 0 = 1 1 = 2 2 = 4 3 = 12 |






<a name="api-DeviceProfile-MeasurementsEntry"></a>

### DeviceProfile.MeasurementsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [Measurement](#api-Measurement) |  |  |






<a name="api-DeviceProfile-TagsEntry"></a>

### DeviceProfile.TagsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="api-DeviceProfileListItem"></a>

### DeviceProfileListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Device-profile ID (UUID). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| name | [string](#string) |  | Name. |
| region | [common.Region](#common-Region) |  | Region. |
| mac_version | [common.MacVersion](#common-MacVersion) |  | LoRaWAN mac-version. |
| reg_params_revision | [common.RegParamsRevision](#common-RegParamsRevision) |  | Regional parameters revision. |
| supports_otaa | [bool](#bool) |  | Supports OTAA. |
| supports_class_b | [bool](#bool) |  | Supports Class-B. |
| supports_class_c | [bool](#bool) |  | Supports Class-C. |






<a name="api-GetDeviceProfileRequest"></a>

### GetDeviceProfileRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID (UUID). |






<a name="api-GetDeviceProfileResponse"></a>

### GetDeviceProfileResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_profile | [DeviceProfile](#api-DeviceProfile) |  | Device-profile object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |






<a name="api-ListDeviceProfileAdrAlgorithmsResponse"></a>

### ListDeviceProfileAdrAlgorithmsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of algorithms. |
| result | [AdrAlgorithmListItem](#api-AdrAlgorithmListItem) | repeated | Result-set. |






<a name="api-ListDeviceProfilesRequest"></a>

### ListDeviceProfilesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  | Max number of device-profiles to return in the result-set. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |
| search | [string](#string) |  | If set, the given string will be used to search on name. |
| tenant_id | [string](#string) |  | Tenant ID to list the device-profiles for. |






<a name="api-ListDeviceProfilesResponse"></a>

### ListDeviceProfilesResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of device-profiles. |
| result | [DeviceProfileListItem](#api-DeviceProfileListItem) | repeated | Result-set. |






<a name="api-Measurement"></a>

### Measurement



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  | Name (user defined). |
| kind | [MeasurementKind](#api-MeasurementKind) |  | Kind. |






<a name="api-UpdateDeviceProfileRequest"></a>

### UpdateDeviceProfileRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| device_profile | [DeviceProfile](#api-DeviceProfile) |  | Device-profile object. |





 


<a name="api-CadPeriodicity"></a>

### CadPeriodicity


| Name | Number | Description |
| ---- | ------ | ----------- |
| SEC_1 | 0 | 1 second. |
| MS_500 | 1 | 500 milliseconds |
| MS_250 | 2 | 250 milliseconds |
| MS_100 | 3 | 100 milliseconds |
| MS_50 | 4 | 50 milliseconds |
| MS_20 | 5 | 20 milliseconds |



<a name="api-CodecRuntime"></a>

### CodecRuntime


| Name | Number | Description |
| ---- | ------ | ----------- |
| NONE | 0 | None. |
| CAYENNE_LPP | 1 | Cayenne LPP. |
| JS | 2 | JavaScript. |



<a name="api-MeasurementKind"></a>

### MeasurementKind


| Name | Number | Description |
| ---- | ------ | ----------- |
| UNKNOWN | 0 | Unknown (in which case it is not tracked). |
| COUNTER | 1 | Incrementing counters that never decrease (these are not reset on each reading). |
| ABSOLUTE | 2 | Counters that do get reset upon reading. |
| GAUGE | 3 | E.g. a temperature value. |
| STRING | 4 | E.g. a firmware version, true / false value. |



<a name="api-RelayModeActivation"></a>

### RelayModeActivation


| Name | Number | Description |
| ---- | ------ | ----------- |
| DISABLE_RELAY_MODE | 0 | Disable the relay mode. |
| ENABLE_RELAY_MODE | 1 | Enable the relay model. |
| DYNAMIC | 2 | Dynamic. |
| END_DEVICE_CONTROLLED | 3 | End-device controlled. |



<a name="api-SecondChAckOffset"></a>

### SecondChAckOffset


| Name | Number | Description |
| ---- | ------ | ----------- |
| KHZ_0 | 0 | 0 kHz. |
| KHZ_200 | 1 | 200 kHz. |
| KHZ_400 | 2 | 400 kHz. |
| KHZ_800 | 3 | 800 kHz. |
| KHZ_1600 | 4 | 1600 kHz. |
| KHZ_3200 | 5 | 3200 kHz. |


 

 


<a name="api-DeviceProfileService"></a>

### DeviceProfileService
DeviceProfileService is the service providing API methods for managing device-profiles.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateDeviceProfileRequest](#api-CreateDeviceProfileRequest) | [CreateDeviceProfileResponse](#api-CreateDeviceProfileResponse) | Create the given device-profile. |
| Get | [GetDeviceProfileRequest](#api-GetDeviceProfileRequest) | [GetDeviceProfileResponse](#api-GetDeviceProfileResponse) | Get the device-profile for the given ID. |
| Update | [UpdateDeviceProfileRequest](#api-UpdateDeviceProfileRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the given device-profile. |
| Delete | [DeleteDeviceProfileRequest](#api-DeleteDeviceProfileRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the device-profile with the given ID. |
| List | [ListDeviceProfilesRequest](#api-ListDeviceProfilesRequest) | [ListDeviceProfilesResponse](#api-ListDeviceProfilesResponse) | List the available device-profiles. |
| ListAdrAlgorithms | [.google.protobuf.Empty](#google-protobuf-Empty) | [ListDeviceProfileAdrAlgorithmsResponse](#api-ListDeviceProfileAdrAlgorithmsResponse) | List available ADR algorithms. |

 



<a name="api_frame_log-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/frame_log.proto



<a name="api-DownlinkFrameLog"></a>

### DownlinkFrameLog



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Time. |
| phy_payload | [bytes](#bytes) |  | PHYPayload. |
| tx_info | [gw.DownlinkTxInfo](#gw-DownlinkTxInfo) |  | TX meta-data. |
| downlink_id | [uint32](#uint32) |  | Downlink ID. |
| gateway_id | [string](#string) |  | Gateway ID (EUI64). |
| m_type | [common.MType](#common-MType) |  | Message type. |
| dev_addr | [string](#string) |  | Device address (optional). |
| dev_eui | [string](#string) |  | Device EUI (optional). |
| plaintext_f_opts | [bool](#bool) |  | Plaintext f_opts mac-commands. |
| plaintext_frm_payload | [bool](#bool) |  | Plaintext frm_payload. |






<a name="api-UplinkFrameLog"></a>

### UplinkFrameLog



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| phy_payload | [bytes](#bytes) |  | PHYPayload. |
| tx_info | [gw.UplinkTxInfo](#gw-UplinkTxInfo) |  | TX meta-data. |
| rx_info | [gw.UplinkRxInfo](#gw-UplinkRxInfo) | repeated | RX meta-data. |
| m_type | [common.MType](#common-MType) |  | Message type. |
| dev_addr | [string](#string) |  | Device address (optional). |
| dev_eui | [string](#string) |  | Device EUI (optional). |
| time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Time. |
| plaintext_f_opts | [bool](#bool) |  | Plaintext f_opts mac-commands. |
| plaintext_frm_payload | [bool](#bool) |  | Plaintext frm_payload. |





 

 

 

 



<a name="api_gateway-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/gateway.proto



<a name="api-CreateGatewayRequest"></a>

### CreateGatewayRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway | [Gateway](#api-Gateway) |  | Gateway object. |






<a name="api-DeleteGatewayRequest"></a>

### DeleteGatewayRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway_id | [string](#string) |  | Gateway ID (EUI64). |






<a name="api-Gateway"></a>

### Gateway



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway_id | [string](#string) |  | Gateway ID (EUI64). |
| name | [string](#string) |  | Name. |
| description | [string](#string) |  | Description. |
| location | [common.Location](#common-Location) |  | Gateway location. |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |
| tags | [Gateway.TagsEntry](#api-Gateway-TagsEntry) | repeated | Tags. |
| metadata | [Gateway.MetadataEntry](#api-Gateway-MetadataEntry) | repeated | Metadata (provided by the gateway). |
| stats_interval | [uint32](#uint32) |  | Stats interval (seconds). This defines the expected interval in which the gateway sends its statistics. |






<a name="api-Gateway-MetadataEntry"></a>

### Gateway.MetadataEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="api-Gateway-TagsEntry"></a>

### Gateway.TagsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="api-GatewayListItem"></a>

### GatewayListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID. |
| gateway_id | [string](#string) |  | Gateway ID (EUI64). |
| name | [string](#string) |  | Name. |
| description | [string](#string) |  | Description. |
| location | [common.Location](#common-Location) |  | Location. |
| properties | [GatewayListItem.PropertiesEntry](#api-GatewayListItem-PropertiesEntry) | repeated | Gateway properties. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| last_seen_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last seen at timestamp. |
| state | [GatewayState](#api-GatewayState) |  | Gateway state. Please note that the state of the gateway is driven by the stats packages that are sent by the gateway. |






<a name="api-GatewayListItem-PropertiesEntry"></a>

### GatewayListItem.PropertiesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="api-GenerateGatewayClientCertificateRequest"></a>

### GenerateGatewayClientCertificateRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway_id | [string](#string) |  | Gateway ID (EUI64). |






<a name="api-GenerateGatewayClientCertificateResponse"></a>

### GenerateGatewayClientCertificateResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tls_cert | [string](#string) |  | TLS certificate. |
| tls_key | [string](#string) |  | TLS key. |
| ca_cert | [string](#string) |  | CA certificate. |
| expires_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Expires at defines the expiration date of the certificate. |






<a name="api-GetGatewayMetricsRequest"></a>

### GetGatewayMetricsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway_id | [string](#string) |  | Gateway ID (EUI64). |
| start | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Interval start timestamp. |
| end | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Interval end timestamp. |
| aggregation | [common.Aggregation](#common-Aggregation) |  | Aggregation. |






<a name="api-GetGatewayMetricsResponse"></a>

### GetGatewayMetricsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| rx_packets | [common.Metric](#common-Metric) |  | RX packets. |
| tx_packets | [common.Metric](#common-Metric) |  | TX packets. |
| tx_packets_per_freq | [common.Metric](#common-Metric) |  | TX packets / frequency. |
| rx_packets_per_freq | [common.Metric](#common-Metric) |  | RX packets / frequency. |
| tx_packets_per_dr | [common.Metric](#common-Metric) |  | TX packets / DR. |
| rx_packets_per_dr | [common.Metric](#common-Metric) |  | RX packets / DR. |
| tx_packets_per_status | [common.Metric](#common-Metric) |  | TX packets per status. |






<a name="api-GetGatewayRequest"></a>

### GetGatewayRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway_id | [string](#string) |  | Gateway ID (EUI64). |






<a name="api-GetGatewayResponse"></a>

### GetGatewayResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway | [Gateway](#api-Gateway) |  | Gateway object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| last_seen_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last seen at timestamp. |






<a name="api-ListGatewaysRequest"></a>

### ListGatewaysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  | Max number of gateways to return in the result-set. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |
| search | [string](#string) |  | If set, the given string will be used to search on name (optional). |
| tenant_id | [string](#string) |  | Tenant ID (UUID) to filter gateways on. To list all gateways as a global admin user, this field can be left blank. |
| multicast_group_id | [string](#string) |  | Multicast-group ID (UUID) to filter gateways on. |






<a name="api-ListGatewaysResponse"></a>

### ListGatewaysResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of gateways. |
| result | [GatewayListItem](#api-GatewayListItem) | repeated | Result-set. |






<a name="api-UpdateGatewayRequest"></a>

### UpdateGatewayRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway | [Gateway](#api-Gateway) |  | Gateway object. |





 


<a name="api-GatewayState"></a>

### GatewayState


| Name | Number | Description |
| ---- | ------ | ----------- |
| NEVER_SEEN | 0 | The gateway has never sent any stats. |
| ONLINE | 1 | Online. |
| OFFLINE | 2 | Offline. |


 

 


<a name="api-GatewayService"></a>

### GatewayService
GatewayService is the service providing API methods for managing gateways.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateGatewayRequest](#api-CreateGatewayRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Create creates the given gateway. |
| Get | [GetGatewayRequest](#api-GetGatewayRequest) | [GetGatewayResponse](#api-GetGatewayResponse) | Get returns the gateway for the given Gateway ID. |
| Update | [UpdateGatewayRequest](#api-UpdateGatewayRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update updates the given gateway. |
| Delete | [DeleteGatewayRequest](#api-DeleteGatewayRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete deletes the gateway matching the given Gateway ID. |
| List | [ListGatewaysRequest](#api-ListGatewaysRequest) | [ListGatewaysResponse](#api-ListGatewaysResponse) | Get the list of gateways. |
| GenerateClientCertificate | [GenerateGatewayClientCertificateRequest](#api-GenerateGatewayClientCertificateRequest) | [GenerateGatewayClientCertificateResponse](#api-GenerateGatewayClientCertificateResponse) | Generate client-certificate for the gateway. |
| GetMetrics | [GetGatewayMetricsRequest](#api-GetGatewayMetricsRequest) | [GetGatewayMetricsResponse](#api-GetGatewayMetricsResponse) | GetMetrics returns the gateway metrics. |

 



<a name="api_multicast_group-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/multicast_group.proto



<a name="api-AddDeviceToMulticastGroupRequest"></a>

### AddDeviceToMulticastGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group_id | [string](#string) |  | Multicast group ID. |
| dev_eui | [string](#string) |  | Device EUI (HEX encoded). |






<a name="api-AddGatewayToMulticastGroupRequest"></a>

### AddGatewayToMulticastGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group_id | [string](#string) |  | Multicast group ID. |
| gateway_id | [string](#string) |  | Gateway ID (HEX encoded). |






<a name="api-CreateMulticastGroupRequest"></a>

### CreateMulticastGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group | [MulticastGroup](#api-MulticastGroup) |  | Multicast group to create. |






<a name="api-CreateMulticastGroupResponse"></a>

### CreateMulticastGroupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID of created multicast group (UUID). |






<a name="api-DeleteMulticastGroupRequest"></a>

### DeleteMulticastGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Multicast group iD. |






<a name="api-EnqueueMulticastGroupQueueItemRequest"></a>

### EnqueueMulticastGroupQueueItemRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| queue_item | [MulticastGroupQueueItem](#api-MulticastGroupQueueItem) |  | Multicast queue-item to enqueue. |






<a name="api-EnqueueMulticastGroupQueueItemResponse"></a>

### EnqueueMulticastGroupQueueItemResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| f_cnt | [uint32](#uint32) |  | Frame-counter of the enqueued payload. |






<a name="api-FlushMulticastGroupQueueRequest"></a>

### FlushMulticastGroupQueueRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group_id | [string](#string) |  | Multicast group ID. |






<a name="api-GetMulticastGroupRequest"></a>

### GetMulticastGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Multicast group ID. |






<a name="api-GetMulticastGroupResponse"></a>

### GetMulticastGroupResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group | [MulticastGroup](#api-MulticastGroup) |  | Multicast group object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |






<a name="api-ListMulticastGroupQueueRequest"></a>

### ListMulticastGroupQueueRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group_id | [string](#string) |  | Multicast group ID. |






<a name="api-ListMulticastGroupQueueResponse"></a>

### ListMulticastGroupQueueResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| items | [MulticastGroupQueueItem](#api-MulticastGroupQueueItem) | repeated |  |






<a name="api-ListMulticastGroupsRequest"></a>

### ListMulticastGroupsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  | Max number of multicast groups to return in the result-set. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |
| search | [string](#string) |  | If set, the given string will be used to search on name. |
| application_id | [string](#string) |  | Application ID to list the multicast groups for. |






<a name="api-ListMulticastGroupsResponse"></a>

### ListMulticastGroupsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of multicast groups. |
| result | [MulticastGroupListItem](#api-MulticastGroupListItem) | repeated | Result-test. |






<a name="api-MulticastGroup"></a>

### MulticastGroup



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID (UUID). This will be generated automatically on create. |
| name | [string](#string) |  | Name. |
| application_id | [string](#string) |  | Application ID. After creation, this can not be updated. |
| region | [common.Region](#common-Region) |  | Region. |
| mc_addr | [string](#string) |  | Multicast address (HEX encoded DevAddr). |
| mc_nwk_s_key | [string](#string) |  | Multicast network session key (HEX encoded AES128 key). |
| mc_app_s_key | [string](#string) |  | Multicast application session key (HEX encoded AES128 key). |
| f_cnt | [uint32](#uint32) |  | Frame-counter. |
| group_type | [MulticastGroupType](#api-MulticastGroupType) |  | Multicast group type. |
| dr | [uint32](#uint32) |  | Data-rate. |
| frequency | [uint32](#uint32) |  | Frequency (Hz). |
| class_b_ping_slot_period | [uint32](#uint32) |  | Ping-slot period (only for Class-B). |
| class_c_scheduling_type | [MulticastGroupSchedulingType](#api-MulticastGroupSchedulingType) |  | Scheduling type (only for Class-C). |






<a name="api-MulticastGroupListItem"></a>

### MulticastGroupListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| name | [string](#string) |  | Name. |
| region | [common.Region](#common-Region) |  | Region. |
| group_type | [MulticastGroupType](#api-MulticastGroupType) |  | Multicast group type. |






<a name="api-MulticastGroupQueueItem"></a>

### MulticastGroupQueueItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group_id | [string](#string) |  | Multicast group ID. |
| f_cnt | [uint32](#uint32) |  | Downlink frame-counter. This will be automatically set on enqueue. |
| f_port | [uint32](#uint32) |  | FPort (must be &gt; 0). |
| data | [bytes](#bytes) |  | Payload. |






<a name="api-RemoveDeviceFromMulticastGroupRequest"></a>

### RemoveDeviceFromMulticastGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group_id | [string](#string) |  | Multicast group ID. |
| dev_eui | [string](#string) |  | Device EUI (HEX encoded). |






<a name="api-RemoveGatewayFromMulticastGroupRequest"></a>

### RemoveGatewayFromMulticastGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group_id | [string](#string) |  | Multicast group ID. |
| gateway_id | [string](#string) |  | Gateway ID (HEX encoded). |






<a name="api-UpdateMulticastGroupRequest"></a>

### UpdateMulticastGroupRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| multicast_group | [MulticastGroup](#api-MulticastGroup) |  | Multicast group object to update. |





 


<a name="api-MulticastGroupSchedulingType"></a>

### MulticastGroupSchedulingType


| Name | Number | Description |
| ---- | ------ | ----------- |
| DELAY | 0 | Delay. If multicast downlinks must be sent through multiple gateways, then these will be sent one by one with a delay between each gateway. |
| GPS_TIME | 1 | Time. If multicast downlinks must be sent through multiple gateways, then these will be sent simultaneously using GPS time synchronization. Note that this does require GPS time-synchronized LoRa gateways. |



<a name="api-MulticastGroupType"></a>

### MulticastGroupType


| Name | Number | Description |
| ---- | ------ | ----------- |
| CLASS_C | 0 | Class C. |
| CLASS_B | 1 | Class-B. |


 

 


<a name="api-MulticastGroupService"></a>

### MulticastGroupService
MulticastGroupService is the service managing multicast-groups.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateMulticastGroupRequest](#api-CreateMulticastGroupRequest) | [CreateMulticastGroupResponse](#api-CreateMulticastGroupResponse) | Create the given multicast group. |
| Get | [GetMulticastGroupRequest](#api-GetMulticastGroupRequest) | [GetMulticastGroupResponse](#api-GetMulticastGroupResponse) | Get returns the multicast group for the given ID. |
| Update | [UpdateMulticastGroupRequest](#api-UpdateMulticastGroupRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the given multicast group. |
| Delete | [DeleteMulticastGroupRequest](#api-DeleteMulticastGroupRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the multicast-group with the given ID. |
| List | [ListMulticastGroupsRequest](#api-ListMulticastGroupsRequest) | [ListMulticastGroupsResponse](#api-ListMulticastGroupsResponse) | List the available multicast groups. |
| AddDevice | [AddDeviceToMulticastGroupRequest](#api-AddDeviceToMulticastGroupRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Add a device to the multicast group. |
| RemoveDevice | [RemoveDeviceFromMulticastGroupRequest](#api-RemoveDeviceFromMulticastGroupRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Remove a device from the multicast group. |
| AddGateway | [AddGatewayToMulticastGroupRequest](#api-AddGatewayToMulticastGroupRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Add a gateway to the multicast group. |
| RemoveGateway | [RemoveGatewayFromMulticastGroupRequest](#api-RemoveGatewayFromMulticastGroupRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Remove a gateway from the multicast group. |
| Enqueue | [EnqueueMulticastGroupQueueItemRequest](#api-EnqueueMulticastGroupQueueItemRequest) | [EnqueueMulticastGroupQueueItemResponse](#api-EnqueueMulticastGroupQueueItemResponse) | Add the given item to the multicast group queue. |
| FlushQueue | [FlushMulticastGroupQueueRequest](#api-FlushMulticastGroupQueueRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Flush the queue for the given multicast group. |
| ListQueue | [ListMulticastGroupQueueRequest](#api-ListMulticastGroupQueueRequest) | [ListMulticastGroupQueueResponse](#api-ListMulticastGroupQueueResponse) | List the items in the multicast group queue. |

 



<a name="api_tenant-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/tenant.proto



<a name="api-AddTenantUserRequest"></a>

### AddTenantUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_user | [TenantUser](#api-TenantUser) |  | Tenant user object. |






<a name="api-CreateTenantRequest"></a>

### CreateTenantRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant | [Tenant](#api-Tenant) |  | Tenant object to create. |






<a name="api-CreateTenantResponse"></a>

### CreateTenantResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Tenant ID. |






<a name="api-DeleteTenantRequest"></a>

### DeleteTenantRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Tenant ID. |






<a name="api-DeleteTenantUserRequest"></a>

### DeleteTenantUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |
| user_id | [string](#string) |  | User ID (UUID). |






<a name="api-GetTenantRequest"></a>

### GetTenantRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Tenant ID. |






<a name="api-GetTenantResponse"></a>

### GetTenantResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant | [Tenant](#api-Tenant) |  | Tenant object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |






<a name="api-GetTenantUserRequest"></a>

### GetTenantUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |
| user_id | [string](#string) |  | User ID (UUID). |






<a name="api-GetTenantUserResponse"></a>

### GetTenantUserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_user | [TenantUser](#api-TenantUser) |  | Tenant user object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |






<a name="api-ListTenantUsersRequest"></a>

### ListTenantUsersRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |
| limit | [uint32](#uint32) |  | Max number of tenants to return in the result-set. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |






<a name="api-ListTenantUsersResponse"></a>

### ListTenantUsersResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of tenants. |
| result | [TenantUserListItem](#api-TenantUserListItem) | repeated | Result-set. |






<a name="api-ListTenantsRequest"></a>

### ListTenantsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  | Max number of tenants to return in the result-set. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |
| search | [string](#string) |  | If set, the given string will be used to search on name. |
| user_id | [string](#string) |  | If set, filters the result set to the tenants of the user. Only global API keys are able to filter by this field. |






<a name="api-ListTenantsResponse"></a>

### ListTenantsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of tenants. |
| result | [TenantListItem](#api-TenantListItem) | repeated | Result-set. |






<a name="api-Tenant"></a>

### Tenant



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Tenant ID (UUID). Note: this value will be automatically generated on create. |
| name | [string](#string) |  | Tenant name, |
| description | [string](#string) |  | Tenant description. |
| can_have_gateways | [bool](#bool) |  | Can the tenant create and &#34;own&#34; Gateways? |
| max_gateway_count | [uint32](#uint32) |  | Max. gateway count for tenant. When set to 0, the tenant can have unlimited gateways. |
| max_device_count | [uint32](#uint32) |  | Max. device count for tenant. When set to 0, the tenant can have unlimited devices. |
| private_gateways_up | [bool](#bool) |  | Private gateways (uplink). If enabled, then uplink messages will not be shared with other tenants. |
| private_gateways_down | [bool](#bool) |  | Private gateways (downlink). If enabled, then other tenants will not be able to schedule downlink messages through the gateways of this tenant. For example, in case you do want to share uplinks with other tenants (private_gateways_up=false), but you want to prevent other tenants from using gateway airtime. |






<a name="api-TenantListItem"></a>

### TenantListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | Tenant ID (UUID). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| name | [string](#string) |  | Tenant name. |
| can_have_gateways | [bool](#bool) |  | Can the tenant create and &#34;own&#34; Gateways? |
| private_gateways_up | [bool](#bool) |  | Private gateways (uplink). |
| private_gateways_down | [bool](#bool) |  | Private gateways (downlink). |
| max_gateway_count | [uint32](#uint32) |  | Max gateway count. 0 = unlimited. |
| max_device_count | [uint32](#uint32) |  | Max device count. 0 = unlimited. |






<a name="api-TenantUser"></a>

### TenantUser



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |
| user_id | [string](#string) |  | User ID (UUID). |
| is_admin | [bool](#bool) |  | User is admin within the context of the tenant. There is no need to set the is_device_admin and is_gateway_admin flags. |
| is_device_admin | [bool](#bool) |  | User is able to modify device related resources (applications, device-profiles, devices, multicast-groups). |
| is_gateway_admin | [bool](#bool) |  | User is able to modify gateways. |
| email | [string](#string) |  | Email (only used on get and when adding a user to a tenant). |






<a name="api-TenantUserListItem"></a>

### TenantUserListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |
| user_id | [string](#string) |  | User ID (UUID). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| email | [string](#string) |  | Email. |
| is_admin | [bool](#bool) |  | User is admin within the context of the tenant. There is no need to set the is_device_admin and is_gateway_admin flags. |
| is_device_admin | [bool](#bool) |  | User is able to modify device related resources (applications, device-profiles, devices, multicast-groups). |
| is_gateway_admin | [bool](#bool) |  | User is able to modify gateways. |






<a name="api-UpdateTenantRequest"></a>

### UpdateTenantRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant | [Tenant](#api-Tenant) |  | Tenant object. |






<a name="api-UpdateTenantUserRequest"></a>

### UpdateTenantUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_user | [TenantUser](#api-TenantUser) |  | Tenant user object. |





 

 

 


<a name="api-TenantService"></a>

### TenantService
TenantService is the service providing API methods for managing tenants.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateTenantRequest](#api-CreateTenantRequest) | [CreateTenantResponse](#api-CreateTenantResponse) | Create a new tenant. |
| Get | [GetTenantRequest](#api-GetTenantRequest) | [GetTenantResponse](#api-GetTenantResponse) | Get the tenant for the given ID. |
| Update | [UpdateTenantRequest](#api-UpdateTenantRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the given tenant. |
| Delete | [DeleteTenantRequest](#api-DeleteTenantRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the tenant with the given ID. |
| List | [ListTenantsRequest](#api-ListTenantsRequest) | [ListTenantsResponse](#api-ListTenantsResponse) | Get the list of tenants. |
| AddUser | [AddTenantUserRequest](#api-AddTenantUserRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Add an user to the tenant. Note: the user must already exist. |
| GetUser | [GetTenantUserRequest](#api-GetTenantUserRequest) | [GetTenantUserResponse](#api-GetTenantUserResponse) | Get the the tenant user for the given tenant and user IDs. |
| UpdateUser | [UpdateTenantUserRequest](#api-UpdateTenantUserRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the given tenant user. |
| DeleteUser | [DeleteTenantUserRequest](#api-DeleteTenantUserRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the given tenant user. |
| ListUsers | [ListTenantUsersRequest](#api-ListTenantUsersRequest) | [ListTenantUsersResponse](#api-ListTenantUsersResponse) | Get the list of tenant users. |

 



<a name="api_user-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/user.proto



<a name="api-CreateUserRequest"></a>

### CreateUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#api-User) |  | User object to create. |
| password | [string](#string) |  | Password to set for the user. |
| tenants | [UserTenant](#api-UserTenant) | repeated | Add the user to the following tenants. |






<a name="api-CreateUserResponse"></a>

### CreateUserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | User ID. |






<a name="api-DeleteUserRequest"></a>

### DeleteUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | User ID. |






<a name="api-GetUserRequest"></a>

### GetUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | User ID. |






<a name="api-GetUserResponse"></a>

### GetUserResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#api-User) |  | User object. |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |






<a name="api-ListUsersRequest"></a>

### ListUsersRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  | Max number of tenants to return in the result-set. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |






<a name="api-ListUsersResponse"></a>

### ListUsersResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of users. |
| result | [UserListItem](#api-UserListItem) | repeated | Result-set. |






<a name="api-UpdateUserPasswordRequest"></a>

### UpdateUserPasswordRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user_id | [string](#string) |  | User ID. |
| password | [string](#string) |  | Password to set. |






<a name="api-UpdateUserRequest"></a>

### UpdateUserRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#api-User) |  | User object. |






<a name="api-User"></a>

### User



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | User ID (UUID). Will be set automatically on create. |
| is_admin | [bool](#bool) |  | Set to true to make the user a global administrator. |
| is_active | [bool](#bool) |  | Set to false to disable the user. |
| email | [string](#string) |  | E-mail of the user. |
| note | [string](#string) |  | Optional note to store with the user. |






<a name="api-UserListItem"></a>

### UserListItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | User ID (UUID). |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| email | [string](#string) |  | Email of the user. |
| is_admin | [bool](#bool) |  | Set to true to make the user a global administrator. |
| is_active | [bool](#bool) |  | Set to false to disable the user. |






<a name="api-UserTenant"></a>

### UserTenant



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID. |
| is_admin | [bool](#bool) |  | User is admin within the context of the tenant. There is no need to set the is_device_admin and is_gateway_admin flags. |
| is_device_admin | [bool](#bool) |  | User is able to modify device related resources (applications, device-profiles, devices, multicast-groups). |
| is_gateway_admin | [bool](#bool) |  | User is able to modify gateways. |





 

 

 


<a name="api-UserService"></a>

### UserService
UserService is the service providing API methods for managing users.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Create | [CreateUserRequest](#api-CreateUserRequest) | [CreateUserResponse](#api-CreateUserResponse) | Create a new user. |
| Get | [GetUserRequest](#api-GetUserRequest) | [GetUserResponse](#api-GetUserResponse) | Get the user for the given ID. |
| Update | [UpdateUserRequest](#api-UpdateUserRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the given user. |
| Delete | [DeleteUserRequest](#api-DeleteUserRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Delete the user with the given ID. |
| List | [ListUsersRequest](#api-ListUsersRequest) | [ListUsersResponse](#api-ListUsersResponse) | Get the list of users. |
| UpdatePassword | [UpdateUserPasswordRequest](#api-UpdateUserPasswordRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | Update the password for the given user. |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

