from google.protobuf import timestamp_pb2 as _timestamp_pb2
from chirpstack_api.common import common_pb2 as _common_pb2
from chirpstack_api.gw import gw_pb2 as _gw_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class RequestLog(_message.Message):
    __slots__ = ["service", "method", "metadata"]
    class MetadataEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    SERVICE_FIELD_NUMBER: _ClassVar[int]
    METHOD_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    service: str
    method: str
    metadata: _containers.ScalarMap[str, str]
    def __init__(self, service: _Optional[str] = ..., method: _Optional[str] = ..., metadata: _Optional[_Mapping[str, str]] = ...) -> None: ...
