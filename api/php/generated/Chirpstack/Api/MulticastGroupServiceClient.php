<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * MulticastGroupService is the service managing multicast-groups.
 */
class MulticastGroupServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Create the given multicast group.
     * @param \Chirpstack\Api\CreateMulticastGroupRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\Chirpstack\Api\CreateMulticastGroupRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/Create',
        $argument,
        ['\Chirpstack\Api\CreateMulticastGroupResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Get returns the multicast group for the given ID.
     * @param \Chirpstack\Api\GetMulticastGroupRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Get(\Chirpstack\Api\GetMulticastGroupRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/Get',
        $argument,
        ['\Chirpstack\Api\GetMulticastGroupResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given multicast group.
     * @param \Chirpstack\Api\UpdateMulticastGroupRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Update(\Chirpstack\Api\UpdateMulticastGroupRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/Update',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the multicast-group with the given ID.
     * @param \Chirpstack\Api\DeleteMulticastGroupRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Delete(\Chirpstack\Api\DeleteMulticastGroupRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/Delete',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * List the available multicast groups.
     * @param \Chirpstack\Api\ListMulticastGroupsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListMulticastGroupsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/List',
        $argument,
        ['\Chirpstack\Api\ListMulticastGroupsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Add a device to the multicast group.
     * @param \Chirpstack\Api\AddDeviceToMulticastGroupRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddDevice(\Chirpstack\Api\AddDeviceToMulticastGroupRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/AddDevice',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Remove a device from the multicast group.
     * @param \Chirpstack\Api\RemoveDeviceFromMulticastGroupRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function RemoveDevice(\Chirpstack\Api\RemoveDeviceFromMulticastGroupRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/RemoveDevice',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Add a gateway to the multicast group.
     * @param \Chirpstack\Api\AddGatewayToMulticastGroupRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddGateway(\Chirpstack\Api\AddGatewayToMulticastGroupRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/AddGateway',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Remove a gateway from the multicast group.
     * @param \Chirpstack\Api\RemoveGatewayFromMulticastGroupRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function RemoveGateway(\Chirpstack\Api\RemoveGatewayFromMulticastGroupRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/RemoveGateway',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Add the given item to the multicast group queue.
     * @param \Chirpstack\Api\EnqueueMulticastGroupQueueItemRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Enqueue(\Chirpstack\Api\EnqueueMulticastGroupQueueItemRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/Enqueue',
        $argument,
        ['\Chirpstack\Api\EnqueueMulticastGroupQueueItemResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Flush the queue for the given multicast group.
     * @param \Chirpstack\Api\FlushMulticastGroupQueueRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function FlushQueue(\Chirpstack\Api\FlushMulticastGroupQueueRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/FlushQueue',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * List the items in the multicast group queue.
     * @param \Chirpstack\Api\ListMulticastGroupQueueRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListQueue(\Chirpstack\Api\ListMulticastGroupQueueRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.MulticastGroupService/ListQueue',
        $argument,
        ['\Chirpstack\Api\ListMulticastGroupQueueResponse', 'decode'],
        $metadata, $options);
    }

}
