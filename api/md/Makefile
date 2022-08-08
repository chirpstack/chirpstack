.PHONY: requirements api

all: requirements api

requirements:
	go install github.com/pseudomuto/protoc-gen-doc/cmd/protoc-gen-doc@v1.5.1

api:
	protoc -I=/googleapis -I=../proto --doc_out=./api --doc_opt=markdown,api.md \
		api/application.proto \
		api/device.proto \
		api/device_profile.proto \
		api/frame_log.proto \
		api/gateway.proto \
		api/multicast_group.proto \
		api/tenant.proto \
		api/user.proto 
		
