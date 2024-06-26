<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: stream/frame.proto

namespace Chirpstack\Stream;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>stream.DownlinkFrameLog</code>
 */
class DownlinkFrameLog extends \Google\Protobuf\Internal\Message
{
    /**
     * Time.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 1;</code>
     */
    protected $time = null;
    /**
     * PHYPayload.
     *
     * Generated from protobuf field <code>bytes phy_payload = 2;</code>
     */
    protected $phy_payload = '';
    /**
     * TX meta-data.
     *
     * Generated from protobuf field <code>.gw.DownlinkTxInfo tx_info = 3;</code>
     */
    protected $tx_info = null;
    /**
     * Downlink ID.
     *
     * Generated from protobuf field <code>uint32 downlink_id = 4;</code>
     */
    protected $downlink_id = 0;
    /**
     * Gateway ID (EUI64).
     *
     * Generated from protobuf field <code>string gateway_id = 5;</code>
     */
    protected $gateway_id = '';
    /**
     * Message type.
     *
     * Generated from protobuf field <code>.common.MType m_type = 6;</code>
     */
    protected $m_type = 0;
    /**
     * Device address (optional).
     *
     * Generated from protobuf field <code>string dev_addr = 7;</code>
     */
    protected $dev_addr = '';
    /**
     * Device EUI (optional).
     *
     * Generated from protobuf field <code>string dev_eui = 8;</code>
     */
    protected $dev_eui = '';
    /**
     * Plaintext f_opts mac-commands.
     *
     * Generated from protobuf field <code>bool plaintext_f_opts = 9;</code>
     */
    protected $plaintext_f_opts = false;
    /**
     * Plaintext frm_payload.
     *
     * Generated from protobuf field <code>bool plaintext_frm_payload = 10;</code>
     */
    protected $plaintext_frm_payload = false;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type \Google\Protobuf\Timestamp $time
     *           Time.
     *     @type string $phy_payload
     *           PHYPayload.
     *     @type \Chirpstack\Gateway\DownlinkTxInfo $tx_info
     *           TX meta-data.
     *     @type int $downlink_id
     *           Downlink ID.
     *     @type string $gateway_id
     *           Gateway ID (EUI64).
     *     @type int $m_type
     *           Message type.
     *     @type string $dev_addr
     *           Device address (optional).
     *     @type string $dev_eui
     *           Device EUI (optional).
     *     @type bool $plaintext_f_opts
     *           Plaintext f_opts mac-commands.
     *     @type bool $plaintext_frm_payload
     *           Plaintext frm_payload.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Stream\Frame::initOnce();
        parent::__construct($data);
    }

    /**
     * Time.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 1;</code>
     * @return \Google\Protobuf\Timestamp|null
     */
    public function getTime()
    {
        return $this->time;
    }

    public function hasTime()
    {
        return isset($this->time);
    }

    public function clearTime()
    {
        unset($this->time);
    }

    /**
     * Time.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 1;</code>
     * @param \Google\Protobuf\Timestamp $var
     * @return $this
     */
    public function setTime($var)
    {
        GPBUtil::checkMessage($var, \Google\Protobuf\Timestamp::class);
        $this->time = $var;

        return $this;
    }

    /**
     * PHYPayload.
     *
     * Generated from protobuf field <code>bytes phy_payload = 2;</code>
     * @return string
     */
    public function getPhyPayload()
    {
        return $this->phy_payload;
    }

    /**
     * PHYPayload.
     *
     * Generated from protobuf field <code>bytes phy_payload = 2;</code>
     * @param string $var
     * @return $this
     */
    public function setPhyPayload($var)
    {
        GPBUtil::checkString($var, False);
        $this->phy_payload = $var;

        return $this;
    }

    /**
     * TX meta-data.
     *
     * Generated from protobuf field <code>.gw.DownlinkTxInfo tx_info = 3;</code>
     * @return \Chirpstack\Gateway\DownlinkTxInfo|null
     */
    public function getTxInfo()
    {
        return $this->tx_info;
    }

    public function hasTxInfo()
    {
        return isset($this->tx_info);
    }

    public function clearTxInfo()
    {
        unset($this->tx_info);
    }

    /**
     * TX meta-data.
     *
     * Generated from protobuf field <code>.gw.DownlinkTxInfo tx_info = 3;</code>
     * @param \Chirpstack\Gateway\DownlinkTxInfo $var
     * @return $this
     */
    public function setTxInfo($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Gateway\DownlinkTxInfo::class);
        $this->tx_info = $var;

        return $this;
    }

    /**
     * Downlink ID.
     *
     * Generated from protobuf field <code>uint32 downlink_id = 4;</code>
     * @return int
     */
    public function getDownlinkId()
    {
        return $this->downlink_id;
    }

    /**
     * Downlink ID.
     *
     * Generated from protobuf field <code>uint32 downlink_id = 4;</code>
     * @param int $var
     * @return $this
     */
    public function setDownlinkId($var)
    {
        GPBUtil::checkUint32($var);
        $this->downlink_id = $var;

        return $this;
    }

    /**
     * Gateway ID (EUI64).
     *
     * Generated from protobuf field <code>string gateway_id = 5;</code>
     * @return string
     */
    public function getGatewayId()
    {
        return $this->gateway_id;
    }

    /**
     * Gateway ID (EUI64).
     *
     * Generated from protobuf field <code>string gateway_id = 5;</code>
     * @param string $var
     * @return $this
     */
    public function setGatewayId($var)
    {
        GPBUtil::checkString($var, True);
        $this->gateway_id = $var;

        return $this;
    }

    /**
     * Message type.
     *
     * Generated from protobuf field <code>.common.MType m_type = 6;</code>
     * @return int
     */
    public function getMType()
    {
        return $this->m_type;
    }

    /**
     * Message type.
     *
     * Generated from protobuf field <code>.common.MType m_type = 6;</code>
     * @param int $var
     * @return $this
     */
    public function setMType($var)
    {
        GPBUtil::checkEnum($var, \Chirpstack\Common\MType::class);
        $this->m_type = $var;

        return $this;
    }

    /**
     * Device address (optional).
     *
     * Generated from protobuf field <code>string dev_addr = 7;</code>
     * @return string
     */
    public function getDevAddr()
    {
        return $this->dev_addr;
    }

    /**
     * Device address (optional).
     *
     * Generated from protobuf field <code>string dev_addr = 7;</code>
     * @param string $var
     * @return $this
     */
    public function setDevAddr($var)
    {
        GPBUtil::checkString($var, True);
        $this->dev_addr = $var;

        return $this;
    }

    /**
     * Device EUI (optional).
     *
     * Generated from protobuf field <code>string dev_eui = 8;</code>
     * @return string
     */
    public function getDevEui()
    {
        return $this->dev_eui;
    }

    /**
     * Device EUI (optional).
     *
     * Generated from protobuf field <code>string dev_eui = 8;</code>
     * @param string $var
     * @return $this
     */
    public function setDevEui($var)
    {
        GPBUtil::checkString($var, True);
        $this->dev_eui = $var;

        return $this;
    }

    /**
     * Plaintext f_opts mac-commands.
     *
     * Generated from protobuf field <code>bool plaintext_f_opts = 9;</code>
     * @return bool
     */
    public function getPlaintextFOpts()
    {
        return $this->plaintext_f_opts;
    }

    /**
     * Plaintext f_opts mac-commands.
     *
     * Generated from protobuf field <code>bool plaintext_f_opts = 9;</code>
     * @param bool $var
     * @return $this
     */
    public function setPlaintextFOpts($var)
    {
        GPBUtil::checkBool($var);
        $this->plaintext_f_opts = $var;

        return $this;
    }

    /**
     * Plaintext frm_payload.
     *
     * Generated from protobuf field <code>bool plaintext_frm_payload = 10;</code>
     * @return bool
     */
    public function getPlaintextFrmPayload()
    {
        return $this->plaintext_frm_payload;
    }

    /**
     * Plaintext frm_payload.
     *
     * Generated from protobuf field <code>bool plaintext_frm_payload = 10;</code>
     * @param bool $var
     * @return $this
     */
    public function setPlaintextFrmPayload($var)
    {
        GPBUtil::checkBool($var);
        $this->plaintext_frm_payload = $var;

        return $this;
    }

}

