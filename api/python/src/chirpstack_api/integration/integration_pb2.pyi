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
DOWNLINK_CODEC: LogCode
DOWNLINK_GATEWAY: LogCode
DOWNLINK_PAYLOAD_SIZE: LogCode
ERROR: LogLevel
INFO: LogLevel
OTAA: LogCode
RELAY_NEW_END_DEVICE: LogCode
UNKNOWN: LogCode
UPLINK_CODEC: LogCode
UPLINK_F_CNT_RESET: LogCode
UPLINK_F_CNT_RETRANSMISSION: LogCode
UPLINK_MIC: LogCode
WARNING: LogLevel

class AckEvent(_message.Message):
    __slots__ = ["acknowledged", "deduplication_id", "device_info", "f_cnt_down", "queue_item_id", "time"]
    ACKNOWLEDGED_FIELD_NUMBER: _ClassVar[int]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    F_CNT_DOWN_FIELD_NUMBER: _ClassVar[int]
    QUEUE_ITEM_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    acknowledged: bool
    deduplication_id: str
    device_info: DeviceInfo
    f_cnt_down: int
    queue_item_id: str
    time: _timestamp_pb2.Timestamp
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., queue_item_id: _Optional[str] = ..., acknowledged: bool = ..., f_cnt_down: _Optional[int] = ...) -> None: ...

class DeviceInfo(_message.Message):
    __slots__ = ["application_id", "application_name", "dev_eui", "device_name", "device_profile_id", "device_profile_name", "tags", "tenant_id", "tenant_name"]
    class TagsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_NAME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_NAME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_PROFILE_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_PROFILE_NAME_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    TENANT_NAME_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    application_name: str
    dev_eui: str
    device_name: str
    device_profile_id: str
    device_profile_name: str
    tags: _containers.ScalarMap[str, str]
    tenant_id: str
    tenant_name: str
    def __init__(self, tenant_id: _Optional[str] = ..., tenant_name: _Optional[str] = ..., application_id: _Optional[str] = ..., application_name: _Optional[str] = ..., device_profile_id: _Optional[str] = ..., device_profile_name: _Optional[str] = ..., device_name: _Optional[str] = ..., dev_eui: _Optional[str] = ..., tags: _Optional[_Mapping[str, str]] = ...) -> None: ...

class DownlinkCommand(_message.Message):
    __slots__ = ["confirmed", "data", "dev_eui", "f_port", "id", "object"]
    CONFIRMED_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    F_PORT_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    confirmed: bool
    data: bytes
    dev_eui: str
    f_port: int
    id: str
    object: _struct_pb2.Struct
    def __init__(self, id: _Optional[str] = ..., dev_eui: _Optional[str] = ..., confirmed: bool = ..., f_port: _Optional[int] = ..., data: _Optional[bytes] = ..., object: _Optional[_Union[_struct_pb2.Struct, _Mapping]] = ...) -> None: ...

class IntegrationEvent(_message.Message):
    __slots__ = ["deduplication_id", "device_info", "event_type", "integration_name", "object", "time"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    EVENT_TYPE_FIELD_NUMBER: _ClassVar[int]
    INTEGRATION_NAME_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    device_info: DeviceInfo
    event_type: str
    integration_name: str
    object: _struct_pb2.Struct
    time: _timestamp_pb2.Timestamp
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., integration_name: _Optional[str] = ..., event_type: _Optional[str] = ..., object: _Optional[_Union[_struct_pb2.Struct, _Mapping]] = ...) -> None: ...

class JoinEvent(_message.Message):
    __slots__ = ["deduplication_id", "dev_addr", "device_info", "relay_rx_info", "time"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    DEV_ADDR_FIELD_NUMBER: _ClassVar[int]
    RELAY_RX_INFO_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    dev_addr: str
    device_info: DeviceInfo
    relay_rx_info: UplinkRelayRxInfo
    time: _timestamp_pb2.Timestamp
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., dev_addr: _Optional[str] = ..., relay_rx_info: _Optional[_Union[UplinkRelayRxInfo, _Mapping]] = ...) -> None: ...

class LocationEvent(_message.Message):
    __slots__ = ["deduplication_id", "device_info", "location", "time"]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    deduplication_id: str
    device_info: DeviceInfo
    location: _common_pb2.Location
    time: _timestamp_pb2.Timestamp
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., location: _Optional[_Union[_common_pb2.Location, _Mapping]] = ...) -> None: ...

class LogEvent(_message.Message):
    __slots__ = ["code", "context", "description", "device_info", "level", "time"]
    class ContextEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    CODE_FIELD_NUMBER: _ClassVar[int]
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    LEVEL_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    code: LogCode
    context: _containers.ScalarMap[str, str]
    description: str
    device_info: DeviceInfo
    level: LogLevel
    time: _timestamp_pb2.Timestamp
    def __init__(self, time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., level: _Optional[_Union[LogLevel, str]] = ..., code: _Optional[_Union[LogCode, str]] = ..., description: _Optional[str] = ..., context: _Optional[_Mapping[str, str]] = ...) -> None: ...

