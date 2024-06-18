.PHONY: dist

PKG_VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

debug-amd64:
	cross build --target x86_64-unknown-linux-musl

release-amd64:
	cross build --target x86_64-unknown-linux-musl --release

dist:
	# Keep these in this order, as aarch64 is based on Debian Buster (older),
	# the others on Bullseye. For some build scripts we want to build against
	# least recent LIBC.
	cross build --target x86_64-unknown-linux-musl --release


	cargo deb --target x86_64-unknown-linux-musl --no-build --no-strip


	cargo generate-rpm --target x86_64-unknown-linux-musl --target-dir ../target


	mkdir -p ../dist

	cp ../target/x86_64-unknown-linux-musl/debian/*.deb ../dist


	cp ../target/x86_64-unknown-linux-musl/generate-rpm/*.rpm ../dist


	tar -czvf ../dist/chirpstack_$(PKG_VERSION)_amd64.tar.gz -C ../target/x86_64-unknown-linux-musl/release chirpstack


test:
	cargo fmt --check
	cargo clippy --no-deps
	TZ=UTC cargo test

test-all:
	cargo fmt --check
	cargo clippy --no-deps
	TZ=UTC cargo test --features test-all-integrations