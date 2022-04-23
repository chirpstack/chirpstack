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

test-server: debug configuration/certs/ca.pem
	/target/debug/chirpstack -c ./configuration

dbshell:
	psql -h postgres -U chirpstack chirpstack

configuration/certs/ca.pem:
	cfssl gencert -initca configuration/certs/ca-csr.json | cfssljson -bare configuration/certs/ca

configuration/certs/mqtt-server.pem: configuration/certs/ca.pem
	cfssl gencert -ca configuration/certs/ca.pem -ca-key configuration/certs/ca-key.pem -config configuration/certs/ca-config.json -profile server configuration/certs/mqtt-server.json | cfssljson -bare configuration/certs/mqtt-server
