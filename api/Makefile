.PHONY: rust grpc-web go js python md java kotlin csharp php

all: rust grpc-web go js python md java kotlin csharp php

rust:
	cd rust && make

grpc-web:
	cd grpc-web && make

go:
	cd go && make

js:
	cd js && make

python:
	docker compose run --rm chirpstack-api-python

md:
	docker compose run --rm chirpstack-api-md

java:
	docker compose run --rm chirpstack-api-java

kotlin:
	docker compose run --rm chirpstack-api-kotlin

csharp:
	docker compose run --rm chirpstack-csharp

php:
	docker compose run --rm chirpstack-api-php