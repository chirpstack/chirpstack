# ChirpStack open-source LoRaWAN(R) Network Server

![CI](https://github.com/chirpstack/chirpstack/actions/workflows/main.yml/badge.svg?branch=master)

ChirpStack is an open-source LoRaWAN(R) Network Server which can be used to set
up LoRaWAN networks. ChirpStack provides a web-interface for the management of
gateways, devices and tenants as well to set up data integrations with the major
cloud providers, databases and services commonly used for handling device data.
ChirpStack provides a gRPC based API that can be used to integrate or extend
ChirpStack.

## Documentation and binaries

Please refer to the [ChirpStack](https://www.chirpstack.io/) website for
documentation and pre-compiled binaries.

## Building from source

### Requirements

Building ChirpStack requires:

* [Nix](https://nixos.org/download.html) (recommended) and
* [Docker](https://www.docker.com/)

#### Nix

Nix is used for setting up the development environment which is used for local
development and for creating the binaries.

If you do not have Nix installed and do not wish to install it, then you can
use the provided Docker Compose based Nix environment. To start this environment
execute the following command:

```bash
make docker-devshell
```

**Note:** You will be able to run the test commands and run `cargo build`, but
cross-compiling will not work within this environment (because it would try start
Docker within Docker).

#### Docker

Docker is used by [cross-rs](https://github.com/cross-rs/cross) for cross-compiling,
 as well as some of the `make` commands.

### Starting the development shell

Run the following command to start the development shell:

```bash
nix-shell
```

Or if you do not have Nix installed, execute the following command:

```bash
make docker-devshell
```

### Building the UI

To build the ChirpStack UI, execute the following command:

```
make build-ui
```

### Running ChirpStack tests

#### Start required services

ChirpStack requires several services like PostgresQL, Redis, Mosquitto, ...
to be running before you can run the tests. You need to start these services
manually if you started the development shell using `nix-shell`:

```bash
docker-compose up -d
```

#### Run tests

Run the following command to run the ChirpStack tests:

```bash
make test
```

### Building ChirpStack binaries

Before compiling the binaries, you need to install some additional development
tools (for cross-compiling, packaging, e.d.). Execute the following command:

```bash
make dev-dependencies
```

Run the following command within the `./chirpstack` sub-folder:

```bash
# Build AMD64 debug build (optimized for build speed)
make debug-amd64

# Build AMD64 release build (optimized for performance and binary size)
make release-amd64

# Build all packages (all targets, .deb, .rpm and .tar.gz files)
make dist
```

## License

ChirpStack Network Server is distributed under the MIT license. See also
[LICENSE](https://github.com/brocaar/chirpstack/blob/master/LICENSE).
