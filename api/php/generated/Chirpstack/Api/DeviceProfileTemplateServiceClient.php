<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * DeviceProfileTemplateService is the service providing API methods for managing device-profile templates.
 */
class DeviceProfileTemplateServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Create the given device-profile template.
     * @param \Chirpstack\Api\CreateDeviceProfileTemplateRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\Chirpstack\Api\CreateDeviceProfileTemplateRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileTemplateService/Create',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the device-profile template for the given ID.
     * @param \Chirpstack\Api\GetDeviceProfileTemplateRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Get(\Chirpstack\Api\GetDeviceProfileTemplateRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileTemplateService/Get',
        $argument,
        ['\Chirpstack\Api\GetDeviceProfileTemplateResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given device-profile template.
     * @param \Chirpstack\Api\UpdateDeviceProfileTemplateRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Update(\Chirpstack\Api\UpdateDeviceProfileTemplateRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileTemplateService/Update',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the device-profile template with the given ID.
     * @param \Chirpstack\Api\DeleteDeviceProfileTemplateRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Delete(\Chirpstack\Api\DeleteDeviceProfileTemplateRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileTemplateService/Delete',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * List the available device-profile templates.
     * @param \Chirpstack\Api\ListDeviceProfileTemplatesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListDeviceProfileTemplatesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceProfileTemplateService/List',
        $argument,
        ['\Chirpstack\Api\ListDeviceProfileTemplatesResponse', 'decode'],
        $metadata, $options);
    }

}
