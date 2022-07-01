create table event_up (
    deduplication_id uuid primary key,
    time timestamp with time zone not null,
    tenant_id uuid not null,
    tenant_name text not null,
    application_id uuid not null,
    application_name text not null,
    device_profile_id uuid not null,
    device_profile_name text not null,
    device_name text not null,
    dev_eui char(16) not null,
    tags jsonb not null,
    dev_addr char(8) not null,
    adr boolean not null,
    dr smallint not null,
    f_cnt bigint not null,
    f_port smallint not null,
    confirmed boolean not null,
    data bytea not null,
    object jsonb not null,
    rx_info jsonb not null,
    tx_info jsonb not null
);

create table event_join (
    deduplication_id uuid primary key,
    time timestamp with time zone not null,
    tenant_id uuid not null,
    tenant_name text not null,
    application_id uuid not null,
    application_name text not null,
    device_profile_id uuid not null,
    device_profile_name text not null,
    device_name text not null,
    dev_eui char(16) not null,
    tags jsonb not null,
    dev_addr char(8) not null
);

create table event_ack (
    queue_item_id uuid primary key,
    deduplication_id uuid not null,
    time timestamp with time zone not null,
    tenant_id uuid not null,
    tenant_name text not null,
    application_id uuid not null,
    application_name text not null,
    device_profile_id uuid not null,
    device_profile_name text not null,
    device_name text not null,
    dev_eui char(16) not null,
    tags jsonb not null,
    acknowledged boolean not null,
    f_cnt_down bigint not null
);

create table event_tx_ack (
    queue_item_id uuid primary key,
    downlink_id bigint not null,
    time timestamp with time zone not null,
    tenant_id uuid not null,
    tenant_name text not null,
    application_id uuid not null,
    application_name text not null,
    device_profile_id uuid not null,
    device_profile_name text not null,
    device_name text not null,
    dev_eui char(16) not null,
    tags jsonb not null,
    f_cnt_down bigint not null,
    gateway_id char(16) not null,
    tx_info jsonb not null
);

create table event_log (
    id bigserial primary key,
    time timestamp with time zone not null,
    tenant_id uuid not null,
    tenant_name text not null,
    application_id uuid not null,
    application_name text not null,
    device_profile_id uuid not null,
    device_profile_name text not null,
    device_name text not null,
    dev_eui char(16) not null,
    tags jsonb not null,
    level text not null,
    code text not null,
    description text not null,
    context jsonb not null
);

create table event_status (
    deduplication_id uuid primary key,
    time timestamp with time zone not null,
    tenant_id uuid not null,
    tenant_name text not null,
    application_id uuid not null,
    application_name text not null,
    device_profile_id uuid not null,
    device_profile_name text not null,
    device_name text not null,
    dev_eui char(16) not null,
    tags jsonb not null,
    margin smallint not null,
    external_power_source boolean not null,
    battery_level_unavailable boolean not null,
    battery_level real not null
);

create table event_location (
    deduplication_id uuid primary key,
    time timestamp with time zone not null,
    tenant_id uuid not null,
    tenant_name text not null,
    application_id uuid not null,
    application_name text not null,
    device_profile_id uuid not null,
    device_profile_name text not null,
    device_name text not null,
    dev_eui char(16) not null,
    tags jsonb not null,
    latitude double precision not null,
    longitude double precision not null,
    altitude double precision not null,
    source text not null,
    accuracy real not null
);

create table event_integration (
    deduplication_id uuid primary key,
    time timestamp with time zone not null,
    tenant_id uuid not null,
    tenant_name text not null,
    application_id uuid not null,
    application_name text not null,
    device_profile_id uuid not null,
    device_profile_name text not null,
    device_name text not null,
    dev_eui char(16) not null,
    tags jsonb not null,
    integration_name text not null,
    event_type text not null,
    object jsonb not null
);
