<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/internal.proto

namespace Chirpstack\Api;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>api.GlobalSearchResult</code>
 */
class GlobalSearchResult extends \Google\Protobuf\Internal\Message
{
    /**
     * Record kind.
     *
     * Generated from protobuf field <code>string kind = 1;</code>
     */
    protected $kind = '';
    /**
     * Search score.
     *
     * Generated from protobuf field <code>float score = 2;</code>
     */
    protected $score = 0.0;
    /**
     * Organization id.
     *
     * Generated from protobuf field <code>string tenant_id = 3;</code>
     */
    protected $tenant_id = '';
    /**
     * Organization name.
     *
     * Generated from protobuf field <code>string tenant_name = 4;</code>
     */
    protected $tenant_name = '';
    /**
     * Application id.
     *
     * Generated from protobuf field <code>string application_id = 5;</code>
     */
    protected $application_id = '';
    /**
     * Application name.
     *
     * Generated from protobuf field <code>string application_name = 6;</code>
     */
    protected $application_name = '';
    /**
     * Device DevEUI (hex encoded).
     *
     * Generated from protobuf field <code>string device_dev_eui = 7;</code>
     */
    protected $device_dev_eui = '';
    /**
     * Device name.
     *
     * Generated from protobuf field <code>string device_name = 8;</code>
     */
    protected $device_name = '';
    /**
     * Gateway MAC (hex encoded).
     *
     * Generated from protobuf field <code>string gateway_id = 9;</code>
     */
    protected $gateway_id = '';
    /**
     * Gateway name.
     *
     * Generated from protobuf field <code>string gateway_name = 10;</code>
     */
    protected $gateway_name = '';

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $kind
     *           Record kind.
     *     @type float $score
     *           Search score.
     *     @type string $tenant_id
     *           Organization id.
     *     @type string $tenant_name
     *           Organization name.
     *     @type string $application_id
     *           Application id.
     *     @type string $application_name
     *           Application name.
     *     @type string $device_dev_eui
     *           Device DevEUI (hex encoded).
     *     @type string $device_name
     *           Device name.
     *     @type string $gateway_id
     *           Gateway MAC (hex encoded).
     *     @type string $gateway_name
     *           Gateway name.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Api\Internal::initOnce();
        parent::__construct($data);
    }

    /**
     * Record kind.
     *
     * Generated from protobuf field <code>string kind = 1;</code>
     * @return string
     */
    public function getKind()
    {
        return $this->kind;
    }

    /**
     * Record kind.
     *
     * Generated from protobuf field <code>string kind = 1;</code>
     * @param string $var
     * @return $this
     */
    public function setKind($var)
    {
        GPBUtil::checkString($var, True);
        $this->kind = $var;

        return $this;
    }

    /**
     * Search score.
     *
     * Generated from protobuf field <code>float score = 2;</code>
     * @return float
     */
    public function getScore()
    {
        return $this->score;
    }

    /**
     * Search score.
     *
     * Generated from protobuf field <code>float score = 2;</code>
     * @param float $var
     * @return $this
     */
    public function setScore($var)
    {
        GPBUtil::checkFloat($var);
        $this->score = $var;

        return $this;
    }

    /**
     * Organization id.
     *
     * Generated from protobuf field <code>string tenant_id = 3;</code>
     * @return string
     */
    public function getTenantId()
    {
        return $this->tenant_id;
    }

    /**
     * Organization id.
     *
     * Generated from protobuf field <code>string tenant_id = 3;</code>
     * @param string $var
     * @return $this
     */
    public function setTenantId($var)
    {
        GPBUtil::checkString($var, True);
        $this->tenant_id = $var;

        return $this;
    }

    /**
     * Organization name.
     *
     * Generated from protobuf field <code>string tenant_name = 4;</code>
     * @return string
     */
    public function getTenantName()
    {
        return $this->tenant_name;
    }

    /**
     * Organization name.
     *
     * Generated from protobuf field <code>string tenant_name = 4;</code>
     * @param string $var
     * @return $this
     */
    public function setTenantName($var)
    {
        GPBUtil::checkString($var, True);
        $this->tenant_name = $var;

        return $this;
    }

    /**
     * Application id.
     *
     * Generated from protobuf field <code>string application_id = 5;</code>
     * @return string
     */
    public function getApplicationId()
    {
        return $this->application_id;
    }

    /**
     * Application id.
     *
     * Generated from protobuf field <code>string application_id = 5;</code>
     * @param string $var
     * @return $this
     */
    public function setApplicationId($var)
    {
        GPBUtil::checkString($var, True);
        $this->application_id = $var;

        return $this;
    }

    /**
     * Application name.
     *
     * Generated from protobuf field <code>string application_name = 6;</code>
     * @return string
     */
    public function getApplicationName()
    {
        return $this->application_name;
    }

    /**
     * Application name.
     *
     * Generated from protobuf field <code>string application_name = 6;</code>
     * @param string $var
     * @return $this
     */
    public function setApplicationName($var)
    {
        GPBUtil::checkString($var, True);
        $this->application_name = $var;

        return $this;
    }

    /**
     * Device DevEUI (hex encoded).
     *
     * Generated from protobuf field <code>string device_dev_eui = 7;</code>
     * @return string
     */
    public function getDeviceDevEui()
    {
        return $this->device_dev_eui;
    }

    /**
     * Device DevEUI (hex encoded).
     *
     * Generated from protobuf field <code>string device_dev_eui = 7;</code>
     * @param string $var
     * @return $this
     */
    public function setDeviceDevEui($var)
    {
        GPBUtil::checkString($var, True);
        $this->device_dev_eui = $var;

        return $this;
    }

    /**
     * Device name.
     *
     * Generated from protobuf field <code>string device_name = 8;</code>
     * @return string
     */
    public function getDeviceName()
    {
        return $this->device_name;
    }

    /**
     * Device name.
     *
     * Generated from protobuf field <code>string device_name = 8;</code>
     * @param string $var
     * @return $this
     */
    public function setDeviceName($var)
    {
        GPBUtil::checkString($var, True);
        $this->device_name = $var;

        return $this;
    }

    /**
     * Gateway MAC (hex encoded).
     *
     * Generated from protobuf field <code>string gateway_id = 9;</code>
     * @return string
     */
    public function getGatewayId()
    {
        return $this->gateway_id;
    }

    /**
     * Gateway MAC (hex encoded).
     *
     * Generated from protobuf field <code>string gateway_id = 9;</code>
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
     * Gateway name.
     *
     * Generated from protobuf field <code>string gateway_name = 10;</code>
     * @return string
     */
    public function getGatewayName()
    {
        return $this->gateway_name;
    }

    /**
     * Gateway name.
     *
     * Generated from protobuf field <code>string gateway_name = 10;</code>
     * @param string $var
     * @return $this
     */
    public function setGatewayName($var)
    {
        GPBUtil::checkString($var, True);
        $this->gateway_name = $var;

        return $this;
    }

}

