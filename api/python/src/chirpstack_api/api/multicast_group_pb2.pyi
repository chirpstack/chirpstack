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

class MulticastGroupType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    CLASS_C: _ClassVar[MulticastGroupType]
    CLASS_B: _ClassVar[MulticastGroupType]

class MulticastGroupSchedulingType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    DELAY: _ClassVar[MulticastGroupSchedulingType]
    GPS_TIME: _ClassVar[MulticastGroupSchedulingType]
CLASS_C: MulticastGroupType
CLASS_B: MulticastGroupType
DELAY: MulticastGroupSchedulingType
GPS_TIME: MulticastGroupSchedulingType

class MulticastGroup(_message.Message):
    __slots__ = ["id", "name", "application_id", "region", "mc_addr", "mc_nwk_s_key", "mc_app_s_key", "f_cnt", "group_type", "dr", "frequency", "class_b_ping_slot_period", "class_c_scheduling_type"]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    REGION_FIELD_NUMBER: _ClassVar[int]
    MC_ADDR_FIELD_NUMBER: _ClassVar[int]
    MC_NWK_S_KEY_FIELD_NUMBER: _ClassVar[int]
    MC_APP_S_KEY_FIELD_NUMBER: _ClassVar[int]
    F_CNT_FIELD_NUMBER: _ClassVar[int]
    GROUP_TYPE_FIELD_NUMBER: _ClassVar[int]
    DR_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    CLASS_B_PING_SLOT_PERIOD_FIELD_NUMBER: _ClassVar[int]
    CLASS_C_SCHEDULING_TYPE_FIELD_NUMBER: _ClassVar[int]
    id: str
    name: str
    application_id: str
    region: _common_pb2.Region
    mc_addr: str
    mc_nwk_s_key: str
    mc_app_s_key: str
    f_cnt: int
    group_type: MulticastGroupType
    dr: int
    frequency: int
    class_b_ping_slot_period: int
    class_c_scheduling_type: MulticastGroupSchedulingType
    def __init__(self, id: _Optional[str] = ..., name: _Optional[str] = ..., application_id: _Optional[str] = ..., region: _Optional[_Union[_common_pb2.Region, str]] = ..., mc_addr: _Optional[str] = ..., mc_nwk_s_key: _Optional[str] = ..., mc_app_s_key: _Optional[str] = ..., f_cnt: _Optional[int] = ..., group_type: _Optional[_Union[MulticastGroupType, str]] = ..., dr: _Optional[int] = ..., frequency: _Optional[int] = ..., class_b_ping_slot_period: _Optional[int] = ..., class_c_scheduling_type: _Optional[_Union[MulticastGroupSchedulingType, str]] = ...) -> None: ...

class MulticastGroupListItem(_message.Message):
    __slots__ = ["id", "created_at", "updated_at", "name", "region", "group_type"]
    ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    REGION_FIELD_NUMBER: _ClassVar[int]
    GROUP_TYPE_FIELD_NUMBER: _ClassVar[int]
    id: str
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    name: str
    region: _common_pb2.Region
    group_type: MulticastGroupType
    def __init__(self, id: _Optional[str] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., name: _Optional[str] = ..., region: _Optional[_Union[_common_pb2.Region, str]] = ..., group_type: _Optional[_Union[MulticastGroupType, str]] = ...) -> None: ...

class CreateMulticastGroupRequest(_message.Message):
    __slots__ = ["multicast_group"]
    MULTICAST_GROUP_FIELD_NUMBER: _ClassVar[int]
    multicast_group: MulticastGroup
    def __init__(self, multicast_group: _Optional[_Union[MulticastGroup, _Mapping]] = ...) -> None: ...

class CreateMulticastGroupResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetMulticastGroupRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetMulticastGroupResponse(_message.Message):
    __slots__ = ["multicast_group", "created_at", "updated_at"]
    MULTICAST_GROUP_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    multicast_group: MulticastGroup
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, multicast_group: _Optional[_Union[MulticastGroup, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class UpdateMulticastGroupRequest(_message.Message):
    __slots__ = ["multicast_group"]
    MULTICAST_GROUP_FIELD_NUMBER: _ClassVar[int]
    multicast_group: MulticastGroup
    def __init__(self, multicast_group: _Optional[_Union[MulticastGroup, _Mapping]] = ...) -> None: ...

class DeleteMulticastGroupRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class ListMulticastGroupsRequest(_message.Message):
    __slots__ = ["limit", "offset", "search", "application_id"]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    SEARCH_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    limit: int
    offset: int
    search: str
    application_id: str
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., search: _Optional[str] = ..., application_id: _Optional[str] = ...) -> None: ...

class ListMulticastGroupsResponse(_message.Message):
    __slots__ = ["total_count", "result"]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    total_count: int
    result: _containers.RepeatedCompositeFieldContainer[MulticastGroupListItem]
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[MulticastGroupListItem, _Mapping]]] = ...) -> None: ...

class AddDeviceToMulticastGroupRequest(_message.Message):
    __slots__ = ["multicast_group_id", "dev_eui"]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    multicast_group_id: str
    dev_eui: str
    def __init__(self, multicast_group_id: _Optional[str] = ..., dev_eui: _Optional[str] = ...) -> None: ...

class RemoveDeviceFromMulticastGroupRequest(_message.Message):
    __slots__ = ["multicast_group_id", "dev_eui"]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    multicast_group_id: str
    dev_eui: str
    def __init__(self, multicast_group_id: _Optional[str] = ..., dev_eui: _Optional[str] = ...) -> None: ...

class AddGatewayToMulticastGroupRequest(_message.Message):
    __slots__ = ["multicast_group_id", "gateway_id"]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    multicast_group_id: str
    gateway_id: str
    def __init__(self, multicast_group_id: _Optional[str] = ..., gateway_id: _Optional[str] = ...) -> None: ...

class RemoveGatewayFromMulticastGroupRequest(_message.Message):
    __slots__ = ["multicast_group_id", "gateway_id"]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    multicast_group_id: str
    gateway_id: str
    def __init__(self, multicast_group_id: _Optional[str] = ..., gateway_id: _Optional[str] = ...) -> None: ...

class MulticastGroupQueueItem(_message.Message):
    __slots__ = ["multicast_group_id", "f_cnt", "f_port", "data"]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    F_CNT_FIELD_NUMBER: _ClassVar[int]
    F_PORT_FIELD_NUMBER: _ClassVar[int]
    DATA_FIELD_NUMBER: _ClassVar[int]
    multicast_group_id: str
    f_cnt: int
    f_port: int
    data: bytes
    def __init__(self, multicast_group_id: _Optional[str] = ..., f_cnt: _Optional[int] = ..., f_port: _Optional[int] = ..., data: _Optional[bytes] = ...) -> None: ...

class EnqueueMulticastGroupQueueItemRequest(_message.Message):
    __slots__ = ["queue_item"]
    QUEUE_ITEM_FIELD_NUMBER: _ClassVar[int]
    queue_item: MulticastGroupQueueItem
    def __init__(self, queue_item: _Optional[_Union[MulticastGroupQueueItem, _Mapping]] = ...) -> None: ...

class EnqueueMulticastGroupQueueItemResponse(_message.Message):
    __slots__ = ["f_cnt"]
    F_CNT_FIELD_NUMBER: _ClassVar[int]
    f_cnt: int
    def __init__(self, f_cnt: _Optional[int] = ...) -> None: ...

class FlushMulticastGroupQueueRequest(_message.Message):
    __slots__ = ["multicast_group_id"]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    multicast_group_id: str
    def __init__(self, multicast_group_id: _Optional[str] = ...) -> None: ...

class ListMulticastGroupQueueRequest(_message.Message):
    __slots__ = ["multicast_group_id"]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    multicast_group_id: str
    def __init__(self, multicast_group_id: _Optional[str] = ...) -> None: ...

class ListMulticastGroupQueueResponse(_message.Message):
    __slots__ = ["items"]
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    items: _containers.RepeatedCompositeFieldContainer[MulticastGroupQueueItem]
    def __init__(self, items: _Optional[_Iterable[_Union[MulticastGroupQueueItem, _Mapping]]] = ...) -> None: ...
