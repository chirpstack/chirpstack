from chirpstack_api.common import common_pb2 as _common_pb2
from google.api import annotations_pb2 as _annotations_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import struct_pb2 as _struct_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ActivateDeviceRequest(_message.Message):
    __slots__ = ["device_activation"]
    DEVICE_ACTIVATION_FIELD_NUMBER: _ClassVar[int]
    device_activation: DeviceActivation
    def __init__(self, device_activation: _Optional[_Union[DeviceActivation, _Mapping]] = ...) -> None: ...

class CreateDeviceKeysRequest(_message.Message):
    __slots__ = ["device_keys"]
    DEVICE_KEYS_FIELD_NUMBER: _ClassVar[int]
    device_keys: DeviceKeys
    def __init__(self, device_keys: _Optional[_Union[DeviceKeys, _Mapping]] = ...) -> None: ...

class CreateDeviceRequest(_message.Message):
    __slots__ = ["device"]
    DEVICE_FIELD_NUMBER: _ClassVar[int]
    device: Device
    def __init__(self, device: _Optional[_Union[Device, _Mapping]] = ...) -> None: ...

class DeactivateDeviceRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class DeleteDeviceKeysRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class DeleteDeviceRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class Device(_message.Message):
    __slots__ = ["application_id", "description", "dev_eui", "device_profile_id", "is_disabled", "join_eui", "name", "skip_fcnt_check", "tags", "variables"]
    class TagsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    class VariablesEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    DEVICE_PROFILE_ID_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    IS_DISABLED_FIELD_NUMBER: _ClassVar[int]
    JOIN_EUI_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    SKIP_FCNT_CHECK_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    VARIABLES_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    description: str
    dev_eui: str
    device_profile_id: str
    is_disabled: bool
    join_eui: str
    name: str
    skip_fcnt_check: bool
    tags: _containers.ScalarMap[str, str]
    variables: _containers.ScalarMap[str, str]
    def __init__(self, dev_eui: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., application_id: _Optional[str] = ..., device_profile_id: _Optional[str] = ..., skip_fcnt_check: bool = ..., is_disabled: bool = ..., variables: _Optional[_Mapping[str, str]] = ..., tags: _Optional[_Mapping[str, str]] = ..., join_eui: _Optional[str] = ...) -> None: ...

class DeviceActivation(_message.Message):
    __slots__ = ["a_f_cnt_down", "app_s_key", "dev_addr", "dev_eui", "f_cnt_up", "f_nwk_s_int_key", "n_f_cnt_down", "nwk_s_enc_key", "s_nwk_s_int_key"]
    APP_S_KEY_FIELD_NUMBER: _ClassVar[int]
    A_F_CNT_DOWN_FIELD_NUMBER: _ClassVar[int]
    DEV_ADDR_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    F_CNT_UP_FIELD_NUMBER: _ClassVar[int]
    F_NWK_S_INT_KEY_FIELD_NUMBER: _ClassVar[int]
    NWK_S_ENC_KEY_FIELD_NUMBER: _ClassVar[int]
    N_F_CNT_DOWN_FIELD_NUMBER: _ClassVar[int]
    S_NWK_S_INT_KEY_FIELD_NUMBER: _ClassVar[int]
    a_f_cnt_down: int
    app_s_key: str
    dev_addr: str
    dev_eui: str
    f_cnt_up: int
    f_nwk_s_int_key: str
    n_f_cnt_down: int
    nwk_s_enc_key: str
    s_nwk_s_int_key: str
    def __init__(self, dev_eui: _Optional[str] = ..., dev_addr: _Optional[str] = ..., app_s_key: _Optional[str] = ..., nwk_s_enc_key: _Optional[str] = ..., s_nwk_s_int_key: _Optional[str] = ..., f_nwk_s_int_key: _Optional[str] = ..., f_cnt_up: _Optional[int] = ..., n_f_cnt_down: _Optional[int] = ..., a_f_cnt_down: _Optional[int] = ...) -> None: ...

class DeviceKeys(_message.Message):
    __slots__ = ["app_key", "dev_eui", "nwk_key"]
    APP_KEY_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    NWK_KEY_FIELD_NUMBER: _ClassVar[int]
    app_key: str
    dev_eui: str
    nwk_key: str
    def __init__(self, dev_eui: _Optional[str] = ..., nwk_key: _Optional[str] = ..., app_key: _Optional[str] = ...) -> None: ...

