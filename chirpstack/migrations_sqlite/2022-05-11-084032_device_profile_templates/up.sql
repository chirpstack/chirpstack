create table device_profile_template (
    id text not null primary key,
    created_at datetime not null,
    updated_at datetime not null,
    name varchar(100) not null,
    description text not null,
    vendor varchar(100) not null,
    firmware varchar(100) not null,
    region varchar(10) not null,
    mac_version varchar(10) not null,
    reg_params_revision varchar(20) not null,
    adr_algorithm_id varchar(100) not null,
    payload_codec_runtime varchar(20) not null,
    payload_codec_script text not null,
    uplink_interval integer not null,
    device_status_req_interval integer not null,
    flush_queue_on_activate boolean not null,
    supports_otaa boolean not null,
    supports_class_b boolean not null,
    supports_class_c boolean not null,
    class_b_timeout integer not null,
    class_b_ping_slot_period integer not null,
    class_b_ping_slot_dr smallint not null,
    class_b_ping_slot_freq bigint not null,
    class_c_timeout integer not null,
    abp_rx1_delay smallint not null,
    abp_rx1_dr_offset smallint not null,
    abp_rx2_dr smallint not null,
    abp_rx2_freq bigint not null,
    tags text not null
);

-- sqlite: changing type of a column requires a specific procedure
-- additionally sqlite doesn't care about smallint vs integer
-- alter table device_profile alter column class_b_ping_slot_dr type smallint,
alter table device_profile add column description text not null default '';

-- sqlite: no drop default
-- alter table device_profile alter column description drop default;
