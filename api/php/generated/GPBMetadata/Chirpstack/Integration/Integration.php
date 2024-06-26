<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: integration/integration.proto

namespace GPBMetadata\Chirpstack\Integration;

class Integration
{
    public static $is_initialized = false;

    public static function initOnce() {
        $pool = \Google\Protobuf\Internal\DescriptorPool::getGeneratedPool();

        if (static::$is_initialized == true) {
          return;
        }
        \GPBMetadata\Chirpstack\Common\Common::initOnce();
        \GPBMetadata\Chirpstack\Gateway\Gw::initOnce();
        \GPBMetadata\Google\Protobuf\Timestamp::initOnce();
        \GPBMetadata\Google\Protobuf\Struct::initOnce();
        $pool->internalAddGeneratedFile(
            '
�
integration/integration.protointegrationgw/gw.protogoogle/protobuf/timestamp.protogoogle/protobuf/struct.proto"�

DeviceInfo
	tenant_id (	
tenant_name (	
application_id (	
application_name (	
device_profile_id (	
device_profile_name (	
device_name (	
dev_eui (	1
device_class_enabled
 (2.common.DeviceClass/
tags	 (2!.integration.DeviceInfo.TagsEntry+
	TagsEntry
key (	
value (	:8"s
UplinkRelayRxInfo
dev_eui (	
	frequency (

dr (
snr (
rssi (
wor_channel ("�
UplinkEvent
deduplication_id (	(
time (2.google.protobuf.Timestamp,
device_info (2.integration.DeviceInfo
dev_addr (	
adr (

dr (
f_cnt (
f_port (
	confirmed	 (
data
 (\'
object (2.google.protobuf.Struct!
rx_info (2.gw.UplinkRxInfo!
tx_info (2.gw.UplinkTxInfo5
relay_rx_info (2.integration.UplinkRelayRxInfo6
join_server_context (2.common.JoinServerContext"�
	JoinEvent
deduplication_id (	(
time (2.google.protobuf.Timestamp,
device_info (2.integration.DeviceInfo
dev_addr (	5
relay_rx_info (2.integration.UplinkRelayRxInfo6
join_server_context (2.common.JoinServerContext"�
AckEvent
deduplication_id (	(
time (2.google.protobuf.Timestamp,
device_info (2.integration.DeviceInfo
queue_item_id (	
acknowledged (

f_cnt_down ("�

TxAckEvent
downlink_id ((
time (2.google.protobuf.Timestamp,
device_info (2.integration.DeviceInfo
queue_item_id (	

f_cnt_down (

gateway_id (	#
tx_info (2.gw.DownlinkTxInfo"�
LogEvent(
time (2.google.protobuf.Timestamp,
device_info (2.integration.DeviceInfo$
level (2.integration.LogLevel"
code (2.integration.LogCode
description (	3
context (2".integration.LogEvent.ContextEntry.
ContextEntry
key (	
value (	:8"�
StatusEvent
deduplication_id (	(
time (2.google.protobuf.Timestamp,
device_info (2.integration.DeviceInfo
margin (
external_power_source (!
battery_level_unavailable (
battery_level ("�
LocationEvent
deduplication_id (	(
time (2.google.protobuf.Timestamp,
device_info (2.integration.DeviceInfo"
location (2.common.Location"�
IntegrationEvent
deduplication_id (	(
time (2.google.protobuf.Timestamp,
device_info (2.integration.DeviceInfo
integration_name (	

event_type (	\'
object (2.google.protobuf.Struct"�
DownlinkCommand

id (	
dev_eui (	
	confirmed (
f_port (
data (\'
object (2.google.protobuf.Struct*,
LogLevel
INFO 
WARNING	
ERROR*�
LogCode
UNKNOWN 
DOWNLINK_PAYLOAD_SIZE
UPLINK_CODEC
DOWNLINK_CODEC
OTAA
UPLINK_F_CNT_RESET

UPLINK_MIC
UPLINK_F_CNT_RETRANSMISSION
DOWNLINK_GATEWAY
RELAY_NEW_END_DEVICE	

F_CNT_DOWN
B�
io.chirpstack.api.integrationBIntegrationProtoPZ3github.com/brocaar/chirpstack/api/go/v4/integration�Chirpstack.Integration�Chirpstack\\Integration�"GPBMetadata\\Chirpstack\\Integrationbproto3'
        , true);

        static::$is_initialized = true;
    }
}

