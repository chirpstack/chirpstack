from chirpstack_api.common import common_pb2 as _common_pb2
from chirpstack_api.gw import gw_pb2 as _gw_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import struct_pb2 as _struct_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class LogLevel(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    INFO: _ClassVar[LogLevel]
    WARNING: _ClassVar[LogLevel]
    ERROR: _ClassVar[LogLevel]

class LogCode(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    UNKNOWN: _ClassVar[LogCode]
    DOWNLINK_PAYLOAD_SIZE: _ClassVar[LogCode]
    UPLINK_CODEC: _ClassVar[LogCode]
    DOWNLINK_CODEC: _ClassVar[LogCode]
    OTAA: _ClassVar[LogCode]
    UPLINK_F_CNT_RESET: _ClassVar[LogCode]
    UPLINK_MIC: _ClassVar[LogCode]
    UPLINK_F_CNT_RETRANSMISSION: _ClassVar[LogCode]
    DOWNLINK_GATEWAY: _ClassVar[LogCode]
    RELAY_NEW_END_DEVICE: _ClassVar[LogCode]
    F_CNT_DOWN: _ClassVar[LogCode]
INFO: LogLevel
WARNING: LogLevel
ERROR: LogLevel
UNKNOWN: LogCode
DOWNLINK_PAYLOAD_SIZE: LogCode
UPLINK_CODEC: LogCode
DOWNLINK_CODEC: LogCode
OTAA: LogCode
UPLINK_F_CNT_RESET: LogCode
UPLINK_MIC: LogCode
UPLINK_F_CNT_RETRANSMISSION: LogCode
DOWNLINK_GATEWAY: LogCode
RELAY_NEW_END_DEVICE: LogCode
F_CNT_DOWN: LogCode

class DeviceInfo(_message.Message):
    __slots__ = ["tenant_id", "tenant_name", "application_id", "application_name", "device_profile_id", "device_profile_name", "device_name", "dev_eui", "device_class_enabled", "tags"]
    class TagsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    TENANT_NAME_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_NAME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_PROFILE_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_PROFILE_NAME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_NAME_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    DEVICE_CLASS_ENABLED_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    tenant_name: str
    application_id: str
    application_name: str
    device_profile_id: str
    device_profile_name: str
    device_name: str
    dev_eui: str
    device_class_enabled: _common_pb2.DeviceClass
    tags: _containers.ScalarMap[str, str]
    def __init__(self, tenant_id: _Optional[str] = ..., tenant_name: _Optional[str] = ..., application_id: _Optional[str] = ..., application_name: _Optional[str] = ..., device_profile_id: _Optional[str] = ..., device_profile_name: _Optional[str] = ..., device_name: _Optional[str] = ..., dev_eui: _Optional[str] = ..., device_class_enabled: _Optional[_Union[_common_pb2.DeviceClass, str]] = ..., tags: _Optional[_Mapping[str, str]] = ...) -> None: ...

class UplinkRelayRxInfo(_message.Message):
    __slots__ = ["dev_eui", "frequency", "dr", "snr", "rssi", "wor_channel"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    DR_FIELD_NUMBER: _ClassVar[int]
    SNR_FIELD_NUMBER: _ClassVar[int]
    RSSI_FIELD_NUMBER: _ClassVar[int]
    WOR_CHANNEL_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    frequency: int
    dr: int
    snr: int
    rssi: int
    wor_channel: int
    def __init__(self, dev_eui: _Optional[str] = ..., frequency: _Optional[int] = ..., dr: _Optional[int] = ..., snr: _Optional[int] = ..., rssi: _Optional[int] = ..., wor_channel: _Optional[int] = ...) -> None: ...

class JoinServerContext(_message.Message):
    __slots__ = ["session_key_id", "app_s_key"]
    SESSION_KEY_ID_FIELD_NUMBER: _ClassVar[int]
    APP_S_KEY_FIELD_NUMBER: _ClassVar[int]
    session_key_id: str
    app_s_key: _common_pb2.KeyEnvelope
    def __init__(self, session_key_id: _Optional[str] = ..., app_s_key: _Optional[_Union[_common_pb2.KeyEnvelope, _Mapping]] = ...) -> None: ...

class UplinkEvent(_message.Message):
    __slots__ = ["deduplication_id", "time", "device_info", "dev_addr", "adr", "dr", "f_cnt", "f_port", "confirmed", "data", "object", "rx_info", "tx_info", "relay_rx_info", "join_server_context"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    DEV_ADDR_FIELD_NUMBER: _ClassVar[int]
    ADR_FIELD_NUMBER: _ClassVar[int]
    DR_FIELD_NUMBER: _ClassVar[int]
    F_CNT_FIELD_NUMBER: _ClassVar[int]
    F_PORT_FIELD_NUMBER: _ClassVar[int]
    CONFIRMED_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    RX_INFO_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    RELAY_RX_INFO_FIELD_NUMBER: _ClassVar[int]
    JOIN_SERVER_CONTEXT_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    time: _timestamp_pb2.Timestamp
    device_info: DeviceInfo
    dev_addr: str
    adr: bool
    dr: int
    f_cnt: int
    f_port: int
    confirmed: bool
    data: bytes
    object: _struct_pb2.Struct
    rx_info: _containers.RepeatedCompositeFieldContainer[_gw_pb2.UplinkRxInfo]
    tx_info: _gw_pb2.UplinkTxInfo
    relay_rx_info: UplinkRelayRxInfo
    join_server_context: JoinServerContext
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., dev_addr: _Optional[str] = ..., adr: bool = ..., dr: _Optional[int] = ..., f_cnt: _Optional[int] = ..., f_port: _Optional[int] = ..., confirmed: bool = ..., data: _Optional[bytes] = ..., object: _Optional[_Union[_struct_pb2.Struct, _Mapping]] = ..., rx_info: _Optional[_Iterable[_Union[_gw_pb2.UplinkRxInfo, _Mapping]]] = ..., tx_info: _Optional[_Union[_gw_pb2.UplinkTxInfo, _Mapping]] = ..., relay_rx_info: _Optional[_Union[UplinkRelayRxInfo, _Mapping]] = ..., join_server_context: _Optional[_Union[JoinServerContext, _Mapping]] = ...) -> None: ...

class JoinEvent(_message.Message):
    __slots__ = ["deduplication_id", "time", "device_info", "dev_addr", "relay_rx_info", "join_server_context"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    DEV_ADDR_FIELD_NUMBER: _ClassVar[int]
    RELAY_RX_INFO_FIELD_NUMBER: _ClassVar[int]
    JOIN_SERVER_CONTEXT_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    time: _timestamp_pb2.Timestamp
    device_info: DeviceInfo
    dev_addr: str
    relay_rx_info: UplinkRelayRxInfo
    join_server_context: JoinServerContext
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., dev_addr: _Optional[str] = ..., relay_rx_info: _Optional[_Union[UplinkRelayRxInfo, _Mapping]] = ..., join_server_context: _Optional[_Union[JoinServerContext, _Mapping]] = ...) -> None: ...

class AckEvent(_message.Message):
    __slots__ = ["deduplication_id", "time", "device_info", "queue_item_id", "acknowledged", "f_cnt_down"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    QUEUE_ITEM_ID_FIELD_NUMBER: _ClassVar[int]
    ACKNOWLEDGED_FIELD_NUMBER: _ClassVar[int]
    F_CNT_DOWN_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    time: _timestamp_pb2.Timestamp
    device_info: DeviceInfo
    queue_item_id: str
    acknowledged: bool
    f_cnt_down: int
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., queue_item_id: _Optional[str] = ..., acknowledged: bool = ..., f_cnt_down: _Optional[int] = ...) -> None: ...

class TxAckEvent(_message.Message):
    __slots__ = ["downlink_id", "time", "device_info", "queue_item_id", "f_cnt_down", "gateway_id", "tx_info"]
    DOWNLINK_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    QUEUE_ITEM_ID_FIELD_NUMBER: _ClassVar[int]
    F_CNT_DOWN_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    downlink_id: int
    time: _timestamp_pb2.Timestamp
    device_info: DeviceInfo
    queue_item_id: str
    f_cnt_down: int
    gateway_id: str
    tx_info: _gw_pb2.DownlinkTxInfo
    def __init__(self, downlink_id: _Optional[int] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., queue_item_id: _Optional[str] = ..., f_cnt_down: _Optional[int] = ..., gateway_id: _Optional[str] = ..., tx_info: _Optional[_Union[_gw_pb2.DownlinkTxInfo, _Mapping]] = ...) -> None: ...

class LogEvent(_message.Message):
    __slots__ = ["time", "device_info", "level", "code", "description", "context"]
    class ContextEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    TIME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    LEVEL_FIELD_NUMBER: _ClassVar[int]
    CODE_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    time: _timestamp_pb2.Timestamp
    device_info: DeviceInfo
    level: LogLevel
    code: LogCode
    description: str
    context: _containers.ScalarMap[str, str]
    def __init__(self, time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., level: _Optional[_Union[LogLevel, str]] = ..., code: _Optional[_Union[LogCode, str]] = ..., description: _Optional[str] = ..., context: _Optional[_Mapping[str, str]] = ...) -> None: ...

class StatusEvent(_message.Message):
    __slots__ = ["deduplication_id", "time", "device_info", "margin", "external_power_source", "battery_level_unavailable", "battery_level"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    MARGIN_FIELD_NUMBER: _ClassVar[int]
    EXTERNAL_POWER_SOURCE_FIELD_NUMBER: _ClassVar[int]
    BATTERY_LEVEL_UNAVAILABLE_FIELD_NUMBER: _ClassVar[int]
    BATTERY_LEVEL_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    time: _timestamp_pb2.Timestamp
    device_info: DeviceInfo
    margin: int
    external_power_source: bool
    battery_level_unavailable: bool
    battery_level: float
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., margin: _Optional[int] = ..., external_power_source: bool = ..., battery_level_unavailable: bool = ..., battery_level: _Optional[float] = ...) -> None: ...

class LocationEvent(_message.Message):
    __slots__ = ["deduplication_id", "time", "device_info", "location"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    time: _timestamp_pb2.Timestamp
    device_info: DeviceInfo
    location: _common_pb2.Location
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., location: _Optional[_Union[_common_pb2.Location, _Mapping]] = ...) -> None: ...

class IntegrationEvent(_message.Message):
    __slots__ = ["deduplication_id", "time", "device_info", "integration_name", "event_type", "object"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    INTEGRATION_NAME_FIELD_NUMBER: _ClassVar[int]
    EVENT_TYPE_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    time: _timestamp_pb2.Timestamp
    device_info: DeviceInfo
    integration_name: str
    event_type: str
    object: _struct_pb2.Struct
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., integration_name: _Optional[str] = ..., event_type: _Optional[str] = ..., object: _Optional[_Union[_struct_pb2.Struct, _Mapping]] = ...) -> None: ...

class DownlinkCommand(_message.Message):
    __slots__ = ["id", "dev_eui", "confirmed", "f_port", "data", "object"]
    ID_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    CONFIRMED_FIELD_NUMBER: _ClassVar[int]
    F_PORT_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    id: str
    dev_eui: str
    confirmed: bool
    f_port: int
    data: bytes
    object: _struct_pb2.Struct
    def __init__(self, id: _Optional[str] = ..., dev_eui: _Optional[str] = ..., confirmed: bool = ..., f_port: _Optional[int] = ..., data: _Optional[bytes] = ..., object: _Optional[_Union[_struct_pb2.Struct, _Mapping]] = ...) -> None: ...
