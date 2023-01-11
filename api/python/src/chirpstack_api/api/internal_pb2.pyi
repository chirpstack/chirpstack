from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from chirpstack_api.common import common_pb2 as _common_pb2
from chirpstack_api.api import user_pb2 as _user_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ApiKey(_message.Message):
    __slots__ = ["id", "is_admin", "name", "tenant_id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    IS_ADMIN_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    is_admin: bool
    name: str
    tenant_id: str
    def __init__(self, id: _Optional[str] = ..., name: _Optional[str] = ..., is_admin: bool = ..., tenant_id: _Optional[str] = ...) -> None: ...

class CreateApiKeyRequest(_message.Message):
    __slots__ = ["api_key"]
    API_KEY_FIELD_NUMBER: _ClassVar[int]
    api_key: ApiKey
    def __init__(self, api_key: _Optional[_Union[ApiKey, _Mapping]] = ...) -> None: ...

class CreateApiKeyResponse(_message.Message):
    __slots__ = ["id", "token"]
    ID_FIELD_NUMBER: _ClassVar[int]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    id: str
    token: str
    def __init__(self, id: _Optional[str] = ..., token: _Optional[str] = ...) -> None: ...

class DeleteApiKeyRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetDevicesSummaryRequest(_message.Message):
    __slots__ = ["tenant_id"]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    def __init__(self, tenant_id: _Optional[str] = ...) -> None: ...

class GetDevicesSummaryResponse(_message.Message):
    __slots__ = ["active_count", "dr_count", "inactive_count", "never_seen_count"]
    class DrCountEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: int
        value: int
        def __init__(self, key: _Optional[int] = ..., value: _Optional[int] = ...) -> None: ...
    ACTIVE_COUNT_FIELD_NUMBER: _ClassVar[int]
    DR_COUNT_FIELD_NUMBER: _ClassVar[int]
    INACTIVE_COUNT_FIELD_NUMBER: _ClassVar[int]
    NEVER_SEEN_COUNT_FIELD_NUMBER: _ClassVar[int]
    active_count: int
    dr_count: _containers.ScalarMap[int, int]
    inactive_count: int
    never_seen_count: int
    def __init__(self, active_count: _Optional[int] = ..., inactive_count: _Optional[int] = ..., dr_count: _Optional[_Mapping[int, int]] = ..., never_seen_count: _Optional[int] = ...) -> None: ...

class GetGatewaysSummaryRequest(_message.Message):
    __slots__ = ["tenant_id"]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    def __init__(self, tenant_id: _Optional[str] = ...) -> None: ...

class GetGatewaysSummaryResponse(_message.Message):
    __slots__ = ["never_seen_count", "offline_count", "online_count"]
    NEVER_SEEN_COUNT_FIELD_NUMBER: _ClassVar[int]
    OFFLINE_COUNT_FIELD_NUMBER: _ClassVar[int]
    ONLINE_COUNT_FIELD_NUMBER: _ClassVar[int]
    never_seen_count: int
    offline_count: int
    online_count: int
    def __init__(self, online_count: _Optional[int] = ..., offline_count: _Optional[int] = ..., never_seen_count: _Optional[int] = ...) -> None: ...

class GetRegionRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetRegionResponse(_message.Message):
    __slots__ = ["class_b_ping_slot_dr", "class_b_ping_slot_frequency", "description", "id", "region", "rx1_delay", "rx1_dr_offset", "rx2_dr", "rx2_frequency", "uplink_channels", "user_info"]
    CLASS_B_PING_SLOT_DR_FIELD_NUMBER: _ClassVar[int]
    CLASS_B_PING_SLOT_FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    REGION_FIELD_NUMBER: _ClassVar[int]
    RX1_DELAY_FIELD_NUMBER: _ClassVar[int]
    RX1_DR_OFFSET_FIELD_NUMBER: _ClassVar[int]
    RX2_DR_FIELD_NUMBER: _ClassVar[int]
    RX2_FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    UPLINK_CHANNELS_FIELD_NUMBER: _ClassVar[int]
    USER_INFO_FIELD_NUMBER: _ClassVar[int]
    class_b_ping_slot_dr: int
    class_b_ping_slot_frequency: int
    description: str
    id: str
    region: _common_pb2.Region
    rx1_delay: int
    rx1_dr_offset: int
    rx2_dr: int
    rx2_frequency: int
    uplink_channels: _containers.RepeatedCompositeFieldContainer[RegionChannel]
    user_info: str
    def __init__(self, id: _Optional[str] = ..., region: _Optional[_Union[_common_pb2.Region, str]] = ..., user_info: _Optional[str] = ..., uplink_channels: _Optional[_Iterable[_Union[RegionChannel, _Mapping]]] = ..., rx1_delay: _Optional[int] = ..., rx1_dr_offset: _Optional[int] = ..., rx2_dr: _Optional[int] = ..., rx2_frequency: _Optional[int] = ..., class_b_ping_slot_dr: _Optional[int] = ..., class_b_ping_slot_frequency: _Optional[int] = ..., description: _Optional[str] = ...) -> None: ...

class GlobalSearchRequest(_message.Message):
    __slots__ = ["limit", "offset", "search"]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    SEARCH_FIELD_NUMBER: _ClassVar[int]
    limit: int
    offset: int
    search: str
    def __init__(self, search: _Optional[str] = ..., limit: _Optional[int] = ..., offset: _Optional[int] = ...) -> None: ...

class GlobalSearchResponse(_message.Message):
    __slots__ = ["result"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    result: _containers.RepeatedCompositeFieldContainer[GlobalSearchResult]
    def __init__(self, result: _Optional[_Iterable[_Union[GlobalSearchResult, _Mapping]]] = ...) -> None: ...

class GlobalSearchResult(_message.Message):
    __slots__ = ["application_id", "application_name", "device_dev_eui", "device_name", "gateway_id", "gateway_name", "kind", "score", "tenant_id", "tenant_name"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_NAME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    DEVICE_NAME_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_NAME_FIELD_NUMBER: _ClassVar[int]
    KIND_FIELD_NUMBER: _ClassVar[int]
    SCORE_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    TENANT_NAME_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    application_name: str
    device_dev_eui: str
    device_name: str
    gateway_id: str
    gateway_name: str
    kind: str
    score: float
    tenant_id: str
    tenant_name: str
    def __init__(self, kind: _Optional[str] = ..., score: _Optional[float] = ..., tenant_id: _Optional[str] = ..., tenant_name: _Optional[str] = ..., application_id: _Optional[str] = ..., application_name: _Optional[str] = ..., device_dev_eui: _Optional[str] = ..., device_name: _Optional[str] = ..., gateway_id: _Optional[str] = ..., gateway_name: _Optional[str] = ...) -> None: ...

class ListApiKeysRequest(_message.Message):
    __slots__ = ["is_admin", "limit", "offset", "tenant_id"]
    IS_ADMIN_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    is_admin: bool
    limit: int
    offset: int
    tenant_id: str
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., is_admin: bool = ..., tenant_id: _Optional[str] = ...) -> None: ...

class ListApiKeysResponse(_message.Message):
    __slots__ = ["result", "total_count"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    result: _containers.RepeatedCompositeFieldContainer[ApiKey]
    total_count: int
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[ApiKey, _Mapping]]] = ...) -> None: ...

