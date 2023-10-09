use handlebars::{no_escape, Handlebars};

use super::super::config;

pub fn run() {
    let template = r#"
# Logging configuration
[logging]

  # Log level.
  #
  # Valid options are:
  #   * TRACE
  #   * DEBUG
  #   * INFO
  #   * WARN
  #   * ERROR
  #   * OFF
  level="{{ logging.level }}"

  # Log as JSON.
  json={{ logging.json }}

# PostgreSQL configuration.
[postgresql]

  # PostgreSQL DSN.
  #
  # Format example: postgres://<USERNAME>:<PASSWORD>@<HOSTNAME>/<DATABASE>?sslmode=<SSLMODE>.
  #
  # SSL mode options:
  #  * disable - no SSL
  #  * require - Always SSL (skip verification)
  #  * verify-ca - Always SSL (verify that the certificate presented by the server was signed by a trusted CA)
  #  * verify-full - Always SSL (verify that the certification presented by the server was signed by a trusted CA and the server host name matches the one in the certificate)
  dsn="{{ postgresql.dsn }}"

  # Max open connections.
  #
  # This sets the max. number of open connections that are allowed in the
  # PostgreSQL connection pool.
  max_open_connections={{ postgresql.max_open_connections }}

  # Min idle connections.
  #
  # This sets the min. number of idle connections in the PostgreSQL connection
  # pool (0 = equal to max_open_connections).
  min_idle_connections={{ postgresql.min_idle_connections }}


# Redis configuration.
[redis]

  # Server address or addresses.
  #
  # Use rediss:// in case of a TLS secured connection.
  #
  # Example formats:
  #   redis://127.0.0.1:6379
  #   rediss://127.0.0.1:6379
  #   redis://:password@127.0.0.1:6379
  #   redis://username:password@127.0.0.1:6379
  #
  # Set multiple addresses when connecting to a cluster.
  servers=[
    {{#each redis.servers}}
    "{{this}}",
    {{/each}}
  ]

  # Redis Cluster.
  #
  # Set this to true when the provided URLs are pointing to a Redis Cluster
  # instance.
  cluster={{ redis.cluster }}

  # Key prefix.
  #
  # A key prefix can be used to avoid key collisions when multiple deployments
  # are using the same Redis database and it is not possible to separate
  # keys by database index (e.g. when using Redis Cluster, which does not
  # support multiple databases).
  key_prefix="{{ redis.key_prefix }}"

  # Max open connections.
  #
  # This sets the max. number of open connections that are allowed in the
  # Redis connection pool.
  max_open_connections={{ redis.max_open_connections }}

  # Min idle connections.
  #
  # This sets the min. number of idle connections in the Redis connection
  # pool (0 = equal to max_open_connections).
  min_idle_connections={{ redis.min_idle_connections }}


# API interface configuration.
[api]

  # interface:port to bind the API interface to.
  bind="{{ api.bind }}"

  # Secret.
  #
  # This secret is used for generating login and API tokens, make sure this
  # is never exposed. Changing this secret will invalidate all login and API
  # tokens. The following command can be used to generate a random secret:
  #   openssl rand -base64 32
  secret="{{ api.secret }}"


# Global gateway configuration.
# Please note that backend configuration can be found in the per-region
# configuration.
[gateway]

  # CA certificate and key file (optional).
  #
  # If setting the CA certificate and key file options, ChirpStack 
  # will generate client certificates which can be used by the gateway for
  # authentication and authorization. The Common Name of the certificate will
  # be set to the Gateway ID.
  ca_key="{{ gateway.ca_key }}"
  ca_cert="{{ gateway.ca_cert }}"

  # Certificate lifetime.
  #
  # This defines how long (after generating) the certificate remains valid.
  client_cert_lifetime="{{ gateway.client_cert_lifetime }}"

  # Allow unknown gateways.
  #
  # If set to true, then uplinks received from gateways not configured in
  # ChirpStack will be allowed.
  allow_unknown_gateways={{ gateway.allow_unknown_gateways }}


# Network related configuration.
[network]

  # Network identifier (NetID, 3 bytes) encoded as HEX (e.g. 010203).
  net_id="{{ network.net_id }}"

  # Secondary NetIDs.
  #
  # Additional NetIDs. At this moment, the additional NetIDs are only used to
  # validate if an uplink belongs to the ChirpStack instance or if it is a
  # roaming device (if roaming is enabled).
  # If you would like to assign DevAddrs from multiple NetIDs, you must specify
  # these in the dev_addr_prefixes configuration.
  secondary_net_ids=[
    {{#each network.secondary_net_ids}}
    "{{this}}",
    {{/each}}
  ]

  # DevAddr prefix(es).
  #
  # This makes it possible to configure one or multiple DevAddr (sub)ranges
  # If left blank, then the complete DevAddr space provided by the configured
  # net_id value will be used. If multiple prefixes are configured, a random
  # prefix will be chosen when generating a DevAddr.
  #
  # Example configuration:
  # dev_addr_prefixes=["0000ff00/24"]
  #
  # This example configures the DevAddr range to 0000ff00 - 0000ffff.
  # The /24 means that the 24MSB of the prefix will be used, meaning that the
  # 8LSB will be used as address space.
  dev_addr_prefixes=[
    {{#each network.dev_addr_prefixes}}
    "{{this}}",
    {{/each}}
  ]

  # Enabled regions.
  #
  # Multiple regions can be enabled simultaneously. Each region must match
  # the 'name' parameter of the region configuration in '[[regions]]'.
  enabled_regions=[
    {{#each network.enabled_regions}}
    "{{this}}",
    {{/each}}
  ]

  # Device session expiration.
  #
  # The TTL value defines the time after which a device-session expires
  # after no activity.
  device_session_ttl="{{ network.device_session_ttl }}"

  # Time to wait for uplink de-duplication.
  #
  # This is the time that ChirpStack will wait for other gateways to receive
  # the same uplink frame. Please note that this value affects the
  # roundtrip time. The total roundtrip time (which includes network latency)
  # must be less than the (first) receive-window.
  deduplication_delay="{{ network.deduplication_delay }}"

  # Get downlink data delay.
  #
  # This is the time that ChirpStack waits between forwarding data to the
  # integration(s) and reading data from the queue. A higher value means that
  # an end-application has more time to schedule a downlink queue item which
  # can be processed within the same uplink / downlink transaction.
  # Please note that this value has influence on the uplink / downlink
  # roundtrip time. Setting this value too high means ChirpStack will be
  # unable to respond to the device within its receive-window.
  get_downlink_data_delay="{{ network.get_downlink_data_delay }}"

  # Mac-commands disabled.
  mac_commands_disabled={{ network.mac_commands_disabled }}

  # Custom ADR plugins.
  #
  # The custom ADR plugin must be implemented in JavaScript. For an example
  # skeleton, please see:
  # https://github.com/chirpstack/chirpstack/blob/master/examples/adr_plugins/plugin_skeleton.js
  adr_plugins=[
    {{#each network.adr_plugins}}
    "{{this}}",
    {{/each}}
  ]


  # Scheduler settings.
  [network.scheduler]

    # Scheduler interval.
    #
    # The interval in which the downlink scheduler for multicast, Class-B and
    # Class-C runs.
    interval="{{ network.scheduler.interval }}"

    # Class-A lock duration.
    #
    # This defines the lock duration between receiving a Class-A uplink and
    # the next scheduler-run for a device. The purpose of this lock is to
    # avoid collisions between Class-A and Class-B/C downlinks.
    class_a_lock_duration="{{ network.scheduler.class_a_lock_duration }}"

    # Class-C lock duration.
    #
    # This defines the lock duration between scheduling two Class-C downlink
    # payloads for the same device. The purpose of this lock is to avoid
    # overlap between scheduling Class-C downlinks and / or spreading the 
    # downlink capacity load on the gateway.
    class_c_lock_duration="{{ network.scheduler.class_c_lock_duration }}"

    # Multicast Class-C margin.
    #
    # This defines the minimum margin between scheduling multiple multicast downlinks
    # (within the same multicast-group). This value must be equal or greater than the
    # scheduler interval.
    multicast_class_c_margin="{{ network.scheduler.multicast_class_c_margin }}"

    # Multicast Class-B margin.
    #
    # This defines the minimum margin between scheduling multiple multicast downlinks
    # (within the same multicast-group). This value must be equal or greater than the
    # scheduler interval.
    multicast_class_b_margin="{{ network.scheduler.multicast_class_b_margin }}"


# Monitoring related configuration.
[monitoring]

  # interface:port to bind the monitoring endpoint to (optional).
  #
  # /health  - Returns 200 in case the healthchecks have passed.
  # /metrics - Returns metrics which can be scraped by Prometheus.
  #
  # If not set, this endpoint will be disabled.
  bind="{{ monitoring.bind }}"
  

  # Meta-log max history.
  #
  # This defines the max number of meta records that will be persisted in Redis Streams.
  # Setting this value to 0 disables this feature.
  meta_log_max_history={{ monitoring.meta_log_max_history }}

  # Gateway frame-log max history.
  #
  # This defines the max number of frame-log records that will be persisted in Redis Streams.
  # This stream contains the uplink and downlink frames of all gateways.
  # Setting this value to 0 disables this feature.
  gateway_frame_log_max_history={{ monitoring.gateway_frame_log_max_history }}

  # Device frame-log max history.
  #
  # This defines the max number of frame-log records that will be persisted in Redis Streams.
  # This stream contains the uplink and downlink frames of all devices.
  # Setting this value to 0 disables this feature.
  device_frame_log_max_history={{ monitoring.device_frame_log_max_history }}

  # Device event-log max history.
  #
  # This defines the max number of event-log records that will be persisted in Redis Streams.
  # This stream contains the events of all devices.
  # Setting this value to 0 disables this feature.
  device_event_log_max_history={{ monitoring.device_event_log_max_history }}

  # Per gateway frame-log max history.
  #
  # Equal to the gateway_frame_log_max_history, but for each gateway a new Redis Stream
  # is created.
  # Setting this value to 0 disables this feature.
  per_gateway_frame_log_max_history={{ monitoring.per_gateway_frame_log_max_history }}

  # Per gateway frame-log TTL.
  #
  # This defines the TTL of the Redis Stream key.
  per_gateway_frame_log_ttl="{{ monitoring.per_gateway_frame_log_ttl }}"

  # Per device frame-log max history.
  #
  # Equal to the device_frame_log_max_history, but for each device a new Redis Stream
  # is created.
  # Setting this value to 0 disables this feature.
  per_device_frame_log_max_history={{ monitoring.per_device_frame_log_max_history }}

  # Per device frame-log TTL.
  #
  # This defines the TTL of the Redis Stream key.
  per_device_frame_log_ttl="{{ monitoring.per_device_frame_log_ttl }}"

  # Per device event-log max history.
  #
  # Equal to the device_event_log_max_history, but for each device a new Redis Stream
  # is created.
  # Setting this value to 0 disables this feature.
  per_device_event_log_max_history={{ monitoring.per_device_event_log_max_history }}

  # Per device event-log TTL.
  #
  # This defines the TTL of the Redis Stream key.
  per_device_event_log_ttl="{{ monitoring.per_device_event_log_ttl }}"


# Global integration related configuration.
[integration]

  # Enabled integrations (global).
  enabled = [
    {{#each integration.enabled}}
    "{{this}}",
    {{/each}}
  ]

  # MQTT integration configuration.
  [integration.mqtt]

    # Event topic template.
    event_topic="{{ integration.mqtt.event_topic }}"

    # Command topic.
    #
    # This is the topic on which the MQTT subscribes for receiving (enqueue) commands.
    command_topic="{{ integration.mqtt.command_topic }}"

    # Use JSON encoding instead of Protobuf (binary).
    json={{ integration.mqtt.json }}

    # MQTT server (e.g. scheme://host:port where scheme is tcp, ssl or ws)
    server="{{ integration.mqtt.server }}"

    # Connect with the given username (optional)
    username="{{ integration.mqtt.username }}"

    # Connect with the given password (optional)
    password="{{ integration.mqtt.password }}"

    # Quality of service level
    #
    # 0: at most once
    # 1: at least once
    # 2: exactly once
    #
    # Note: an increase of this value will decrease the performance.
    # For more information: https://www.hivemq.com/blog/mqtt-essentials-part-6-mqtt-quality-of-service-levels
    qos={{ integration.mqtt.qos }}

    # Clean session
    #
    # Set the "clean session" flag in the connect message when this client
    # connects to an MQTT broker. By setting this flag you are indicating
    # that no messages saved by the broker for this client should be delivered.
    clean_session={{ integration.mqtt.clean_session }}

    # Client ID
    #
    # Set the client id to be used by this client when connecting to the MQTT
    # broker. A client id must be no longer than 23 characters. If left blank,
    # a random id will be generated by ChirpStack.
    client_id="{{ integration.mqtt.client_id }}"

    # Keep alive interval.
    #
    # This defines the maximum time that that should pass without communication
    # between the client and server.
    keep_alive_interval="{{ integration.mqtt.keep_alive_interval }}"

    # CA certificate file (optional)
    #
    # Use this when setting up a secure connection (when server uses ssl://...)
    # but the certificate used by the server is not trusted by any CA certificate
    # on the server (e.g. when self generated).
    ca_cert="{{ integration.mqtt.ca_cert }}"

    # TLS certificate file (optional)
    tls_cert="{{ integration.mqtt.tls_cert }}"

    # TLS key file (optional)
    tls_key="{{ integration.mqtt.tls_key }}"


    # Configuration for MQTT clients.
    [integration.mqtt.client]

      # CA certificate and key file (optional).
      #
      # If setting the CA certificate and key file options, ChirpStack 
      # will generate client certificates which can be used by the MQTT clients for
      # authentication and authorization. The Common Name of the certificate will
      # be set to the ID of the application.
      ca_key="{{ integration.mqtt.client.ca_key }}"
      ca_cert="{{ integration.mqtt.client.ca_cert }}"

      # Certificate lifetime.
      #
      # This defines how long (after generating) the certificate remains valid.
      client_cert_lifetime="{{ integration.mqtt.client.client_cert_lifetime }}"


  # PostgreSQL integration configuration.
  [integration.postgresql]

    # PostgreSQL DSN.
    #
    # Format example: postgres://<USERNAME>:<PASSWORD>@<HOSTNAME>/<DATABASE>?sslmode=<SSLMODE>.
    #
    # SSL mode options:
    #  * disable - no SSL
    #  * require - Always SSL (skip verification)
    #  * verify-ca - Always SSL (verify that the certificate presented by the server was signed by a trusted CA)
    #  * verify-full - Always SSL (verify that the certification presented by the server was signed by a trusted CA and the server host name matches the one in the certificate)
    dsn="{{ integration.postgresql.dsn }}"

    # Max open connections.
    #
    # This sets the max. number of open connections that are allowed in the
    # PostgreSQL connection pool.
    max_open_connections={{ integration.postgresql.max_open_connections }}

    # Min idle connections.
    #
    # This sets the min. number of idle connections in the PostgreSQL connection
    # pool (0 = equal to max_open_connections).
    min_idle_connections={{ integration.postgresql.min_idle_connections }}


  # AMQP / RabbitMQ integration configuration.
  [integration.amqp]

    # Server URL.
    #
    # See for a specification of all the possible options:
    # https://www.rabbitmq.com/uri-spec.html
    url="{{ integration.amqp.url }}"

    # Event routing key.
    #
    # This is the event routing-key template used when publishing device
    # events. Messages will be published to the "amq.topic" exchange.
    event_routing_key="{{ integration.amqp.event_routing_key }}"

    # Use JSON encoding instead of Protobuf (binary).
    json={{ integration.amqp.json }}


  # Kafka integration configuration.
  [integration.kafka]

    # Brokers.
    brokers=[
      {{#each integration.kafka.brokers}}
      "{{this}}",
      {{/each}}
    ]

    # TLS.
    #
    # Set this to true when the Kafka client must connect using TLS to the Broker.
    tls={{ integration.kafka.tls }}

    # Topic for events.
    topic="{{ integration.kafka.topic }}"

    # Template for keys included in Kafka messages.
    # Kafka uses the key for distributing messages over partitions. You can use
    # this to ensure some subset of messages end up in the same partition, so
    # they can be consumed in-order. And Kafka can use the key for data retention
    # decisions.  A header "event" with the event type is included in each
    # message. There is no need to parse it from the key.
    event_key="{{ integration.kafka.event_key }}"

    # Username (optional).
    username="{{ integration.kafka.username }}"

    # Password.
    password="{{ integration.kafka.password }}"

    # Mechanism.
    #
    # Valid options are:
    # * PLAIN
    # * SCRAM-SHA-256
    # * SCRAM-SHA-512
    mechanism="{{ integration.kafka.mechanism }}"

    # Use JSON encoding instead of Protobuf (binary).
    json={{ integration.kafka.json }}


# Codec configuration.
[codec]

  # JS codec configuration.
  [codec.js]

    # Maximum execution time.
    max_execution_time="{{ codec.js.max_execution_time }}"


# User authentication configuration.
[user_authentication]

  # OpenID Connect.
  [user_authentication.openid_connect]

    # Enable OpenID Connect authentication.
    #
    # Enabling this option replaces password authentication.
    enabled={{ user_authentication.openid_connect.enabled }}

    # Registration enabled.
    #
    # Enabling this will automatically register the user when it is not yet present
    # in the ChirpStack database. There is no registration form as the user information
    # is automatically received using the OpenID Connect provided information.
    # The user will not be associated with any organization, but in order to
    # facilitate the automatic onboarding of users, it is possible to configure a
    # registration callback URL (next config option).
    registration_enabled={{ user_authentication.openid_connect.registration_enabled }}

    # Registration callback URL.
    #
    # This (optional) endpoint will be called on the registration of the user and
    # can implement the association of the user with an organization, create a new
    # organization, ...
    # ChirpStack will make a HTTP POST call to this endpoint,
    # with the following URL parameters:
    # - user_id, of the newly created user in ChirpStack.
    #
    # The POST body contains a JSON payload with the OpenID Connect UserInfo payload.
    registration_callback_url="{{ user_authentication.openid_connect.registration_callback_url }}"

    # Provider URL.
    # This is the URL of the OpenID Connect provider.
    # Example: https://auth.example.org
    provider_url="{{ user_authentication.openid_connect.provider_url }}"

    # Client ID.
    client_id="{{ user_authentication.openid_connect.client_id }}"

    # Client secret.
    client_secret="{{ user_authentication.openid_connect.client_secret }}"

    # Redirect URL.
    #
    # This must contain the ChirpStack Application Server web-interface hostname
    # with '/auth/oidc/callback' path, e.g. https://example.com/auth/oidc/callback.
    redirect_url="{{ user_authentication.openid_connect.redirect_url }}"

    # Logout URL.
    #
    # When set, ChirpStack Application Server will redirect to this URL instead
    # of redirecting to the login page.
    logout_url="{{ user_authentication.openid_connect.logout_url }}"

    # Login label.
    #
    # The login label is used in the web-interface login form.
    login_label="{{ user_authentication.openid_connect.login_label }}"


# Join Server configuration.
[join_server]

    # Per Join Server configuration (this can be repeated).
    # Example:
    # [[join_server.servers]]
    #
    #   # JoinEUI of the Join Server.
    #   join_eui="0102030405060708"
    #
    #   # Server endpoint.
    #   server="https://example.com:1234/join/endpoint"

    #   # Use the async interface scheme.
    #   async_interface=false

    #   # Async interface request timeout.
    #   async_interface_timeout="1s"

    #   # CA certificate (optional).
    #   #
    #   # Set this to validate the join-server server certificate (e.g. if the
    #   # certificate was self-signed).
    #   ca_cert="/path/to/ca.pem"

    #   # TLS client-certificate (optional).
    #   #
    #   # Set this to enable client-certificate authentication with the join-server.
    #   tls_cert="/path/to/tls_cert.pem"

    #   # TLS client-certificate key (optional).
    #   #
    #   # Set this to enable client-certificate authentication with the join-server.
    #   tls_key="/path/to/tls_key.pem"
    {{#each join_server.servers}}

    [[join_server.servers]]
      join_eui="{{ this.join_eui }}"
      server="{{ this.server }}"
      async_interface={{ this.async_interface }}
      async_interface_timeout="{{ this.async_interface_timeout }}"
      ca_cert="{{ this.ca_cert }}"
      tls_cert="{{ this.tls_cert }}"
      tls_key="{{ this.tls_key }}"
    {{/each}}


# Backend Interfaces configuration (optional).
[backend_interfaces]

  # interface:port to bind the Backend Interfaces API to.
  #
  # Note: this interface is used both for passive-roaming and when
  # integrating with Join Servers that implement the async interface.
  # Leaving this option blank will disable the Backend Interfaces API,
  # which is fine in most cases.
  bind="{{ backend_interfaces.bind }}"

  # CA certificate (path).
  ca_cert="{{ backend_interfaces.ca_cert }}"

  # TLS certificate (path).
  tls_cert="{{ backend_interfaces.tls_cert }}"

  # TLS key (path).
  tls_key="{{ backend_interfaces.tls_key }}"


# Roaming configuration.
[roaming]

  # Resolve NetID domain suffix.
  resolve_net_id_domain_suffix="{{ backend_interfaces.resolve_net_id_domain_suffix }}"

  # Per server roaming configuration (this can be repeated).
  # Example:
  # [[roaming.servers]]
  #
  #  # NetID of the roaming server.
  #  net_id="010203"
  #
  #  # Async timeout (set to 0 to disable async interface).
  #  async_timeout="0s"
  #
  #  # Passive-roaming session lifetime (set to 0 for stateless).
  #  passive_roaming_lifetime="0s"
  #
  #  # Passive-roaming KEK label (optional).
  #  #
  #  # If set, the session-keys will be encrypted using the given KEK.
  #  passive_roaming_kek_label=""
  #
  #  # Server.
  #  #
  #  # If set, this will bypass the DNS resolving of the server.
  #  server="https://example.com:1234"
  #
  #  # Use target role suffix.
  #  #
  #  # Depending the context of the remote server, this will add
  #  # the /sns or /fns path to the server endpoint.
  #  use_target_role_suffix=false
  #
  #  # CA certificate (path).
  #  ca_cert=""
  #
  #  # TLS certificate (path).
  #  tls_cert=""
  #
  #  # TLS key (path).
  #  tls_key=""
  #
  #  # Authorization header.
  #  #
  #  # Optional value of the Authorization header, e.g. token or password.
  #  authorization_header=""
  {{#each roaming.servers}}

  [[roaming.servers]]
    net_id="{{ this.net_id }}"
    async_timeout="{{ this.async_timeout }}"
    passive_roaming_lifetime="{{ this.passive_roaming_lifetime }}"
    passive_roaming_kek_label="{{ this.passive_roaming_kek_label }}"
    server="{{ this.server }}"
    use_target_role_suffix="{{ this.use_target_role_suffix }}"
    ca_cert="{{ this.ca_cert }}"
    tls_cert="{{ this.tls_cert }}"
    tls_key="{{ this.tls_key }}"
    authorization_header="{{ this.authorization_header }}"
  {{/each}}


# Key encryption keys (KEKs).
#
# KEKs can be used to encrypt session-keys between two endpoints,
# for example a Join Server and Network Server, or between two
# Network Servers in case of a roaming agreement. If used, the
# sender will encrypt the session-key with the KEK and indicates
# to the receiver the label of the KEK that was used for encryption,
# such that the receiver is able to decrypt the session-key.
#
# Example (can be repeated):
# [[keks]]
#
#   # KEK label.
#   label="kek-label"

#   # Encryption key.
#   kek="01020304050607080102030405060708"
{{#each keks}}

[[keks]]
  label="{{ this.label }}"
  kek="{{ this.kek }}"
{{/each}}
"#;

    let mut reg = Handlebars::new();
    reg.register_escape_fn(no_escape);
    let conf = config::get();
    println!(
        "{}",
        reg.render_template(template, &conf)
            .expect("render configfile error")
    );
}
