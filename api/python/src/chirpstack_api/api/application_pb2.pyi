from google.api import annotations_pb2 as _annotations_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Encoding(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    JSON: _ClassVar[Encoding]
    PROTOBUF: _ClassVar[Encoding]

class IntegrationKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    HTTP: _ClassVar[IntegrationKind]
    INFLUX_DB: _ClassVar[IntegrationKind]
    THINGS_BOARD: _ClassVar[IntegrationKind]
    MY_DEVICES: _ClassVar[IntegrationKind]
    LORA_CLOUD: _ClassVar[IntegrationKind]
    GCP_PUB_SUB: _ClassVar[IntegrationKind]
    AWS_SNS: _ClassVar[IntegrationKind]
    AZURE_SERVICE_BUS: _ClassVar[IntegrationKind]
    PILOT_THINGS: _ClassVar[IntegrationKind]
    MQTT_GLOBAL: _ClassVar[IntegrationKind]
    IFTTT: _ClassVar[IntegrationKind]

class InfluxDbPrecision(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    NS: _ClassVar[InfluxDbPrecision]
    U: _ClassVar[InfluxDbPrecision]
    MS: _ClassVar[InfluxDbPrecision]
    S: _ClassVar[InfluxDbPrecision]
    M: _ClassVar[InfluxDbPrecision]
    H: _ClassVar[InfluxDbPrecision]

class InfluxDbVersion(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    INFLUXDB_1: _ClassVar[InfluxDbVersion]
    INFLUXDB_2: _ClassVar[InfluxDbVersion]
JSON: Encoding
PROTOBUF: Encoding
HTTP: IntegrationKind
INFLUX_DB: IntegrationKind
THINGS_BOARD: IntegrationKind
MY_DEVICES: IntegrationKind
LORA_CLOUD: IntegrationKind
GCP_PUB_SUB: IntegrationKind
AWS_SNS: IntegrationKind
AZURE_SERVICE_BUS: IntegrationKind
PILOT_THINGS: IntegrationKind
MQTT_GLOBAL: IntegrationKind
IFTTT: IntegrationKind
NS: InfluxDbPrecision
U: InfluxDbPrecision
MS: InfluxDbPrecision
S: InfluxDbPrecision
M: InfluxDbPrecision
H: InfluxDbPrecision
INFLUXDB_1: InfluxDbVersion
INFLUXDB_2: InfluxDbVersion

class Application(_message.Message):
    __slots__ = ["id", "name", "description", "tenant_id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    name: str
    description: str
    tenant_id: str
    def __init__(self, id: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., tenant_id: _Optional[str] = ...) -> None: ...

class ApplicationListItem(_message.Message):
    __slots__ = ["id", "created_at", "updated_at", "name", "description"]
    ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    id: str
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    name: str
    description: str
    def __init__(self, id: _Optional[str] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., name: _Optional[str] = ..., description: _Optional[str] = ...) -> None: ...

class CreateApplicationRequest(_message.Message):
    __slots__ = ["application"]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    application: Application
    def __init__(self, application: _Optional[_Union[Application, _Mapping]] = ...) -> None: ...

class CreateApplicationResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetApplicationRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetApplicationResponse(_message.Message):
    __slots__ = ["application", "created_at", "updated_at", "measurement_keys"]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    MEASUREMENT_KEYS_FIELD_NUMBER: _ClassVar[int]
    application: Application
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    measurement_keys: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, application: _Optional[_Union[Application, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., measurement_keys: _Optional[_Iterable[str]] = ...) -> None: ...

class UpdateApplicationRequest(_message.Message):
    __slots__ = ["application"]
    APPLICATION_FIELD_NUMBER: _ClassVar[int]
    application: Application
    def __init__(self, application: _Optional[_Union[Application, _Mapping]] = ...) -> None: ...

class DeleteApplicationRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class ListApplicationsRequest(_message.Message):
    __slots__ = ["limit", "offset", "search", "tenant_id"]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    SEARCH_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    limit: int
    offset: int
    search: str
    tenant_id: str
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., search: _Optional[str] = ..., tenant_id: _Optional[str] = ...) -> None: ...

class ListApplicationsResponse(_message.Message):
    __slots__ = ["total_count", "result"]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    total_count: int
    result: _containers.RepeatedCompositeFieldContainer[ApplicationListItem]
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[ApplicationListItem, _Mapping]]] = ...) -> None: ...

class ListIntegrationsRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class IntegrationListItem(_message.Message):
    __slots__ = ["kind"]
    KIND_FIELD_NUMBER: _ClassVar[int]
    kind: IntegrationKind
    def __init__(self, kind: _Optional[_Union[IntegrationKind, str]] = ...) -> None: ...

class ListIntegrationsResponse(_message.Message):
    __slots__ = ["total_count", "result"]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    total_count: int
    result: _containers.RepeatedCompositeFieldContainer[IntegrationListItem]
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[IntegrationListItem, _Mapping]]] = ...) -> None: ...

