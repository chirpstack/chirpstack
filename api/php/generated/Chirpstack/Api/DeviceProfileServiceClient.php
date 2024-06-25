<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * DeviceProfileService is the service providing API methods for managing
 * device-profiles.
 */
class DeviceProfileServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Create the given device-profile.
     * @param \Chirpstack\Api\CreateDeviceProfileRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\Chirpstack\Api\CreateDeviceProfileRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileService/Create',
        $argument,
        ['\Chirpstack\Api\CreateDeviceProfileResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the device-profile for the given ID.
     * @param \Chirpstack\Api\GetDeviceProfileRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Get(\Chirpstack\Api\GetDeviceProfileRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileService/Get',
        $argument,
        ['\Chirpstack\Api\GetDeviceProfileResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given device-profile.
     * @param \Chirpstack\Api\UpdateDeviceProfileRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Update(\Chirpstack\Api\UpdateDeviceProfileRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileService/Update',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the device-profile with the given ID.
     * @param \Chirpstack\Api\DeleteDeviceProfileRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Delete(\Chirpstack\Api\DeleteDeviceProfileRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileService/Delete',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * List the available device-profiles.
     * @param \Chirpstack\Api\ListDeviceProfilesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListDeviceProfilesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileService/List',
        $argument,
        ['\Chirpstack\Api\ListDeviceProfilesResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * List available ADR algorithms.
     * @param \Google\Protobuf\GPBEmpty $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListAdrAlgorithms(\Google\Protobuf\GPBEmpty $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileService/ListAdrAlgorithms',
        $argument,
        ['\Chirpstack\Api\ListDeviceProfileAdrAlgorithmsResponse', 'decode'],
        $metadata, $options);
    }

}
