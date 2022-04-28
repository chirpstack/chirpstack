.PHONY: dist

# Builds a debug / development binary.
build-debug:
	docker-compose run --rm chirpstack make debug

# Builds a release binary.
build-release:
	docker-compose run --rm chirpstack make release

# Build distributable binaries.
dist:
	docker-compose run --rm chirpstack-build-amd64 make dist
	docker-compose run --rm chirpstack-build-arm64 make dist

# Builds the UI.
build-ui:
	docker-compose run --rm chirpstack-ui make build

# Enters the devshell for ChirpStack development.
devshell:
	docker-compose run --rm --service-ports chirpstack bash

# Enters the devshell for ChirpStack UI development.
devshell-ui:
	docker-compose run --rm --service-ports chirpstack-ui bash

# Runs the tests
test:
	docker-compose run --rm chirpstack make test
	docker-compose run --rm chirpstack make test-lrwn

# Starts the ChirpStack server (for testing only).
test-server: build-ui
	docker-compose run --rm --service-ports chirpstack make test-server

# Update the Docker development images
update-images:
	docker-compose pull chirpstack
	docker-compose build chirpstack-ui
