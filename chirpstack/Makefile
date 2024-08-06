.PHONY: dist

PKG_VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')
DATABASE ?= postgres

debug-amd64:
	cross build --target x86_64-unknown-linux-musl --no-default-features --features="$(DATABASE)"

release-amd64:
	cross build --target x86_64-unknown-linux-musl --release --no-default-features --features="$(DATABASE)"

dist:
	# Keep these in this order, as aarch64 is based on Debian Buster (older),
	# the others on Bullseye. For some build scripts we want to build against
	# least recent LIBC.
	cross build --target aarch64-unknown-linux-musl --release --no-default-features --features="$(DATABASE)"
	cross build --target x86_64-unknown-linux-musl --release --no-default-features --features="$(DATABASE)"
	cross build --target armv7-unknown-linux-musleabihf --release --no-default-features --features="$(DATABASE)"

	cargo deb --target x86_64-unknown-linux-musl --no-build --no-strip
	cargo deb --target armv7-unknown-linux-musleabihf --no-build --no-strip
	cargo deb --target aarch64-unknown-linux-musl --no-build --no-strip

	cargo generate-rpm --target x86_64-unknown-linux-musl --target-dir ../target
	cargo generate-rpm --target armv7-unknown-linux-musleabihf --target-dir ../target
	cargo generate-rpm --target aarch64-unknown-linux-musl --target-dir ../target

	mkdir -p ../dist

	cp ../target/x86_64-unknown-linux-musl/debian/*.deb ../dist
	cp ../target/armv7-unknown-linux-musleabihf/debian/*.deb ../dist
	cp ../target/aarch64-unknown-linux-musl/debian/*.deb ../dist

	cp ../target/x86_64-unknown-linux-musl/generate-rpm/*.rpm ../dist
	cp ../target/armv7-unknown-linux-musleabihf/generate-rpm/*.rpm ../dist
	cp ../target/aarch64-unknown-linux-musl/generate-rpm/*.rpm ../dist

	tar -czvf ../dist/chirpstack_$(PKG_VERSION)_amd64.tar.gz -C ../target/x86_64-unknown-linux-musl/release chirpstack
	tar -czvf ../dist/chirpstack_$(PKG_VERSION)_armv7hf.tar.gz -C ../target/armv7-unknown-linux-musleabihf/release chirpstack
	tar -czvf ../dist/chirpstack_$(PKG_VERSION)_arm64.tar.gz -C ../target/aarch64-unknown-linux-musl/release chirpstack

test:
	cargo fmt --check
	cargo clippy --no-deps --no-default-features --features="$(DATABASE)"
	TZ=UTC cargo test --no-default-features --features="$(DATABASE)"

test-all:
	cargo fmt --check
	cargo clippy --no-deps --no-default-features --features="$(DATABASE)"
	TZ=UTC cargo test --no-default-features --features="$(DATABASE),test-all-integrations"

migration-generate:
ifeq ($(NAME),)
	@echo "You must provide a NAME parameter, e.g. make migration-generate NAME=test-migration"
else
	diesel --config-file diesel_$(DATABASE).toml migration --migration-dir migrations_$(DATABASE) generate $(NAME)
endif

migration-run: chirpstack_test.sqlite
ifeq ($(DATABASE),postgres)
	diesel --config-file diesel_postgres.toml migration --migration-dir migrations_postgres run
endif
ifeq ($(DATABASE),sqlite)
	DATABASE_URL="chirpstack_test.sqlite" diesel --config-file diesel_sqlite.toml migration --migration-dir migrations_sqlite run
	sed -i 's/Timestamp/TimestamptzSqlite/g' src/storage/schema_sqlite.rs
endif

migration-revert: chirpstack_test.sqlite
ifeq ($(DATABASE),postgres)
	diesel --config-file diesel_postgres.toml migration --migration-dir migrations_postgres revert
endif
ifeq ($(DATABASE),sqlite)
	DATABASE_URL="chirpstack_test.sqlite" diesel --config-file diesel_sqlite.toml migration --migration-dir migrations_sqlite revert
	sed -i 's/Timestamp/TimestamptzSqlite/g' src/storage/schema_sqlite.rs
endif

chirpstack_test.sqlite:
	DATABASE_URL=chirpstack_test.sqlite diesel --config-file diesel_sqlite.toml setup --migration-dir migrations_sqlite
