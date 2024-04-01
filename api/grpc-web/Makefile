.PHONY: common api google-api


PROTOC_PATH := ./node_modules/grpc-tools/bin/protoc
PROTOC_ARGS := -I=../proto --js_out=import_style=commonjs:. --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:.

all: requirements common gw api integration google-api

requirements:
	yarn install

common:
	mkdir -p common
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/common/common.proto

gw:
	mkdir -p gw
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/gw/gw.proto

api:
	mkdir -p api
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/internal.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/user.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/tenant.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/application.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/device_profile.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/device_profile_template.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/device.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/gateway.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/multicast_group.proto
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/api/relay.proto

integration:
	mkdir -p integration
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/integration/integration.proto


google-api:
	$(PROTOC_PATH) $(PROTOC_ARGS) ../proto/google/api/*.proto
