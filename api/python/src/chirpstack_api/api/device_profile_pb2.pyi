from google.api import annotations_pb2 as _annotations_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from chirpstack_api.common import common_pb2 as _common_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

ABSOLUTE: MeasurementKind
CAYENNE_LPP: CodecRuntime
COUNTER: MeasurementKind
DESCRIPTOR: _descriptor.FileDescriptor
GAUGE: MeasurementKind
JS: CodecRuntime
NONE: CodecRuntime
STRING: MeasurementKind
UNKNOWN: MeasurementKind

class AdrAlgorithmListItem(_message.Message):
    __slots__ = ["id", "name"]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    id: str
    name: str
    def __init__(self, id: _Optional[str] = ..., name: _Optional[str] = ...) -> None: ...

class CreateDeviceProfileRequest(_message.Message):
    __slots__ = ["device_profile"]
    DEVICE_PROFILE_FIELD_NUMBER: _ClassVar[int]
    device_profile: DeviceProfile
    def __init__(self, device_profile: _Optional[_Union[DeviceProfile, _Mapping]] = ...) -> None: ...

class CreateDeviceProfileResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class DeleteDeviceProfileRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class DeviceProfile(_message.Message):
    __slots__ = ["abp_rx1_delay", "abp_rx1_dr_offset", "abp_rx2_dr", "abp_rx2_freq", "adr_algorithm_id", "auto_detect_measurements", "class_b_ping_slot_dr", "class_b_ping_slot_freq", "class_b_ping_slot_nb_k", "class_b_timeout", "class_c_timeout", "description", "device_status_req_interval", "flush_queue_on_activate", "id", "mac_version", "measurements", "name", "payload_codec_runtime", "payload_codec_script", "reg_params_revision", "region", "region_config_id", "supports_class_b", "supports_class_c", "supports_otaa", "tags", "tenant_id", "uplink_interval"]
    class MeasurementsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: Measurement
        def __init__(self, key: _Optional[str] = ..., value: _Optional[_Union[Measurement, _Mapping]] = ...) -> None: ...
    class TagsEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    ABP_RX1_DELAY_FIELD_NUMBER: _ClassVar[int]
    ABP_RX1_DR_OFFSET_FIELD_NUMBER: _ClassVar[int]
    ABP_RX2_DR_FIELD_NUMBER: _ClassVar[int]
    ABP_RX2_FREQ_FIELD_NUMBER: _ClassVar[int]
    ADR_ALGORITHM_ID_FIELD_NUMBER: _ClassVar[int]
    AUTO_DETECT_MEASUREMENTS_FIELD_NUMBER: _ClassVar[int]
    CLASS_B_PING_SLOT_DR_FIELD_NUMBER: _ClassVar[int]
    CLASS_B_PING_SLOT_FREQ_FIELD_NUMBER: _ClassVar[int]
    CLASS_B_PING_SLOT_NB_K_FIELD_NUMBER: _ClassVar[int]
    CLASS_B_TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    CLASS_C_TIMEOUT_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    DEVICE_STATUS_REQ_INTERVAL_FIELD_NUMBER: _ClassVar[int]
    FLUSH_QUEUE_ON_ACTIVATE_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    MAC_VERSION_FIELD_NUMBER: _ClassVar[int]
    MEASUREMENTS_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    PAYLOAD_CODEC_RUNTIME_FIELD_NUMBER: _ClassVar[int]
    PAYLOAD_CODEC_SCRIPT_FIELD_NUMBER: _ClassVar[int]
    REGION_CONFIG_ID_FIELD_NUMBER: _ClassVar[int]
    REGION_FIELD_NUMBER: _ClassVar[int]
    REG_PARAMS_REVISION_FIELD_NUMBER: _ClassVar[int]
    SUPPORTS_CLASS_B_FIELD_NUMBER: _ClassVar[int]
    SUPPORTS_CLASS_C_FIELD_NUMBER: _ClassVar[int]
    SUPPORTS_OTAA_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    UPLINK_INTERVAL_FIELD_NUMBER: _ClassVar[int]
    abp_rx1_delay: int
    abp_rx1_dr_offset: int
    abp_rx2_dr: int
    abp_rx2_freq: int
    adr_algorithm_id: str
    auto_detect_measurements: bool
    class_b_ping_slot_dr: int
    class_b_ping_slot_freq: int
    class_b_ping_slot_nb_k: int
    class_b_timeout: int
    class_c_timeout: int
    description: str
    device_status_req_interval: int
    flush_queue_on_activate: bool
    id: str
    mac_version: _common_pb2.MacVersion
    measurements: _containers.MessageMap[str, Measurement]
    name: str
    payload_codec_runtime: CodecRuntime
    payload_codec_script: str
    reg_params_revision: _common_pb2.RegParamsRevision
    region: _common_pb2.Region
    region_config_id: str
    supports_class_b: bool
    supports_class_c: bool
    supports_otaa: bool
    tags: _containers.ScalarMap[str, str]
    tenant_id: str
    uplink_interval: int
    def __init__(self, id: _Optional[str] = ..., tenant_id: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., region: _Optional[_Union[_common_pb2.Region, str]] = ..., mac_version: _Optional[_Union[_common_pb2.MacVersion, str]] = ..., reg_params_revision: _Optional[_Union[_common_pb2.RegParamsRevision, str]] = ..., adr_algorithm_id: _Optional[str] = ..., payload_codec_runtime: _Optional[_Union[CodecRuntime, str]] = ..., payload_codec_script: _Optional[str] = ..., flush_queue_on_activate: bool = ..., uplink_interval: _Optional[int] = ..., device_status_req_interval: _Optional[int] = ..., supports_otaa: bool = ..., supports_class_b: bool = ..., supports_class_c: bool = ..., class_b_timeout: _Optional[int] = ..., class_b_ping_slot_nb_k: _Optional[int] = ..., class_b_ping_slot_dr: _Optional[int] = ..., class_b_ping_slot_freq: _Optional[int] = ..., class_c_timeout: _Optional[int] = ..., abp_rx1_delay: _Optional[int] = ..., abp_rx1_dr_offset: _Optional[int] = ..., abp_rx2_dr: _Optional[int] = ..., abp_rx2_freq: _Optional[int] = ..., tags: _Optional[_Mapping[str, str]] = ..., measurements: _Optional[_Mapping[str, Measurement]] = ..., auto_detect_measurements: bool = ..., region_config_id: _Optional[str] = ...) -> None: ...

