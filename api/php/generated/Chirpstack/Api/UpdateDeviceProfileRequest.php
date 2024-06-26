<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/device_profile.proto

namespace Chirpstack\Api;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>api.UpdateDeviceProfileRequest</code>
 */
class UpdateDeviceProfileRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Device-profile object.
     *
     * Generated from protobuf field <code>.api.DeviceProfile device_profile = 1;</code>
     */
    protected $device_profile = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type \Chirpstack\Api\DeviceProfile $device_profile
     *           Device-profile object.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Api\DeviceProfile::initOnce();
        parent::__construct($data);
    }

    /**
     * Device-profile object.
     *
     * Generated from protobuf field <code>.api.DeviceProfile device_profile = 1;</code>
     * @return \Chirpstack\Api\DeviceProfile|null
     */
    public function getDeviceProfile()
    {
        return $this->device_profile;
    }

    public function hasDeviceProfile()
    {
        return isset($this->device_profile);
    }

    public function clearDeviceProfile()
    {
        unset($this->device_profile);
    }

    /**
     * Device-profile object.
     *
     * Generated from protobuf field <code>.api.DeviceProfile device_profile = 1;</code>
     * @param \Chirpstack\Api\DeviceProfile $var
     * @return $this
     */
    public function setDeviceProfile($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Api\DeviceProfile::class);
        $this->device_profile = $var;

        return $this;
    }

}

