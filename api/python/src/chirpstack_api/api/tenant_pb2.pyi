from google.api import annotations_pb2 as _annotations_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Tenant(_message.Message):
    __slots__ = ["id", "name", "description", "can_have_gateways", "max_gateway_count", "max_device_count", "private_gateways_up", "private_gateways_down"]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    DESCRIPTION_FIELD_NUMBER: _ClassVar[int]
    CAN_HAVE_GATEWAYS_FIELD_NUMBER: _ClassVar[int]
    MAX_GATEWAY_COUNT_FIELD_NUMBER: _ClassVar[int]
    MAX_DEVICE_COUNT_FIELD_NUMBER: _ClassVar[int]
    PRIVATE_GATEWAYS_UP_FIELD_NUMBER: _ClassVar[int]
    PRIVATE_GATEWAYS_DOWN_FIELD_NUMBER: _ClassVar[int]
    id: str
    name: str
    description: str
    can_have_gateways: bool
    max_gateway_count: int
    max_device_count: int
    private_gateways_up: bool
    private_gateways_down: bool
    def __init__(self, id: _Optional[str] = ..., name: _Optional[str] = ..., description: _Optional[str] = ..., can_have_gateways: bool = ..., max_gateway_count: _Optional[int] = ..., max_device_count: _Optional[int] = ..., private_gateways_up: bool = ..., private_gateways_down: bool = ...) -> None: ...

class TenantListItem(_message.Message):
    __slots__ = ["id", "created_at", "updated_at", "name", "can_have_gateways", "private_gateways_up", "private_gateways_down", "max_gateway_count", "max_device_count"]
    ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CAN_HAVE_GATEWAYS_FIELD_NUMBER: _ClassVar[int]
    PRIVATE_GATEWAYS_UP_FIELD_NUMBER: _ClassVar[int]
    PRIVATE_GATEWAYS_DOWN_FIELD_NUMBER: _ClassVar[int]
    MAX_GATEWAY_COUNT_FIELD_NUMBER: _ClassVar[int]
    MAX_DEVICE_COUNT_FIELD_NUMBER: _ClassVar[int]
    id: str
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    name: str
    can_have_gateways: bool
    private_gateways_up: bool
    private_gateways_down: bool
    max_gateway_count: int
    max_device_count: int
    def __init__(self, id: _Optional[str] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., name: _Optional[str] = ..., can_have_gateways: bool = ..., private_gateways_up: bool = ..., private_gateways_down: bool = ..., max_gateway_count: _Optional[int] = ..., max_device_count: _Optional[int] = ...) -> None: ...

class CreateTenantRequest(_message.Message):
    __slots__ = ["tenant"]
    TENANT_FIELD_NUMBER: _ClassVar[int]
    tenant: Tenant
    def __init__(self, tenant: _Optional[_Union[Tenant, _Mapping]] = ...) -> None: ...

class CreateTenantResponse(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetTenantRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class GetTenantResponse(_message.Message):
    __slots__ = ["tenant", "created_at", "updated_at"]
    TENANT_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    tenant: Tenant
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, tenant: _Optional[_Union[Tenant, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class UpdateTenantRequest(_message.Message):
    __slots__ = ["tenant"]
    TENANT_FIELD_NUMBER: _ClassVar[int]
    tenant: Tenant
    def __init__(self, tenant: _Optional[_Union[Tenant, _Mapping]] = ...) -> None: ...

class DeleteTenantRequest(_message.Message):
    __slots__ = ["id"]
    ID_FIELD_NUMBER: _ClassVar[int]
    id: str
    def __init__(self, id: _Optional[str] = ...) -> None: ...

class ListTenantsRequest(_message.Message):
    __slots__ = ["limit", "offset", "search", "user_id"]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    SEARCH_FIELD_NUMBER: _ClassVar[int]
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    limit: int
    offset: int
    search: str
    user_id: str
    def __init__(self, limit: _Optional[int] = ..., offset: _Optional[int] = ..., search: _Optional[str] = ..., user_id: _Optional[str] = ...) -> None: ...

class ListTenantsResponse(_message.Message):
    __slots__ = ["total_count", "result"]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    total_count: int
    result: _containers.RepeatedCompositeFieldContainer[TenantListItem]
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[TenantListItem, _Mapping]]] = ...) -> None: ...

