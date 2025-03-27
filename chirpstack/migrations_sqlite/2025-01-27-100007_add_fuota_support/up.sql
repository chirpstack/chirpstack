create table fuota_deployment (
    id text not null primary key,
    created_at datetime not null,
    updated_at datetime not null,
    started_at datetime null,
    completed_at datetime null,
    name varchar(100) not null,
    application_id text not null references application on delete cascade,
    device_profile_id text not null references device_profile on delete cascade,
    multicast_addr blob not null,
    multicast_key blob not null,
    multicast_group_type char(1) not null,
    multicast_class_c_scheduling_type varchar(20) not null,
    multicast_dr smallint not null,
    multicast_class_b_ping_slot_nb_k smallint not null,
    multicast_frequency bigint not null,
    multicast_timeout smallint not null,
    multicast_session_start datetime null,
    multicast_session_end datetime null,
    unicast_max_retry_count smallint not null,
    fragmentation_fragment_size smallint not null,
    fragmentation_redundancy_percentage smallint not null,
    fragmentation_session_index smallint not null,
    fragmentation_matrix smallint not null,
    fragmentation_block_ack_delay smallint not null,
    fragmentation_descriptor blob not null,
    request_fragmentation_session_status varchar(20) not null,
    payload blob not null,
    on_complete_set_device_tags text not null
);

create table fuota_deployment_device (
    fuota_deployment_id text not null references fuota_deployment on delete cascade,
    dev_eui blob not null references device on delete cascade,
    created_at datetime not null,
    completed_at datetime null,
    mc_group_setup_completed_at datetime null,
    mc_session_completed_at datetime null,
    frag_session_setup_completed_at datetime null,
    frag_status_completed_at datetime null,
    error_msg text not null,

    primary key (fuota_deployment_id, dev_eui)
);

create table fuota_deployment_gateway (
    fuota_deployment_id text not null references fuota_deployment on delete cascade,
    gateway_id blob not null references gateway on delete cascade,
    created_at datetime not null,

    primary key (fuota_deployment_id, gateway_id)
);

create table fuota_deployment_job (
    fuota_deployment_id text not null references fuota_deployment on delete cascade,
    job varchar(20) not null,
    created_at datetime not null,
    completed_at datetime null,
    max_retry_count smallint not null,
    attempt_count smallint not null,
    scheduler_run_after datetime not null,
    warning_msg text not null,
    error_msg text not null,

    primary key (fuota_deployment_id, job)
);

create index idx_fuota_deployment_job_completed_at on fuota_deployment_job(completed_at);
create index idx_fuota_deployment_job_scheduler_run_after on fuota_deployment_job(scheduler_run_after);

alter table device_keys
    add column gen_app_key blob not null default x'00000000000000000000000000000000';

alter table device
    add column app_layer_params text not null default '{}';
