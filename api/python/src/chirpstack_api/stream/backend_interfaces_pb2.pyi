from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class BackendInterfacesRequest(_message.Message):
    __slots__ = ("sender_id", "receiver_id", "time", "transaction_id", "message_type", "result_code", "request_body", "request_error", "response_body")
    SENDER_ID_FIELD_NUMBER: _ClassVar[int]
    RECEIVER_ID_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    TRANSACTION_ID_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_TYPE_FIELD_NUMBER: _ClassVar[int]
    RESULT_CODE_FIELD_NUMBER: _ClassVar[int]
    REQUEST_BODY_FIELD_NUMBER: _ClassVar[int]
    REQUEST_ERROR_FIELD_NUMBER: _ClassVar[int]
    RESPONSE_BODY_FIELD_NUMBER: _ClassVar[int]
    sender_id: str
    receiver_id: str
    time: _timestamp_pb2.Timestamp
    transaction_id: int
    message_type: str
    result_code: str
    request_body: str
    request_error: str
    response_body: str
    def __init__(self, sender_id: _Optional[str] = ..., receiver_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., transaction_id: _Optional[int] = ..., message_type: _Optional[str] = ..., result_code: _Optional[str] = ..., request_body: _Optional[str] = ..., request_error: _Optional[str] = ..., response_body: _Optional[str] = ...) -> None: ...
