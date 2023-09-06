-- user
create table "user" (
    id text not null primary key,
    external_id text null,
    created_at datetime not null,
    updated_at datetime not null,
    is_admin boolean not null,
    is_active boolean not null,
    email text not null,
    email_verified boolean not null,
    password_hash varchar(200) not null,
    note text not null
);

create unique index idx_user_email on "user"(email);
create unique index idx_user_external_id on "user"(external_id);

insert into "user" (
    id,
    created_at,
    updated_at,
    is_admin,
    is_active,
    email,
    email_verified,
    password_hash,
    note
) values (
    '05244f12-6daf-4e1f-8315-c66783a0ab56',
    datetime('now'),
    datetime('now'),
    'true',
    'true',
    'admin',
    'false',
    '$pbkdf2-sha512$i=1,l=64$l8zGKtxRESq3PA2kFhHRWA$H3lGMxOt55wjwoc+myeOoABofJY9oDpldJa7fhqdjbh700V6FLPML75UmBOt9J5VFNjAL1AvqCozA1HJM0QVGA',
    ''
);

-- tenant
create table tenant (
    id text not null primary key,
    created_at datetime not null,
    updated_at datetime not null,
    name varchar(100) not null,
    description text not null,
    can_have_gateways boolean not null,
    max_device_count integer not null,
    max_gateway_count integer not null,
    private_gateways boolean not null
);

-- sqlite has advanced text search with https://www.sqlite.org/fts5.html
-- but looks like it is for a full table and not specific per column, to investigate
create index idx_tenant_name_trgm on "tenant"(name);

insert into "tenant" (
    id,
    created_at,
    updated_at,
    name,
    description,
    can_have_gateways,
    max_device_count,
    max_gateway_count,
    private_gateways
) values (
    '52f14cd4-c6f1-4fbd-8f87-4025e1d49242',
    datetime('now'),
    datetime('now'),
    'ChirpStack',
    '',
    'true',
    0,
    0,
    'false'
);

-- tenant user
create table tenant_user (
    tenant_id text not null references tenant on delete cascade,
    user_id text not null references "user" on delete cascade,
    created_at datetime not null,
    updated_at datetime not null,
    is_admin boolean not null,
    is_device_admin boolean not null,
    is_gateway_admin boolean not null,
    primary key (tenant_id, user_id)
);

create index idx_tenant_user_user_id on tenant_user (user_id);

-- gateway
create table gateway (
    gateway_id blob not null primary key,
    tenant_id text not null references tenant on delete cascade,
    created_at datetime not null,
    updated_at datetime not null,
    last_seen_at datetime,
    name varchar(100) not null,
    description text not null,
    latitude double precision not null,
    longitude double precision not null,
    altitude real not null,
    stats_interval_secs integer not null,
    tls_certificate blob,
    tags text not null,
    properties text not null
);

create index idx_gateway_tenant_id on gateway (tenant_id);
create index idx_gateway_name_trgm on gateway (name);
create index idx_gateway_id_trgm on gateway (hex(gateway_id));
create index idx_gateway_tags on gateway (tags);

-- application
create table application (
    id text not null primary key,
    tenant_id text not null references tenant on delete cascade,
    created_at datetime not null,
    updated_at datetime not null,
    name varchar(100) not null,
    description text not null,
    mqtt_tls_cert blob
);

create index idx_application_tenant_id on application (tenant_id);
create index idx_application_name_trgm on application (name);

-- application integration
create table application_integration (
    application_id text not null references application on delete cascade,
    kind varchar(20) not null,
    created_at datetime not null,
    updated_at datetime not null,
    configuration text not null,

    primary key(application_id, kind)
);

-- api-key
create table api_key (
    id text not null primary key,
    created_at datetime not null,
    name varchar(100) not null,
    is_admin boolean not null,
    tenant_id text null references tenant on delete cascade
);

create index idx_api_key_tenant_id on api_key (tenant_id);

