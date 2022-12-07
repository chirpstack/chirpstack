"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""
import builtins
import chirpstack_api.common.common_pb2
import chirpstack_api.gw.gw_pb2
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.internal.enum_type_wrapper
import google.protobuf.message
import google.protobuf.struct_pb2
import google.protobuf.timestamp_pb2
import typing
import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor = ...

class LogLevel(_LogLevel, metaclass=_LogLevelEnumTypeWrapper):
    pass
class _LogLevel:
    V = typing.NewType('V', builtins.int)
class _LogLevelEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_LogLevel.V], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor = ...
    # Info.
    INFO = LogLevel.V(0)
    # Warning.
    WARNING = LogLevel.V(1)
    # Error.
    ERROR = LogLevel.V(2)

# Info.
INFO = LogLevel.V(0)
# Warning.
WARNING = LogLevel.V(1)
# Error.
ERROR = LogLevel.V(2)
global___LogLevel = LogLevel


class LogCode(_LogCode, metaclass=_LogCodeEnumTypeWrapper):
    pass
class _LogCode:
    V = typing.NewType('V', builtins.int)
class _LogCodeEnumTypeWrapper(google.protobuf.internal.enum_type_wrapper._EnumTypeWrapper[_LogCode.V], builtins.type):
    DESCRIPTOR: google.protobuf.descriptor.EnumDescriptor = ...
    # Unknown type.
    UNKNOWN = LogCode.V(0)
    # Error related to the downlink payload size.
    # Usually seen when the payload exceeded the maximum allowed payload size.
    DOWNLINK_PAYLOAD_SIZE = LogCode.V(1)
    # Uplink codec error.
    UPLINK_CODEC = LogCode.V(2)
    # Downlink codec error.
    DOWNLINK_CODEC = LogCode.V(3)
    # OTAA error.
    OTAA = LogCode.V(4)
    # Uplink frame-counter was reset.
    UPLINK_F_CNT_RESET = LogCode.V(5)
    # Uplink MIC error.
    UPLINK_MIC = LogCode.V(6)
    # Uplink frame-counter retransmission.
    UPLINK_F_CNT_RETRANSMISSION = LogCode.V(7)
    # Downlink gateway error.
    DOWNLINK_GATEWAY = LogCode.V(8)

# Unknown type.
UNKNOWN = LogCode.V(0)
# Error related to the downlink payload size.
# Usually seen when the payload exceeded the maximum allowed payload size.
DOWNLINK_PAYLOAD_SIZE = LogCode.V(1)
# Uplink codec error.
UPLINK_CODEC = LogCode.V(2)
# Downlink codec error.
DOWNLINK_CODEC = LogCode.V(3)
# OTAA error.
OTAA = LogCode.V(4)
# Uplink frame-counter was reset.
UPLINK_F_CNT_RESET = LogCode.V(5)
# Uplink MIC error.
UPLINK_MIC = LogCode.V(6)
# Uplink frame-counter retransmission.
UPLINK_F_CNT_RETRANSMISSION = LogCode.V(7)
# Downlink gateway error.
DOWNLINK_GATEWAY = LogCode.V(8)
global___LogCode = LogCode


# Device information.
class DeviceInfo(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    class TagsEntry(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
        KEY_FIELD_NUMBER: builtins.int
        VALUE_FIELD_NUMBER: builtins.int
        key: typing.Text = ...
        value: typing.Text = ...
        def __init__(self,
            *,
            key : typing.Text = ...,
            value : typing.Text = ...,
            ) -> None: ...
        def ClearField(self, field_name: typing_extensions.Literal[u"key",b"key",u"value",b"value"]) -> None: ...

    TENANT_ID_FIELD_NUMBER: builtins.int
    TENANT_NAME_FIELD_NUMBER: builtins.int
    APPLICATION_ID_FIELD_NUMBER: builtins.int
    APPLICATION_NAME_FIELD_NUMBER: builtins.int
    DEVICE_PROFILE_ID_FIELD_NUMBER: builtins.int
    DEVICE_PROFILE_NAME_FIELD_NUMBER: builtins.int
    DEVICE_NAME_FIELD_NUMBER: builtins.int
    DEV_EUI_FIELD_NUMBER: builtins.int
    TAGS_FIELD_NUMBER: builtins.int
    # Tenant ID (UUID).
    tenant_id: typing.Text = ...
    # Tenant name.
    tenant_name: typing.Text = ...
    # Application ID (UUID).
    application_id: typing.Text = ...
    # Application name.
    application_name: typing.Text = ...
    # Device-profile ID (UUID).
    device_profile_id: typing.Text = ...
    # Device-profile name.
    device_profile_name: typing.Text = ...
    # Device name.
    device_name: typing.Text = ...
    # Device EUI.
    dev_eui: typing.Text = ...
    # Device-profile and device tags.
    @property
    def tags(self) -> google.protobuf.internal.containers.ScalarMap[typing.Text, typing.Text]: ...
    def __init__(self,
        *,
        tenant_id : typing.Text = ...,
        tenant_name : typing.Text = ...,
        application_id : typing.Text = ...,
        application_name : typing.Text = ...,
        device_profile_id : typing.Text = ...,
        device_profile_name : typing.Text = ...,
        device_name : typing.Text = ...,
        dev_eui : typing.Text = ...,
        tags : typing.Optional[typing.Mapping[typing.Text, typing.Text]] = ...,
        ) -> None: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"application_id",b"application_id",u"application_name",b"application_name",u"dev_eui",b"dev_eui",u"device_name",b"device_name",u"device_profile_id",b"device_profile_id",u"device_profile_name",b"device_profile_name",u"tags",b"tags",u"tenant_id",b"tenant_id",u"tenant_name",b"tenant_name"]) -> None: ...
