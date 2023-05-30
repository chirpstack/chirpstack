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

Nix is used for defining the development environment which is used for local
development and for creating the final binaries.

If you do not have Nix installed and do not want to, then you could also look at
the dependencies specified in the `shell.nix` file and install these manually.
Alternatively use [Vagrant](https://www.vagrantup.com/) to setup a VM with Nix and Docker
installed. See also the provided `Vagrantfile`.

#### Docker

Docker is used by [cross-rs](https://github.com/cross-rs/cross) for cross-compiling,
 as well as some of the `make` commands you will find in the ChirpStack project.

### Starting the development shell

Run the following command to start the development shell:

```bash
nix-shell
```

### Building the UI

To build the ChirpStack UI, execute the following command:

```
make build-ui
```

Note that the ChirpStack UI is built using 

### Running ChirpStack tests

#### Start required services

ChirpStack requires several services like PostgresQL, Redis, Mosquitto, ...
to be running before you can run the tests.

Execute the following command to start these:

```bash
docker-compose up -d
```

#### Run tests

Run the following command to run the ChirpStack tests:

```bash
make test
```

### Building ChirpStack

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
