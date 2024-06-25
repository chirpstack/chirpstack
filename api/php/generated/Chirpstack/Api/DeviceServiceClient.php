<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * DeviceService is the service providing API methods for managing devices.
 */
class DeviceServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Create the given device.
     * @param \Chirpstack\Api\CreateDeviceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\Chirpstack\Api\CreateDeviceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/Create',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get returns the device for the given DevEUI.
     * @param \Chirpstack\Api\GetDeviceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Get(\Chirpstack\Api\GetDeviceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/Get',
        $argument,
        ['\Chirpstack\Api\GetDeviceResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given device.
     * @param \Chirpstack\Api\UpdateDeviceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Update(\Chirpstack\Api\UpdateDeviceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/Update',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the device with the given DevEUI.
     * @param \Chirpstack\Api\DeleteDeviceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Delete(\Chirpstack\Api\DeleteDeviceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/Delete',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the list of devices.
     * @param \Chirpstack\Api\ListDevicesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListDevicesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/List',
        $argument,
        ['\Chirpstack\Api\ListDevicesResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Create the given device-keys.
     * @param \Chirpstack\Api\CreateDeviceKeysRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateKeys(\Chirpstack\Api\CreateDeviceKeysRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/CreateKeys',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the device-keys for the given DevEUI.
     * @param \Chirpstack\Api\GetDeviceKeysRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetKeys(\Chirpstack\Api\GetDeviceKeysRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/GetKeys',
        $argument,
        ['\Chirpstack\Api\GetDeviceKeysResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given device-keys.
     * @param \Chirpstack\Api\UpdateDeviceKeysRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateKeys(\Chirpstack\Api\UpdateDeviceKeysRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/UpdateKeys',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the device-keys for the given DevEUI.
     * @param \Chirpstack\Api\DeleteDeviceKeysRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteKeys(\Chirpstack\Api\DeleteDeviceKeysRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/DeleteKeys',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * FlushDevNonces flushes the OTAA device nonces.
     * @param \Chirpstack\Api\FlushDevNoncesRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FlushDevNonces(\Chirpstack\Api\FlushDevNoncesRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/FlushDevNonces',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Activate (re)activates the device with the given parameters (for ABP or for
     * importing OTAA activations).
     * @param \Chirpstack\Api\ActivateDeviceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Activate(\Chirpstack\Api\ActivateDeviceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/Activate',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Deactivate de-activates the device.
     * @param \Chirpstack\Api\DeactivateDeviceRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Deactivate(\Chirpstack\Api\DeactivateDeviceRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/Deactivate',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * GetActivation returns the current activation details of the device (OTAA or
     * ABP).
     * @param \Chirpstack\Api\GetDeviceActivationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetActivation(\Chirpstack\Api\GetDeviceActivationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/GetActivation',
        $argument,
        ['\Chirpstack\Api\GetDeviceActivationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetRandomDevAddr returns a random DevAddr taking the NwkID prefix into
     * account.
     * @param \Chirpstack\Api\GetRandomDevAddrRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetRandomDevAddr(\Chirpstack\Api\GetRandomDevAddrRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/GetRandomDevAddr',
        $argument,
        ['\Chirpstack\Api\GetRandomDevAddrResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetMetrics returns the device metrics.
     * Note that this requires a device-profile with codec and measurements
     * configured.
     * @param \Chirpstack\Api\GetDeviceMetricsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetMetrics(\Chirpstack\Api\GetDeviceMetricsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/GetMetrics',
        $argument,
        ['\Chirpstack\Api\GetDeviceMetricsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetLinkMetrics returns the device link metrics.
     * This includes uplinks, downlinks, RSSI, SNR, etc...
     * @param \Chirpstack\Api\GetDeviceLinkMetricsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLinkMetrics(\Chirpstack\Api\GetDeviceLinkMetricsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/GetLinkMetrics',
        $argument,
        ['\Chirpstack\Api\GetDeviceLinkMetricsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Enqueue adds the given item to the downlink queue.
     * @param \Chirpstack\Api\EnqueueDeviceQueueItemRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Enqueue(\Chirpstack\Api\EnqueueDeviceQueueItemRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/Enqueue',
        $argument,
        ['\Chirpstack\Api\EnqueueDeviceQueueItemResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * FlushQueue flushes the downlink device-queue.
     * @param \Chirpstack\Api\FlushDeviceQueueRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FlushQueue(\Chirpstack\Api\FlushDeviceQueueRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/FlushQueue',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * GetQueue returns the downlink device-queue.
     * @param \Chirpstack\Api\GetDeviceQueueItemsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetQueue(\Chirpstack\Api\GetDeviceQueueItemsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/GetQueue',
        $argument,
        ['\Chirpstack\Api\GetDeviceQueueItemsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * GetNextFCntDown returns the next FCntDown to use for enqueing encrypted
     * downlinks. The difference with the DeviceActivation f_cont_down is that
     * this method takes potential existing queue-items into account.
     * @param \Chirpstack\Api\GetDeviceNextFCntDownRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetNextFCntDown(\Chirpstack\Api\GetDeviceNextFCntDownRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.DeviceService/GetNextFCntDown',
        $argument,
        ['\Chirpstack\Api\GetDeviceNextFCntDownResponse', 'decode'],
        $metadata, $options);
    }

}
