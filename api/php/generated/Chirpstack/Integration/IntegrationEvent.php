<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: integration/integration.proto

namespace Chirpstack\Integration;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * IntegrationEvent is the message that can be sent by an integration.
 * It allows for sending events which are provided by an external integration
 * which are "not native" to ChirpStack.
 *
 * Generated from protobuf message <code>integration.IntegrationEvent</code>
 */
class IntegrationEvent extends \Google\Protobuf\Internal\Message
{
    /**
     * Deduplication ID (UUID).
     *
     * Generated from protobuf field <code>string deduplication_id = 1;</code>
     */
    protected $deduplication_id = '';
    /**
     * Timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 2;</code>
     */
    protected $time = null;
    /**
     * Device info.
     *
     * Generated from protobuf field <code>.integration.DeviceInfo device_info = 3;</code>
     */
    protected $device_info = null;
    /**
     * Integration name.
     *
     * Generated from protobuf field <code>string integration_name = 4;</code>
     */
    protected $integration_name = '';
    /**
     * Event type.
     *
     * Generated from protobuf field <code>string event_type = 5;</code>
     */
    protected $event_type = '';
    /**
     * Struct containing the event object.
     *
     * Generated from protobuf field <code>.google.protobuf.Struct object = 6;</code>
     */
    protected $object = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $deduplication_id
     *           Deduplication ID (UUID).
     *     @type \Google\Protobuf\Timestamp $time
     *           Timestamp.
     *     @type \Chirpstack\Integration\DeviceInfo $device_info
     *           Device info.
     *     @type string $integration_name
     *           Integration name.
     *     @type string $event_type
     *           Event type.
     *     @type \Google\Protobuf\Struct $object
     *           Struct containing the event object.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Integration\Integration::initOnce();
        parent::__construct($data);
    }

    /**
     * Deduplication ID (UUID).
     *
     * Generated from protobuf field <code>string deduplication_id = 1;</code>
     * @return string
     */
    public function getDeduplicationId()
    {
        return $this->deduplication_id;
    }

    /**
     * Deduplication ID (UUID).
     *
     * Generated from protobuf field <code>string deduplication_id = 1;</code>
     * @param string $var
     * @return $this
     */
    public function setDeduplicationId($var)
    {
        GPBUtil::checkString($var, True);
        $this->deduplication_id = $var;

        return $this;
    }

    /**
     * Timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 2;</code>
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
     * Timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 2;</code>
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
     * Device info.
     *
     * Generated from protobuf field <code>.integration.DeviceInfo device_info = 3;</code>
     * @return \Chirpstack\Integration\DeviceInfo|null
     */
    public function getDeviceInfo()
    {
        return $this->device_info;
    }

    public function hasDeviceInfo()
    {
        return isset($this->device_info);
    }

    public function clearDeviceInfo()
    {
        unset($this->device_info);
    }

    /**
     * Device info.
     *
     * Generated from protobuf field <code>.integration.DeviceInfo device_info = 3;</code>
     * @param \Chirpstack\Integration\DeviceInfo $var
     * @return $this
     */
    public function setDeviceInfo($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Integration\DeviceInfo::class);
        $this->device_info = $var;

        return $this;
    }

    /**
     * Integration name.
     *
     * Generated from protobuf field <code>string integration_name = 4;</code>
     * @return string
     */
    public function getIntegrationName()
    {
        return $this->integration_name;
    }

    /**
     * Integration name.
     *
     * Generated from protobuf field <code>string integration_name = 4;</code>
     * @param string $var
     * @return $this
     */
    public function setIntegrationName($var)
    {
        GPBUtil::checkString($var, True);
        $this->integration_name = $var;

        return $this;
    }

    /**
     * Event type.
     *
     * Generated from protobuf field <code>string event_type = 5;</code>
     * @return string
     */
    public function getEventType()
    {
        return $this->event_type;
    }

    /**
     * Event type.
     *
     * Generated from protobuf field <code>string event_type = 5;</code>
     * @param string $var
     * @return $this
     */
    public function setEventType($var)
    {
        GPBUtil::checkString($var, True);
        $this->event_type = $var;

        return $this;
    }

    /**
     * Struct containing the event object.
     *
     * Generated from protobuf field <code>.google.protobuf.Struct object = 6;</code>
     * @return \Google\Protobuf\Struct|null
     */
    public function getObject()
    {
        return $this->object;
    }

    public function hasObject()
    {
        return isset($this->object);
    }

    public function clearObject()
    {
        unset($this->object);
    }

    /**
     * Struct containing the event object.
     *
     * Generated from protobuf field <code>.google.protobuf.Struct object = 6;</code>
     * @param \Google\Protobuf\Struct $var
     * @return $this
     */
    public function setObject($var)
    {
        GPBUtil::checkMessage($var, \Google\Protobuf\Struct::class);
        $this->object = $var;

        return $this;
    }

}

