# chirpstack-api

ChirpStack gRPC API message and service wrappers for Kotlin.

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
  <artifactId>chirpstack-api-kotlin</artifactId>
  <version>1.0-SNAPSHOT</version>
</dependency>
```

#### Gradle

```gradle
dependencies {
    implementation("io.chirpstack:chirpstack-api-kotlin:1.0-SNAPSHOT")
}
```

#### Example #1: List tenants

Note: To make this example work you will also need to add kotlin coroutines as a dependency

```gradle
dependencies {
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.6.4")
}
```

```kotlin
import io.chirpstack.api.TenantServiceGrpcKt
import io.chirpstack.api.listTenantsRequest
import io.grpc.ManagedChannelBuilder
import io.grpc.Metadata
import kotlinx.coroutines.runBlocking

fun main() {
    val channel = ManagedChannelBuilder
        .forAddress("localhost", 8080)
        .usePlaintext()
        .build()
    val token = "Bearer <INSERT_TOKEN_HERE>"
    val metadata = Metadata()
    val key = Metadata.Key.of("authorization", Metadata.ASCII_STRING_MARSHALLER)
    metadata.put(key, token)
    val stub = TenantServiceGrpcKt.TenantServiceCoroutineStub(channel)
    val request = listTenantsRequest {
        limit=10
    }
    val response = runBlocking { stub.list(request, metadata) }
    println("Number of tenants: ${response.totalCount}")
}
```
