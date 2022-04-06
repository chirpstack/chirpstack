.PHONY: debug release release-deb test test-server dbshell

debug:
	cargo build

release:
	cargo build --release

release-deb-x86-64:
	cargo deb --target x86_64-unknown-linux-gnu

release-deb-arm:
	cargo deb --target arm-unknown-linux-gnueabi

release-deb-armv7:
	cargo deb --garget armv7-unknown-linux-gnueabihf

release-deb-arm64:
	cargo deb --target aarch64-unknown-linux-gnu

test:
	cargo fmt --check
	cargo clippy --no-deps
	cargo test

test-lrwn:
	cd ../lrwn && make test

test-server: debug configuration/certs
	../target/debug/chirpstack -c ./configuration

dbshell:
	psql -h postgres -U chirpstack chirpstack

configuration/certs:
	mkdir configuration/certs
	openssl req -new -newkey rsa:4096 -days 3650 -nodes -x509 -subj "/CN=ChirpStack test CA" -keyout configuration/certs/ca.key -out configuration/certs/ca.crt
