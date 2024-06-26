<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/device.proto

namespace Chirpstack\Api;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>api.UpdateDeviceRequest</code>
 */
class UpdateDeviceRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Device object.
     *
     * Generated from protobuf field <code>.api.Device device = 1;</code>
     */
    protected $device = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type \Chirpstack\Api\Device $device
     *           Device object.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Api\Device::initOnce();
        parent::__construct($data);
    }

    /**
     * Device object.
     *
     * Generated from protobuf field <code>.api.Device device = 1;</code>
     * @return \Chirpstack\Api\Device|null
     */
    public function getDevice()
    {
        return $this->device;
    }

    public function hasDevice()
    {
        return isset($this->device);
    }

    public function clearDevice()
    {
        unset($this->device);
    }

    /**
     * Device object.
     *
     * Generated from protobuf field <code>.api.Device device = 1;</code>
     * @param \Chirpstack\Api\Device $var
     * @return $this
     */
    public function setDevice($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Api\Device::class);
        $this->device = $var;

        return $this;
    }

}

