<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: common/common.proto

namespace Chirpstack\Common;

use UnexpectedValueException;

/**
 * Protobuf type <code>common.Aggregation</code>
 */
class Aggregation
{
    /**
     * Hour.
     *
     * Generated from protobuf enum <code>HOUR = 0;</code>
     */
    const HOUR = 0;
    /**
     * Day.
     *
     * Generated from protobuf enum <code>DAY = 1;</code>
     */
    const DAY = 1;
    /**
     * Month.
     *
     * Generated from protobuf enum <code>MONTH = 2;</code>
     */
    const MONTH = 2;
    /**
     * Minute.
     *
     * Generated from protobuf enum <code>MINUTE = 3;</code>
     */
    const MINUTE = 3;

    private static $valueToName = [
        self::HOUR => 'HOUR',
        self::DAY => 'DAY',
        self::MONTH => 'MONTH',
        self::MINUTE => 'MINUTE',
    ];

    public static function name($value)
    {
        if (!isset(self::$valueToName[$value])) {
            throw new UnexpectedValueException(sprintf(
                    'Enum %s has no name defined for value %s', __CLASS__, $value));
        }
        return self::$valueToName[$value];
    }


    public static function value($name)
    {
        $const = __CLASS__ . '::' . strtoupper($name);
        if (!defined($const)) {
            throw new UnexpectedValueException(sprintf(
                    'Enum %s has no value defined for name %s', __CLASS__, $name));
        }
        return constant($const);
    }
}

