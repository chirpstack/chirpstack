from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

A: RegParamsRevision
ABSOLUTE: MetricKind
AS923: Region
AS923_2: Region
AS923_3: Region
AS923_4: Region
AU915: Region
B: RegParamsRevision
CLASS_A: DeviceClass
CLASS_B: DeviceClass
CLASS_C: DeviceClass
CN470: Region
CN779: Region
CONFIG: LocationSource
CONFIRMED_DATA_DOWN: MType
CONFIRMED_DATA_UP: MType
COUNTER: MetricKind
DAY: Aggregation
DESCRIPTOR: _descriptor.FileDescriptor
EU433: Region
EU868: Region
FSK: Modulation
GAUGE: MetricKind
GEO_RESOLVER_GNSS: LocationSource
GEO_RESOLVER_RSSI: LocationSource
GEO_RESOLVER_TDOA: LocationSource
GEO_RESOLVER_WIFI: LocationSource
GPS: LocationSource
HOUR: Aggregation
IN865: Region
ISM2400: Region
JOIN_ACCEPT: MType
JOIN_REQUEST: MType
KR920: Region
LORA: Modulation
LORAWAN_1_0_0: MacVersion
LORAWAN_1_0_1: MacVersion
LORAWAN_1_0_2: MacVersion
LORAWAN_1_0_3: MacVersion
LORAWAN_1_0_4: MacVersion
LORAWAN_1_1_0: MacVersion
LR_FHSS: Modulation
MONTH: Aggregation
PROPRIETARY: MType
REJOIN_REQUEST: MType
RP002_1_0_0: RegParamsRevision
RP002_1_0_1: RegParamsRevision
RP002_1_0_2: RegParamsRevision
RP002_1_0_3: RegParamsRevision
RU864: Region
UNCONFIRMED_DATA_DOWN: MType
UNCONFIRMED_DATA_UP: MType
UNKNOWN: LocationSource
US915: Region

class KeyEnvelope(_message.Message):
    __slots__ = ["aes_key", "kek_label"]
    AES_KEY_FIELD_NUMBER: _ClassVar[int]
    KEK_LABEL_FIELD_NUMBER: _ClassVar[int]
    aes_key: bytes
    kek_label: str
    def __init__(self, kek_label: _Optional[str] = ..., aes_key: _Optional[bytes] = ...) -> None: ...

class Location(_message.Message):
    __slots__ = ["accuracy", "altitude", "latitude", "longitude", "source"]
    ACCURACY_FIELD_NUMBER: _ClassVar[int]
    ALTITUDE_FIELD_NUMBER: _ClassVar[int]
    LATITUDE_FIELD_NUMBER: _ClassVar[int]
    LONGITUDE_FIELD_NUMBER: _ClassVar[int]
    SOURCE_FIELD_NUMBER: _ClassVar[int]
    accuracy: float
    altitude: float
    latitude: float
    longitude: float
    source: LocationSource
    def __init__(self, latitude: _Optional[float] = ..., longitude: _Optional[float] = ..., altitude: _Optional[float] = ..., source: _Optional[_Union[LocationSource, str]] = ..., accuracy: _Optional[float] = ...) -> None: ...

class Metric(_message.Message):
    __slots__ = ["datasets", "kind", "name", "timestamps"]
    DATASETS_FIELD_NUMBER: _ClassVar[int]
    KIND_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    TIMESTAMPS_FIELD_NUMBER: _ClassVar[int]
    datasets: _containers.RepeatedCompositeFieldContainer[MetricDataset]
    kind: MetricKind
    name: str
    timestamps: _containers.RepeatedCompositeFieldContainer[_timestamp_pb2.Timestamp]
    def __init__(self, name: _Optional[str] = ..., timestamps: _Optional[_Iterable[_Union[_timestamp_pb2.Timestamp, _Mapping]]] = ..., datasets: _Optional[_Iterable[_Union[MetricDataset, _Mapping]]] = ..., kind: _Optional[_Union[MetricKind, str]] = ...) -> None: ...

class MetricDataset(_message.Message):
    __slots__ = ["data", "label"]
    DATA_FIELD_NUMBER: _ClassVar[int]
    LABEL_FIELD_NUMBER: _ClassVar[int]
    data: _containers.RepeatedScalarFieldContainer[float]
    label: str
    def __init__(self, label: _Optional[str] = ..., data: _Optional[_Iterable[float]] = ...) -> None: ...

class Modulation(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class Region(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class MType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class MacVersion(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class RegParamsRevision(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class LocationSource(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class Aggregation(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class MetricKind(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class DeviceClass(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
