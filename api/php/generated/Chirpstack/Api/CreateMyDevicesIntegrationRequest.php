<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/application.proto

namespace Chirpstack\Api;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>api.CreateMyDevicesIntegrationRequest</code>
 */
class CreateMyDevicesIntegrationRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Integration object to create.
     *
     * Generated from protobuf field <code>.api.MyDevicesIntegration integration = 1;</code>
     */
    protected $integration = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type \Chirpstack\Api\MyDevicesIntegration $integration
     *           Integration object to create.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Api\Application::initOnce();
        parent::__construct($data);
    }

    /**
     * Integration object to create.
     *
     * Generated from protobuf field <code>.api.MyDevicesIntegration integration = 1;</code>
     * @return \Chirpstack\Api\MyDevicesIntegration|null
     */
    public function getIntegration()
    {
        return $this->integration;
    }

    public function hasIntegration()
    {
        return isset($this->integration);
    }

    public function clearIntegration()
    {
        unset($this->integration);
    }

    /**
     * Integration object to create.
     *
     * Generated from protobuf field <code>.api.MyDevicesIntegration integration = 1;</code>
     * @param \Chirpstack\Api\MyDevicesIntegration $var
     * @return $this
     */
    public function setIntegration($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Api\MyDevicesIntegration::class);
        $this->integration = $var;

        return $this;
    }

}

