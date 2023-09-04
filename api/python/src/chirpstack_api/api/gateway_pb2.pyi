from google.api import annotations_pb2 as _annotations_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from chirpstack_api.common import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class GatewayState(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    NEVER_SEEN: _ClassVar[GatewayState]
    ONLINE: _ClassVar[GatewayState]
    OFFLINE: _ClassVar[GatewayState]
NEVER_SEEN: GatewayState
ONLINE: GatewayState
OFFLINE: GatewayState

class Gateway(_message.Message):
    __slots__ = ["gateway_id", "name", "description", "location", "tenant_id", "tags", "metadata", "stats_interval"]
    class TagsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    class MetadataEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    STATS_INTERVAL_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    name: str
    description: str
    location: _common_pb2.Location
    tenant_id: str
    tags: _containers.ScalarMap[str, str]
    metadata: _containers.ScalarMap[str, str]
    stats_interval: int
    def __init__(self, gateway_id: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., location: _Optional[_Union[_common_pb2.Location, _Mapping]] = ..., tenant_id: _Optional[str] = ..., tags: _Optional[_Mapping[str, str]] = ..., metadata: _Optional[_Mapping[str, str]] = ..., stats_interval: _Optional[int] = ...) -> None: ...

class GatewayListItem(_message.Message):
    __slots__ = ["tenant_id", "gateway_id", "name", "description", "location", "properties", "created_at", "updated_at", "last_seen_at", "state"]
    class PropertiesEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    PROPERTIES_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    LAST_SEEN_AT_FIELD_NUMBER: _ClassVar[int]
    STATE_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    gateway_id: str
    name: str
    description: str
    location: _common_pb2.Location
    properties: _containers.ScalarMap[str, str]
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    last_seen_at: _timestamp_pb2.Timestamp
    state: GatewayState
    def __init__(self, tenant_id: _Optional[str] = ..., gateway_id: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., location: _Optional[_Union[_common_pb2.Location, _Mapping]] = ..., properties: _Optional[_Mapping[str, str]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., last_seen_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., state: _Optional[_Union[GatewayState, str]] = ...) -> None: ...

class CreateGatewayRequest(_message.Message):
    __slots__ = ["gateway"]
    GATEWAY_FIELD_NUMBER: _ClassVar[int]
    gateway: Gateway
    def __init__(self, gateway: _Optional[_Union[Gateway, _Mapping]] = ...) -> None: ...

class GetGatewayRequest(_message.Message):
    __slots__ = ["gateway_id"]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    def __init__(self, gateway_id: _Optional[str] = ...) -> None: ...

class GetGatewayResponse(_message.Message):
    __slots__ = ["gateway", "created_at", "updated_at", "last_seen_at"]
    GATEWAY_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    LAST_SEEN_AT_FIELD_NUMBER: _ClassVar[int]
    gateway: Gateway
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    last_seen_at: _timestamp_pb2.Timestamp
    def __init__(self, gateway: _Optional[_Union[Gateway, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., last_seen_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class UpdateGatewayRequest(_message.Message):
    __slots__ = ["gateway"]
    GATEWAY_FIELD_NUMBER: _ClassVar[int]
    gateway: Gateway
    def __init__(self, gateway: _Optional[_Union[Gateway, _Mapping]] = ...) -> None: ...

class DeleteGatewayRequest(_message.Message):
    __slots__ = ["gateway_id"]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    def __init__(self, gateway_id: _Optional[str] = ...) -> None: ...

class ListGatewaysRequest(_message.Message):
    __slots__ = ["limit", "offset", "search", "tenant_id", "multicast_group_id"]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    SEARCH_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    limit: int
    offset: int
    search: str
    tenant_id: str
    multicast_group_id: str
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., search: _Optional[str] = ..., tenant_id: _Optional[str] = ..., multicast_group_id: _Optional[str] = ...) -> None: ...

class ListGatewaysResponse(_message.Message):
    __slots__ = ["total_count", "result"]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    total_count: int
    result: _containers.RepeatedCompositeFieldContainer[GatewayListItem]
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[GatewayListItem, _Mapping]]] = ...) -> None: ...

class GenerateGatewayClientCertificateRequest(_message.Message):
    __slots__ = ["gateway_id"]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    def __init__(self, gateway_id: _Optional[str] = ...) -> None: ...

class GenerateGatewayClientCertificateResponse(_message.Message):
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

class GetGatewayMetricsRequest(_message.Message):
    __slots__ = ["gateway_id", "start", "end", "aggregation"]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    START_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    AGGREGATION_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    start: _timestamp_pb2.Timestamp
    end: _timestamp_pb2.Timestamp
    aggregation: _common_pb2.Aggregation
    def __init__(self, gateway_id: _Optional[str] = ..., start: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., aggregation: _Optional[_Union[_common_pb2.Aggregation, str]] = ...) -> None: ...

class GetGatewayMetricsResponse(_message.Message):
    __slots__ = ["rx_packets", "tx_packets", "tx_packets_per_freq", "rx_packets_per_freq", "tx_packets_per_dr", "rx_packets_per_dr", "tx_packets_per_status"]
    RX_PACKETS_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_PER_FREQ_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_PER_FREQ_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_PER_DR_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_PER_DR_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_PER_STATUS_FIELD_NUMBER: _ClassVar[int]
    rx_packets: _common_pb2.Metric
    tx_packets: _common_pb2.Metric
    tx_packets_per_freq: _common_pb2.Metric
    rx_packets_per_freq: _common_pb2.Metric
    tx_packets_per_dr: _common_pb2.Metric
    rx_packets_per_dr: _common_pb2.Metric
    tx_packets_per_status: _common_pb2.Metric
    def __init__(self, rx_packets: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., tx_packets: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., tx_packets_per_freq: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., rx_packets_per_freq: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., tx_packets_per_dr: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., rx_packets_per_dr: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., tx_packets_per_status: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ...) -> None: ...