global___DeviceInfo = DeviceInfo

# UplinkEvent is the message sent when an uplink payload has been received.
class UplinkEvent(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    DEDUPLICATION_ID_FIELD_NUMBER: builtins.int
    TIME_FIELD_NUMBER: builtins.int
    DEVICE_INFO_FIELD_NUMBER: builtins.int
    DEV_ADDR_FIELD_NUMBER: builtins.int
    ADR_FIELD_NUMBER: builtins.int
    DR_FIELD_NUMBER: builtins.int
    F_CNT_FIELD_NUMBER: builtins.int
    F_PORT_FIELD_NUMBER: builtins.int
    CONFIRMED_FIELD_NUMBER: builtins.int
    DATA_FIELD_NUMBER: builtins.int
    OBJECT_FIELD_NUMBER: builtins.int
    RX_INFO_FIELD_NUMBER: builtins.int
    TX_INFO_FIELD_NUMBER: builtins.int
    # Deduplication ID (UUID).
    deduplication_id: typing.Text = ...
    # Timestamp.
    @property
    def time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    # Device information.
    @property
    def device_info(self) -> global___DeviceInfo: ...
    # Device address.
    dev_addr: typing.Text = ...
    # Device has ADR bit set.
    adr: builtins.bool = ...
    # Data-rate.
    dr: builtins.int = ...
    # Frame counter.
    f_cnt: builtins.int = ...
    # Frame port.
    f_port: builtins.int = ...
    # Uplink was of type confirmed.
    confirmed: builtins.bool = ...
    # FRMPayload data.
    data: builtins.bytes = ...
    # Note that this is only set when a codec is configured in the Device Profile.
    @property
    def object(self) -> google.protobuf.struct_pb2.Struct: ...
    # Receiving gateway RX info.
    @property
    def rx_info(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[chirpstack_api.gw.gw_pb2.UplinkRxInfo]: ...
    # TX info.
    @property
    def tx_info(self) -> chirpstack_api.gw.gw_pb2.UplinkTxInfo: ...
    def __init__(self,
        *,
        deduplication_id : typing.Text = ...,
        time : typing.Optional[google.protobuf.timestamp_pb2.Timestamp] = ...,
        device_info : typing.Optional[global___DeviceInfo] = ...,
        dev_addr : typing.Text = ...,
        adr : builtins.bool = ...,
        dr : builtins.int = ...,
        f_cnt : builtins.int = ...,
        f_port : builtins.int = ...,
        confirmed : builtins.bool = ...,
        data : builtins.bytes = ...,
        object : typing.Optional[google.protobuf.struct_pb2.Struct] = ...,
        rx_info : typing.Optional[typing.Iterable[chirpstack_api.gw.gw_pb2.UplinkRxInfo]] = ...,
        tx_info : typing.Optional[chirpstack_api.gw.gw_pb2.UplinkTxInfo] = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"object",b"object",u"time",b"time",u"tx_info",b"tx_info"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"adr",b"adr",u"confirmed",b"confirmed",u"data",b"data",u"deduplication_id",b"deduplication_id",u"dev_addr",b"dev_addr",u"device_info",b"device_info",u"dr",b"dr",u"f_cnt",b"f_cnt",u"f_port",b"f_port",u"object",b"object",u"rx_info",b"rx_info",u"time",b"time",u"tx_info",b"tx_info"]) -> None: ...
global___UplinkEvent = UplinkEvent

# JoinEvent is the message sent when a device joined the network.
# Note: this event is sent at the first uplink after OTAA.
class JoinEvent(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    DEDUPLICATION_ID_FIELD_NUMBER: builtins.int
    TIME_FIELD_NUMBER: builtins.int
    DEVICE_INFO_FIELD_NUMBER: builtins.int
    DEV_ADDR_FIELD_NUMBER: builtins.int
    # Deduplication ID (UUID).
    deduplication_id: typing.Text = ...
    # Timestamp.
    @property
    def time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    # Device info.
    @property
    def device_info(self) -> global___DeviceInfo: ...
    # Device address.
    dev_addr: typing.Text = ...
    def __init__(self,
        *,
        deduplication_id : typing.Text = ...,
        time : typing.Optional[google.protobuf.timestamp_pb2.Timestamp] = ...,
        device_info : typing.Optional[global___DeviceInfo] = ...,
        dev_addr : typing.Text = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"time",b"time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"deduplication_id",b"deduplication_id",u"dev_addr",b"dev_addr",u"device_info",b"device_info",u"time",b"time"]) -> None: ...
global___JoinEvent = JoinEvent

# AckEvent is the message sent when a confirmation on a confirmed downlink
# has been received -or- when the downlink timed out.
class AckEvent(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    DEDUPLICATION_ID_FIELD_NUMBER: builtins.int
    TIME_FIELD_NUMBER: builtins.int
    DEVICE_INFO_FIELD_NUMBER: builtins.int
    QUEUE_ITEM_ID_FIELD_NUMBER: builtins.int
    ACKNOWLEDGED_FIELD_NUMBER: builtins.int
    F_CNT_DOWN_FIELD_NUMBER: builtins.int
    # Deduplication ID (UUID).
    deduplication_id: typing.Text = ...
    # Timestamp.
    @property
    def time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    # Device info.
    @property
    def device_info(self) -> global___DeviceInfo: ...
    # Downlink queue item ID (UUID).
    queue_item_id: typing.Text = ...
    # Frame was acknowledged.
    acknowledged: builtins.bool = ...
    # Downlink frame counter to which the acknowledgement relates.
    f_cnt_down: builtins.int = ...
    def __init__(self,
        *,
        deduplication_id : typing.Text = ...,
        time : typing.Optional[google.protobuf.timestamp_pb2.Timestamp] = ...,
        device_info : typing.Optional[global___DeviceInfo] = ...,
        queue_item_id : typing.Text = ...,
        acknowledged : builtins.bool = ...,
        f_cnt_down : builtins.int = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"time",b"time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"acknowledged",b"acknowledged",u"deduplication_id",b"deduplication_id",u"device_info",b"device_info",u"f_cnt_down",b"f_cnt_down",u"queue_item_id",b"queue_item_id",u"time",b"time"]) -> None: ...
global___AckEvent = AckEvent

# TxAckEvent is the message sent when a downlink was acknowledged by the gateway
# for transmission. As a downlink can be scheduled in the future, this event
# does not confirm that the message has already been transmitted.
class TxAckEvent(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    DOWNLINK_ID_FIELD_NUMBER: builtins.int
    TIME_FIELD_NUMBER: builtins.int
    DEVICE_INFO_FIELD_NUMBER: builtins.int
    QUEUE_ITEM_ID_FIELD_NUMBER: builtins.int
    F_CNT_DOWN_FIELD_NUMBER: builtins.int
    GATEWAY_ID_FIELD_NUMBER: builtins.int
    TX_INFO_FIELD_NUMBER: builtins.int
    # Downlink ID.
    downlink_id: builtins.int = ...
    # Timestamp.
    @property
    def time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    # Device info.
    @property
    def device_info(self) -> global___DeviceInfo: ...
    # Downlink queue item ID (UUID).
    queue_item_id: typing.Text = ...
    # Downlink frame-counter.
    f_cnt_down: builtins.int = ...
    # Gateway ID.
    gateway_id: typing.Text = ...
    # TX info.
    @property
    def tx_info(self) -> chirpstack_api.gw.gw_pb2.DownlinkTxInfo: ...
    def __init__(self,
        *,
        downlink_id : builtins.int = ...,
        time : typing.Optional[google.protobuf.timestamp_pb2.Timestamp] = ...,
        device_info : typing.Optional[global___DeviceInfo] = ...,
        queue_item_id : typing.Text = ...,
        f_cnt_down : builtins.int = ...,
        gateway_id : typing.Text = ...,
        tx_info : typing.Optional[chirpstack_api.gw.gw_pb2.DownlinkTxInfo] = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"time",b"time",u"tx_info",b"tx_info"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"downlink_id",b"downlink_id",u"f_cnt_down",b"f_cnt_down",u"gateway_id",b"gateway_id",u"queue_item_id",b"queue_item_id",u"time",b"time",u"tx_info",b"tx_info"]) -> None: ...
global___TxAckEvent = TxAckEvent

# LogEvent is the message sent when a device-related log was sent.
class LogEvent(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    class ContextEntry(google.protobuf.message.Message):
        DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
        KEY_FIELD_NUMBER: builtins.int
        VALUE_FIELD_NUMBER: builtins.int
        key: typing.Text = ...
        value: typing.Text = ...
        def __init__(self,
            *,
            key : typing.Text = ...,
            value : typing.Text = ...,
            ) -> None: ...
        def ClearField(self, field_name: typing_extensions.Literal[u"key",b"key",u"value",b"value"]) -> None: ...

    TIME_FIELD_NUMBER: builtins.int
    DEVICE_INFO_FIELD_NUMBER: builtins.int
    LEVEL_FIELD_NUMBER: builtins.int
    CODE_FIELD_NUMBER: builtins.int
    DESCRIPTION_FIELD_NUMBER: builtins.int
    CONTEXT_FIELD_NUMBER: builtins.int
    # Timestamp.
    @property
    def time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    # Device info.
    @property
    def device_info(self) -> global___DeviceInfo: ...
    # Log level.
    level: global___LogLevel.V = ...
    # Log code.
    code: global___LogCode.V = ...
    # Description message.
    description: typing.Text = ...
    # Context map.
    @property
    def context(self) -> google.protobuf.internal.containers.ScalarMap[typing.Text, typing.Text]: ...
    def __init__(self,
        *,
        time : typing.Optional[google.protobuf.timestamp_pb2.Timestamp] = ...,
        device_info : typing.Optional[global___DeviceInfo] = ...,
        level : global___LogLevel.V = ...,
        code : global___LogCode.V = ...,
        description : typing.Text = ...,
        context : typing.Optional[typing.Mapping[typing.Text, typing.Text]] = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"time",b"time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"code",b"code",u"context",b"context",u"description",b"description",u"device_info",b"device_info",u"level",b"level",u"time",b"time"]) -> None: ...
global___LogEvent = LogEvent

# StatusEvent is the message sent when a device-status mac-command was sent
# by the device.
class StatusEvent(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    DEDUPLICATION_ID_FIELD_NUMBER: builtins.int
    TIME_FIELD_NUMBER: builtins.int
    DEVICE_INFO_FIELD_NUMBER: builtins.int
    MARGIN_FIELD_NUMBER: builtins.int
    EXTERNAL_POWER_SOURCE_FIELD_NUMBER: builtins.int
    BATTERY_LEVEL_UNAVAILABLE_FIELD_NUMBER: builtins.int
    BATTERY_LEVEL_FIELD_NUMBER: builtins.int
    # Deduplication ID (UUID).
    deduplication_id: typing.Text = ...
    # Timestamp.
    @property
    def time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    # Device info.
    @property
    def device_info(self) -> global___DeviceInfo: ...
    # The demodulation signal-to-noise ratio in dB for the last successfully
    # received device-status request by the Network Server.
    margin: builtins.int = ...
    # Device is connected to an external power source.
    external_power_source: builtins.bool = ...
    # Battery level is not available.
    battery_level_unavailable: builtins.bool = ...
    # Battery level.
    battery_level: builtins.float = ...
    def __init__(self,
        *,
        deduplication_id : typing.Text = ...,
        time : typing.Optional[google.protobuf.timestamp_pb2.Timestamp] = ...,
        device_info : typing.Optional[global___DeviceInfo] = ...,
        margin : builtins.int = ...,
        external_power_source : builtins.bool = ...,
        battery_level_unavailable : builtins.bool = ...,
        battery_level : builtins.float = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"time",b"time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"battery_level",b"battery_level",u"battery_level_unavailable",b"battery_level_unavailable",u"deduplication_id",b"deduplication_id",u"device_info",b"device_info",u"external_power_source",b"external_power_source",u"margin",b"margin",u"time",b"time"]) -> None: ...
global___StatusEvent = StatusEvent

# LocationEvent is the message sent when a geolocation resolve was returned.
class LocationEvent(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    DEDUPLICATION_ID_FIELD_NUMBER: builtins.int
    TIME_FIELD_NUMBER: builtins.int
    DEVICE_INFO_FIELD_NUMBER: builtins.int
    LOCATION_FIELD_NUMBER: builtins.int
    # Deduplication ID (UUID).
    deduplication_id: typing.Text = ...
    # Timestamp.
    @property
    def time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    # Device info.
    @property
    def device_info(self) -> global___DeviceInfo: ...
    # Location.
    @property
    def location(self) -> chirpstack_api.common.common_pb2.Location: ...
    def __init__(self,
        *,
        deduplication_id : typing.Text = ...,
        time : typing.Optional[google.protobuf.timestamp_pb2.Timestamp] = ...,
        device_info : typing.Optional[global___DeviceInfo] = ...,
        location : typing.Optional[chirpstack_api.common.common_pb2.Location] = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"location",b"location",u"time",b"time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"deduplication_id",b"deduplication_id",u"device_info",b"device_info",u"location",b"location",u"time",b"time"]) -> None: ...
global___LocationEvent = LocationEvent

# IntegrationEvent is the message that can be sent by an integration.
# It allows for sending events which are provided by an external integration
# which are "not native" to ChirpStack.
class IntegrationEvent(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    DEDUPLICATION_ID_FIELD_NUMBER: builtins.int
    TIME_FIELD_NUMBER: builtins.int
    DEVICE_INFO_FIELD_NUMBER: builtins.int
    INTEGRATION_NAME_FIELD_NUMBER: builtins.int
    EVENT_TYPE_FIELD_NUMBER: builtins.int
    OBJECT_FIELD_NUMBER: builtins.int
    # Deduplication ID (UUID).
    deduplication_id: typing.Text = ...
    # Timestamp.
    @property
    def time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    # Device info.
    @property
    def device_info(self) -> global___DeviceInfo: ...
    # Integration name.
    integration_name: typing.Text = ...
    # Event type.
    event_type: typing.Text = ...
    # Struct containing the event object.
    @property
    def object(self) -> google.protobuf.struct_pb2.Struct: ...
    def __init__(self,
        *,
        deduplication_id : typing.Text = ...,
        time : typing.Optional[google.protobuf.timestamp_pb2.Timestamp] = ...,
        device_info : typing.Optional[global___DeviceInfo] = ...,
        integration_name : typing.Text = ...,
        event_type : typing.Text = ...,
        object : typing.Optional[google.protobuf.struct_pb2.Struct] = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"device_info",b"device_info",u"object",b"object",u"time",b"time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"deduplication_id",b"deduplication_id",u"device_info",b"device_info",u"event_type",b"event_type",u"integration_name",b"integration_name",u"object",b"object",u"time",b"time"]) -> None: ...
global___IntegrationEvent = IntegrationEvent

# DownlinkCommand is the command to enqueue a downlink payload for the given
# device.
class DownlinkCommand(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    ID_FIELD_NUMBER: builtins.int
    DEV_EUI_FIELD_NUMBER: builtins.int
    CONFIRMED_FIELD_NUMBER: builtins.int
    F_PORT_FIELD_NUMBER: builtins.int
    DATA_FIELD_NUMBER: builtins.int
    OBJECT_FIELD_NUMBER: builtins.int
    # ID (UUID).
    # If left blank, a random UUID will be generated.
    id: typing.Text = ...
    # Device EUI (EUI64).
    dev_eui: typing.Text = ...
    # Confirmed.
    confirmed: builtins.bool = ...
    # FPort (must be > 0).
    f_port: builtins.int = ...
    # Data.
    # Or use the json_object field when a codec has been configured.
    data: builtins.bytes = ...
    # Only use this when a codec has been configured that can encode this
    # object to bytes.
    @property
    def object(self) -> google.protobuf.struct_pb2.Struct: ...
    def __init__(self,
        *,
        id : typing.Text = ...,
        dev_eui : typing.Text = ...,
        confirmed : builtins.bool = ...,
        f_port : builtins.int = ...,
        data : builtins.bytes = ...,
        object : typing.Optional[google.protobuf.struct_pb2.Struct] = ...,
        ) -> None: ...
    def HasField(self, field_name: typing_extensions.Literal[u"object",b"object"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing_extensions.Literal[u"confirmed",b"confirmed",u"data",b"data",u"dev_eui",b"dev_eui",u"f_port",b"f_port",u"id",b"id",u"object",b"object"]) -> None: ...
global___DownlinkCommand = DownlinkCommand
