.PHONY: rust grpc-web go

all:
	docker-compose up

rust:
	docker-compose run --rm chirpstack-api-rust

grpc-web:
	docker-compose run --rm chirpstack-api-grpc-web

go:
	docker-compose run --rm chirpstack-api-go
