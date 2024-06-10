.PHONY: requirements common gw api integration stream

PROTOC_ARGS := -I=../proto --grpc_out=./generated --plugin=protoc-gen-grpc=/opt/homebrew/bin/grpc_php_plugin --php_out=./generated

all: requirements common gw api integration stream

requirements:
	mkdir generated

common:
	protoc ${PROTOC_ARGS} common/common.proto

gw:
	protoc ${PROTOC_ARGS} gw/gw.proto

api:
	protoc ${PROTOC_ARGS} api/internal.proto
	protoc ${PROTOC_ARGS} api/user.proto
	protoc ${PROTOC_ARGS} api/tenant.proto
	protoc ${PROTOC_ARGS} api/application.proto
	protoc ${PROTOC_ARGS} api/device_profile.proto
	protoc ${PROTOC_ARGS} api/device_profile_template.proto
	protoc ${PROTOC_ARGS} api/device.proto
	protoc ${PROTOC_ARGS} api/gateway.proto
	protoc ${PROTOC_ARGS} api/multicast_group.proto
	protoc ${PROTOC_ARGS} api/relay.proto

integration:
	protoc ${PROTOC_ARGS} integration/integration.proto

stream:
	protoc ${PROTOC_ARGS} stream/meta.proto
	protoc ${PROTOC_ARGS} stream/frame.proto
	protoc ${PROTOC_ARGS} stream/api_request.proto
	protoc ${PROTOC_ARGS} stream/backend_interfaces.proto