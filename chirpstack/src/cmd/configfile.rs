use handlebars::{no_escape, Handlebars};

use super::super::config;

pub fn run() {
    let template = r#"
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


# Network related configuration.
[network]

  # Network identifier (NetID, 3 bytes) encoded as HEX (e.g. 010203).
  net_id="{{ network.net_id }}"

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

  # Mac-commands disabled.
  mac_commands_disabled={{ network.mac_commands_disabled }}


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

    # Multicast Class-C use GPS time.
    #
    # Use GPS time for scheduling multicast class-c downlinks. If this is enabled
    # and the downlink must be send by multiple gateways to cover all devices
    # within the multicast-group, these downlinks will be sent at exactly the same
    # time by these gateways. If disabled, ChirpStack will use the configured
    # margin. Only enable this features when all gateways have GNSS support.
    multicast_class_c_use_gps_time={{ network.scheduler.multicast_class_c_use_gps_time }}

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

    # State topic template.
    #
    # Events that expose a certain state of the device, are published as retained messages
    # to the state topic.
    state_topic="{{ integration.mqtt.state_topic }}"

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
    # broker. A client id must be no longer than 23 characters. When left blank,
    # a random id will be generated. This requires clean_session=true.
    client_id="{{ integration.mqtt.client_id }}"

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