class TenantUser(_message.Message):
    __slots__ = ["tenant_id", "user_id", "is_admin", "is_device_admin", "is_gateway_admin", "email"]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    IS_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_DEVICE_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_GATEWAY_ADMIN_FIELD_NUMBER: _ClassVar[int]
    EMAIL_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    user_id: str
    is_admin: bool
    is_device_admin: bool
    is_gateway_admin: bool
    email: str
    def __init__(self, tenant_id: _Optional[str] = ..., user_id: _Optional[str] = ..., is_admin: bool = ..., is_device_admin: bool = ..., is_gateway_admin: bool = ..., email: _Optional[str] = ...) -> None: ...

class TenantUserListItem(_message.Message):
    __slots__ = ["tenant_id", "user_id", "created_at", "updated_at", "email", "is_admin", "is_device_admin", "is_gateway_admin"]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    EMAIL_FIELD_NUMBER: _ClassVar[int]
    IS_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_DEVICE_ADMIN_FIELD_NUMBER: _ClassVar[int]
    IS_GATEWAY_ADMIN_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    user_id: str
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    email: str
    is_admin: bool
    is_device_admin: bool
    is_gateway_admin: bool
    def __init__(self, tenant_id: _Optional[str] = ..., user_id: _Optional[str] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., email: _Optional[str] = ..., is_admin: bool = ..., is_device_admin: bool = ..., is_gateway_admin: bool = ...) -> None: ...

class AddTenantUserRequest(_message.Message):
    __slots__ = ["tenant_user"]
    TENANT_USER_FIELD_NUMBER: _ClassVar[int]
    tenant_user: TenantUser
    def __init__(self, tenant_user: _Optional[_Union[TenantUser, _Mapping]] = ...) -> None: ...

class GetTenantUserRequest(_message.Message):
    __slots__ = ["tenant_id", "user_id"]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    user_id: str
    def __init__(self, tenant_id: _Optional[str] = ..., user_id: _Optional[str] = ...) -> None: ...

class GetTenantUserResponse(_message.Message):
    __slots__ = ["tenant_user", "created_at", "updated_at"]
    TENANT_USER_FIELD_NUMBER: _ClassVar[int]
    CREATED_AT_FIELD_NUMBER: _ClassVar[int]
    UPDATED_AT_FIELD_NUMBER: _ClassVar[int]
    tenant_user: TenantUser
    created_at: _timestamp_pb2.Timestamp
    updated_at: _timestamp_pb2.Timestamp
    def __init__(self, tenant_user: _Optional[_Union[TenantUser, _Mapping]] = ..., created_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., updated_at: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class UpdateTenantUserRequest(_message.Message):
    __slots__ = ["tenant_user"]
    TENANT_USER_FIELD_NUMBER: _ClassVar[int]
    tenant_user: TenantUser
    def __init__(self, tenant_user: _Optional[_Union[TenantUser, _Mapping]] = ...) -> None: ...

class DeleteTenantUserRequest(_message.Message):
    __slots__ = ["tenant_id", "user_id"]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    USER_ID_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    user_id: str
    def __init__(self, tenant_id: _Optional[str] = ..., user_id: _Optional[str] = ...) -> None: ...

class ListTenantUsersRequest(_message.Message):
    __slots__ = ["tenant_id", "limit", "offset"]
    TENANT_ID_FIELD_NUMBER: _ClassVar[int]
    LIMIT_FIELD_NUMBER: _ClassVar[int]
    OFFSET_FIELD_NUMBER: _ClassVar[int]
    tenant_id: str
    limit: int
    offset: int
    def __init__(self, tenant_id: _Optional[str] = ..., limit: _Optional[int] = ..., offset: _Optional[int] = ...) -> None: ...

class ListTenantUsersResponse(_message.Message):
    __slots__ = ["total_count", "result"]
    TOTAL_COUNT_FIELD_NUMBER: _ClassVar[int]
    RESULT_FIELD_NUMBER: _ClassVar[int]
    total_count: int
    result: _containers.RepeatedCompositeFieldContainer[TenantUserListItem]
    def __init__(self, total_count: _Optional[int] = ..., result: _Optional[_Iterable[_Union[TenantUserListItem, _Mapping]]] = ...) -> None: ...
