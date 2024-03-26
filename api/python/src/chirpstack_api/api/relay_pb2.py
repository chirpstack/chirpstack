# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: chirpstack-api/api/relay.proto
# Protobuf Python Version: 4.25.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.api import annotations_pb2 as google_dot_api_dot_annotations__pb2
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2
from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1e\x63hirpstack-api/api/relay.proto\x12\x03\x61pi\x1a\x1cgoogle/api/annotations.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a\x1bgoogle/protobuf/empty.proto\".\n\rRelayListItem\x12\x0f\n\x07\x64\x65v_eui\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\"J\n\x11ListRelaysRequest\x12\r\n\x05limit\x18\x01 \x01(\r\x12\x0e\n\x06offset\x18\x02 \x01(\r\x12\x16\n\x0e\x61pplication_id\x18\x03 \x01(\t\"M\n\x12ListRelaysResponse\x12\x13\n\x0btotal_count\x18\x01 \x01(\r\x12\"\n\x06result\x18\x02 \x03(\x0b\x32\x12.api.RelayListItem\"F\n\x15\x41\x64\x64RelayDeviceRequest\x12\x15\n\rrelay_dev_eui\x18\x01 \x01(\t\x12\x16\n\x0e\x64\x65vice_dev_eui\x18\x02 \x01(\t\"I\n\x18RemoveRelayDeviceRequest\x12\x15\n\rrelay_dev_eui\x18\x01 \x01(\t\x12\x16\n\x0e\x64\x65vice_dev_eui\x18\x02 \x01(\t\"O\n\x17ListRelayDevicesRequest\x12\r\n\x05limit\x18\x01 \x01(\r\x12\x0e\n\x06offset\x18\x02 \x01(\r\x12\x15\n\rrelay_dev_eui\x18\x03 \x01(\t\"d\n\x13RelayDeviceListItem\x12\x0f\n\x07\x64\x65v_eui\x18\x01 \x01(\t\x12.\n\ncreated_at\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.Timestamp\x12\x0c\n\x04name\x18\x03 \x01(\t\"Y\n\x18ListRelayDevicesResponse\x12\x13\n\x0btotal_count\x18\x01 \x01(\r\x12(\n\x06result\x18\x02 \x03(\x0b\x32\x18.api.RelayDeviceListItem2\xc4\x03\n\x0cRelayService\x12L\n\x04List\x12\x16.api.ListRelaysRequest\x1a\x17.api.ListRelaysResponse\"\x13\x82\xd3\xe4\x93\x02\r\x12\x0b/api/relays\x12o\n\tAddDevice\x12\x1a.api.AddRelayDeviceRequest\x1a\x16.google.protobuf.Empty\".\x82\xd3\xe4\x93\x02(\"#/api/relays/{relay_dev_eui}/devices:\x01*\x12|\n\x0cRemoveDevice\x12\x1d.api.RemoveRelayDeviceRequest\x1a\x16.google.protobuf.Empty\"5\x82\xd3\xe4\x93\x02/*-/api/relays/{relay_dev_eui}/devices/{dev_eui}\x12w\n\x0bListDevices\x12\x1c.api.ListRelayDevicesRequest\x1a\x1d.api.ListRelayDevicesResponse\"+\x82\xd3\xe4\x93\x02%\x12#/api/relays/{relay_dev_eui}/devicesBb\n\x11io.chirpstack.apiB\nRelayProtoP\x01Z.github.com/chirpstack/chirpstack/api/go/v4/api\xaa\x02\x0e\x43hirpstack.Apib\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'chirpstack_api.api.relay_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:
  _globals['DESCRIPTOR']._options = None
  _globals['DESCRIPTOR']._serialized_options = b'\n\021io.chirpstack.apiB\nRelayProtoP\001Z.github.com/chirpstack/chirpstack/api/go/v4/api\252\002\016Chirpstack.Api'
  _globals['_RELAYSERVICE'].methods_by_name['List']._options = None
  _globals['_RELAYSERVICE'].methods_by_name['List']._serialized_options = b'\202\323\344\223\002\r\022\013/api/relays'
  _globals['_RELAYSERVICE'].methods_by_name['AddDevice']._options = None
  _globals['_RELAYSERVICE'].methods_by_name['AddDevice']._serialized_options = b'\202\323\344\223\002(\"#/api/relays/{relay_dev_eui}/devices:\001*'
  _globals['_RELAYSERVICE'].methods_by_name['RemoveDevice']._options = None
  _globals['_RELAYSERVICE'].methods_by_name['RemoveDevice']._serialized_options = b'\202\323\344\223\002/*-/api/relays/{relay_dev_eui}/devices/{dev_eui}'
  _globals['_RELAYSERVICE'].methods_by_name['ListDevices']._options = None
  _globals['_RELAYSERVICE'].methods_by_name['ListDevices']._serialized_options = b'\202\323\344\223\002%\022#/api/relays/{relay_dev_eui}/devices'
  _globals['_RELAYLISTITEM']._serialized_start=131
  _globals['_RELAYLISTITEM']._serialized_end=177
  _globals['_LISTRELAYSREQUEST']._serialized_start=179
  _globals['_LISTRELAYSREQUEST']._serialized_end=253
  _globals['_LISTRELAYSRESPONSE']._serialized_start=255
  _globals['_LISTRELAYSRESPONSE']._serialized_end=332
  _globals['_ADDRELAYDEVICEREQUEST']._serialized_start=334
  _globals['_ADDRELAYDEVICEREQUEST']._serialized_end=404
  _globals['_REMOVERELAYDEVICEREQUEST']._serialized_start=406
  _globals['_REMOVERELAYDEVICEREQUEST']._serialized_end=479
  _globals['_LISTRELAYDEVICESREQUEST']._serialized_start=481
  _globals['_LISTRELAYDEVICESREQUEST']._serialized_end=560
  _globals['_RELAYDEVICELISTITEM']._serialized_start=562
  _globals['_RELAYDEVICELISTITEM']._serialized_end=662
  _globals['_LISTRELAYDEVICESRESPONSE']._serialized_start=664
  _globals['_LISTRELAYDEVICESRESPONSE']._serialized_end=753
  _globals['_RELAYSERVICE']._serialized_start=756
  _globals['_RELAYSERVICE']._serialized_end=1208
# @@protoc_insertion_point(module_scope)
