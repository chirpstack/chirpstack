<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: gw/gw.proto

namespace Chirpstack\Gateway;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>gw.UplinkTxInfoLegacy</code>
 */
class UplinkTxInfoLegacy extends \Google\Protobuf\Internal\Message
{
    /**
     * Frequency (Hz).
     *
     * Generated from protobuf field <code>uint32 frequency = 1;</code>
     */
    protected $frequency = 0;
    /**
     * Modulation.
     *
     * Generated from protobuf field <code>.common.Modulation modulation = 2;</code>
     */
    protected $modulation = 0;
    protected $modulation_info;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type int $frequency
     *           Frequency (Hz).
     *     @type int $modulation
     *           Modulation.
     *     @type \Chirpstack\Gateway\LoraModulationInfo $lora_modulation_info
     *           LoRa modulation information.
     *     @type \Chirpstack\Gateway\FskModulationInfo $fsk_modulation_info
     *           FSK modulation information.
     *     @type \Chirpstack\Gateway\LrFhssModulationInfo $lr_fhss_modulation_info
     *           LR-FHSS modulation information.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Gateway\Gw::initOnce();
        parent::__construct($data);
    }

    /**
     * Frequency (Hz).
     *
     * Generated from protobuf field <code>uint32 frequency = 1;</code>
     * @return int
     */
    public function getFrequency()
    {
        return $this->frequency;
    }

    /**
     * Frequency (Hz).
     *
     * Generated from protobuf field <code>uint32 frequency = 1;</code>
     * @param int $var
     * @return $this
     */
    public function setFrequency($var)
    {
        GPBUtil::checkUint32($var);
        $this->frequency = $var;

        return $this;
    }

    /**
     * Modulation.
     *
     * Generated from protobuf field <code>.common.Modulation modulation = 2;</code>
     * @return int
     */
    public function getModulation()
    {
        return $this->modulation;
    }

    /**
     * Modulation.
     *
     * Generated from protobuf field <code>.common.Modulation modulation = 2;</code>
     * @param int $var
     * @return $this
     */
    public function setModulation($var)
    {
        GPBUtil::checkEnum($var, \Chirpstack\Common\Modulation::class);
        $this->modulation = $var;

        return $this;
    }

    /**
     * LoRa modulation information.
     *
     * Generated from protobuf field <code>.gw.LoraModulationInfo lora_modulation_info = 3;</code>
     * @return \Chirpstack\Gateway\LoraModulationInfo|null
     */
    public function getLoraModulationInfo()
    {
        return $this->readOneof(3);
    }

    public function hasLoraModulationInfo()
    {
        return $this->hasOneof(3);
    }

    /**
     * LoRa modulation information.
     *
     * Generated from protobuf field <code>.gw.LoraModulationInfo lora_modulation_info = 3;</code>
     * @param \Chirpstack\Gateway\LoraModulationInfo $var
     * @return $this
     */
    public function setLoraModulationInfo($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Gateway\LoraModulationInfo::class);
        $this->writeOneof(3, $var);

        return $this;
    }

    /**
     * FSK modulation information.
     *
     * Generated from protobuf field <code>.gw.FskModulationInfo fsk_modulation_info = 4;</code>
     * @return \Chirpstack\Gateway\FskModulationInfo|null
     */
    public function getFskModulationInfo()
    {
        return $this->readOneof(4);
    }

    public function hasFskModulationInfo()
    {
        return $this->hasOneof(4);
    }

    /**
     * FSK modulation information.
     *
     * Generated from protobuf field <code>.gw.FskModulationInfo fsk_modulation_info = 4;</code>
     * @param \Chirpstack\Gateway\FskModulationInfo $var
     * @return $this
     */
    public function setFskModulationInfo($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Gateway\FskModulationInfo::class);
        $this->writeOneof(4, $var);

        return $this;
    }

    /**
     * LR-FHSS modulation information.
     *
     * Generated from protobuf field <code>.gw.LrFhssModulationInfo lr_fhss_modulation_info = 5;</code>
     * @return \Chirpstack\Gateway\LrFhssModulationInfo|null
     */
    public function getLrFhssModulationInfo()
    {
        return $this->readOneof(5);
    }

    public function hasLrFhssModulationInfo()
    {
        return $this->hasOneof(5);
    }

    /**
     * LR-FHSS modulation information.
     *
     * Generated from protobuf field <code>.gw.LrFhssModulationInfo lr_fhss_modulation_info = 5;</code>
     * @param \Chirpstack\Gateway\LrFhssModulationInfo $var
     * @return $this
     */
    public function setLrFhssModulationInfo($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Gateway\LrFhssModulationInfo::class);
        $this->writeOneof(5, $var);

        return $this;
    }

    /**
     * @return string
     */
    public function getModulationInfo()
    {
        return $this->whichOneof("modulation_info");
    }

}

