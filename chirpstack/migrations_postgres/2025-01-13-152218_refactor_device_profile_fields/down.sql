alter table device_profile
  add column abp_rx1_delay smallint not null default 0,
  add column abp_rx1_dr_offset smallint not null default 0,
  add column abp_rx2_dr smallint not null default 0,
  add column abp_rx2_freq bigint not null default 0,
  add column class_b_timeout integer not null default 0,
  add column class_b_ping_slot_nb_k integer not null default 0,
  add column class_b_ping_slot_dr smallint not null default 0,
  add column class_b_ping_slot_freq bigint not null default 0;

update device_profile
  set
    abp_rx1_delay = (abp_params->'rx1_delay')::smallint,
    abp_rx1_dr_offset = (abp_params->'rx1_dr_offset')::smallint,
    abp_rx2_dr = (abp_params->'rx2_dr')::smallint,
    abp_rx2_freq = (abp_params->'rx2_freq')::bigint
  where
    abp_params is not null;

update device_profile
  set
    class_b_timeout = (class_b_params->'timeout')::integer,
    class_b_ping_slot_nb_k = (class_b_params->'ping_slot_nb_k')::integer,
    class_b_ping_slot_dr = (class_b_params->'ping_slot_dr')::smallint,
    class_b_ping_slot_freq = (class_b_params->'ping_slot_freq')::bigint
  where
    class_b_params is not null;

alter table device_profile
  drop column abp_params,
  drop column class_b_params;

