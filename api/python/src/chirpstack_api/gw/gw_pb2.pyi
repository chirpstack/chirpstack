from chirpstack_api.common import common_pb2 as _common_pb2
from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import duration_pb2 as _duration_pb2
from google.protobuf import struct_pb2 as _struct_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

BAD_CRC: CRCStatus
COLLISION_BEACON: TxAckStatus
COLLISION_PACKET: TxAckStatus
CRC_OK: CRCStatus
CR_1_4: CodeRate
CR_1_6: CodeRate
CR_2_6: CodeRate
CR_3_8: CodeRate
CR_4_5: CodeRate
CR_4_6: CodeRate
CR_4_7: CodeRate
CR_4_8: CodeRate
CR_5_6: CodeRate
CR_LI_4_5: CodeRate
CR_LI_4_6: CodeRate
CR_LI_4_8: CodeRate
CR_UNDEFINED: CodeRate
DELAY: DownlinkTiming
DESCRIPTOR: _descriptor.FileDescriptor
ENCRYPTED: FineTimestampType
GPS_EPOCH: DownlinkTiming
GPS_UNLOCKED: TxAckStatus
IGNORED: TxAckStatus
IMMEDIATELY: DownlinkTiming
INTERNAL_ERROR: TxAckStatus
NONE: FineTimestampType
NO_CRC: CRCStatus
OK: TxAckStatus
PLAIN: FineTimestampType
QUEUE_FULL: TxAckStatus
TOO_EARLY: TxAckStatus
TOO_LATE: TxAckStatus
TX_FREQ: TxAckStatus
TX_POWER: TxAckStatus

class ChannelConfiguration(_message.Message):
    __slots__ = ["board", "demodulator", "frequency", "fsk_modulation_config", "lora_modulation_config", "modulation_legacy"]
    BOARD_FIELD_NUMBER: _ClassVar[int]
    DEMODULATOR_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    FSK_MODULATION_CONFIG_FIELD_NUMBER: _ClassVar[int]
    LORA_MODULATION_CONFIG_FIELD_NUMBER: _ClassVar[int]
    MODULATION_LEGACY_FIELD_NUMBER: _ClassVar[int]
    board: int
    demodulator: int
    frequency: int
    fsk_modulation_config: FskModulationConfig
    lora_modulation_config: LoraModulationConfig
    modulation_legacy: _common_pb2.Modulation
    def __init__(self, frequency: _Optional[int] = ..., modulation_legacy: _Optional[_Union[_common_pb2.Modulation, str]] = ..., lora_modulation_config: _Optional[_Union[LoraModulationConfig, _Mapping]] = ..., fsk_modulation_config: _Optional[_Union[FskModulationConfig, _Mapping]] = ..., board: _Optional[int] = ..., demodulator: _Optional[int] = ...) -> None: ...

