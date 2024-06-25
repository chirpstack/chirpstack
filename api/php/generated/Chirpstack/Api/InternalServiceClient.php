<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * InternalService is the service providing API endpoints for internal usage.
 */
class InternalServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Log in a user
     * @param \Chirpstack\Api\LoginRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Login(\Chirpstack\Api\LoginRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/Login',
        $argument,
        ['\Chirpstack\Api\LoginResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the current user's profile
     * @param \Google\Protobuf\GPBEmpty $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Profile(\Google\Protobuf\GPBEmpty $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/Profile',
        $argument,
        ['\Chirpstack\Api\ProfileResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Perform a global search.
     * @param \Chirpstack\Api\GlobalSearchRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GlobalSearch(\Chirpstack\Api\GlobalSearchRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/GlobalSearch',
        $argument,
        ['\Chirpstack\Api\GlobalSearchResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * CreateApiKey creates the given API key.
     * @param \Chirpstack\Api\CreateApiKeyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateApiKey(\Chirpstack\Api\CreateApiKeyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/CreateApiKey',
        $argument,
        ['\Chirpstack\Api\CreateApiKeyResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * DeleteApiKey deletes the API key.
     * @param \Chirpstack\Api\DeleteApiKeyRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteApiKey(\Chirpstack\Api\DeleteApiKeyRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/DeleteApiKey',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * ListApiKeys lists the available API keys.
     * @param \Chirpstack\Api\ListApiKeysRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListApiKeys(\Chirpstack\Api\ListApiKeysRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/ListApiKeys',
        $argument,
        ['\Chirpstack\Api\ListApiKeysResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the global settings.
     * @param \Google\Protobuf\GPBEmpty $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Settings(\Google\Protobuf\GPBEmpty $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/Settings',
        $argument,
        ['\Chirpstack\Api\SettingsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * OpenId Connect login.
     * @param \Chirpstack\Api\OpenIdConnectLoginRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function OpenIdConnectLogin(\Chirpstack\Api\OpenIdConnectLoginRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/OpenIdConnectLogin',
        $argument,
        ['\Chirpstack\Api\OpenIdConnectLoginResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * OAuth2 login.
     * @param \Chirpstack\Api\OAuth2LoginRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function OAuth2Login(\Chirpstack\Api\OAuth2LoginRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/OAuth2Login',
        $argument,
        ['\Chirpstack\Api\OAuth2LoginResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetDevicesSummary returns an aggregated summary of the devices.
     * @param \Chirpstack\Api\GetDevicesSummaryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDevicesSummary(\Chirpstack\Api\GetDevicesSummaryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/GetDevicesSummary',
        $argument,
        ['\Chirpstack\Api\GetDevicesSummaryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetGatewaysSummary returns an aggregated summary of the gateways.
     * @param \Chirpstack\Api\GetGatewaysSummaryRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetGatewaysSummary(\Chirpstack\Api\GetGatewaysSummaryRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/GetGatewaysSummary',
        $argument,
        ['\Chirpstack\Api\GetGatewaysSummaryResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Stream frame for the given Gateway ID.
     * @param \Chirpstack\Api\StreamGatewayFramesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function StreamGatewayFrames(\Chirpstack\Api\StreamGatewayFramesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/api.InternalService/StreamGatewayFrames',
        $argument,
        ['\Chirpstack\Api\LogItem', 'decode'],
        $metadata, $options);
    }

    /**
     * Stream frames for the given Device EUI.
     * @param \Chirpstack\Api\StreamDeviceFramesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function StreamDeviceFrames(\Chirpstack\Api\StreamDeviceFramesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/api.InternalService/StreamDeviceFrames',
        $argument,
        ['\Chirpstack\Api\LogItem', 'decode'],
        $metadata, $options);
    }

    /**
     * Stream events for the given Device EUI.
     * @param \Chirpstack\Api\StreamDeviceEventsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\ServerStreamingCall
     */
    public function StreamDeviceEvents(\Chirpstack\Api\StreamDeviceEventsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_serverStreamRequest('/api.InternalService/StreamDeviceEvents',
        $argument,
        ['\Chirpstack\Api\LogItem', 'decode'],
        $metadata, $options);
    }

    /**
     * ListRegions lists the available (configured) regions.
     * @param \Google\Protobuf\GPBEmpty $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListRegions(\Google\Protobuf\GPBEmpty $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/ListRegions',
        $argument,
        ['\Chirpstack\Api\ListRegionsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetRegion returns the region details for the given region.
     * @param \Chirpstack\Api\GetRegionRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRegion(\Chirpstack\Api\GetRegionRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/GetRegion',
        $argument,
        ['\Chirpstack\Api\GetRegionResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetVersion returns the ChirpStack version.
     * @param \Google\Protobuf\GPBEmpty $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetVersion(\Google\Protobuf\GPBEmpty $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.InternalService/GetVersion',
        $argument,
        ['\Chirpstack\Api\GetVersionResponse', 'decode'],
        $metadata, $options);
    }

}
