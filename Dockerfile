# Copy binary stage
FROM --platform=$BUILDPLATFORM alpine:3.18.0 as binary

ARG TARGETPLATFORM

COPY target/x86_64-unknown-linux-musl/release/chirpstack /usr/bin/chirpstack-x86_64
COPY target/armv7-unknown-linux-musleabihf/release/chirpstack /usr/bin/chirpstack-armv7hf
COPY target/aarch64-unknown-linux-musl/release/chirpstack /usr/bin/chirpstack-aarch64

RUN case "$TARGETPLATFORM" in \
	"linux/amd64") \
		cp /usr/bin/chirpstack-x86_64 /usr/bin/chirpstack; \
		;; \
	"linux/arm/v7") \
		cp /usr/bin/chirpstack-armv7hf /usr/bin/chirpstack; \
		;; \
	"linux/arm64") \
		cp /usr/bin/chirpstack-aarch64 /usr/bin/chirpstack; \
		;; \
	esac;

# Final stage
FROM alpine:3.18.0

RUN apk --no-cache add \
    ca-certificates

COPY --from=binary /usr/bin/chirpstack /usr/bin/chirpstack
USER nobody:nogroup
ENTRYPOINT ["/usr/bin/chirpstack"]