class ConnState(_message.Message):
    __slots__ = ["gateway_id", "gateway_id_legacy", "state"]
    class State(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    OFFLINE: ConnState.State
    ONLINE: ConnState.State
    STATE_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    gateway_id_legacy: bytes
    state: ConnState.State
    def __init__(self, gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ..., state: _Optional[_Union[ConnState.State, str]] = ...) -> None: ...

class DelayTimingInfo(_message.Message):
    __slots__ = ["delay"]
    DELAY_FIELD_NUMBER: _ClassVar[int]
    delay: _duration_pb2.Duration
    def __init__(self, delay: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class DownlinkFrame(_message.Message):
    __slots__ = ["downlink_id", "downlink_id_legacy", "gateway_id", "gateway_id_legacy", "items"]
    DOWNLINK_ID_FIELD_NUMBER: _ClassVar[int]
    DOWNLINK_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    downlink_id: int
    downlink_id_legacy: bytes
    gateway_id: str
    gateway_id_legacy: bytes
    items: _containers.RepeatedCompositeFieldContainer[DownlinkFrameItem]
    def __init__(self, downlink_id: _Optional[int] = ..., downlink_id_legacy: _Optional[bytes] = ..., items: _Optional[_Iterable[_Union[DownlinkFrameItem, _Mapping]]] = ..., gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ...) -> None: ...

class DownlinkFrameItem(_message.Message):
    __slots__ = ["phy_payload", "tx_info", "tx_info_legacy"]
    PHY_PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_LEGACY_FIELD_NUMBER: _ClassVar[int]
    phy_payload: bytes
    tx_info: DownlinkTxInfo
    tx_info_legacy: DownlinkTxInfoLegacy
    def __init__(self, phy_payload: _Optional[bytes] = ..., tx_info_legacy: _Optional[_Union[DownlinkTxInfoLegacy, _Mapping]] = ..., tx_info: _Optional[_Union[DownlinkTxInfo, _Mapping]] = ...) -> None: ...

class DownlinkTxAck(_message.Message):
    __slots__ = ["downlink_id", "downlink_id_legacy", "gateway_id", "gateway_id_legacy", "items"]
    DOWNLINK_ID_FIELD_NUMBER: _ClassVar[int]
    DOWNLINK_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    ITEMS_FIELD_NUMBER: _ClassVar[int]
    downlink_id: int
    downlink_id_legacy: bytes
    gateway_id: str
    gateway_id_legacy: bytes
    items: _containers.RepeatedCompositeFieldContainer[DownlinkTxAckItem]
    def __init__(self, gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ..., downlink_id: _Optional[int] = ..., downlink_id_legacy: _Optional[bytes] = ..., items: _Optional[_Iterable[_Union[DownlinkTxAckItem, _Mapping]]] = ...) -> None: ...

class DownlinkTxAckItem(_message.Message):
    __slots__ = ["status"]
    STATUS_FIELD_NUMBER: _ClassVar[int]
    status: TxAckStatus
    def __init__(self, status: _Optional[_Union[TxAckStatus, str]] = ...) -> None: ...

class DownlinkTxInfo(_message.Message):
    __slots__ = ["antenna", "board", "context", "frequency", "modulation", "power", "timing"]
    ANTENNA_FIELD_NUMBER: _ClassVar[int]
    BOARD_FIELD_NUMBER: _ClassVar[int]
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    MODULATION_FIELD_NUMBER: _ClassVar[int]
    POWER_FIELD_NUMBER: _ClassVar[int]
    TIMING_FIELD_NUMBER: _ClassVar[int]
    antenna: int
    board: int
    context: bytes
    frequency: int
    modulation: Modulation
    power: int
    timing: Timing
    def __init__(self, frequency: _Optional[int] = ..., power: _Optional[int] = ..., modulation: _Optional[_Union[Modulation, _Mapping]] = ..., board: _Optional[int] = ..., antenna: _Optional[int] = ..., timing: _Optional[_Union[Timing, _Mapping]] = ..., context: _Optional[bytes] = ...) -> None: ...

class DownlinkTxInfoLegacy(_message.Message):
    __slots__ = ["antenna", "board", "context", "delay_timing_info", "frequency", "fsk_modulation_info", "gateway_id", "gps_epoch_timing_info", "immediately_timing_info", "lora_modulation_info", "modulation", "power", "timing"]
    ANTENNA_FIELD_NUMBER: _ClassVar[int]
    BOARD_FIELD_NUMBER: _ClassVar[int]
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    DELAY_TIMING_INFO_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    FSK_MODULATION_INFO_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GPS_EPOCH_TIMING_INFO_FIELD_NUMBER: _ClassVar[int]
    IMMEDIATELY_TIMING_INFO_FIELD_NUMBER: _ClassVar[int]
    LORA_MODULATION_INFO_FIELD_NUMBER: _ClassVar[int]
    MODULATION_FIELD_NUMBER: _ClassVar[int]
    POWER_FIELD_NUMBER: _ClassVar[int]
    TIMING_FIELD_NUMBER: _ClassVar[int]
    antenna: int
    board: int
    context: bytes
    delay_timing_info: DelayTimingInfo
    frequency: int
    fsk_modulation_info: FskModulationInfo
    gateway_id: bytes
    gps_epoch_timing_info: GPSEpochTimingInfo
    immediately_timing_info: ImmediatelyTimingInfo
    lora_modulation_info: LoraModulationInfo
    modulation: _common_pb2.Modulation
    power: int
    timing: DownlinkTiming
    def __init__(self, gateway_id: _Optional[bytes] = ..., frequency: _Optional[int] = ..., power: _Optional[int] = ..., modulation: _Optional[_Union[_common_pb2.Modulation, str]] = ..., lora_modulation_info: _Optional[_Union[LoraModulationInfo, _Mapping]] = ..., fsk_modulation_info: _Optional[_Union[FskModulationInfo, _Mapping]] = ..., board: _Optional[int] = ..., antenna: _Optional[int] = ..., timing: _Optional[_Union[DownlinkTiming, str]] = ..., immediately_timing_info: _Optional[_Union[ImmediatelyTimingInfo, _Mapping]] = ..., delay_timing_info: _Optional[_Union[DelayTimingInfo, _Mapping]] = ..., gps_epoch_timing_info: _Optional[_Union[GPSEpochTimingInfo, _Mapping]] = ..., context: _Optional[bytes] = ...) -> None: ...

class EncryptedFineTimestamp(_message.Message):
    __slots__ = ["aes_key_index", "encrypted_ns", "fpga_id"]
    AES_KEY_INDEX_FIELD_NUMBER: _ClassVar[int]
    ENCRYPTED_NS_FIELD_NUMBER: _ClassVar[int]
    FPGA_ID_FIELD_NUMBER: _ClassVar[int]
    aes_key_index: int
    encrypted_ns: bytes
    fpga_id: bytes
    def __init__(self, aes_key_index: _Optional[int] = ..., encrypted_ns: _Optional[bytes] = ..., fpga_id: _Optional[bytes] = ...) -> None: ...

class FskModulationConfig(_message.Message):
    __slots__ = ["bandwidth", "bandwidth_legacy", "bitrate"]
    BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    BANDWIDTH_LEGACY_FIELD_NUMBER: _ClassVar[int]
    BITRATE_FIELD_NUMBER: _ClassVar[int]
    bandwidth: int
    bandwidth_legacy: int
    bitrate: int
    def __init__(self, bandwidth_legacy: _Optional[int] = ..., bandwidth: _Optional[int] = ..., bitrate: _Optional[int] = ...) -> None: ...

class FskModulationInfo(_message.Message):
    __slots__ = ["datarate", "frequency_deviation"]
    DATARATE_FIELD_NUMBER: _ClassVar[int]
    FREQUENCY_DEVIATION_FIELD_NUMBER: _ClassVar[int]
    datarate: int
    frequency_deviation: int
    def __init__(self, frequency_deviation: _Optional[int] = ..., datarate: _Optional[int] = ...) -> None: ...

class GPSEpochTimingInfo(_message.Message):
    __slots__ = ["time_since_gps_epoch"]
    TIME_SINCE_GPS_EPOCH_FIELD_NUMBER: _ClassVar[int]
    time_since_gps_epoch: _duration_pb2.Duration
    def __init__(self, time_since_gps_epoch: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class GatewayCommandExecRequest(_message.Message):
    __slots__ = ["command", "environment", "exec_id", "gateway_id", "gateway_id_legacy", "stdin"]
    class EnvironmentEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    COMMAND_FIELD_NUMBER: _ClassVar[int]
    ENVIRONMENT_FIELD_NUMBER: _ClassVar[int]
    EXEC_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    STDIN_FIELD_NUMBER: _ClassVar[int]
    command: str
    environment: _containers.ScalarMap[str, str]
    exec_id: int
    gateway_id: str
    gateway_id_legacy: bytes
    stdin: bytes
    def __init__(self, gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ..., command: _Optional[str] = ..., exec_id: _Optional[int] = ..., stdin: _Optional[bytes] = ..., environment: _Optional[_Mapping[str, str]] = ...) -> None: ...

class GatewayCommandExecResponse(_message.Message):
    __slots__ = ["error", "exec_id", "gateway_id", "gateway_id_legacy", "stderr", "stdout"]
    ERROR_FIELD_NUMBER: _ClassVar[int]
    EXEC_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    STDERR_FIELD_NUMBER: _ClassVar[int]
    STDOUT_FIELD_NUMBER: _ClassVar[int]
    error: str
    exec_id: int
    gateway_id: str
    gateway_id_legacy: bytes
    stderr: bytes
    stdout: bytes
    def __init__(self, gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ..., exec_id: _Optional[int] = ..., stdout: _Optional[bytes] = ..., stderr: _Optional[bytes] = ..., error: _Optional[str] = ...) -> None: ...

class GatewayConfiguration(_message.Message):
    __slots__ = ["channels", "gateway_id", "gateway_id_legacy", "stats_interval", "version"]
    CHANNELS_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    STATS_INTERVAL_FIELD_NUMBER: _ClassVar[int]
    VERSION_FIELD_NUMBER: _ClassVar[int]
    channels: _containers.RepeatedCompositeFieldContainer[ChannelConfiguration]
    gateway_id: str
    gateway_id_legacy: bytes
    stats_interval: _duration_pb2.Duration
    version: str
    def __init__(self, gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ..., version: _Optional[str] = ..., channels: _Optional[_Iterable[_Union[ChannelConfiguration, _Mapping]]] = ..., stats_interval: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ...) -> None: ...

class GatewayStats(_message.Message):
    __slots__ = ["config_version", "gateway_id", "gateway_id_legacy", "location", "metadata", "rx_packets_per_frequency", "rx_packets_per_modulation", "rx_packets_received", "rx_packets_received_ok", "time", "tx_packets_emitted", "tx_packets_per_frequency", "tx_packets_per_modulation", "tx_packets_per_status", "tx_packets_received"]
    class MetadataEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    class RxPacketsPerFrequencyEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: int
        value: int
        def __init__(self, key: _Optional[int] = ..., value: _Optional[int] = ...) -> None: ...
    class TxPacketsPerFrequencyEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: int
        value: int
        def __init__(self, key: _Optional[int] = ..., value: _Optional[int] = ...) -> None: ...
    class TxPacketsPerStatusEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: int
        def __init__(self, key: _Optional[str] = ..., value: _Optional[int] = ...) -> None: ...
    CONFIG_VERSION_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_PER_FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_PER_MODULATION_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_RECEIVED_FIELD_NUMBER: _ClassVar[int]
    RX_PACKETS_RECEIVED_OK_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_EMITTED_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_PER_FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_PER_MODULATION_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_PER_STATUS_FIELD_NUMBER: _ClassVar[int]
    TX_PACKETS_RECEIVED_FIELD_NUMBER: _ClassVar[int]
    config_version: str
    gateway_id: str
    gateway_id_legacy: bytes
    location: _common_pb2.Location
    metadata: _containers.ScalarMap[str, str]
    rx_packets_per_frequency: _containers.ScalarMap[int, int]
    rx_packets_per_modulation: _containers.RepeatedCompositeFieldContainer[PerModulationCount]
    rx_packets_received: int
    rx_packets_received_ok: int
    time: _timestamp_pb2.Timestamp
    tx_packets_emitted: int
    tx_packets_per_frequency: _containers.ScalarMap[int, int]
    tx_packets_per_modulation: _containers.RepeatedCompositeFieldContainer[PerModulationCount]
    tx_packets_per_status: _containers.ScalarMap[str, int]
    tx_packets_received: int
    def __init__(self, gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., location: _Optional[_Union[_common_pb2.Location, _Mapping]] = ..., config_version: _Optional[str] = ..., rx_packets_received: _Optional[int] = ..., rx_packets_received_ok: _Optional[int] = ..., tx_packets_received: _Optional[int] = ..., tx_packets_emitted: _Optional[int] = ..., metadata: _Optional[_Mapping[str, str]] = ..., tx_packets_per_frequency: _Optional[_Mapping[int, int]] = ..., rx_packets_per_frequency: _Optional[_Mapping[int, int]] = ..., tx_packets_per_modulation: _Optional[_Iterable[_Union[PerModulationCount, _Mapping]]] = ..., rx_packets_per_modulation: _Optional[_Iterable[_Union[PerModulationCount, _Mapping]]] = ..., tx_packets_per_status: _Optional[_Mapping[str, int]] = ...) -> None: ...

class ImmediatelyTimingInfo(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class LoraModulationConfig(_message.Message):
    __slots__ = ["bandwidth", "bandwidth_legacy", "spreading_factors"]
    BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    BANDWIDTH_LEGACY_FIELD_NUMBER: _ClassVar[int]
    SPREADING_FACTORS_FIELD_NUMBER: _ClassVar[int]
    bandwidth: int
    bandwidth_legacy: int
    spreading_factors: _containers.RepeatedScalarFieldContainer[int]
    def __init__(self, bandwidth_legacy: _Optional[int] = ..., bandwidth: _Optional[int] = ..., spreading_factors: _Optional[_Iterable[int]] = ...) -> None: ...

class LoraModulationInfo(_message.Message):
    __slots__ = ["bandwidth", "code_rate", "code_rate_legacy", "polarization_inversion", "spreading_factor"]
    BANDWIDTH_FIELD_NUMBER: _ClassVar[int]
    CODE_RATE_FIELD_NUMBER: _ClassVar[int]
    CODE_RATE_LEGACY_FIELD_NUMBER: _ClassVar[int]
    POLARIZATION_INVERSION_FIELD_NUMBER: _ClassVar[int]
    SPREADING_FACTOR_FIELD_NUMBER: _ClassVar[int]
    bandwidth: int
    code_rate: CodeRate
    code_rate_legacy: str
    polarization_inversion: bool
    spreading_factor: int
    def __init__(self, bandwidth: _Optional[int] = ..., spreading_factor: _Optional[int] = ..., code_rate_legacy: _Optional[str] = ..., code_rate: _Optional[_Union[CodeRate, str]] = ..., polarization_inversion: bool = ...) -> None: ...

class LrFhssModulationInfo(_message.Message):
    __slots__ = ["code_rate", "code_rate_legacy", "grid_steps", "operating_channel_width"]
    CODE_RATE_FIELD_NUMBER: _ClassVar[int]
    CODE_RATE_LEGACY_FIELD_NUMBER: _ClassVar[int]
    GRID_STEPS_FIELD_NUMBER: _ClassVar[int]
    OPERATING_CHANNEL_WIDTH_FIELD_NUMBER: _ClassVar[int]
    code_rate: CodeRate
    code_rate_legacy: str
    grid_steps: int
    operating_channel_width: int
    def __init__(self, operating_channel_width: _Optional[int] = ..., code_rate_legacy: _Optional[str] = ..., code_rate: _Optional[_Union[CodeRate, str]] = ..., grid_steps: _Optional[int] = ...) -> None: ...

class Modulation(_message.Message):
    __slots__ = ["fsk", "lora", "lr_fhss"]
    FSK_FIELD_NUMBER: _ClassVar[int]
    LORA_FIELD_NUMBER: _ClassVar[int]
    LR_FHSS_FIELD_NUMBER: _ClassVar[int]
    fsk: FskModulationInfo
    lora: LoraModulationInfo
    lr_fhss: LrFhssModulationInfo
    def __init__(self, lora: _Optional[_Union[LoraModulationInfo, _Mapping]] = ..., fsk: _Optional[_Union[FskModulationInfo, _Mapping]] = ..., lr_fhss: _Optional[_Union[LrFhssModulationInfo, _Mapping]] = ...) -> None: ...

class PerModulationCount(_message.Message):
    __slots__ = ["count", "modulation"]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    MODULATION_FIELD_NUMBER: _ClassVar[int]
    count: int
    modulation: Modulation
    def __init__(self, modulation: _Optional[_Union[Modulation, _Mapping]] = ..., count: _Optional[int] = ...) -> None: ...

class PlainFineTimestamp(_message.Message):
    __slots__ = ["time"]
    TIME_FIELD_NUMBER: _ClassVar[int]
    time: _timestamp_pb2.Timestamp
    def __init__(self, time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class RawPacketForwarderCommand(_message.Message):
    __slots__ = ["gateway_id", "gateway_id_legacy", "payload"]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    gateway_id_legacy: bytes
    payload: bytes
    def __init__(self, gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ..., payload: _Optional[bytes] = ...) -> None: ...

class RawPacketForwarderEvent(_message.Message):
    __slots__ = ["gateway_id", "gateway_id_legacy", "payload"]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_LEGACY_FIELD_NUMBER: _ClassVar[int]
    PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    gateway_id: str
    gateway_id_legacy: bytes
    payload: bytes
    def __init__(self, gateway_id_legacy: _Optional[bytes] = ..., gateway_id: _Optional[str] = ..., payload: _Optional[bytes] = ...) -> None: ...

class Timing(_message.Message):
    __slots__ = ["delay", "gps_epoch", "immediately"]
    DELAY_FIELD_NUMBER: _ClassVar[int]
    GPS_EPOCH_FIELD_NUMBER: _ClassVar[int]
    IMMEDIATELY_FIELD_NUMBER: _ClassVar[int]
    delay: DelayTimingInfo
    gps_epoch: GPSEpochTimingInfo
    immediately: ImmediatelyTimingInfo
    def __init__(self, immediately: _Optional[_Union[ImmediatelyTimingInfo, _Mapping]] = ..., delay: _Optional[_Union[DelayTimingInfo, _Mapping]] = ..., gps_epoch: _Optional[_Union[GPSEpochTimingInfo, _Mapping]] = ...) -> None: ...

class UplinkFrame(_message.Message):
    __slots__ = ["phy_payload", "rx_info", "rx_info_legacy", "tx_info", "tx_info_legacy"]
    PHY_PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    RX_INFO_FIELD_NUMBER: _ClassVar[int]
    RX_INFO_LEGACY_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_LEGACY_FIELD_NUMBER: _ClassVar[int]
    phy_payload: bytes
    rx_info: UplinkRxInfo
    rx_info_legacy: UplinkRxInfoLegacy
    tx_info: UplinkTxInfo
    tx_info_legacy: UplinkTxInfoLegacy
    def __init__(self, phy_payload: _Optional[bytes] = ..., tx_info_legacy: _Optional[_Union[UplinkTxInfoLegacy, _Mapping]] = ..., rx_info_legacy: _Optional[_Union[UplinkRxInfoLegacy, _Mapping]] = ..., tx_info: _Optional[_Union[UplinkTxInfo, _Mapping]] = ..., rx_info: _Optional[_Union[UplinkRxInfo, _Mapping]] = ...) -> None: ...

class UplinkFrameSet(_message.Message):
    __slots__ = ["phy_payload", "rx_info", "tx_info"]
    PHY_PAYLOAD_FIELD_NUMBER: _ClassVar[int]
    RX_INFO_FIELD_NUMBER: _ClassVar[int]
    TX_INFO_FIELD_NUMBER: _ClassVar[int]
    phy_payload: bytes
    rx_info: _containers.RepeatedCompositeFieldContainer[UplinkRxInfo]
    tx_info: UplinkTxInfo
    def __init__(self, phy_payload: _Optional[bytes] = ..., tx_info: _Optional[_Union[UplinkTxInfo, _Mapping]] = ..., rx_info: _Optional[_Iterable[_Union[UplinkRxInfo, _Mapping]]] = ...) -> None: ...

class UplinkRxInfo(_message.Message):
    __slots__ = ["antenna", "board", "channel", "context", "fine_time_since_gps_epoch", "gateway_id", "location", "metadata", "rf_chain", "rssi", "snr", "time", "time_since_gps_epoch", "uplink_id"]
    class MetadataEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    ANTENNA_FIELD_NUMBER: _ClassVar[int]
    BOARD_FIELD_NUMBER: _ClassVar[int]
    CHANNEL_FIELD_NUMBER: _ClassVar[int]
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    FINE_TIME_SINCE_GPS_EPOCH_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    RF_CHAIN_FIELD_NUMBER: _ClassVar[int]
    RSSI_FIELD_NUMBER: _ClassVar[int]
    SNR_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    TIME_SINCE_GPS_EPOCH_FIELD_NUMBER: _ClassVar[int]
    UPLINK_ID_FIELD_NUMBER: _ClassVar[int]
    antenna: int
    board: int
    channel: int
    context: bytes
    fine_time_since_gps_epoch: _duration_pb2.Duration
    gateway_id: str
    location: _common_pb2.Location
    metadata: _containers.ScalarMap[str, str]
    rf_chain: int
    rssi: int
    snr: float
    time: _timestamp_pb2.Timestamp
    time_since_gps_epoch: _duration_pb2.Duration
    uplink_id: int
    def __init__(self, gateway_id: _Optional[str] = ..., uplink_id: _Optional[int] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., time_since_gps_epoch: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ..., fine_time_since_gps_epoch: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ..., rssi: _Optional[int] = ..., snr: _Optional[float] = ..., channel: _Optional[int] = ..., rf_chain: _Optional[int] = ..., board: _Optional[int] = ..., antenna: _Optional[int] = ..., location: _Optional[_Union[_common_pb2.Location, _Mapping]] = ..., context: _Optional[bytes] = ..., metadata: _Optional[_Mapping[str, str]] = ...) -> None: ...

class UplinkRxInfoLegacy(_message.Message):
    __slots__ = ["antenna", "board", "channel", "context", "crc_status", "encrypted_fine_timestamp", "fine_timestamp_type", "gateway_id", "location", "lora_snr", "metadata", "plain_fine_timestamp", "rf_chain", "rssi", "time", "time_since_gps_epoch", "uplink_id"]
    class MetadataEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    ANTENNA_FIELD_NUMBER: _ClassVar[int]
    BOARD_FIELD_NUMBER: _ClassVar[int]
    CHANNEL_FIELD_NUMBER: _ClassVar[int]
    CONTEXT_FIELD_NUMBER: _ClassVar[int]
    CRC_STATUS_FIELD_NUMBER: _ClassVar[int]
    ENCRYPTED_FINE_TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    FINE_TIMESTAMP_TYPE_FIELD_NUMBER: _ClassVar[int]
    GATEWAY_ID_FIELD_NUMBER: _ClassVar[int]
    LOCATION_FIELD_NUMBER: _ClassVar[int]
    LORA_SNR_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    PLAIN_FINE_TIMESTAMP_FIELD_NUMBER: _ClassVar[int]
    RF_CHAIN_FIELD_NUMBER: _ClassVar[int]
    RSSI_FIELD_NUMBER: _ClassVar[int]
    TIME_FIELD_NUMBER: _ClassVar[int]
    TIME_SINCE_GPS_EPOCH_FIELD_NUMBER: _ClassVar[int]
    UPLINK_ID_FIELD_NUMBER: _ClassVar[int]
    antenna: int
    board: int
    channel: int
    context: bytes
    crc_status: CRCStatus
    encrypted_fine_timestamp: EncryptedFineTimestamp
    fine_timestamp_type: FineTimestampType
    gateway_id: bytes
    location: _common_pb2.Location
    lora_snr: float
    metadata: _containers.ScalarMap[str, str]
    plain_fine_timestamp: PlainFineTimestamp
    rf_chain: int
    rssi: int
    time: _timestamp_pb2.Timestamp
    time_since_gps_epoch: _duration_pb2.Duration
    uplink_id: bytes
    def __init__(self, gateway_id: _Optional[bytes] = ..., time: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., time_since_gps_epoch: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ..., rssi: _Optional[int] = ..., lora_snr: _Optional[float] = ..., channel: _Optional[int] = ..., rf_chain: _Optional[int] = ..., board: _Optional[int] = ..., antenna: _Optional[int] = ..., location: _Optional[_Union[_common_pb2.Location, _Mapping]] = ..., fine_timestamp_type: _Optional[_Union[FineTimestampType, str]] = ..., encrypted_fine_timestamp: _Optional[_Union[EncryptedFineTimestamp, _Mapping]] = ..., plain_fine_timestamp: _Optional[_Union[PlainFineTimestamp, _Mapping]] = ..., context: _Optional[bytes] = ..., uplink_id: _Optional[bytes] = ..., crc_status: _Optional[_Union[CRCStatus, str]] = ..., metadata: _Optional[_Mapping[str, str]] = ...) -> None: ...

class UplinkTxInfo(_message.Message):
    __slots__ = ["frequency", "modulation"]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    MODULATION_FIELD_NUMBER: _ClassVar[int]
    frequency: int
    modulation: Modulation
    def __init__(self, frequency: _Optional[int] = ..., modulation: _Optional[_Union[Modulation, _Mapping]] = ...) -> None: ...

class UplinkTxInfoLegacy(_message.Message):
    __slots__ = ["frequency", "fsk_modulation_info", "lora_modulation_info", "lr_fhss_modulation_info", "modulation"]
    FREQUENCY_FIELD_NUMBER: _ClassVar[int]
    FSK_MODULATION_INFO_FIELD_NUMBER: _ClassVar[int]
    LORA_MODULATION_INFO_FIELD_NUMBER: _ClassVar[int]
    LR_FHSS_MODULATION_INFO_FIELD_NUMBER: _ClassVar[int]
    MODULATION_FIELD_NUMBER: _ClassVar[int]
    frequency: int
    fsk_modulation_info: FskModulationInfo
    lora_modulation_info: LoraModulationInfo
    lr_fhss_modulation_info: LrFhssModulationInfo
    modulation: _common_pb2.Modulation
    def __init__(self, frequency: _Optional[int] = ..., modulation: _Optional[_Union[_common_pb2.Modulation, str]] = ..., lora_modulation_info: _Optional[_Union[LoraModulationInfo, _Mapping]] = ..., fsk_modulation_info: _Optional[_Union[FskModulationInfo, _Mapping]] = ..., lr_fhss_modulation_info: _Optional[_Union[LrFhssModulationInfo, _Mapping]] = ...) -> None: ...

class CodeRate(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class DownlinkTiming(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class FineTimestampType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class CRCStatus(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class TxAckStatus(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
