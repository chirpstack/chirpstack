# UI build stage
FROM --platform=$BUILDPLATFORM alpine:3.17.3 AS ui-build

ENV PROJECT_PATH=/chirpstack

RUN apk add --no-cache make git bash build-base nodejs npm yarn

RUN mkdir -p $PROJECT_PATH
COPY ./api/grpc-web $PROJECT_PATH/api/grpc-web
COPY ./ui $PROJECT_PATH/ui

RUN cd $PROJECT_PATH/ui && \
	yarn install --network-timeout 600000 && \
	yarn build


# ChirpStack build stage
FROM --platform=$BUILDPLATFORM rust:1.68.2-buster AS rust-build

ENV PROJECT_PATH=/chirpstack
RUN mkdir -p $PROJECT_PATH

RUN dpkg --add-architecture armhf
RUN dpkg --add-architecture arm64

RUN apt-get update && \
	apt-get install -y \
	make \
	cmake \
	git \
	libpq-dev \
	clang \
	libclang-dev \
	jq \
	protobuf-compiler \
	gcc-arm-linux-gnueabihf \
	g++-arm-linux-gnueabihf \
	gcc-aarch64-linux-gnu \
	g++-aarch64-linux-gnu \
	zlib1g-dev:armhf \
	zlib1g-dev:arm64

RUN rustup target add armv7-unknown-linux-gnueabihf
RUN rustup target add aarch64-unknown-linux-gnu

ARG TARGETPLATFORM
RUN mkdir -p /release/$TARGETPLATFORM

COPY . $PROJECT_PATH
COPY --from=ui-build $PROJECT_PATH/ui/build $PROJECT_PATH/ui/build

RUN case "$TARGETPLATFORM" in \
	"linux/amd64") \
		cd $PROJECT_PATH/chirpstack && make release-amd64; \
		cp $PROJECT_PATH/target/release/chirpstack /release/$TARGETPLATFORM; \
		;; \
	"linux/arm/v7") \
		cd $PROJECT_PATH/chirpstack && make release-armv7hf; \
		cp $PROJECT_PATH/target/armv7-unknown-linux-gnueabihf/release/chirpstack /release/$TARGETPLATFORM; \
		;; \
	"linux/arm64") \
		cd $PROJECT_PATH/chirpstack && make release-arm64; \
		cp $PROJECT_PATH/target/aarch64-unknown-linux-gnu/release/chirpstack /release/$TARGETPLATFORM; \
		;; \
	esac;


# Final stage
FROM debian:buster-slim as production

RUN apt-get update && \
		apt-get install -y \
		ca-certificates \
		libpq5 \
		&& rm -rf /var/lib/apt/lists/*

ARG TARGETPLATFORM

COPY --from=rust-build /release/$TARGETPLATFORM/chirpstack /usr/bin/chirpstack
COPY --from=rust-build /chirpstack/chirpstack/configuration/* /etc/chirpstack/
USER nobody:nogroup
ENTRYPOINT ["/usr/bin/chirpstack"]
