<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: gw/gw.proto

namespace Chirpstack\Gateway;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Gateway Mesh heartbeat (sent periodically by the Relay Gateways).
 *
 * Generated from protobuf message <code>gw.MeshHeartbeat</code>
 */
class MeshHeartbeat extends \Google\Protobuf\Internal\Message
{
    /**
     * Gateway ID (of the Border Gateway).
     *
     * Generated from protobuf field <code>string gateway_id = 1;</code>
     */
    protected $gateway_id = '';
    /**
     * Relay ID.
     *
     * Generated from protobuf field <code>string relay_id = 2;</code>
     */
    protected $relay_id = '';
    /**
     * Timestamp (second precision).
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 3;</code>
     */
    protected $time = null;
    /**
     * Relay path.
     *
     * Generated from protobuf field <code>repeated .gw.MeshHeartbeatRelayPath relay_path = 4;</code>
     */
    private $relay_path;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $gateway_id
     *           Gateway ID (of the Border Gateway).
     *     @type string $relay_id
     *           Relay ID.
     *     @type \Google\Protobuf\Timestamp $time
     *           Timestamp (second precision).
     *     @type array<\Chirpstack\Gateway\MeshHeartbeatRelayPath>|\Google\Protobuf\Internal\RepeatedField $relay_path
     *           Relay path.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Gateway\Gw::initOnce();
        parent::__construct($data);
    }

    /**
     * Gateway ID (of the Border Gateway).
     *
     * Generated from protobuf field <code>string gateway_id = 1;</code>
     * @return string
     */
    public function getGatewayId()
    {
        return $this->gateway_id;
    }

    /**
     * Gateway ID (of the Border Gateway).
     *
     * Generated from protobuf field <code>string gateway_id = 1;</code>
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
     * Relay ID.
     *
     * Generated from protobuf field <code>string relay_id = 2;</code>
     * @return string
     */
    public function getRelayId()
    {
        return $this->relay_id;
    }

    /**
     * Relay ID.
     *
     * Generated from protobuf field <code>string relay_id = 2;</code>
     * @param string $var
     * @return $this
     */
    public function setRelayId($var)
    {
        GPBUtil::checkString($var, True);
        $this->relay_id = $var;

        return $this;
    }

    /**
     * Timestamp (second precision).
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 3;</code>
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
     * Timestamp (second precision).
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp time = 3;</code>
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
     * Relay path.
     *
     * Generated from protobuf field <code>repeated .gw.MeshHeartbeatRelayPath relay_path = 4;</code>
     * @return \Google\Protobuf\Internal\RepeatedField
     */
    public function getRelayPath()
    {
        return $this->relay_path;
    }

    /**
     * Relay path.
     *
     * Generated from protobuf field <code>repeated .gw.MeshHeartbeatRelayPath relay_path = 4;</code>
     * @param array<\Chirpstack\Gateway\MeshHeartbeatRelayPath>|\Google\Protobuf\Internal\RepeatedField $var
     * @return $this
     */
    public function setRelayPath($var)
    {
        $arr = GPBUtil::checkRepeatedField($var, \Google\Protobuf\Internal\GPBType::MESSAGE, \Chirpstack\Gateway\MeshHeartbeatRelayPath::class);
        $this->relay_path = $arr;

        return $this;
    }

}

