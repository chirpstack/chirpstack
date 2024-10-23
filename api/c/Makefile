.PHONY: all pre-build

PROTOC := protoc-c
PROTOC_ARGS := -I=proto -I=proto/chirpstack-api --c_out=./src

GOOGLE_PROTOBUF_REQUIREMENTS =
GOOGLE_PROTOBUF_REQUIREMENTS += timestamp.proto
GOOGLE_PROTOBUF_REQUIREMENTS += duration.proto
GOOGLE_PROTOBUF_REQUIREMENTS += struct.proto
GOOGLE_PROTOBUF_REQUIREMENTS += empty.proto
GOOGLE_PROTOBUF_REQUIREMENTS += descriptor.proto
GOOGLE_PROTOBUF_OUTPUT = $(patsubst %.proto, src/google/protobuf/%.pb-c.h, $(GOOGLE_PROTOBUF_REQUIREMENTS))

all: pre-build $(GOOGLE_PROTOBUF_OUTPUT) build

pre-build:
	rm -rf proto src
	mkdir -p proto/chirpstack-api
	mkdir -p proto/google/protobuf
	cp -r ../proto/* proto/chirpstack-api/
	mv proto/chirpstack-api/google/api proto/google/api
	mkdir -p src
	sed -i \
		-r 's/(\s+bytes std)(in|out|err)(\s+=.*)/\1_\2\3/' \
		proto/chirpstack-api/gw/gw.proto

src/google/protobuf/%.pb-c.h: /protobuf/src/google/protobuf/%.proto
	cp $< proto/google/protobuf/

build:
	find proto \
		-name '*.proto' \
		-type f \
		-exec $(PROTOC) $(PROTOC_ARGS) {} \;
