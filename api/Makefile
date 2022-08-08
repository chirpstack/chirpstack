.PHONY: rust grpc-web go js python md

all:
	docker-compose up

rust:
	docker-compose run --rm chirpstack-api-rust

grpc-web:
	docker-compose run --rm chirpstack-api-grpc-web

go:
	docker-compose run --rm chirpstack-api-go

js:
	docker-compose run --rm chirpstack-api-js

python:
	docker-compose run --rm chirpstack-api-python

md:
	docker-compose run --rm chirpstack-api-md
