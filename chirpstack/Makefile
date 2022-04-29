.PHONY: debug release release-deb test test-server dbshell dist

TARGET_ARCH := $(shell rustc --print cfg |grep target_arch |sed 's/target_arch="\(.*\)"/\1/')
PKG_VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

debug:
	cargo build

release:
	cargo build --release

dist: release-deb release-rpm release-targz

release-deb:
	mkdir -p /chirpstack/dist
	cargo deb
	cp /target/debian/*.deb /chirpstack/dist

release-rpm:
	mkdir -p /chirpstack/dist
	cargo rpm build
	find /target/release/rpmbuild/RPMS -type f -exec cp {} /chirpstack/dist \;

release-targz:
	mkdir -p /chirpstack/dist
	tar -czvf /chirpstack/dist/chirpstack_$(PKG_VERSION)_$(TARGET_ARCH).tar.gz -C /target/release chirpstack

test:
	cargo fmt --check
	cargo clippy --no-deps
	cargo test

test-lrwn:
	cd ../lrwn && make test

test-server: debug configuration/certs/ca.pem
	/target/debug/chirpstack -c ./configuration

dbshell:
	psql -h postgres -U chirpstack chirpstack

configuration/certs/ca.pem:
	cfssl gencert -initca configuration/certs/ca-csr.json | cfssljson -bare configuration/certs/ca

configuration/certs/mqtt-server.pem: configuration/certs/ca.pem
	cfssl gencert -ca configuration/certs/ca.pem -ca-key configuration/certs/ca-key.pem -config configuration/certs/ca-config.json -profile server configuration/certs/mqtt-server.json | cfssljson -bare configuration/certs/mqtt-server
