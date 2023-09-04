from google.api import annotations_pb2 as _annotations_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class User(_message.Message):
    __slots__ = ["id", "is_admin", "is_active", "email", "note"]
    ID_FIELD_NUMBER: _ClassVar[int]
    IS_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_ACTIVE_FIELD_NUMBER: _ClassVar[int]
    EMAIL_FIELD_NUMBER: _ClassVar[int]
    NOTE_FIELD_NUMBER: _ClassVar[int]
    id: str
    is_admin: bool
    is_active: bool
    email: str
    note: str
    def __init__(self, id: _Optional[str] = ..., is_admin: bool = ..., is_active: bool = ..., email: _Optional[str] = ..., note: _Optional[str] = ...) -> None: ...

class UserListItem(_message.Message):
    __slots__ = ["id", "created_at", "updated_at", "email", "is_admin", "is_active"]
    ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    EMAIL_FIELD_NUMBER: _ClassVar[int]
    IS_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_ACTIVE_FIELD_NUMBER: _ClassVar[int]
    id: str
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    email: str
    is_admin: bool
    is_active: bool
    def __init__(self, id: _Optional[str] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., email: _Optional[str] = ..., is_admin: bool = ..., is_active: bool = ...) -> None: ...

class UserTenant(_message.Message):
    __slots__ = ["tenant_id", "is_admin", "is_device_admin", "is_gateway_admin"]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    IS_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_DEVICE_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_GATEWAY_ADMIN_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    is_admin: bool
    is_device_admin: bool
    is_gateway_admin: bool
    def __init__(self, tenant_id: _Optional[str] = ..., is_admin: bool = ..., is_device_admin: bool = ..., is_gateway_admin: bool = ...) -> None: ...

class CreateUserRequest(_message.Message):
    __slots__ = ["user", "password", "tenants"]
    USER_FIELD_NUMBER: _ClassVar[int]
    PASSWORD_FIELD_NUMBER: _ClassVar[int]
    TENANTS_FIELD_NUMBER: _ClassVar[int]
    user: User
    password: str
    tenants: _containers.RepeatedCompositeFieldContainer[UserTenant]
    def __init__(self, user: _Optional[_Union[User, _Mapping]] = ..., password: _Optional[str] = ..., tenants: _Optional[_Iterable[_Union[UserTenant, _Mapping]]] = ...) -> None: ...

class CreateUserResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetUserRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetUserResponse(_message.Message):
    __slots__ = ["user", "created_at", "updated_at"]
    USER_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    user: User
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, user: _Optional[_Union[User, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class UpdateUserRequest(_message.Message):
    __slots__ = ["user"]
    USER_FIELD_NUMBER: _ClassVar[int]
    user: User
    def __init__(self, user: _Optional[_Union[User, _Mapping]] = ...) -> None: ...

class DeleteUserRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class ListUsersRequest(_message.Message):
    __slots__ = ["limit", "offset"]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    limit: int
    offset: int
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ...) -> None: ...

class ListUsersResponse(_message.Message):
    __slots__ = ["total_count", "result"]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    total_count: int
    result: _containers.RepeatedCompositeFieldContainer[UserListItem]
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[UserListItem, _Mapping]]] = ...) -> None: ...

class UpdateUserPasswordRequest(_message.Message):
    __slots__ = ["user_id", "password"]
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    PASSWORD_FIELD_NUMBER: _ClassVar[int]
    user_id: str
    password: str
    def __init__(self, user_id: _Optional[str] = ..., password: _Optional[str] = ...) -> None: ...
