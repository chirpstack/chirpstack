.PHONY: requirements common gw api integration meta

PROTOC_ARGS := -I=/googleapis -I=../proto --go_out=. --go_opt=paths=source_relative --go-grpc_out=. --go-grpc_opt=paths=source_relative

all: requirements common gw api integration meta

requirements:
	go mod download
	go install google.golang.org/protobuf/cmd/protoc-gen-go
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc

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
	protoc ${PROTOC_ARGS} api/frame_log.proto
	protoc ${PROTOC_ARGS} api/multicast_group.proto
	protoc ${PROTOC_ARGS} api/request_log.proto
	protoc ${PROTOC_ARGS} api/relay.proto

integration:
	protoc ${PROTOC_ARGS} integration/integration.proto

meta:
	protoc ${PROTOC_ARGS} meta/meta.proto
