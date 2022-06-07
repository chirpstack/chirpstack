.PHONY: common api google-api

GOOGLEAPIS_PATH := /googleapis
PROTOC_ARGS := --js_out=import_style=commonjs:. --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:.

all: common gw api integration google-api

common:
	mkdir -p common
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/common/common.proto

gw:
	mkdir -p gw
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/gw/gw.proto

api:
	mkdir -p api
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/internal.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/user.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/tenant.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/application.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/device_profile.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/device_profile_template.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/device.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/gateway.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/frame_log.proto
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/api/multicast_group.proto

integration:
	mkdir -p integration
	protoc -I=$(GOOGLEAPIS_PATH) -I=../protobuf -I=../proto $(PROTOC_ARGS) ../proto/integration/integration.proto


google-api:
	protoc -I=$(GOOGLEAPIS_PATH) $(PROTOC_ARGS) $(GOOGLEAPIS_PATH)/google/api/*.proto
