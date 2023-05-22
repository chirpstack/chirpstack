from google.api import annotations_pb2 as _annotations_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class AddRelayDeviceRequest(_message.Message):
    __slots__ = ["device_dev_eui", "relay_dev_eui"]
    DEVICE_DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    RELAY_DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    device_dev_eui: str
    relay_dev_eui: str
    def __init__(self, relay_dev_eui: _Optional[str] = ..., device_dev_eui: _Optional[str] = ...) -> None: ...

class ListRelayDevicesRequest(_message.Message):
    __slots__ = ["limit", "offset", "relay_dev_eui"]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    RELAY_DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    limit: int
    offset: int
    relay_dev_eui: str
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., relay_dev_eui: _Optional[str] = ...) -> None: ...

class ListRelayDevicesResponse(_message.Message):
    __slots__ = ["result", "total_count"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    result: _containers.RepeatedCompositeFieldContainer[RelayDeviceListItem]
    total_count: int
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[RelayDeviceListItem, _Mapping]]] = ...) -> None: ...

class ListRelaysRequest(_message.Message):
    __slots__ = ["application_id", "limit", "offset"]
    APPLICATION_ID_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    application_id: str
    limit: int
    offset: int
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., application_id: _Optional[str] = ...) -> None: ...

class ListRelaysResponse(_message.Message):
    __slots__ = ["result", "total_count"]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    result: _containers.RepeatedCompositeFieldContainer[RelayListItem]
    total_count: int
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[RelayListItem, _Mapping]]] = ...) -> None: ...

class RelayDeviceListItem(_message.Message):
    __slots__ = ["created_at", "dev_eui", "name"]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    created_at: _timestamp_pb2.Timestamp
    dev_eui: str
    name: str
    def __init__(self, dev_eui: _Optional[str] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., name: _Optional[str] = ...) -> None: ...

class RelayListItem(_message.Message):
    __slots__ = ["dev_eui", "name"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    name: str
    def __init__(self, dev_eui: _Optional[str] = ..., name: _Optional[str] = ...) -> None: ...

class RemoveRelayDeviceRequest(_message.Message):
    __slots__ = ["device_dev_eui", "relay_dev_eui"]
    DEVICE_DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    RELAY_DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    device_dev_eui: str
    relay_dev_eui: str
    def __init__(self, relay_dev_eui: _Optional[str] = ..., device_dev_eui: _Optional[str] = ...) -> None: ...
