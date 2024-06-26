<?php
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: api/internal.proto

namespace Chirpstack\Api;

use Google\Protobuf\Internal\GPBType;
use Google\Protobuf\Internal\RepeatedField;
use Google\Protobuf\Internal\GPBUtil;

/**
 * Generated from protobuf message <code>api.SettingsResponse</code>
 */
class SettingsResponse extends \Google\Protobuf\Internal\Message
{
    /**
     * OpenId Connect settings.
     *
     * Generated from protobuf field <code>.api.OpenIdConnect openid_connect = 1;</code>
     */
    protected $openid_connect = null;
    /**
     * OAuth2 settings.
     *
     * Generated from protobuf field <code>.api.OAuth2 oauth2 = 2;</code>
     */
    protected $oauth2 = null;

    /**
     * Constructor.
     *
     * @param array $data {
     *     Optional. Data for populating the Message object.
     *
     *     @type \Chirpstack\Api\OpenIdConnect $openid_connect
     *           OpenId Connect settings.
     *     @type \Chirpstack\Api\OAuth2 $oauth2
     *           OAuth2 settings.
     * }
     */
    public function __construct($data = NULL) {
        \GPBMetadata\Chirpstack\Api\Internal::initOnce();
        parent::__construct($data);
    }

    /**
     * OpenId Connect settings.
     *
     * Generated from protobuf field <code>.api.OpenIdConnect openid_connect = 1;</code>
     * @return \Chirpstack\Api\OpenIdConnect|null
     */
    public function getOpenidConnect()
    {
        return $this->openid_connect;
    }

    public function hasOpenidConnect()
    {
        return isset($this->openid_connect);
    }

    public function clearOpenidConnect()
    {
        unset($this->openid_connect);
    }

    /**
     * OpenId Connect settings.
     *
     * Generated from protobuf field <code>.api.OpenIdConnect openid_connect = 1;</code>
     * @param \Chirpstack\Api\OpenIdConnect $var
     * @return $this
     */
    public function setOpenidConnect($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Api\OpenIdConnect::class);
        $this->openid_connect = $var;

        return $this;
    }

    /**
     * OAuth2 settings.
     *
     * Generated from protobuf field <code>.api.OAuth2 oauth2 = 2;</code>
     * @return \Chirpstack\Api\OAuth2|null
     */
    public function getOauth2()
    {
        return $this->oauth2;
    }

    public function hasOauth2()
    {
        return isset($this->oauth2);
    }

    public function clearOauth2()
    {
        unset($this->oauth2);
    }

    /**
     * OAuth2 settings.
     *
     * Generated from protobuf field <code>.api.OAuth2 oauth2 = 2;</code>
     * @param \Chirpstack\Api\OAuth2 $var
     * @return $this
     */
    public function setOauth2($var)
    {
        GPBUtil::checkMessage($var, \Chirpstack\Api\OAuth2::class);
        $this->oauth2 = $var;

        return $this;
    }

}

