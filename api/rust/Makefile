.PHONY: proto version test

all: proto version test

proto:
	rm -rf proto
	mkdir -p proto/chirpstack
	cp -r ../proto/* proto/chirpstack
	mkdir -p proto/google/google/api
	cp  /googleapis/google/api/annotations.proto proto/google/google/api
	cp  /googleapis/google/api/http.proto proto/google/google/api

version:
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./Cargo.toml

test:
	cargo test

publish:
	cargo publish
