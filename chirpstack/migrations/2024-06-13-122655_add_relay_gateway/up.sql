create table relay_gateway (
    tenant_id uuid not null references tenant on delete cascade,
    relay_id bytea not null,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null,
    last_seen_at timestamp with time zone,
    name varchar(100) not null,
    description text not null,
    stats_interval_secs integer not null,
    region_config_id varchar(100) not null,

    primary key (tenant_id, relay_id)
);
