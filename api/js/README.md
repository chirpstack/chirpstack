# chirpstack-api

ChirpStack gRPC API message and service wrappers for Javascript. Typescript definitions are included.

## Install

With npm:

```sh
npm install @chirpstack/chirpstack-api --save
```

Or with yarn:

```sh
yarn add @chirpstack/chirpstack-api
```

## Usage

All messages, services, constants, etc. are auto-generated from the ChirpStack protobuf definitions. The result is that
this package structure matches that of the protobuf definitions. There is no ES6 index gathering all of the exports, so
full import/require paths should be used. The generated code is all callback based, but can be promisified.

The protobuf definitions can be found here: https://github.com/brocaar/chirpstack-api/tree/master/protobuf

The generated code all depends on the `grpc` package, and for most use cases you will probably need to make use of the
`grpc` package directly as well. This is seen in the example below.

### Example

This example shows how to log in to ChirpStack via the gRPC API and then create a gRPC metadata object containing the
JWT. This metadata could then be passed to any future requests that require authorization.

```javascript
import * as grpc from '@grpc/grpc-js';

import * as internalService from '@chirpstack/chirpstack-api/as/external/api/internal_grpc_pb';
import * as internalMessages from '@chirpstack/chirpstack-api/as/external/api/internal_pb';

// Create the client for the 'internal' service
const internalServiceClient = new internalService.InternalServiceClient(
    'localhost:8080',
    grpc.credentials.createInsecure()
);

// Create and build the login request message
const loginRequest = new internalMessages.LoginRequest();

loginRequest.setEmail('email');
loginRequest.setPassword('password');

// Send the login request
internalServiceClient.login(loginRequest, (error, response) => {
    // Build a gRPC metadata object, setting the authorization key to the JWT we
    // got back from logging in.
    const metadata = new grpc.Metadata();
    metadata.set('authorization', response.getJwt());

    // This metadata can now be passed for requests to APIs that require authorization
    // e.g.
    // deviceServiceClient.create(createDeviceRequest, metadata, callback);
});
```

