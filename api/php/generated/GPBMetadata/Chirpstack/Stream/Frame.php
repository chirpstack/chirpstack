<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: stream/frame.proto

namespace GPBMetadata\Chirpstack\Stream;

class Frame
{
    public static $is_initialized = false;

    public static function initOnce() {
        $pool = \Google\Protobuf\Internal\DescriptorPool::getGeneratedPool();

        if (static::$is_initialized == true) {
          return;
        }
        \GPBMetadata\Google\Protobuf\Timestamp::initOnce();
        \GPBMetadata\Chirpstack\Common\Common::initOnce();
        \GPBMetadata\Chirpstack\Gateway\Gw::initOnce();
        $pool->internalAddGeneratedFile(
            '
�
stream/frame.protostreamcommon/common.protogw/gw.proto"�
UplinkFrameLog
phy_payload (!
tx_info (2.gw.UplinkTxInfo!
rx_info (2.gw.UplinkRxInfo
m_type (2.common.MType
dev_addr (	
dev_eui (	(
time (2.google.protobuf.Timestamp
plaintext_f_opts (
plaintext_frm_payload	 ("�
DownlinkFrameLog(
time (2.google.protobuf.Timestamp
phy_payload (#
tx_info (2.gw.DownlinkTxInfo
downlink_id (

gateway_id (	
m_type (2.common.MType
dev_addr (	
dev_eui (	
plaintext_f_opts	 (
plaintext_frm_payload
 (B�
io.chirpstack.api.streamB
FrameProtoPZ1github.com/chirpstack/chirpstack/api/go/v4/stream�Chirpstack.Stream�Chirpstack\\Stream�GPBMetadata\\Chirpstack\\Streambproto3'
        , true);

        static::$is_initialized = true;
    }
}

