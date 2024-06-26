<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/gateway.proto

namespace Chirpstack\Api;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>api.GetGatewayMetricsRequest</code>
 */
class GetGatewayMetricsRequest extends \Google\Protobuf\Internal\Message
{
    /**
     * Gateway ID (EUI64).
     *
     * Generated from protobuf field <code>string gateway_id = 1;</code>
     */
    protected $gateway_id = '';
    /**
     * Interval start timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp start = 2;</code>
     */
    protected $start = null;
    /**
     * Interval end timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp end = 3;</code>
     */
    protected $end = null;
    /**
     * Aggregation.
     *
     * Generated from protobuf field <code>.common.Aggregation aggregation = 4;</code>
     */
    protected $aggregation = 0;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type string $gateway_id
     *           Gateway ID (EUI64).
     *     @type \Google\Protobuf\Timestamp $start
     *           Interval start timestamp.
     *     @type \Google\Protobuf\Timestamp $end
     *           Interval end timestamp.
     *     @type int $aggregation
     *           Aggregation.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Api\Gateway::initOnce();
        parent::__construct($data);
    }

    /**
     * Gateway ID (EUI64).
     *
     * Generated from protobuf field <code>string gateway_id = 1;</code>
     * @return string
     */
    public function getGatewayId()
    {
        return $this->gateway_id;
    }

    /**
     * Gateway ID (EUI64).
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
     * Interval start timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp start = 2;</code>
     * @return \Google\Protobuf\Timestamp|null
     */
    public function getStart()
    {
        return $this->start;
    }

    public function hasStart()
    {
        return isset($this->start);
    }

    public function clearStart()
    {
        unset($this->start);
    }

    /**
     * Interval start timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp start = 2;</code>
     * @param \Google\Protobuf\Timestamp $var
     * @return $this
     */
    public function setStart($var)
    {
        GPBUtil::checkMessage($var, \Google\Protobuf\Timestamp::class);
        $this->start = $var;

        return $this;
    }

    /**
     * Interval end timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp end = 3;</code>
     * @return \Google\Protobuf\Timestamp|null
     */
    public function getEnd()
    {
        return $this->end;
    }

    public function hasEnd()
    {
        return isset($this->end);
    }

    public function clearEnd()
    {
        unset($this->end);
    }

    /**
     * Interval end timestamp.
     *
     * Generated from protobuf field <code>.google.protobuf.Timestamp end = 3;</code>
     * @param \Google\Protobuf\Timestamp $var
     * @return $this
     */
    public function setEnd($var)
    {
        GPBUtil::checkMessage($var, \Google\Protobuf\Timestamp::class);
        $this->end = $var;

        return $this;
    }

    /**
     * Aggregation.
     *
     * Generated from protobuf field <code>.common.Aggregation aggregation = 4;</code>
     * @return int
     */
    public function getAggregation()
    {
        return $this->aggregation;
    }

    /**
     * Aggregation.
     *
     * Generated from protobuf field <code>.common.Aggregation aggregation = 4;</code>
     * @param int $var
     * @return $this
     */
    public function setAggregation($var)
    {
        GPBUtil::checkEnum($var, \Chirpstack\Common\Aggregation::class);
        $this->aggregation = $var;

        return $this;
    }

}

