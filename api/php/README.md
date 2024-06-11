# chirpstack-api

ChirpStack gRPC API message and service wrappers for PHP.

## Install

With composer:

```sh
composer require chirpstack/chirpstack-api
```

## Usage

All messages, services, constants, etc. are auto-generated from the ChirpStack protobuf definitions. The result is that
this package structure matches that of the protobuf definitions.

The protobuf definitions can be found here: https://github.com/chirpstack/chirpstack/tree/master/api/proto

## Example

```php
<?php

namespace Test;

use Chirpstack\API\ApplicationServiceClient;
use Chirpstack\API\ListApplicationsRequest;
use Grpc\Channel;
use Grpc\ChannelCredentials;

require dirname(__FILE__) . '/vendor/autoload.php';

function main() {
    $channel = new Channel('url',  ['credentials' => ChannelCredentials::createInsecure()]);
    $client = new ApplicationServiceClient('url', [], $channel);
    $request = new ListApplicationsRequest();
    $response = $client->List($request);
    $data = $response->wait();
    print_r($data);
}

main();
```
