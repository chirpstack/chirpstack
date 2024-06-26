<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/gateway.proto

namespace Chirpstack\Api;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>api.UpdateRelayGatewayRequest</code>
 */
class UpdateRelayGatewayRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Relay Gateway object.
     *
     * Generated from protobuf field <code>.api.RelayGateway relay_gateway = 1;</code>
     */
    protected $relay_gateway = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type \Chirpstack\Api\RelayGateway $relay_gateway
     *           Relay Gateway object.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Api\Gateway::initOnce();
        parent::__construct($data);
    }

    /**
     * Relay Gateway object.
     *
     * Generated from protobuf field <code>.api.RelayGateway relay_gateway = 1;</code>
     * @return \Chirpstack\Api\RelayGateway|null
     */
    public function getRelayGateway()
    {
        return $this->relay_gateway;
    }

    public function hasRelayGateway()
    {
        return isset($this->relay_gateway);
    }

    public function clearRelayGateway()
    {
        unset($this->relay_gateway);
    }

    /**
     * Relay Gateway object.
     *
     * Generated from protobuf field <code>.api.RelayGateway relay_gateway = 1;</code>
     * @param \Chirpstack\Api\RelayGateway $var
     * @return $this
     */
    public function setRelayGateway($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Api\RelayGateway::class);
        $this->relay_gateway = $var;

        return $this;
    }

}

