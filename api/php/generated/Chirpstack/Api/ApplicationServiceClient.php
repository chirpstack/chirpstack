<?php
// GENERATED CODE -- DO NOT EDIT!

namespace Chirpstack\Api;

/**
 * ApplicationService is the service providing API methods for managing
 * applications.
 */
class ApplicationServiceClient extends \Grpc\BaseStub {

    /**
     * @param string $hostname hostname
     * @param array $opts channel options
     * @param \Grpc\Channel $channel (optional) re-use channel object
     */
    public function __construct($hostname, $opts, $channel = null) {
        parent::__construct($hostname, $opts, $channel);
    }

    /**
     * Create creates the given application.
     * @param \Chirpstack\Api\CreateApplicationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Create(\Chirpstack\Api\CreateApplicationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/Create',
        $argument,
        ['\Chirpstack\Api\CreateApplicationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the application for the given ID.
     * @param \Chirpstack\Api\GetApplicationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Get(\Chirpstack\Api\GetApplicationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/Get',
        $argument,
        ['\Chirpstack\Api\GetApplicationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update updates the given application.
     * @param \Chirpstack\Api\UpdateApplicationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Update(\Chirpstack\Api\UpdateApplicationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/Update',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the application for the given ID.
     * @param \Chirpstack\Api\DeleteApplicationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function Delete(\Chirpstack\Api\DeleteApplicationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/Delete',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the list of applications.
     * @param \Chirpstack\Api\ListApplicationsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function List(\Chirpstack\Api\ListApplicationsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/List',
        $argument,
        ['\Chirpstack\Api\ListApplicationsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * List all configured integrations.
     * @param \Chirpstack\Api\ListIntegrationsRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function ListIntegrations(\Chirpstack\Api\ListIntegrationsRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/ListIntegrations',
        $argument,
        ['\Chirpstack\Api\ListIntegrationsResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Create HTTP integration.
     * @param \Chirpstack\Api\CreateHttpIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateHttpIntegration(\Chirpstack\Api\CreateHttpIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateHttpIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get the configured HTTP integration.
     * @param \Chirpstack\Api\GetHttpIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetHttpIntegration(\Chirpstack\Api\GetHttpIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetHttpIntegration',
        $argument,
        ['\Chirpstack\Api\GetHttpIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update the HTTP integration.
     * @param \Chirpstack\Api\UpdateHttpIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateHttpIntegration(\Chirpstack\Api\UpdateHttpIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateHttpIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete the HTTP integration.
     * @param \Chirpstack\Api\DeleteHttpIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteHttpIntegration(\Chirpstack\Api\DeleteHttpIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteHttpIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create InfluxDb integration.
     * @param \Chirpstack\Api\CreateInfluxDbIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateInfluxDbIntegration(\Chirpstack\Api\CreateInfluxDbIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateInfluxDbIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get InfluxDb integration.
     * @param \Chirpstack\Api\GetInfluxDbIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetInfluxDbIntegration(\Chirpstack\Api\GetInfluxDbIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetInfluxDbIntegration',
        $argument,
        ['\Chirpstack\Api\GetInfluxDbIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update InfluxDb integration.
     * @param \Chirpstack\Api\UpdateInfluxDbIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateInfluxDbIntegration(\Chirpstack\Api\UpdateInfluxDbIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateInfluxDbIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete InfluxDb integration.
     * @param \Chirpstack\Api\DeleteInfluxDbIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteInfluxDbIntegration(\Chirpstack\Api\DeleteInfluxDbIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteInfluxDbIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create ThingsBoard integration.
     * @param \Chirpstack\Api\CreateThingsBoardIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateThingsBoardIntegration(\Chirpstack\Api\CreateThingsBoardIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateThingsBoardIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get ThingsBoard integration.
     * @param \Chirpstack\Api\GetThingsBoardIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetThingsBoardIntegration(\Chirpstack\Api\GetThingsBoardIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetThingsBoardIntegration',
        $argument,
        ['\Chirpstack\Api\GetThingsBoardIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update ThingsBoard integration.
     * @param \Chirpstack\Api\UpdateThingsBoardIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateThingsBoardIntegration(\Chirpstack\Api\UpdateThingsBoardIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateThingsBoardIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete ThingsBoard integration.
     * @param \Chirpstack\Api\DeleteThingsBoardIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteThingsBoardIntegration(\Chirpstack\Api\DeleteThingsBoardIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteThingsBoardIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create myDevices integration.
     * @param \Chirpstack\Api\CreateMyDevicesIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateMyDevicesIntegration(\Chirpstack\Api\CreateMyDevicesIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateMyDevicesIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get myDevices integration.
     * @param \Chirpstack\Api\GetMyDevicesIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetMyDevicesIntegration(\Chirpstack\Api\GetMyDevicesIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetMyDevicesIntegration',
        $argument,
        ['\Chirpstack\Api\GetMyDevicesIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update myDevices integration.
     * @param \Chirpstack\Api\UpdateMyDevicesIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateMyDevicesIntegration(\Chirpstack\Api\UpdateMyDevicesIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateMyDevicesIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete myDevices integration.
     * @param \Chirpstack\Api\DeleteMyDevicesIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteMyDevicesIntegration(\Chirpstack\Api\DeleteMyDevicesIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteMyDevicesIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create LoRaCloud integration.
     * @param \Chirpstack\Api\CreateLoraCloudIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateLoraCloudIntegration(\Chirpstack\Api\CreateLoraCloudIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateLoraCloudIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get LoRaCloud integration.
     * @param \Chirpstack\Api\GetLoraCloudIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetLoraCloudIntegration(\Chirpstack\Api\GetLoraCloudIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetLoraCloudIntegration',
        $argument,
        ['\Chirpstack\Api\GetLoraCloudIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update LoRaCloud integration.
     * @param \Chirpstack\Api\UpdateLoraCloudIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateLoraCloudIntegration(\Chirpstack\Api\UpdateLoraCloudIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateLoraCloudIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete LoRaCloud integration.
     * @param \Chirpstack\Api\DeleteLoraCloudIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteLoraCloudIntegration(\Chirpstack\Api\DeleteLoraCloudIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteLoraCloudIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create GCP Pub/Sub integration.
     * @param \Chirpstack\Api\CreateGcpPubSubIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateGcpPubSubIntegration(\Chirpstack\Api\CreateGcpPubSubIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateGcpPubSubIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get GCP Pub/Sub integration.
     * @param \Chirpstack\Api\GetGcpPubSubIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetGcpPubSubIntegration(\Chirpstack\Api\GetGcpPubSubIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetGcpPubSubIntegration',
        $argument,
        ['\Chirpstack\Api\GetGcpPubSubIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update GCP Pub/Sub integration.
     * @param \Chirpstack\Api\UpdateGcpPubSubIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateGcpPubSubIntegration(\Chirpstack\Api\UpdateGcpPubSubIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateGcpPubSubIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete GCP Pub/Sub integration.
     * @param \Chirpstack\Api\DeleteGcpPubSubIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteGcpPubSubIntegration(\Chirpstack\Api\DeleteGcpPubSubIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteGcpPubSubIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create AWS SNS integration.
     * @param \Chirpstack\Api\CreateAwsSnsIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateAwsSnsIntegration(\Chirpstack\Api\CreateAwsSnsIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateAwsSnsIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get AWS SNS integration.
     * @param \Chirpstack\Api\GetAwsSnsIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetAwsSnsIntegration(\Chirpstack\Api\GetAwsSnsIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetAwsSnsIntegration',
        $argument,
        ['\Chirpstack\Api\GetAwsSnsIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update AWS SNS integration.
     * @param \Chirpstack\Api\UpdateAwsSnsIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateAwsSnsIntegration(\Chirpstack\Api\UpdateAwsSnsIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateAwsSnsIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete AWS SNS integration.
     * @param \Chirpstack\Api\DeleteAwsSnsIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteAwsSnsIntegration(\Chirpstack\Api\DeleteAwsSnsIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteAwsSnsIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create Azure Service-Bus integration.
     * @param \Chirpstack\Api\CreateAzureServiceBusIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateAzureServiceBusIntegration(\Chirpstack\Api\CreateAzureServiceBusIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateAzureServiceBusIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get Azure Service-Bus integration.
     * @param \Chirpstack\Api\GetAzureServiceBusIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetAzureServiceBusIntegration(\Chirpstack\Api\GetAzureServiceBusIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetAzureServiceBusIntegration',
        $argument,
        ['\Chirpstack\Api\GetAzureServiceBusIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update Azure Service-Bus integration.
     * @param \Chirpstack\Api\UpdateAzureServiceBusIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateAzureServiceBusIntegration(\Chirpstack\Api\UpdateAzureServiceBusIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateAzureServiceBusIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete Azure Service-Bus integration.
     * @param \Chirpstack\Api\DeleteAzureServiceBusIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteAzureServiceBusIntegration(\Chirpstack\Api\DeleteAzureServiceBusIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteAzureServiceBusIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create Pilot Things integration.
     * @param \Chirpstack\Api\CreatePilotThingsIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreatePilotThingsIntegration(\Chirpstack\Api\CreatePilotThingsIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreatePilotThingsIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get Pilot Things integration.
     * @param \Chirpstack\Api\GetPilotThingsIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetPilotThingsIntegration(\Chirpstack\Api\GetPilotThingsIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetPilotThingsIntegration',
        $argument,
        ['\Chirpstack\Api\GetPilotThingsIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update Pilot Things integration.
     * @param \Chirpstack\Api\UpdatePilotThingsIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdatePilotThingsIntegration(\Chirpstack\Api\UpdatePilotThingsIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdatePilotThingsIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete Pilot Things integration.
     * @param \Chirpstack\Api\DeletePilotThingsIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeletePilotThingsIntegration(\Chirpstack\Api\DeletePilotThingsIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeletePilotThingsIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Create IFTTT integration.
     * @param \Chirpstack\Api\CreateIftttIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function CreateIftttIntegration(\Chirpstack\Api\CreateIftttIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/CreateIftttIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Get IFTTT integration.
     * @param \Chirpstack\Api\GetIftttIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GetIftttIntegration(\Chirpstack\Api\GetIftttIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GetIftttIntegration',
        $argument,
        ['\Chirpstack\Api\GetIftttIntegrationResponse', 'decode'],
        $metadata, $options);
    }

    /**
     * Update IFTTT integration.
     * @param \Chirpstack\Api\UpdateIftttIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function UpdateIftttIntegration(\Chirpstack\Api\UpdateIftttIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/UpdateIftttIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Delete IFTTT integration.
     * @param \Chirpstack\Api\DeleteIftttIntegrationRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function DeleteIftttIntegration(\Chirpstack\Api\DeleteIftttIntegrationRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/DeleteIftttIntegration',
        $argument,
        ['\Google\Protobuf\GPBEmpty', 'decode'],
        $metadata, $options);
    }

    /**
     * Generates application ID specific client-certificate.
     * @param \Chirpstack\Api\GenerateMqttIntegrationClientCertificateRequest $argument input argument
     * @param array $metadata metadata
     * @param array $options call options
     * @return \Grpc\UnaryCall
     */
    public function GenerateMqttIntegrationClientCertificate(\Chirpstack\Api\GenerateMqttIntegrationClientCertificateRequest $argument,
      $metadata = [], $options = []) {
        return $this->_simpleRequest('/api.ApplicationService/GenerateMqttIntegrationClientCertificate',
        $argument,
        ['\Chirpstack\Api\GenerateMqttIntegrationClientCertificateResponse', 'decode'],
        $metadata, $options);
    }

}
