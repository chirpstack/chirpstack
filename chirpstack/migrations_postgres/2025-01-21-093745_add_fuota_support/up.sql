create table fuota_deployment (
  id uuid primary key,
  created_at timestamp with time zone not null,
  updated_at timestamp with time zone not null,
  started_at timestamp with time zone null,
  completed_at timestamp with time zone null,
  name varchar(100) not null,
  application_id uuid not null references application on delete cascade,
  device_profile_id uuid not null references device_profile on delete cascade,
  multicast_addr bytea not null,
  multicast_key bytea not null,
  multicast_group_type char(1) not null,
  multicast_class_c_scheduling_type varchar(20) not null,
  multicast_dr smallint not null,
  multicast_class_b_ping_slot_nb_k smallint not null,
  multicast_frequency bigint not null,
  multicast_timeout smallint not null,
  multicast_session_start timestamp with time zone null,
  multicast_session_end timestamp with time zone null,
  unicast_max_retry_count smallint not null,
  fragmentation_fragment_size smallint not null,
  fragmentation_redundancy_percentage smallint not null,
  fragmentation_session_index smallint not null,
  fragmentation_matrix smallint not null,
  fragmentation_block_ack_delay smallint not null,
  fragmentation_descriptor bytea not null,
  request_fragmentation_session_status varchar(20) not null,
  payload bytea not null,
  on_complete_set_device_tags jsonb not null
);

create table fuota_deployment_device (
  fuota_deployment_id uuid not null references fuota_deployment on delete cascade,
  dev_eui bytea not null references device on delete cascade,
  created_at timestamp with time zone not null,
  completed_at timestamp with time zone null,
  mc_group_setup_completed_at timestamp with time zone null,
  mc_session_completed_at timestamp with time zone null,
  frag_session_setup_completed_at timestamp with time zone null,
  frag_status_completed_at timestamp with time zone null,
  error_msg text not null,

  primary key (fuota_deployment_id, dev_eui)
);

create table fuota_deployment_gateway (
  fuota_deployment_id uuid not null references fuota_deployment on delete cascade,
  gateway_id bytea not null references gateway on delete cascade,
  created_at timestamp with time zone not null,

  primary key (fuota_deployment_id, gateway_id)
);

create table fuota_deployment_job (
  fuota_deployment_id uuid not null references fuota_deployment on delete cascade,
  job varchar(20) not null,
  created_at timestamp with time zone not null,
  completed_at timestamp with time zone null,
  max_retry_count smallint not null,
  attempt_count smallint not null,
  scheduler_run_after timestamp with time zone not null,
  warning_msg text not null,
  error_msg text not null,

  primary key (fuota_deployment_id, job)
);

create index idx_fuota_deployment_job_completed_at on fuota_deployment_job(completed_at);
create index idx_fuota_deployment_job_scheduler_run_after on fuota_deployment_job(scheduler_run_after);

alter table device_keys
  add column gen_app_key bytea not null default decode('00000000000000000000000000000000', 'hex');

alter table device_keys
  alter column gen_app_key drop default;

alter table device
  add column app_layer_params jsonb not null default '{}';

alter table device
  alter column app_layer_params drop default;
