.PHONY: dist

TARGET_ARCH := $(shell rustc --print cfg |grep target_arch |sed 's/target_arch="\(.*\)"/\1/')
PKG_VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

debug-amd64:
	apt-get install -y libpq-dev:amd64
	cargo build

debug-armv7hf:
	# We can not install this by default, as only one libpq-dev version can be
	# installed at a time.
	apt-get install -y libpq-dev:armhf
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/arm-linux-gnueabihf" \
	PKG_CONFIG_SYSROOT_DIR="/usr/arm-linux-gnueabihf" \
		cargo build --target armv7-unknown-linux-gnueabihf

debug-arm64:
	apt-get install -y libpq-dev:arm64
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/aarch64-linux-gnu" \
	PKG_CONFIG_SYSROOT_DIR="/usr/aarch64-linux-gnu" \
		cargo build --target aarch64-unknown-linux-gnu

release-amd64:
	apt-get install -y libpq-dev:amd64
	cargo build --release

release-armv7hf:
	apt-get install -y libpq-dev:armhf
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/arm-linux-gnueabihf" \
	PKG_CONFIG_SYSROOT_DIR="/usr/arm-linux-gnueabihf" \
		cargo build --target armv7-unknown-linux-gnueabihf --release

release-arm64:
	apt-get install -y libpq-dev:arm64
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/aarch64-linux-gnu" \
	PKG_CONFIG_SYSROOT_DIR="/usr/aarch64-linux-gnu" \
		cargo build --target aarch64-unknown-linux-gnu --release

dist: release-deb-amd64 \
	release-deb-armv7hf \
	release-deb-arm64 \
	release-rpm-amd64 \
	release-rpm-armv7hf \
	release-rpm-arm64 \
	release-targz-amd64 \
	release-targz-armv7hf \
	release-targz-arm64 \

release-deb-amd64:
	apt-get install -y libpq-dev:amd64
	mkdir -p /chirpstack/dist
	cargo deb
	cp ../target/debian/*.deb /chirpstack/dist

release-deb-armv7hf:
	apt-get install -y libpq-dev:armhf
	mkdir -p /chirpstack/dist
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/arm-linux-gnueabihf" \
	PKG_CONFIG_SYSROOT_DIR="/usr/arm-linux-gnueabihf" \
		cargo deb --target armv7-unknown-linux-gnueabihf
	cp ../target/armv7-unknown-linux-gnueabihf/debian/*.deb /chirpstack/dist

release-deb-arm64:
	apt-get install -y libpq-dev:arm64
	mkdir -p /chirpstack/dist
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/aarch64-linux-gnu" \
	PKG_CONFIG_SYSROOT_DIR="/usr/aarch64-linux-gnu" \
		cargo deb --target aarch64-unknown-linux-gnu
	cp ../target/aarch64-unknown-linux-gnu/debian/*.deb /chirpstack/dist

release-rpm-amd64:
	apt-get install -y libpq-dev:amd64
	mkdir -p /chirpstack/dist
	cargo rpm build
	find ../target/release/rpmbuild/RPMS -type f -exec cp {} /chirpstack/dist \;

release-rpm-armv7hf:
	apt-get install -y libpq-dev:armhf
	mkdir -p /chirpstack/dist
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/arm-linux-gnueabihf" \
	PKG_CONFIG_SYSROOT_DIR="/usr/arm-linux-gnueabihf" \
		cargo rpm build --target armv7-unknown-linux-gnueabihf
	find ../target/armv7-unknown-linux-gnueabihf/release/rpmbuild/RPMS -type f -exec cp {} /chirpstack/dist \;

release-rpm-arm64:
	apt-get install -y libpq-dev:arm64
	mkdir -p /chirpstack/dist
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/usr/aarch64-linux-gnu" \
	PKG_CONFIG_SYSROOT_DIR="/usr/aarch64-linux-gnu" \
		cargo rpm build --target aarch64-unknown-linux-gnu
	find ../target/aarch64-unknown-linux-gnu/release/rpmbuild/RPMS -type f -exec cp {} /chirpstack/dist \;

release-targz-amd64: release-amd64
	mkdir -p /chirpstack/dist
	tar -czvf /chirpstack/dist/chirpstack_$(PKG_VERSION)_amd64.tar.gz -C ../target/release chirpstack

release-targz-armv7hf: release-armv7hf
	mkdir -p /chirpstack/dist
	tar -czvf /chirpstack/dist/chirpstack_$(PKG_VERSION)_armv7hf.tar.gz -C ../target/armv7-unknown-linux-gnueabihf/release chirpstack

release-targz-arm64: release-arm64
	mkdir -p /chirpstack/dist
	tar -czvf /chirpstack/dist/chirpstack_$(PKG_VERSION)_arm64.tar.gz -C ../target/aarch64-unknown-linux-gnu/release chirpstack

test:
	cargo fmt --check
	cargo clippy --no-deps
	cargo test

test-lrwn:
	cd ../lrwn && make test

test-lrwn-filters:
	cd ../lrwn-filters && make test

test-server: debug-amd64
	../target/debug/chirpstack -c ./configuration

dbshell:
	psql -h postgres -U chirpstack chirpstack
