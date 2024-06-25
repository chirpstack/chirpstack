<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * GatewayService is the service providing API methods for managing gateways.
 */
class GatewayServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Create creates the given gateway.
     * @param \Chirpstack\Api\CreateGatewayRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\Chirpstack\Api\CreateGatewayRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/Create',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get returns the gateway for the given Gateway ID.
     * @param \Chirpstack\Api\GetGatewayRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Get(\Chirpstack\Api\GetGatewayRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/Get',
        $argument,
        ['\Chirpstack\Api\GetGatewayResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update updates the given gateway.
     * @param \Chirpstack\Api\UpdateGatewayRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Update(\Chirpstack\Api\UpdateGatewayRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/Update',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete deletes the gateway matching the given Gateway ID.
     * @param \Chirpstack\Api\DeleteGatewayRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Delete(\Chirpstack\Api\DeleteGatewayRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/Delete',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the list of gateways.
     * @param \Chirpstack\Api\ListGatewaysRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListGatewaysRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/List',
        $argument,
        ['\Chirpstack\Api\ListGatewaysResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Generate client-certificate for the gateway.
     * @param \Chirpstack\Api\GenerateGatewayClientCertificateRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GenerateClientCertificate(\Chirpstack\Api\GenerateGatewayClientCertificateRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/GenerateClientCertificate',
        $argument,
        ['\Chirpstack\Api\GenerateGatewayClientCertificateResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetMetrics returns the gateway metrics.
     * @param \Chirpstack\Api\GetGatewayMetricsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetMetrics(\Chirpstack\Api\GetGatewayMetricsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/GetMetrics',
        $argument,
        ['\Chirpstack\Api\GetGatewayMetricsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetDutyCycleMetrics returns the duty-cycle metrics.
     * Note that only the last 2 hours of data are stored. Currently only per minute aggregation is available.
     * @param \Chirpstack\Api\GetGatewayDutyCycleMetricsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetDutyCycleMetrics(\Chirpstack\Api\GetGatewayDutyCycleMetricsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/GetDutyCycleMetrics',
        $argument,
        ['\Chirpstack\Api\GetGatewayDutyCycleMetricsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the given Relay Gateway.
     * @param \Chirpstack\Api\GetRelayGatewayRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRelayGateway(\Chirpstack\Api\GetRelayGatewayRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/GetRelayGateway',
        $argument,
        ['\Chirpstack\Api\GetRelayGatewayResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * List the detected Relay Gateways.
     * @param \Chirpstack\Api\ListRelayGatewaysRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListRelayGateways(\Chirpstack\Api\ListRelayGatewaysRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/ListRelayGateways',
        $argument,
        ['\Chirpstack\Api\ListRelayGatewaysResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given Relay Gateway.
     * @param \Chirpstack\Api\UpdateRelayGatewayRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateRelayGateway(\Chirpstack\Api\UpdateRelayGatewayRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/UpdateRelayGateway',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the given Relay Gateway.
     * @param \Chirpstack\Api\DeleteRelayGatewayRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteRelayGateway(\Chirpstack\Api\DeleteRelayGatewayRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.GatewayService/DeleteRelayGateway',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

}