class DeviceProfileListItem(_message.Message):
    __slots__ = ["created_at", "id", "mac_version", "name", "reg_params_revision", "region", "supports_class_b", "supports_class_c", "supports_otaa", "updated_at"]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    MAC_VERSION_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    REGION_FIELD_NUMBER: _ClassVar[int]
    REG_PARAMS_REVISION_FIELD_NUMBER: _ClassVar[int]
    SUPPORTS_CLASS_B_FIELD_NUMBER: _ClassVar[int]
    SUPPORTS_CLASS_C_FIELD_NUMBER: _ClassVar[int]
    SUPPORTS_OTAA_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    created_at: _timestamp_pb2.Timestamp
    id: str
    mac_version: _common_pb2.MacVersion
    name: str
    reg_params_revision: _common_pb2.RegParamsRevision
    region: _common_pb2.Region
    supports_class_b: bool
    supports_class_c: bool
    supports_otaa: bool
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, id: _Optional[str] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., name: _Optional[str] = ..., region: _Optional[_Union[_common_pb2.Region, str]] = ..., mac_version: _Optional[_Union[_common_pb2.MacVersion, str]] = ..., reg_params_revision: _Optional[_Union[_common_pb2.RegParamsRevision, str]] = ..., supports_otaa: bool = ..., supports_class_b: bool = ..., supports_class_c: bool = ...) -> None: ...

class GetDeviceProfileRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetDeviceProfileResponse(_message.Message):
    __slots__ = ["created_at", "device_profile", "updated_at"]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    DEVICE_PROFILE_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    created_at: _timestamp_pb2.Timestamp
    device_profile: DeviceProfile
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, device_profile: _Optional[_Union[DeviceProfile, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class ListDeviceProfileAdrAlgorithmsResponse(_message.Message):
    __slots__ = ["result", "total_count"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    result: _containers.RepeatedCompositeFieldContainer[AdrAlgorithmListItem]
    total_count: int
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[AdrAlgorithmListItem, _Mapping]]] = ...) -> None: ...

class ListDeviceProfilesRequest(_message.Message):
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

class ListDeviceProfilesResponse(_message.Message):
    __slots__ = ["result", "total_count"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    result: _containers.RepeatedCompositeFieldContainer[DeviceProfileListItem]
    total_count: int
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[DeviceProfileListItem, _Mapping]]] = ...) -> None: ...

class Measurement(_message.Message):
    __slots__ = ["kind", "name"]
    KIND_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    kind: MeasurementKind
    name: str
    def __init__(self, name: _Optional[str] = ..., kind: _Optional[_Union[MeasurementKind, str]] = ...) -> None: ...

class UpdateDeviceProfileRequest(_message.Message):
    __slots__ = ["device_profile"]
    DEVICE_PROFILE_FIELD_NUMBER: _ClassVar[int]
    device_profile: DeviceProfile
    def __init__(self, device_profile: _Optional[_Union[DeviceProfile, _Mapping]] = ...) -> None: ...

class CodecRuntime(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class MeasurementKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