class StatusEvent(_message.Message):
    __slots__ = ["battery_level", "battery_level_unavailable", "deduplication_id", "device_info", "external_power_source", "margin", "time"]
    BATTERY_LEVEL_FIELD_NUMBER: _ClassVar[int]
    BATTERY_LEVEL_UNAVAILABLE_FIELD_NUMBER: _ClassVar[int]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    EXTERNAL_POWER_SOURCE_FIELD_NUMBER: _ClassVar[int]
    MARGIN_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    battery_level: float
    battery_level_unavailable: bool
    deduplication_id: str
    device_info: DeviceInfo
    external_power_source: bool
    margin: int
    time: _timestamp_pb2.Timestamp
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., margin: _Optional[int] = ..., external_power_source: bool = ..., battery_level_unavailable: bool = ..., battery_level: _Optional[float] = ...) -> None: ...

class TxAckEvent(_message.Message):
    __slots__ = ["device_info", "downlink_id", "f_cnt_down", "gateway_id", "queue_item_id", "time", "tx_info"]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    DOWNLINK_ID_FIELD_NUMBER: _ClassVar[int]
    F_CNT_DOWN_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    QUEUE_ITEM_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    device_info: DeviceInfo
    downlink_id: int
    f_cnt_down: int
    gateway_id: str
    queue_item_id: str
    time: _timestamp_pb2.Timestamp
    tx_info: _gw_pb2.DownlinkTxInfo
    def __init__(self, downlink_id: _Optional[int] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., queue_item_id: _Optional[str] = ..., f_cnt_down: _Optional[int] = ..., gateway_id: _Optional[str] = ..., tx_info: _Optional[_Union[_gw_pb2.DownlinkTxInfo, _Mapping]] = ...) -> None: ...

class UplinkEvent(_message.Message):
    __slots__ = ["adr", "confirmed", "data", "deduplication_id", "dev_addr", "device_info", "dr", "f_cnt", "f_port", "object", "relay_rx_info", "rx_info", "time", "tx_info"]
    ADR_FIELD_NUMBER: _ClassVar[int]
    CONFIRMED_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    DEDUPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_INFO_FIELD_NUMBER: _ClassVar[int]
    DEV_ADDR_FIELD_NUMBER: _ClassVar[int]
    DR_FIELD_NUMBER: _ClassVar[int]
    F_CNT_FIELD_NUMBER: _ClassVar[int]
    F_PORT_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    RELAY_RX_INFO_FIELD_NUMBER: _ClassVar[int]
    RX_INFO_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    adr: bool
    confirmed: bool
    data: bytes
    deduplication_id: str
    dev_addr: str
    device_info: DeviceInfo
    dr: int
    f_cnt: int
    f_port: int
    object: _struct_pb2.Struct
    relay_rx_info: UplinkRelayRxInfo
    rx_info: _containers.RepeatedCompositeFieldContainer[_gw_pb2.UplinkRxInfo]
    time: _timestamp_pb2.Timestamp
    tx_info: _gw_pb2.UplinkTxInfo
    def __init__(self, deduplication_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_info: _Optional[_Union[DeviceInfo, _Mapping]] = ..., dev_addr: _Optional[str] = ..., adr: bool = ..., dr: _Optional[int] = ..., f_cnt: _Optional[int] = ..., f_port: _Optional[int] = ..., confirmed: bool = ..., data: _Optional[bytes] = ..., object: _Optional[_Union[_struct_pb2.Struct, _Mapping]] = ..., rx_info: _Optional[_Iterable[_Union[_gw_pb2.UplinkRxInfo, _Mapping]]] = ..., tx_info: _Optional[_Union[_gw_pb2.UplinkTxInfo, _Mapping]] = ..., relay_rx_info: _Optional[_Union[UplinkRelayRxInfo, _Mapping]] = ...) -> None: ...

class UplinkRelayRxInfo(_message.Message):
    __slots__ = ["dev_eui", "dr", "frequency", "rssi", "snr", "wor_channel"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    DR_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    RSSI_FIELD_NUMBER: _ClassVar[int]
    SNR_FIELD_NUMBER: _ClassVar[int]
    WOR_CHANNEL_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    dr: int
    frequency: int
    rssi: int
    snr: int
    wor_channel: int
    def __init__(self, dev_eui: _Optional[str] = ..., frequency: _Optional[int] = ..., dr: _Optional[int] = ..., snr: _Optional[int] = ..., rssi: _Optional[int] = ..., wor_channel: _Optional[int] = ...) -> None: ...

class LogLevel(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class LogCode(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