class HttpIntegration(_message.Message):
    __slots__ = ["application_id", "headers", "encoding", "event_endpoint_url"]
    class HeadersEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    HEADERS_FIELD_NUMBER: _ClassVar[int]
    ENCODING_FIELD_NUMBER: _ClassVar[int]
    EVENT_ENDPOINT_URL_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    headers: _containers.ScalarMap[str, str]
    encoding: Encoding
    event_endpoint_url: str
    def __init__(self, application_id: _Optional[str] = ..., headers: _Optional[_Mapping[str, str]] = ..., encoding: _Optional[_Union[Encoding, str]] = ..., event_endpoint_url: _Optional[str] = ...) -> None: ...

class CreateHttpIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: HttpIntegration
    def __init__(self, integration: _Optional[_Union[HttpIntegration, _Mapping]] = ...) -> None: ...

class GetHttpIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetHttpIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: HttpIntegration
    def __init__(self, integration: _Optional[_Union[HttpIntegration, _Mapping]] = ...) -> None: ...

class UpdateHttpIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: HttpIntegration
    def __init__(self, integration: _Optional[_Union[HttpIntegration, _Mapping]] = ...) -> None: ...

class DeleteHttpIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class InfluxDbIntegration(_message.Message):
    __slots__ = ["application_id", "endpoint", "db", "username", "password", "retention_policy_name", "precision", "version", "token", "organization", "bucket"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    ENDPOINT_FIELD_NUMBER: _ClassVar[int]
    DB_FIELD_NUMBER: _ClassVar[int]
    USERNAME_FIELD_NUMBER: _ClassVar[int]
    PASSWORD_FIELD_NUMBER: _ClassVar[int]
    RETENTION_POLICY_NAME_FIELD_NUMBER: _ClassVar[int]
    PRECISION_FIELD_NUMBER: _ClassVar[int]
    VERSION_FIELD_NUMBER: _ClassVar[int]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_FIELD_NUMBER: _ClassVar[int]
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    endpoint: str
    db: str
    username: str
    password: str
    retention_policy_name: str
    precision: InfluxDbPrecision
    version: InfluxDbVersion
    token: str
    organization: str
    bucket: str
    def __init__(self, application_id: _Optional[str] = ..., endpoint: _Optional[str] = ..., db: _Optional[str] = ..., username: _Optional[str] = ..., password: _Optional[str] = ..., retention_policy_name: _Optional[str] = ..., precision: _Optional[_Union[InfluxDbPrecision, str]] = ..., version: _Optional[_Union[InfluxDbVersion, str]] = ..., token: _Optional[str] = ..., organization: _Optional[str] = ..., bucket: _Optional[str] = ...) -> None: ...

class CreateInfluxDbIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: InfluxDbIntegration
    def __init__(self, integration: _Optional[_Union[InfluxDbIntegration, _Mapping]] = ...) -> None: ...

class GetInfluxDbIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetInfluxDbIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: InfluxDbIntegration
    def __init__(self, integration: _Optional[_Union[InfluxDbIntegration, _Mapping]] = ...) -> None: ...

class UpdateInfluxDbIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: InfluxDbIntegration
    def __init__(self, integration: _Optional[_Union[InfluxDbIntegration, _Mapping]] = ...) -> None: ...

class DeleteInfluxDbIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class ThingsBoardIntegration(_message.Message):
    __slots__ = ["application_id", "server"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    SERVER_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    server: str
    def __init__(self, application_id: _Optional[str] = ..., server: _Optional[str] = ...) -> None: ...

class CreateThingsBoardIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: ThingsBoardIntegration
    def __init__(self, integration: _Optional[_Union[ThingsBoardIntegration, _Mapping]] = ...) -> None: ...

class GetThingsBoardIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetThingsBoardIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: ThingsBoardIntegration
    def __init__(self, integration: _Optional[_Union[ThingsBoardIntegration, _Mapping]] = ...) -> None: ...

class UpdateThingsBoardIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: ThingsBoardIntegration
    def __init__(self, integration: _Optional[_Union[ThingsBoardIntegration, _Mapping]] = ...) -> None: ...

class DeleteThingsBoardIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class MyDevicesIntegration(_message.Message):
    __slots__ = ["application_id", "endpoint"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    ENDPOINT_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    endpoint: str
    def __init__(self, application_id: _Optional[str] = ..., endpoint: _Optional[str] = ...) -> None: ...

class CreateMyDevicesIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: MyDevicesIntegration
    def __init__(self, integration: _Optional[_Union[MyDevicesIntegration, _Mapping]] = ...) -> None: ...

class GetMyDevicesIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetMyDevicesIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: MyDevicesIntegration
    def __init__(self, integration: _Optional[_Union[MyDevicesIntegration, _Mapping]] = ...) -> None: ...

class UpdateMyDevicesIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: MyDevicesIntegration
    def __init__(self, integration: _Optional[_Union[MyDevicesIntegration, _Mapping]] = ...) -> None: ...

class DeleteMyDevicesIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class LoraCloudIntegration(_message.Message):
    __slots__ = ["application_id", "modem_geolocation_services"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    MODEM_GEOLOCATION_SERVICES_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    modem_geolocation_services: LoraCloudModemGeolocationServices
    def __init__(self, application_id: _Optional[str] = ..., modem_geolocation_services: _Optional[_Union[LoraCloudModemGeolocationServices, _Mapping]] = ...) -> None: ...

class LoraCloudModemGeolocationServices(_message.Message):
    __slots__ = ["token", "modem_enabled", "forward_f_ports", "gnss_use_rx_time", "gnss_use_gateway_location", "parse_tlv", "geolocation_buffer_ttl", "geolocation_min_buffer_size", "geolocation_tdoa", "geolocation_rssi", "geolocation_gnss", "geolocation_gnss_payload_field", "geolocation_gnss_use_rx_time", "geolocation_wifi", "geolocation_wifi_payload_field"]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    MODEM_ENABLED_FIELD_NUMBER: _ClassVar[int]
    FORWARD_F_PORTS_FIELD_NUMBER: _ClassVar[int]
    GNSS_USE_RX_TIME_FIELD_NUMBER: _ClassVar[int]
    GNSS_USE_GATEWAY_LOCATION_FIELD_NUMBER: _ClassVar[int]
    PARSE_TLV_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_BUFFER_TTL_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_MIN_BUFFER_SIZE_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_TDOA_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_RSSI_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_GNSS_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_GNSS_PAYLOAD_FIELD_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_GNSS_USE_RX_TIME_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_WIFI_FIELD_NUMBER: _ClassVar[int]
    GEOLOCATION_WIFI_PAYLOAD_FIELD_FIELD_NUMBER: _ClassVar[int]
    token: str
    modem_enabled: bool
    forward_f_ports: _containers.RepeatedScalarFieldContainer[int]
    gnss_use_rx_time: bool
    gnss_use_gateway_location: bool
    parse_tlv: bool
    geolocation_buffer_ttl: int
    geolocation_min_buffer_size: int
    geolocation_tdoa: bool
    geolocation_rssi: bool
    geolocation_gnss: bool
    geolocation_gnss_payload_field: str
    geolocation_gnss_use_rx_time: bool
    geolocation_wifi: bool
    geolocation_wifi_payload_field: str
    def __init__(self, token: _Optional[str] = ..., modem_enabled: bool = ..., forward_f_ports: _Optional[_Iterable[int]] = ..., gnss_use_rx_time: bool = ..., gnss_use_gateway_location: bool = ..., parse_tlv: bool = ..., geolocation_buffer_ttl: _Optional[int] = ..., geolocation_min_buffer_size: _Optional[int] = ..., geolocation_tdoa: bool = ..., geolocation_rssi: bool = ..., geolocation_gnss: bool = ..., geolocation_gnss_payload_field: _Optional[str] = ..., geolocation_gnss_use_rx_time: bool = ..., geolocation_wifi: bool = ..., geolocation_wifi_payload_field: _Optional[str] = ...) -> None: ...

class CreateLoraCloudIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: LoraCloudIntegration
    def __init__(self, integration: _Optional[_Union[LoraCloudIntegration, _Mapping]] = ...) -> None: ...

class GetLoraCloudIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetLoraCloudIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: LoraCloudIntegration
    def __init__(self, integration: _Optional[_Union[LoraCloudIntegration, _Mapping]] = ...) -> None: ...

class UpdateLoraCloudIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: LoraCloudIntegration
    def __init__(self, integration: _Optional[_Union[LoraCloudIntegration, _Mapping]] = ...) -> None: ...

class DeleteLoraCloudIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GcpPubSubIntegration(_message.Message):
    __slots__ = ["application_id", "encoding", "credentials_file", "project_id", "topic_name"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    ENCODING_FIELD_NUMBER: _ClassVar[int]
    CREDENTIALS_FILE_FIELD_NUMBER: _ClassVar[int]
    PROJECT_ID_FIELD_NUMBER: _ClassVar[int]
    TOPIC_NAME_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    encoding: Encoding
    credentials_file: str
    project_id: str
    topic_name: str
    def __init__(self, application_id: _Optional[str] = ..., encoding: _Optional[_Union[Encoding, str]] = ..., credentials_file: _Optional[str] = ..., project_id: _Optional[str] = ..., topic_name: _Optional[str] = ...) -> None: ...

class CreateGcpPubSubIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: GcpPubSubIntegration
    def __init__(self, integration: _Optional[_Union[GcpPubSubIntegration, _Mapping]] = ...) -> None: ...

class GetGcpPubSubIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetGcpPubSubIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: GcpPubSubIntegration
    def __init__(self, integration: _Optional[_Union[GcpPubSubIntegration, _Mapping]] = ...) -> None: ...

class UpdateGcpPubSubIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: GcpPubSubIntegration
    def __init__(self, integration: _Optional[_Union[GcpPubSubIntegration, _Mapping]] = ...) -> None: ...

class DeleteGcpPubSubIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class AwsSnsIntegration(_message.Message):
    __slots__ = ["application_id", "encoding", "region", "access_key_id", "secret_access_key", "topic_arn"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    ENCODING_FIELD_NUMBER: _ClassVar[int]
    REGION_FIELD_NUMBER: _ClassVar[int]
    ACCESS_KEY_ID_FIELD_NUMBER: _ClassVar[int]
    SECRET_ACCESS_KEY_FIELD_NUMBER: _ClassVar[int]
    TOPIC_ARN_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    encoding: Encoding
    region: str
    access_key_id: str
    secret_access_key: str
    topic_arn: str
    def __init__(self, application_id: _Optional[str] = ..., encoding: _Optional[_Union[Encoding, str]] = ..., region: _Optional[str] = ..., access_key_id: _Optional[str] = ..., secret_access_key: _Optional[str] = ..., topic_arn: _Optional[str] = ...) -> None: ...

class CreateAwsSnsIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: AwsSnsIntegration
    def __init__(self, integration: _Optional[_Union[AwsSnsIntegration, _Mapping]] = ...) -> None: ...

class GetAwsSnsIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetAwsSnsIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: AwsSnsIntegration
    def __init__(self, integration: _Optional[_Union[AwsSnsIntegration, _Mapping]] = ...) -> None: ...

class UpdateAwsSnsIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: AwsSnsIntegration
    def __init__(self, integration: _Optional[_Union[AwsSnsIntegration, _Mapping]] = ...) -> None: ...

class DeleteAwsSnsIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class AzureServiceBusIntegration(_message.Message):
    __slots__ = ["application_id", "encoding", "connection_string", "publish_name"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    ENCODING_FIELD_NUMBER: _ClassVar[int]
    CONNECTION_STRING_FIELD_NUMBER: _ClassVar[int]
    PUBLISH_NAME_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    encoding: Encoding
    connection_string: str
    publish_name: str
    def __init__(self, application_id: _Optional[str] = ..., encoding: _Optional[_Union[Encoding, str]] = ..., connection_string: _Optional[str] = ..., publish_name: _Optional[str] = ...) -> None: ...

class CreateAzureServiceBusIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: AzureServiceBusIntegration
    def __init__(self, integration: _Optional[_Union[AzureServiceBusIntegration, _Mapping]] = ...) -> None: ...

class GetAzureServiceBusIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetAzureServiceBusIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: AzureServiceBusIntegration
    def __init__(self, integration: _Optional[_Union[AzureServiceBusIntegration, _Mapping]] = ...) -> None: ...

class UpdateAzureServiceBusIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: AzureServiceBusIntegration
    def __init__(self, integration: _Optional[_Union[AzureServiceBusIntegration, _Mapping]] = ...) -> None: ...

class DeleteAzureServiceBusIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class PilotThingsIntegration(_message.Message):
    __slots__ = ["application_id", "server", "token"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    SERVER_FIELD_NUMBER: _ClassVar[int]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    server: str
    token: str
    def __init__(self, application_id: _Optional[str] = ..., server: _Optional[str] = ..., token: _Optional[str] = ...) -> None: ...

class CreatePilotThingsIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: PilotThingsIntegration
    def __init__(self, integration: _Optional[_Union[PilotThingsIntegration, _Mapping]] = ...) -> None: ...

class GetPilotThingsIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetPilotThingsIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: PilotThingsIntegration
    def __init__(self, integration: _Optional[_Union[PilotThingsIntegration, _Mapping]] = ...) -> None: ...

class UpdatePilotThingsIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: PilotThingsIntegration
    def __init__(self, integration: _Optional[_Union[PilotThingsIntegration, _Mapping]] = ...) -> None: ...

class DeletePilotThingsIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class IftttIntegration(_message.Message):
    __slots__ = ["application_id", "key", "uplink_values", "arbitrary_json", "event_prefix"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    KEY_FIELD_NUMBER: _ClassVar[int]
    UPLINK_VALUES_FIELD_NUMBER: _ClassVar[int]
    ARBITRARY_JSON_FIELD_NUMBER: _ClassVar[int]
    EVENT_PREFIX_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    key: str
    uplink_values: _containers.RepeatedScalarFieldContainer[str]
    arbitrary_json: bool
    event_prefix: str
    def __init__(self, application_id: _Optional[str] = ..., key: _Optional[str] = ..., uplink_values: _Optional[_Iterable[str]] = ..., arbitrary_json: bool = ..., event_prefix: _Optional[str] = ...) -> None: ...

class CreateIftttIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: IftttIntegration
    def __init__(self, integration: _Optional[_Union[IftttIntegration, _Mapping]] = ...) -> None: ...

class GetIftttIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GetIftttIntegrationResponse(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: IftttIntegration
    def __init__(self, integration: _Optional[_Union[IftttIntegration, _Mapping]] = ...) -> None: ...

class UpdateIftttIntegrationRequest(_message.Message):
    __slots__ = ["integration"]
    INTEGRATION_FIELD_NUMBER: _ClassVar[int]
    integration: IftttIntegration
    def __init__(self, integration: _Optional[_Union[IftttIntegration, _Mapping]] = ...) -> None: ...

class DeleteIftttIntegrationRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GenerateMqttIntegrationClientCertificateRequest(_message.Message):
    __slots__ = ["application_id"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    def __init__(self, application_id: _Optional[str] = ...) -> None: ...

class GenerateMqttIntegrationClientCertificateResponse(_message.Message):
    __slots__ = ["tls_cert", "tls_key", "ca_cert", "expires_at"]
    TLS_CERT_FIELD_NUMBER: _ClassVar[int]
    TLS_KEY_FIELD_NUMBER: _ClassVar[int]
    CA_CERT_FIELD_NUMBER: _ClassVar[int]
    EXPIRES_AT_FIELD_NUMBER: _ClassVar[int]
    tls_cert: str
    tls_key: str
    ca_cert: str
    expires_at: _timestamp_pb2.Timestamp
    def __init__(self, tls_cert: _Optional[str] = ..., tls_key: _Optional[str] = ..., ca_cert: _Optional[str] = ..., expires_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...
