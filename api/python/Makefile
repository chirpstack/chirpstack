.PHONY: requirements common gw api integration stream

PROTOC := python -m grpc_tools.protoc
PROTOC_ARGS := -I=/googleapis -I=proto --python_out=src --grpc_python_out=src --pyi_out=src
PACKAGE_NAME := import \"chirpstack-api/

all: requirements pre-build common gw api integration stream

requirements:
	pip install grpcio-tools

# See: https://github.com/protocolbuffers/protobuf/issues/7061
pre-build:
	rm -rf proto
	mkdir -p proto/chirpstack-api
	cp -r ../proto/* proto/chirpstack-api/
	sed -i 's@^import "common/@$(PACKAGE_NAME)common/@g' `find proto/chirpstack-api -type f -name "*.proto*"`
	sed -i 's@^import "gw/@$(PACKAGE_NAME)gw/@g' `find proto/chirpstack-api -type f -name "*.proto*"`
	sed -i 's@^import "api/@$(PACKAGE_NAME)api/@g' `find proto/chirpstack-api -type f -name "*.proto*"`
	mkdir -p src

common:
	$(PROTOC) $(PROTOC_ARGS) chirpstack-api/common/common.proto

gw:
	$(PROTOC) $(PROTOC_ARGS) chirpstack-api/gw/gw.proto


api:
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/internal.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/user.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/tenant.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/application.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/device_profile.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/device_profile_template.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/device.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/gateway.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/multicast_group.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/api/relay.proto

integration:
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/integration/integration.proto

stream:
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/stream/meta.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/stream/frame.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/stream/api_request.proto
	$(PROTOC) ${PROTOC_ARGS} chirpstack-api/stream/backend_interfaces.proto
