<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/application.proto

namespace Chirpstack\Api;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>api.CreateHttpIntegrationRequest</code>
 */
class CreateHttpIntegrationRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Integration object to create.
     *
     * Generated from protobuf field <code>.api.HttpIntegration integration = 1;</code>
     */
    protected $integration = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type \Chirpstack\Api\HttpIntegration $integration
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
     * Generated from protobuf field <code>.api.HttpIntegration integration = 1;</code>
     * @return \Chirpstack\Api\HttpIntegration|null
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
     * Generated from protobuf field <code>.api.HttpIntegration integration = 1;</code>
     * @param \Chirpstack\Api\HttpIntegration $var
     * @return $this
     */
    public function setIntegration($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Api\HttpIntegration::class);
        $this->integration = $var;

        return $this;
    }

}

