.PHONY: dist api

# Build distributable binaries.
dist:
	cd api && make grpc-web
	cd chirpstack && make dist

# Install dev dependencies
dev-dependencies:
	cargo install cross --version 0.2.5
	cargo install diesel_cli --version 2.2.1 --no-default-features --features postgres,sqlite
	cargo install cargo-deb --version 1.43.1
	cargo install cargo-generate-rpm --version 0.12.1

# Set the versions
version:
	test -n "$(VERSION)"
	sed -i 's/^  version.*/  version = "$(VERSION)"/g' ./chirpstack/Cargo.toml
	sed -i 's/^  version.*/  version = "$(VERSION)"/g' ./backend/Cargo.toml
	sed -i 's/^  version.*/  version = "$(VERSION)"/g' ./lrwn/Cargo.toml
	sed -i 's/^  version.*/  version = "$(VERSION)"/g' ./lrwn-filters/Cargo.toml
	sed -i 's/^  version.*/  version = "$(VERSION)"/g' ./chirpstack-integration/Cargo.toml
	sed -i 's/"version.*/"version": "$(VERSION)",/g' ./ui/package.json
	sed -i 's/"version.*/"version": "$(VERSION)",/g' ./api/grpc-web/package.json
	sed -i 's/"version.*/"version": "$(VERSION)",/g' ./api/js/package.json
	sed -i 's/version.*/version = "$(VERSION)",/g' ./api/python/src/setup.py
	sed -i 's/^  version.*/  version = "$(VERSION)"/g' ./api/rust/Cargo.toml
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./api/java/build.gradle.kts
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./api/kotlin/build.gradle.kts
	sed -i 's/"version.*/"version": "$(VERSION)",/g' ./api/php/composer.json

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
	cd api && make grpc-web
	cd ui && make build

# Enter the devshell.
devshell:
	nix-shell

# Enters the Docker devshell for ChirpStack development.
docker-devshell:
	docker compose run --rm --service-ports --name chirpstack chirpstack

# Enters the devshell for ChirpStack UI development.
docker-devshell-ui:
	docker compose run --rm --service-ports --name chirpstack-ui chirpstack-ui bash

# Runs the tests
test:
	cd api && make rust
	cd backend && cargo test
	cd chirpstack && make test
	cd lrwn && make test
	cd lrwn-filters && make test

# Runs all the tests (including some that normally are ignored)
test-all:
	cd api && make rust
	cd backend && cargo test
	cd chirpstack && make test-all
	cd chirpstack-integration && cargo test
	cd lrwn && make test
	cd lrwn-filters && make test