class ListRegionsResponse(_message.Message):
    __slots__ = ["regions"]
    REGIONS_FIELD_NUMBER: _ClassVar[int]
    regions: _containers.RepeatedCompositeFieldContainer[RegionListItem]
    def __init__(self, regions: _Optional[_Iterable[_Union[RegionListItem, _Mapping]]] = ...) -> None: ...

class LogItem(_message.Message):
    __slots__ = ["body", "description", "id", "properties", "time"]
    class PropertiesEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    BODY_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    PROPERTIES_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    body: str
    description: str
    id: str
    properties: _containers.ScalarMap[str, str]
    time: _timestamp_pb2.Timestamp
    def __init__(self, id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., description: _Optional[str] = ..., body: _Optional[str] = ..., properties: _Optional[_Mapping[str, str]] = ...) -> None: ...

class LoginRequest(_message.Message):
    __slots__ = ["email", "password"]
    EMAIL_FIELD_NUMBER: _ClassVar[int]
    PASSWORD_FIELD_NUMBER: _ClassVar[int]
    email: str
    password: str
    def __init__(self, email: _Optional[str] = ..., password: _Optional[str] = ...) -> None: ...

class LoginResponse(_message.Message):
    __slots__ = ["jwt"]
    JWT_FIELD_NUMBER: _ClassVar[int]
    jwt: str
    def __init__(self, jwt: _Optional[str] = ...) -> None: ...

