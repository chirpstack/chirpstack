<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * RelayService is the service providing API methos for managing relays.
 */
class RelayServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * List lists the relays for the given application id.
     * @param \Chirpstack\Api\ListRelaysRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListRelaysRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.RelayService/List',
        $argument,
        ['\Chirpstack\Api\ListRelaysResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * AddDevice adds the given device to the relay.
     * @param \Chirpstack\Api\AddRelayDeviceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddDevice(\Chirpstack\Api\AddRelayDeviceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.RelayService/AddDevice',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * RemoveDevice removes the given device from the relay.
     * @param \Chirpstack\Api\RemoveRelayDeviceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function RemoveDevice(\Chirpstack\Api\RemoveRelayDeviceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.RelayService/RemoveDevice',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * ListDevices lists the devices for the given relay.
     * @param \Chirpstack\Api\ListRelayDevicesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListDevices(\Chirpstack\Api\ListRelayDevicesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.RelayService/ListDevices',
        $argument,
        ['\Chirpstack\Api\ListRelayDevicesResponse', 'decode'],
        $metadata, $options);
    }

}
