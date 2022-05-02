FROM alpine:3.15.0 AS ui-build

ENV PROJECT_PATH=/chirpstack

RUN apk add --no-cache make git bash build-base nodejs npm yarn

RUN mkdir -p $PROJECT_PATH
COPY ./api/grpc-web $PROJECT_PATH/api/grpc-web
COPY ./ui $PROJECT_PATH/ui

RUN cd $PROJECT_PATH/ui && \
		yarn install --network-timeout 600000 && \
		yarn build

FROM chirpstack/chirpstack-dev-cache:latest AS rust-build

COPY . $PROJECT_PATH
COPY --from=ui-build $PROJECT_PATH/ui/build $PROJECT_PATH/ui/build
RUN cd $PROJECT_PATH/chirpstack && cargo build --release

FROM debian:buster-slim as production

RUN apt-get update && \
		apt-get install -y \
		ca-certificates \
		libpq5 \
		&& rm -rf /var/lib/apt/lists/*

COPY --from=rust-build /target/release/chirpstack /usr/bin/chirpstack
COPY --from=rust-build /chirpstack/chirpstack/configuration/* /etc/chirpstack/
USER nobody:nogroup
ENTRYPOINT ["/usr/bin/chirpstack"]