-- device-profile
create table device_profile (
    id text not null primary key,
    tenant_id text not null references tenant on delete cascade,
    created_at datetime not null,
    updated_at datetime not null,
    name varchar(100) not null,
    region varchar(10) not null,
    mac_version varchar(10) not null,
    reg_params_revision varchar(20) not null,
    adr_algorithm_id varchar(100) not null,
    payload_codec_runtime varchar(20) not null,
    payload_encoder_config text not null,
    payload_decoder_config text not null,
    uplink_interval integer not null,
    device_status_req_interval integer not null,
    supports_otaa boolean not null,
    supports_class_b boolean not null,
    supports_class_c boolean not null,
    class_b_timeout integer not null,
    class_b_ping_slot_period integer not null,
    class_b_ping_slot_dr integer not null, -- switched to smallint in base schema (was added in device_profile_templates)
    class_b_ping_slot_freq bigint not null,
    class_c_timeout integer not null,
    abp_rx1_delay smallint not null,
    abp_rx1_dr_offset smallint not null,
    abp_rx2_dr smallint not null,
    abp_rx2_freq bigint not null,
    tags text not null
);

create index idx_device_profile_tenant_id on device_profile (tenant_id);
create index idx_device_profile_name_trgm on device_profile (name);
create index idx_device_profile_tags on device_profile (tags);

-- device
create table device (
    dev_eui blob not null primary key,
    application_id text not null references application on delete cascade,
    device_profile_id text not null references device_profile on delete cascade,
    created_at datetime not null,
    updated_at datetime not null,
    last_seen_at datetime,
    scheduler_run_after datetime,
    name varchar(100) not null,
    description text not null,
    external_power_source boolean not null,
    battery_level numeric(5, 2),
    margin int,
    dr smallint,
    latitude double precision,
    longitude double precision,
    altitude real,
    dev_addr blob,
    enabled_class char(1) not null, 
    skip_fcnt_check boolean not null,
    is_disabled boolean not null,
    tags text not null,
    variables text not null
);

create index idx_device_application_id on device (application_id);
create index idx_device_device_profile_id on device (device_profile_id);
create index idx_device_name_trgm on device (name);
create index idx_device_dev_eui_trgm on device (hex(dev_eui));
create index idx_device_dev_addr_trgm on device (hex(dev_addr));
create index idx_device_tags on device (tags);

create table device_keys (
    dev_eui blob not null primary key references device on delete cascade,
    created_at datetime not null,
    updated_at datetime not null,
    nwk_key blob not null,
    app_key blob not null,
    dev_nonces text not null,
    join_nonce int not null
);

create table device_queue_item (
    id text not null primary key,
    dev_eui blob references device on delete cascade not null,
    created_at datetime not null,
    f_port smallint not null,
    confirmed boolean not null,
    data blob not null,
    is_pending boolean not null,
    f_cnt_down bigint null,
    timeout_after datetime
);

create index idx_device_queue_item_dev_eui on device_queue_item (dev_eui);
create index idx_device_queue_item_created_at on device_queue_item (created_at);
create index idx_device_queue_item_timeout_after on device_queue_item (timeout_after);


-- multicast groups
create table multicast_group (
    id text not null primary key,
    application_id text not null references application on delete cascade,
    created_at datetime not null,
    updated_at datetime not null,
    name varchar(100) not null,
    region varchar(10) not null,
    mc_addr blob not null,
    mc_nwk_s_key blob not null,
    mc_app_s_key blob not null,
    f_cnt bigint not null,
    group_type char(1) not null,
    dr smallint not null,
    frequency bigint not null,
    class_b_ping_slot_period integer not null
);

create index idx_multicast_group_application_id on multicast_group (application_id);
create index idx_multicast_group_name_trgm on multicast_group (name);

create table multicast_group_device (
    multicast_group_id text not null references multicast_group on delete cascade,
    dev_eui blob not null references device on delete cascade,
    created_at datetime not null,
    primary key (multicast_group_id, dev_eui)
);

create table multicast_group_queue_item (
    id text not null primary key,
    created_at datetime not null,
    scheduler_run_after datetime not null,
    multicast_group_id text not null references multicast_group on delete cascade,
    gateway_id blob not null references gateway on delete cascade,
    f_cnt bigint not null,
    f_port smallint not null,
    data blob not null,
    emit_at_time_since_gps_epoch bigint
);

create index idx_multicast_group_queue_item_multicast_group_id on multicast_group_queue_item (multicast_group_id);
create index idx_multicast_group_queue_item_scheduler_run_after on multicast_group_queue_item (scheduler_run_after);
