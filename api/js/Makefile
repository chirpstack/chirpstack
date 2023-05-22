.PHONY: requirements common gw api integration meta

PROTOC_PATH := ./node_modules/grpc-tools/bin/protoc
PROTOC_GEN_TS_PATH := ./node_modules/.bin/protoc-gen-ts
PROTOC_GEN_GRPC_PATH := ./node_modules/.bin/grpc_tools_node_protoc_plugin
PROTOC_ARGS := -I=/googleapis -I=../proto --plugin=protoc-gen-ts=$(PROTOC_GEN_TS_PATH) --plugin=protoc-gen-grpc=$(PROTOC_GEN_GRPC_PATH) --js_out=import_style=commonjs,binary:. --ts_out=service=grpc-node,mode=grpc-js:.
PROTOC_GRPC_ARGS := $(PROTOC_ARGS) --grpc_out=grpc_js:.

all: requirements common gw api integration meta google-api

requirements:
	yarn install

common:
	protoc $(PROTOC_ARGS) ../proto/common/common.proto

gw:
	protoc $(PROTOC_ARGS) ../proto/gw/gw.proto

api:
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/internal.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/user.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/tenant.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/application.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/device_profile.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/device_profile_template.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/device.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/gateway.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/frame_log.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/multicast_group.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/request_log.proto
	protoc ${PROTOC_GRPC_ARGS} ../proto/api/relay.proto

integration:
	protoc ${PROTOC_ARGS} ../proto/integration/integration.proto

meta:
	protoc ${PROTOC_ARGS} ../proto/meta/meta.proto

google-api:
	protoc $(PROTOC_ARGS) /googleapis/google/api/*.proto
