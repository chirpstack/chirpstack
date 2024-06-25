<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * TenantService is the service providing API methods for managing tenants.
 */
class TenantServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Create a new tenant.
     * @param \Chirpstack\Api\CreateTenantRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\Chirpstack\Api\CreateTenantRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/Create',
        $argument,
        ['\Chirpstack\Api\CreateTenantResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the tenant for the given ID.
     * @param \Chirpstack\Api\GetTenantRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Get(\Chirpstack\Api\GetTenantRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/Get',
        $argument,
        ['\Chirpstack\Api\GetTenantResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given tenant.
     * @param \Chirpstack\Api\UpdateTenantRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Update(\Chirpstack\Api\UpdateTenantRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/Update',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the tenant with the given ID.
     * @param \Chirpstack\Api\DeleteTenantRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Delete(\Chirpstack\Api\DeleteTenantRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/Delete',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the list of tenants.
     * @param \Chirpstack\Api\ListTenantsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListTenantsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/List',
        $argument,
        ['\Chirpstack\Api\ListTenantsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Add an user to the tenant.
     * Note: the user must already exist.
     * @param \Chirpstack\Api\AddTenantUserRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function AddUser(\Chirpstack\Api\AddTenantUserRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/AddUser',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the the tenant user for the given tenant and user IDs.
     * @param \Chirpstack\Api\GetTenantUserRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetUser(\Chirpstack\Api\GetTenantUserRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/GetUser',
        $argument,
        ['\Chirpstack\Api\GetTenantUserResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the given tenant user.
     * @param \Chirpstack\Api\UpdateTenantUserRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateUser(\Chirpstack\Api\UpdateTenantUserRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/UpdateUser',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the given tenant user.
     * @param \Chirpstack\Api\DeleteTenantUserRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteUser(\Chirpstack\Api\DeleteTenantUserRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/DeleteUser',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the list of tenant users.
     * @param \Chirpstack\Api\ListTenantUsersRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListUsers(\Chirpstack\Api\ListTenantUsersRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.TenantService/ListUsers',
        $argument,
        ['\Chirpstack\Api\ListTenantUsersResponse', 'decode'],
        $metadata, $options);
    }

}