class DeviceListItem(_message.Message):
    __slots__ = ["created_at", "description", "dev_eui", "device_profile_id", "device_profile_name", "device_status", "last_seen_at", "name", "updated_at"]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    DEVICE_PROFILE_ID_FIELD_NUMBER: _ClassVar[int]
    DEVICE_PROFILE_NAME_FIELD_NUMBER: _ClassVar[int]
    DEVICE_STATUS_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    LAST_SEEN_AT_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    created_at: _timestamp_pb2.Timestamp
    description: str
    dev_eui: str
    device_profile_id: str
    device_profile_name: str
    device_status: DeviceStatus
    last_seen_at: _timestamp_pb2.Timestamp
    name: str
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, dev_eui: _Optional[str] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., last_seen_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., device_profile_id: _Optional[str] = ..., device_profile_name: _Optional[str] = ..., device_status: _Optional[_Union[DeviceStatus, _Mapping]] = ...) -> None: ...

class DeviceQueueItem(_message.Message):
    __slots__ = ["confirmed", "data", "dev_eui", "f_cnt_down", "f_port", "id", "is_pending", "object"]
    CONFIRMED_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    F_CNT_DOWN_FIELD_NUMBER: _ClassVar[int]
    F_PORT_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    IS_PENDING_FIELD_NUMBER: _ClassVar[int]
    OBJECT_FIELD_NUMBER: _ClassVar[int]
    confirmed: bool
    data: bytes
    dev_eui: str
    f_cnt_down: int
    f_port: int
    id: str
    is_pending: bool
    object: _struct_pb2.Struct
    def __init__(self, id: _Optional[str] = ..., dev_eui: _Optional[str] = ..., confirmed: bool = ..., f_port: _Optional[int] = ..., data: _Optional[bytes] = ..., object: _Optional[_Union[_struct_pb2.Struct, _Mapping]] = ..., is_pending: bool = ..., f_cnt_down: _Optional[int] = ...) -> None: ...

class DeviceState(_message.Message):
    __slots__ = ["name", "value"]
    NAME_FIELD_NUMBER: _ClassVar[int]
    VALUE_FIELD_NUMBER: _ClassVar[int]
    name: str
    value: str
    def __init__(self, name: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...

class DeviceStatus(_message.Message):
    __slots__ = ["battery_level", "external_power_source", "margin"]
    BATTERY_LEVEL_FIELD_NUMBER: _ClassVar[int]
    EXTERNAL_POWER_SOURCE_FIELD_NUMBER: _ClassVar[int]
    MARGIN_FIELD_NUMBER: _ClassVar[int]
    battery_level: float
    external_power_source: bool
    margin: int
    def __init__(self, margin: _Optional[int] = ..., external_power_source: bool = ..., battery_level: _Optional[float] = ...) -> None: ...

class EnqueueDeviceQueueItemRequest(_message.Message):
    __slots__ = ["queue_item"]
    QUEUE_ITEM_FIELD_NUMBER: _ClassVar[int]
    queue_item: DeviceQueueItem
    def __init__(self, queue_item: _Optional[_Union[DeviceQueueItem, _Mapping]] = ...) -> None: ...

class EnqueueDeviceQueueItemResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class FlushDevNoncesRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class FlushDeviceQueueRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class GetDeviceActivationRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class GetDeviceActivationResponse(_message.Message):
    __slots__ = ["device_activation"]
    DEVICE_ACTIVATION_FIELD_NUMBER: _ClassVar[int]
    device_activation: DeviceActivation
    def __init__(self, device_activation: _Optional[_Union[DeviceActivation, _Mapping]] = ...) -> None: ...

class GetDeviceKeysRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class GetDeviceKeysResponse(_message.Message):
    __slots__ = ["created_at", "device_keys", "updated_at"]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    DEVICE_KEYS_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    created_at: _timestamp_pb2.Timestamp
    device_keys: DeviceKeys
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, device_keys: _Optional[_Union[DeviceKeys, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class GetDeviceLinkMetricsRequest(_message.Message):
    __slots__ = ["aggregation", "dev_eui", "end", "start"]
    AGGREGATION_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    START_FIELD_NUMBER: _ClassVar[int]
    aggregation: _common_pb2.Aggregation
    dev_eui: str
    end: _timestamp_pb2.Timestamp
    start: _timestamp_pb2.Timestamp
    def __init__(self, dev_eui: _Optional[str] = ..., start: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., aggregation: _Optional[_Union[_common_pb2.Aggregation, str]] = ...) -> None: ...

class GetDeviceLinkMetricsResponse(_message.Message):
    __slots__ = ["errors", "gw_rssi", "gw_snr", "rx_packets", "rx_packets_per_dr", "rx_packets_per_freq"]
    ERRORS_FIELD_NUMBER: _ClassVar[int]
    GW_RSSI_FIELD_NUMBER: _ClassVar[int]
    GW_SNR_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_PER_DR_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_PER_FREQ_FIELD_NUMBER: _ClassVar[int]
    errors: _common_pb2.Metric
    gw_rssi: _common_pb2.Metric
    gw_snr: _common_pb2.Metric
    rx_packets: _common_pb2.Metric
    rx_packets_per_dr: _common_pb2.Metric
    rx_packets_per_freq: _common_pb2.Metric
    def __init__(self, rx_packets: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., gw_rssi: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., gw_snr: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., rx_packets_per_freq: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., rx_packets_per_dr: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ..., errors: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ...) -> None: ...

class GetDeviceMetricsRequest(_message.Message):
    __slots__ = ["aggregation", "dev_eui", "end", "start"]
    AGGREGATION_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    END_FIELD_NUMBER: _ClassVar[int]
    START_FIELD_NUMBER: _ClassVar[int]
    aggregation: _common_pb2.Aggregation
    dev_eui: str
    end: _timestamp_pb2.Timestamp
    start: _timestamp_pb2.Timestamp
    def __init__(self, dev_eui: _Optional[str] = ..., start: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., end: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., aggregation: _Optional[_Union[_common_pb2.Aggregation, str]] = ...) -> None: ...

class GetDeviceMetricsResponse(_message.Message):
    __slots__ = ["metrics", "states"]
    class MetricsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: _common_pb2.Metric
        def __init__(self, key: _Optional[str] = ..., value: _Optional[_Union[_common_pb2.Metric, _Mapping]] = ...) -> None: ...
    class StatesEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: DeviceState
        def __init__(self, key: _Optional[str] = ..., value: _Optional[_Union[DeviceState, _Mapping]] = ...) -> None: ...
    METRICS_FIELD_NUMBER: _ClassVar[int]
    STATES_FIELD_NUMBER: _ClassVar[int]
    metrics: _containers.MessageMap[str, _common_pb2.Metric]
    states: _containers.MessageMap[str, DeviceState]
    def __init__(self, metrics: _Optional[_Mapping[str, _common_pb2.Metric]] = ..., states: _Optional[_Mapping[str, DeviceState]] = ...) -> None: ...

class GetDeviceQueueItemsRequest(_message.Message):
    __slots__ = ["count_only", "dev_eui"]
    COUNT_ONLY_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    count_only: bool
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ..., count_only: bool = ...) -> None: ...

class GetDeviceQueueItemsResponse(_message.Message):
    __slots__ = ["result", "total_count"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    result: _containers.RepeatedCompositeFieldContainer[DeviceQueueItem]
    total_count: int
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[DeviceQueueItem, _Mapping]]] = ...) -> None: ...

class GetDeviceRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class GetDeviceResponse(_message.Message):
    __slots__ = ["created_at", "device", "device_status", "last_seen_at", "updated_at"]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    DEVICE_FIELD_NUMBER: _ClassVar[int]
    DEVICE_STATUS_FIELD_NUMBER: _ClassVar[int]
    LAST_SEEN_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    created_at: _timestamp_pb2.Timestamp
    device: Device
    device_status: DeviceStatus
    last_seen_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, device: _Optional[_Union[Device, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., last_seen_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., device_status: _Optional[_Union[DeviceStatus, _Mapping]] = ...) -> None: ...

class GetRandomDevAddrRequest(_message.Message):
    __slots__ = ["dev_eui"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    def __init__(self, dev_eui: _Optional[str] = ...) -> None: ...

class GetRandomDevAddrResponse(_message.Message):
    __slots__ = ["dev_addr"]
    DEV_ADDR_FIELD_NUMBER: _ClassVar[int]
    dev_addr: str
    def __init__(self, dev_addr: _Optional[str] = ...) -> None: ...

class ListDevicesRequest(_message.Message):
    __slots__ = ["application_id", "limit", "multicast_group_id", "offset", "search"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    SEARCH_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    limit: int
    multicast_group_id: str
    offset: int
    search: str
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., search: _Optional[str] = ..., application_id: _Optional[str] = ..., multicast_group_id: _Optional[str] = ...) -> None: ...

class ListDevicesResponse(_message.Message):
    __slots__ = ["result", "total_count"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    result: _containers.RepeatedCompositeFieldContainer[DeviceListItem]
    total_count: int
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[DeviceListItem, _Mapping]]] = ...) -> None: ...

class UpdateDeviceKeysRequest(_message.Message):
    __slots__ = ["device_keys"]
    DEVICE_KEYS_FIELD_NUMBER: _ClassVar[int]
    device_keys: DeviceKeys
    def __init__(self, device_keys: _Optional[_Union[DeviceKeys, _Mapping]] = ...) -> None: ...

class UpdateDeviceRequest(_message.Message):
    __slots__ = ["device"]
    DEVICE_FIELD_NUMBER: _ClassVar[int]
    device: Device
    def __init__(self, device: _Optional[_Union[Device, _Mapping]] = ...) -> None: ...
