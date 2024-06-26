<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/device_profile.proto

namespace Chirpstack\Api;

use UnexpectedValueException;

/**
 * Protobuf type <code>api.CodecRuntime</code>
 */
class CodecRuntime
{
    /**
     * None.
     *
     * Generated from protobuf enum <code>NONE = 0;</code>
     */
    const NONE = 0;
    /**
     * Cayenne LPP.
     *
     * Generated from protobuf enum <code>CAYENNE_LPP = 1;</code>
     */
    const CAYENNE_LPP = 1;
    /**
     * JavaScript.
     *
     * Generated from protobuf enum <code>JS = 2;</code>
     */
    const JS = 2;

    private static $valueToName = [
        self::NONE => 'NONE',
        self::CAYENNE_LPP => 'CAYENNE_LPP',
        self::JS => 'JS',
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

