# chirpstack-api

ChirpStack gRPC API message and service wrappers for Java.

## Install

An artifact is not yet public on Maven Central. You can build your own jar with

```sh
make jar
```

or install the artifact in your local Maven repository `$HOME/.m2` with

```sh
make install
```

## Usage

All messages, services, constants, etc. are auto-generated from the ChirpStack protobuf definitions. The result is that
this package structure matches that of the protobuf definitions.

The protobuf definitions can be found here: https://github.com/chirpstack/chirpstack/tree/master/api/proto

### Example

#### Maven

```xml
<dependency>
  <groupId>io.chirpstack</groupId>
  <artifactId>chirpstack-api</artifactId>
  <version>1.0-SNAPSHOT</version>
</dependency>
```

#### Gradle

```gradle
dependencies {
    implementation("io.chirpstack:chirpstack-api:1.0-SNAPSHOT")
}
```

#### Example #1: List tenants

```java
import io.chirpstack.api.*;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.Metadata;
import io.grpc.stub.MetadataUtils;

public class ChirpStack {
    public static void main(String[] args) {
        ManagedChannel channel = ManagedChannelBuilder
            .forAddress("localhost", 8080)
            .usePlaintext()
            .build();
        Metadata metadata = new Metadata();
        Metadata.Key<String> key = Metadata.Key.of("authorization", Metadata.ASCII_STRING_MARSHALLER);
        String token = "Bearer <INSERT_TOKEN_HERE>";
        metadata.put(key, token);
        TenantServiceGrpc.TenantServiceBlockingStub stub = TenantServiceGrpc
            .newBlockingStub(channel)
            .withInterceptors(MetadataUtils.newAttachHeadersInterceptor(metadata));
        ListTenantsRequest request = ListTenantsRequest.newBuilder()
            .setLimit(10)
            .build();
        ListTenantsResponse response = stub.list(request);
        System.out.println("Number of tenants: " + response.getResultCount());
    }
}

```
