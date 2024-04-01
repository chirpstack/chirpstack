.PHONY: proto version test

all: proto test

proto:
	rm -rf proto
	mkdir -p proto/chirpstack
	mkdir -p proto/google
	cp -r ../proto/* proto/chirpstack
	mv proto/chirpstack/google proto/google/

test:
	cargo test

publish:
	cargo publish
