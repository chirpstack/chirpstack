# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [api/internal.proto](#api_internal-proto)
    - [ApiKey](#api-ApiKey)
    - [CreateApiKeyRequest](#api-CreateApiKeyRequest)
    - [CreateApiKeyResponse](#api-CreateApiKeyResponse)
    - [DeleteApiKeyRequest](#api-DeleteApiKeyRequest)
    - [GetDevicesSummaryRequest](#api-GetDevicesSummaryRequest)
    - [GetDevicesSummaryResponse](#api-GetDevicesSummaryResponse)
    - [GetDevicesSummaryResponse.DrCountEntry](#api-GetDevicesSummaryResponse-DrCountEntry)
    - [GetGatewaysSummaryRequest](#api-GetGatewaysSummaryRequest)
    - [GetGatewaysSummaryResponse](#api-GetGatewaysSummaryResponse)
    - [GlobalSearchRequest](#api-GlobalSearchRequest)
    - [GlobalSearchResponse](#api-GlobalSearchResponse)
    - [GlobalSearchResult](#api-GlobalSearchResult)
    - [ListApiKeysRequest](#api-ListApiKeysRequest)
    - [ListApiKeysResponse](#api-ListApiKeysResponse)
    - [LogItem](#api-LogItem)
    - [LogItem.PropertiesEntry](#api-LogItem-PropertiesEntry)
    - [LoginRequest](#api-LoginRequest)
    - [LoginResponse](#api-LoginResponse)
    - [OpenIdConnect](#api-OpenIdConnect)
    - [OpenIdConnectLoginRequest](#api-OpenIdConnectLoginRequest)
    - [OpenIdConnectLoginResponse](#api-OpenIdConnectLoginResponse)
    - [ProfileResponse](#api-ProfileResponse)
    - [SettingsResponse](#api-SettingsResponse)
    - [StreamDeviceEventsRequest](#api-StreamDeviceEventsRequest)
    - [StreamDeviceFramesRequest](#api-StreamDeviceFramesRequest)
    - [StreamGatewayFramesRequest](#api-StreamGatewayFramesRequest)
    - [UserTenantLink](#api-UserTenantLink)
  
    - [InternalService](#api-InternalService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="api_internal-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## api/internal.proto



<a name="api-ApiKey"></a>

### ApiKey



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | API key ID. This value will be automatically generated on create. |
| name | [string](#string) |  | Name. |
| is_admin | [bool](#bool) |  | Is global admin key. |
| tenant_id | [string](#string) |  | Tenant ID. In case the API key is intended to manage resources under a single tenant. |






<a name="api-CreateApiKeyRequest"></a>

### CreateApiKeyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| api_key | [ApiKey](#api-ApiKey) |  | The API key to create. |






<a name="api-CreateApiKeyResponse"></a>

### CreateApiKeyResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | API key ID. |
| token | [string](#string) |  | API token for authentication API requests. |






<a name="api-DeleteApiKeyRequest"></a>

### DeleteApiKeyRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | API key ID. |






<a name="api-GetDevicesSummaryRequest"></a>

### GetDevicesSummaryRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |






<a name="api-GetDevicesSummaryResponse"></a>

### GetDevicesSummaryResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| active_count | [uint32](#uint32) |  | Active count. |
| inactive_count | [uint32](#uint32) |  | Inactive count. |
| dr_count | [GetDevicesSummaryResponse.DrCountEntry](#api-GetDevicesSummaryResponse-DrCountEntry) | repeated | per data-rate count. Devices that have never been seen are excluded. |
| never_seen_count | [uint32](#uint32) |  | Never seen count. |






<a name="api-GetDevicesSummaryResponse-DrCountEntry"></a>

### GetDevicesSummaryResponse.DrCountEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [uint32](#uint32) |  |  |
| value | [uint32](#uint32) |  |  |






<a name="api-GetGatewaysSummaryRequest"></a>

### GetGatewaysSummaryRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tenant_id | [string](#string) |  | Tenant ID (UUID). |






<a name="api-GetGatewaysSummaryResponse"></a>

### GetGatewaysSummaryResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| active_count | [uint32](#uint32) |  | Active count. |
| inactive_count | [uint32](#uint32) |  | Inactive count. |
| never_seen_count | [uint32](#uint32) |  | Never seen count. |






<a name="api-GlobalSearchRequest"></a>

### GlobalSearchRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| search | [string](#string) |  | Search query. |
| limit | [int64](#int64) |  | Max number of results to return. |
| offset | [int64](#int64) |  | Offset offset of the result-set (for pagination). |






<a name="api-GlobalSearchResponse"></a>

### GlobalSearchResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| result | [GlobalSearchResult](#api-GlobalSearchResult) | repeated |  |






<a name="api-GlobalSearchResult"></a>

### GlobalSearchResult



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| kind | [string](#string) |  | Record kind. |
| score | [float](#float) |  | Search score. |
| tenant_id | [string](#string) |  | Organization id. |
| tenant_name | [string](#string) |  | Organization name. |
| application_id | [string](#string) |  | Application id. |
| application_name | [string](#string) |  | Application name. |
| device_dev_eui | [string](#string) |  | Device DevEUI (hex encoded). |
| device_name | [string](#string) |  | Device name. |
| gateway_id | [string](#string) |  | Gateway MAC (hex encoded). |
| gateway_name | [string](#string) |  | Gateway name. |






<a name="api-ListApiKeysRequest"></a>

### ListApiKeysRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  | Max number of items to return. |
| offset | [uint32](#uint32) |  | Offset in the result-set (for pagination). |
| is_admin | [bool](#bool) |  | Return only admin keys. |
| tenant_id | [string](#string) |  | Filter on tenant ID. |






<a name="api-ListApiKeysResponse"></a>

### ListApiKeysResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| total_count | [uint32](#uint32) |  | Total number of API keys. |
| result | [ApiKey](#api-ApiKey) | repeated |  |






<a name="api-LogItem"></a>

### LogItem



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | ID. |
| time | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Timestamp. |
| description | [string](#string) |  | Message. |
| body | [string](#string) |  | Body. |
| properties | [LogItem.PropertiesEntry](#api-LogItem-PropertiesEntry) | repeated | Properties. |






<a name="api-LogItem-PropertiesEntry"></a>

### LogItem.PropertiesEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [string](#string) |  |  |






<a name="api-LoginRequest"></a>

### LoginRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| email | [string](#string) |  | Email of the user. |
| password | [string](#string) |  | Password of the user. |






<a name="api-LoginResponse"></a>

### LoginResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| jwt | [string](#string) |  | The JWT tag to be used to access chirpstack-application-server interfaces. |






<a name="api-OpenIdConnect"></a>

### OpenIdConnect



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| enabled | [bool](#bool) |  | Enable OpenId Connect authentication. |
| login_url | [string](#string) |  | Login url. |
| login_label | [string](#string) |  | Login label. |
| logout_url | [string](#string) |  | Logout url. |






<a name="api-OpenIdConnectLoginRequest"></a>

### OpenIdConnectLoginRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| code | [string](#string) |  | OpenId Connect callback code. |
| state | [string](#string) |  | OpenId Connect callback state. |






<a name="api-OpenIdConnectLoginResponse"></a>

### OpenIdConnectLoginResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | Token to use for authentication. |






<a name="api-ProfileResponse"></a>

### ProfileResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| user | [User](#api-User) |  | User object. |
| tenants | [UserTenantLink](#api-UserTenantLink) | repeated | Tenants to which the user is associated. |






<a name="api-SettingsResponse"></a>

### SettingsResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| openid_connect | [OpenIdConnect](#api-OpenIdConnect) |  | OpenId Connect settings. |






<a name="api-StreamDeviceEventsRequest"></a>

### StreamDeviceEventsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | Device EUI. |






<a name="api-StreamDeviceFramesRequest"></a>

### StreamDeviceFramesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| dev_eui | [string](#string) |  | Device EUI. |






<a name="api-StreamGatewayFramesRequest"></a>

### StreamGatewayFramesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gateway_id | [string](#string) |  | Gateway ID (EUI64). |






<a name="api-UserTenantLink"></a>

### UserTenantLink
Defines a tenant to which the user is associated.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Created at timestamp. |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  | Last update timestamp. |
| tenant_id | [string](#string) |  | Tenant ID. |
| is_admin | [bool](#bool) |  | User is admin within the context of this tenant. There is no need to set the is_device_admin and is_gateway_admin flags. |
| is_device_admin | [bool](#bool) |  | User is able to modify device related resources (applications, device-profiles, devices, multicast-groups). |
| is_gateway_admin | [bool](#bool) |  | User is able to modify gateways. |





 

 

 


<a name="api-InternalService"></a>

### InternalService
InternalService is the service providing API endpoints for internal usage.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Login | [LoginRequest](#api-LoginRequest) | [LoginResponse](#api-LoginResponse) | Log in a user |
| Profile | [.google.protobuf.Empty](#google-protobuf-Empty) | [ProfileResponse](#api-ProfileResponse) | Get the current user&#39;s profile |
| GlobalSearch | [GlobalSearchRequest](#api-GlobalSearchRequest) | [GlobalSearchResponse](#api-GlobalSearchResponse) | Perform a global search. |
| CreateApiKey | [CreateApiKeyRequest](#api-CreateApiKeyRequest) | [CreateApiKeyResponse](#api-CreateApiKeyResponse) | CreateApiKey creates the given API key. |
| DeleteApiKey | [DeleteApiKeyRequest](#api-DeleteApiKeyRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) | DeleteApiKey deletes the API key. |
| ListApiKeys | [ListApiKeysRequest](#api-ListApiKeysRequest) | [ListApiKeysResponse](#api-ListApiKeysResponse) | ListApiKeys lists the available API keys. |
| Settings | [.google.protobuf.Empty](#google-protobuf-Empty) | [SettingsResponse](#api-SettingsResponse) | Get the global settings. |
| OpenIdConnectLogin | [OpenIdConnectLoginRequest](#api-OpenIdConnectLoginRequest) | [OpenIdConnectLoginResponse](#api-OpenIdConnectLoginResponse) | OpenId Connect login. |
| GetDevicesSummary | [GetDevicesSummaryRequest](#api-GetDevicesSummaryRequest) | [GetDevicesSummaryResponse](#api-GetDevicesSummaryResponse) | GetDevicesSummary returns an aggregated summary of the devices. |
| GetGatewaysSummary | [GetGatewaysSummaryRequest](#api-GetGatewaysSummaryRequest) | [GetGatewaysSummaryResponse](#api-GetGatewaysSummaryResponse) | GetGatewaysSummary returns an aggregated summary of the gateways. |
| StreamGatewayFrames | [StreamGatewayFramesRequest](#api-StreamGatewayFramesRequest) | [LogItem](#api-LogItem) stream | Stream frame for the given Gateway ID. |
| StreamDeviceFrames | [StreamDeviceFramesRequest](#api-StreamDeviceFramesRequest) | [LogItem](#api-LogItem) stream | Stream frames for the given Device EUI. |
| StreamDeviceEvents | [StreamDeviceEventsRequest](#api-StreamDeviceEventsRequest) | [LogItem](#api-LogItem) stream | Stream events for the given Device EUI. |

 



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