class OpenIdConnect(_message.Message):
    __slots__ = ["enabled", "login_label", "login_url", "logout_url"]
    ENABLED_FIELD_NUMBER: _ClassVar[int]
    LOGIN_LABEL_FIELD_NUMBER: _ClassVar[int]
    LOGIN_URL_FIELD_NUMBER: _ClassVar[int]
    LOGOUT_URL_FIELD_NUMBER: _ClassVar[int]
    enabled: bool
    login_label: str
    login_url: str
    logout_url: str
    def __init__(self, enabled: bool = ..., login_url: _Optional[str] = ..., login_label: _Optional[str] = ..., logout_url: _Optional[str] = ...) -> None: ...

class OpenIdConnectLoginRequest(_message.Message):
    __slots__ = ["code", "state"]
    CODE_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    code: str
    state: str
    def __init__(self, code: _Optional[str] = ..., state: _Optional[str] = ...) -> None: ...

class OpenIdConnectLoginResponse(_message.Message):
    __slots__ = ["token"]
    TOKEN_FIELD_NUMBER: _ClassVar[int]
    token: str
    def __init__(self, token: _Optional[str] = ...) -> None: ...

class ProfileResponse(_message.Message):
    __slots__ = ["tenants", "user"]
    TENANTS_FIELD_NUMBER: _ClassVar[int]
    USER_FIELD_NUMBER: _ClassVar[int]
    tenants: _containers.RepeatedCompositeFieldContainer[UserTenantLink]
    user: _user_pb2.User
    def __init__(self, user: _Optional[_Union[_user_pb2.User, _Mapping]] = ..., tenants: _Optional[_Iterable[_Union[UserTenantLink, _Mapping]]] = ...) -> None: ...

class RegionChannel(_message.Message):
    __slots__ = ["dr_max", "dr_min", "frequency"]
    DR_MAX_FIELD_NUMBER: _ClassVar[int]
    DR_MIN_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    dr_max: int
    dr_min: int
    frequency: int
    def __init__(self, frequency: _Optional[int] = ..., dr_min: _Optional[int] = ..., dr_max: _Optional[int] = ...) -> None: ...

class RegionListItem(_message.Message):
    __slots__ = ["description", "id", "region"]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    REGION_FIELD_NUMBER: _ClassVar[int]
    description: str
    id: str
    region: _common_pb2.Region
    def __init__(self, id: _Optional[str] = ..., region: _Optional[_Union[_common_pb2.Region, str]] = ..., description: _Optional[str] = ...) -> None: ...

class SettingsResponse(_message.Message):
    __slots__ = ["openid_connect"]
    OPENID_CONNECT_FIELD_NUMBER: _ClassVar[int]
    openid_connect: OpenIdConnect
    def __init__(self, openid_connect: _Optional[_Union[OpenIdConnect, _Mapping]] = ...) -> None: ...

class StreamDeviceEventsRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class StreamDeviceFramesRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class StreamGatewayFramesRequest(_message.Message):
    __slots__ = ["gateway_id"]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    def __init__(self, gateway_id: _Optional[str] = ...) -> None: ...

class UserTenantLink(_message.Message):
    __slots__ = ["created_at", "is_admin", "is_device_admin", "is_gateway_admin", "tenant_id", "updated_at"]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    IS_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_DEVICE_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_GATEWAY_ADMIN_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    created_at: _timestamp_pb2.Timestamp
    is_admin: bool
    is_device_admin: bool
    is_gateway_admin: bool
    tenant_id: str
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., tenant_id: _Optional[str] = ..., is_admin: bool = ..., is_device_admin: bool = ..., is_gateway_admin: bool = ...) -> None: ...
