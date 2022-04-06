.PHONY: build-development build-release build-ui devshell devshell-ui test test-server update-images

# Builds a debug / development binary.
build-debug: build-ui
	docker-compose run --rm chirpstack make debug

build-release: build-ui
	docker-compose run --rm chirpstack make release

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
	docker-compose build chirpstack
	docker-compose build chirpstack-ui
