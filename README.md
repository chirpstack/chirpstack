# ChirpStack open-source LoRaWAN Network Server

![CI](https://github.com/chirpstack/chirpstack/actions/workflows/main.yml/badge.svg?branch=master)

ChirpStack is an open-source LoRaWAN Network Server, part of the
[ChirpStack](https://www.chirpstack.io/) project.

**Note:** this repository contains the source of what is going to be
ChirpStack v4. This release merges the ChirpStack Network Server and
ChirpStack Application Server components into a single service, making
it a lot easier to setup a multi-region ChirpStack instance. This is
still work in progress.

Please refer to the forum announcement for background information:
https://forum.chirpstack.io/t/changes-coming-to-chirpstack/13101

## Testing / building from source

To build ChirpStack from source, run the following command:

```bash
make test-server
```

Note: this requires a Linux environment With Docker and Docker Compose
setup. Pre-compiled (test) binaries will be provided soon.

## License

ChirpStack Network Server is distributed under the MIT license. See also
[LICENSE](https://github.com/brocaar/chirpstack/blob/master/LICENSE).
