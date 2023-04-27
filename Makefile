.PHONY: dist api

# Build distributable binaries.
dist:
	docker-compose run --rm --no-deps chirpstack make dist

# Set the versions
version:
	test -n "$(VERSION)"
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./chirpstack/Cargo.toml
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./backend/Cargo.toml
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./lrwn/Cargo.toml
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./lrwn-filters/Cargo.toml
	sed -i 's/"version.*/"version": "$(VERSION)",/g' ./ui/package.json
	sed -i 's/"version.*/"version": "$(VERSION)",/g' ./api/grpc-web/package.json
	sed -i 's/"version.*/"version": "$(VERSION)",/g' ./api/js/package.json
	sed -i 's/version.*/version = "$(VERSION)",/g' ./api/python/src/setup.py
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./api/rust/Cargo.toml
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./api/java/build.gradle.kts
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./api/kotlin/build.gradle.kts

	cd api && make
	make build-ui
	make test
	git add .
	git commit -v -m "Bump version to $(VERSION)"
	git tag -a v$(VERSION) -m "v$(VERSION)"
	git tag -a api/go/v$(VERSION) -m "api/go/v$(VERSION)"

api: version
	cd api && make

# Builds the UI.
build-ui:
	docker-compose run --rm --no-deps chirpstack-ui make build

# Enters the devshell for ChirpStack development.
devshell:
	docker-compose run --rm --service-ports --name chirpstack chirpstack bash

# Enters the devshell for ChirpStack UI development.
devshell-ui:
	docker-compose run --rm --service-ports --name chirpstack-ui chirpstack-ui bash

# Runs the tests
test:
	docker-compose run --rm chirpstack make test
	docker-compose run --rm chirpstack make test-lrwn
	docker-compose run --rm chirpstack make test-lrwn-filters

# Starts the ChirpStack server (for testing only).
test-server: build-ui
	docker-compose run --rm --service-ports chirpstack make test-server

# Update the Docker development images
update-images:
	docker-compose build chirpstack
	docker-compose build chirpstack-ui
