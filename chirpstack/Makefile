.PHONY: debug release release-deb test test-server dbshell

debug:
	cargo build

release:
	cargo build --release

release-deb:
	cargo deb

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
