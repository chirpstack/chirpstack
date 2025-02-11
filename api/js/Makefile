.PHONY: requirements common gw api integration stream

PROTOC_PATH := ./node_modules/grpc-tools/bin/protoc
PROTOC_GEN_TS_PATH := ./node_modules/.bin/protoc-gen-ts
PROTOC_GEN_GRPC_PATH := ./node_modules/.bin/grpc_tools_node_protoc_plugin
PROTOC_ARGS := -I=../proto --plugin=protoc-gen-ts=$(PROTOC_GEN_TS_PATH) --plugin=protoc-gen-grpc=$(PROTOC_GEN_GRPC_PATH) --js_out=import_style=commonjs,binary:. --ts_out=service=grpc-node,mode=grpc-js:.
PROTOC_GRPC_ARGS := $(PROTOC_ARGS) --grpc_out=grpc_js:.

all: requirements common gw api integration stream google-api

requirements:
	yarn install

common:
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/common/common.proto

gw:
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/gw/gw.proto

api:
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/internal.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/user.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/tenant.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/application.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/device_profile.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/device_profile_template.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/device.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/gateway.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/multicast_group.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/relay.proto
	$(PROTOC_PATH) ${PROTOC_GRPC_ARGS} ../proto/api/fuota.proto

integration:
	$(PROTOC_PATH) ${PROTOC_ARGS} ../proto/integration/integration.proto

stream:
	$(PROTOC_PATH) ${PROTOC_ARGS} ../proto/stream/meta.proto
	$(PROTOC_PATH) ${PROTOC_ARGS} ../proto/stream/frame.proto
	$(PROTOC_PATH) ${PROTOC_ARGS} ../proto/stream/api_request.proto
	$(PROTOC_PATH) ${PROTOC_ARGS} ../proto/stream/backend_interfaces.proto

google-api:
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/google/api/*.proto
