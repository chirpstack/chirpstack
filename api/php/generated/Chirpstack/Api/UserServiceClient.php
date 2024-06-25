<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * UserService is the service providing API methods for managing users.
 */
class UserServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Create a new user.
     * @param \Chirpstack\Api\CreateUserRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\Chirpstack\Api\CreateUserRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.UserService/Create',
        $argument,
        ['\Chirpstack\Api\CreateUserResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the user for the given ID.
     * @param \Chirpstack\Api\GetUserRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Get(\Chirpstack\Api\GetUserRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.UserService/Get',
        $argument,
        ['\Chirpstack\Api\GetUserResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given user.
     * @param \Chirpstack\Api\UpdateUserRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Update(\Chirpstack\Api\UpdateUserRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.UserService/Update',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the user with the given ID.
     * @param \Chirpstack\Api\DeleteUserRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Delete(\Chirpstack\Api\DeleteUserRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.UserService/Delete',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the list of users.
     * @param \Chirpstack\Api\ListUsersRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListUsersRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.UserService/List',
        $argument,
        ['\Chirpstack\Api\ListUsersResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the password for the given user.
     * @param \Chirpstack\Api\UpdateUserPasswordRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdatePassword(\Chirpstack\Api\UpdateUserPasswordRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.UserService/UpdatePassword',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

}
