FROM rust:1.59.0-alpine as development

ENV PROJECT_PATH=/chirpstack

# See: https://github.com/diesel-rs/diesel/issues/700
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk --no-cache add \
	make cmake build-base clang perl protobuf libpq-dev \
	nodejs npm yarn

RUN rustup component add rustfmt

RUN mkdir -p $PROJECT_PATH
COPY . $PROJECT_PATH

RUN cd $PROJECT_PATH/ui && yarn install && yarn build
RUN cd $PROJECT_PATH/chirpstack && cargo build --release

FROM alpine:3.15.0 as production

run apk --no-cache add ca-certificates tzdata libpq
COPY --from=development /chirpstack/target/release/chirpstack /usr/bin/chirpstack
COPY --from=development /chirpstack/chirpstack/configuration/* /etc/chirpstack/
USER nobody:nogroup
ENTRYPOINT ["/usr/bin/chirpstack"]
