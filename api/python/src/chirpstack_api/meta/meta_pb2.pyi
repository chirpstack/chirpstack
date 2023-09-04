from chirpstack_api.common import common_pb2 as _common_pb2
from chirpstack_api.gw import gw_pb2 as _gw_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class UplinkMeta(_message.Message):
    __slots__ = ["dev_eui", "tx_info", "rx_info", "phy_payload_byte_count", "mac_command_byte_count", "application_payload_byte_count", "message_type"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    RX_INFO_FIELD_NUMBER: _ClassVar[int]
    PHY_PAYLOAD_BYTE_COUNT_FIELD_NUMBER: _ClassVar[int]
    MAC_COMMAND_BYTE_COUNT_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_PAYLOAD_BYTE_COUNT_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_TYPE_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    tx_info: _gw_pb2.UplinkTxInfo
    rx_info: _containers.RepeatedCompositeFieldContainer[_gw_pb2.UplinkRxInfo]
    phy_payload_byte_count: int
    mac_command_byte_count: int
    application_payload_byte_count: int
    message_type: _common_pb2.MType
    def __init__(self, dev_eui: _Optional[str] = ..., tx_info: _Optional[_Union[_gw_pb2.UplinkTxInfo, _Mapping]] = ..., rx_info: _Optional[_Iterable[_Union[_gw_pb2.UplinkRxInfo, _Mapping]]] = ..., phy_payload_byte_count: _Optional[int] = ..., mac_command_byte_count: _Optional[int] = ..., application_payload_byte_count: _Optional[int] = ..., message_type: _Optional[_Union[_common_pb2.MType, str]] = ...) -> None: ...

class DownlinkMeta(_message.Message):
    __slots__ = ["dev_eui", "multicast_group_id", "tx_info", "phy_payload_byte_count", "mac_command_byte_count", "application_payload_byte_count", "message_type", "gateway_id"]
    DEV_EUI_FIELD_NUMBER: _ClassVar[int]
    MULTICAST_GROUP_ID_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    PHY_PAYLOAD_BYTE_COUNT_FIELD_NUMBER: _ClassVar[int]
    MAC_COMMAND_BYTE_COUNT_FIELD_NUMBER: _ClassVar[int]
    APPLICATION_PAYLOAD_BYTE_COUNT_FIELD_NUMBER: _ClassVar[int]
    MESSAGE_TYPE_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    dev_eui: str
    multicast_group_id: str
    tx_info: _gw_pb2.DownlinkTxInfo
    phy_payload_byte_count: int
    mac_command_byte_count: int
    application_payload_byte_count: int
    message_type: _common_pb2.MType
    gateway_id: str
    def __init__(self, dev_eui: _Optional[str] = ..., multicast_group_id: _Optional[str] = ..., tx_info: _Optional[_Union[_gw_pb2.DownlinkTxInfo, _Mapping]] = ..., phy_payload_byte_count: _Optional[int] = ..., mac_command_byte_count: _Optional[int] = ..., application_payload_byte_count: _Optional[int] = ..., message_type: _Optional[_Union[_common_pb2.MType, str]] = ..., gateway_id: _Optional[str] = ...) -> None: ...
