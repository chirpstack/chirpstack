.PHONY: common api google-api

PROTOC_ARGS := -I=/googleapis -I=../proto --js_out=import_style=commonjs:. --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:.

all: requirements common gw api integration google-api

requirements:
	yarn install

common:
	mkdir -p common
	protoc $(PROTOC_ARGS) ../proto/common/common.proto

gw:
	mkdir -p gw
	protoc $(PROTOC_ARGS) ../proto/gw/gw.proto

api:
	mkdir -p api
	protoc $(PROTOC_ARGS) ../proto/api/internal.proto
	protoc $(PROTOC_ARGS) ../proto/api/user.proto
	protoc $(PROTOC_ARGS) ../proto/api/tenant.proto
	protoc $(PROTOC_ARGS) ../proto/api/application.proto
	protoc $(PROTOC_ARGS) ../proto/api/device_profile.proto
	protoc $(PROTOC_ARGS) ../proto/api/device_profile_template.proto
	protoc $(PROTOC_ARGS) ../proto/api/device.proto
	protoc $(PROTOC_ARGS) ../proto/api/gateway.proto
	protoc $(PROTOC_ARGS) ../proto/api/frame_log.proto
	protoc $(PROTOC_ARGS) ../proto/api/multicast_group.proto
	protoc $(PROTOC_ARGS) ../proto/api/request_log.proto
	protoc $(PROTOC_ARGS) ../proto/api/relay.proto

integration:
	mkdir -p integration
	protoc $(PROTOC_ARGS) ../proto/integration/integration.proto


google-api:
	protoc $(PROTOC_ARGS) /googleapis/google/api/*.proto
