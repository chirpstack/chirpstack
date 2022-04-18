FROM chirpstack/chirpstack-dev-cache:latest AS development

COPY . $PROJECT_PATH

# --network-timeout as yarn throws a ESOCKETTIMEDOUT timeout with GitHub Actions
RUN cd $PROJECT_PATH/ui && \
		yarn install --network-timeout 600000 && \
		yarn build && \
		rm -rf node_modules

RUN cd $PROJECT_PATH/chirpstack && cargo build --release

FROM debian:buster-slim as production

RUN apt-get update && \
		apt-get install -y \
		ca-certificates \
		libpq5 \
		&& rm -rf /var/lib/apt/lists/*

COPY --from=development /target/release/chirpstack /usr/bin/chirpstack
COPY --from=development /chirpstack/chirpstack/configuration/* /etc/chirpstack/
USER nobody:nogroup
ENTRYPOINT ["/usr/bin/chirpstack"]
