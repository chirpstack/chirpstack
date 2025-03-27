alter table device_profile
  add column abp_rx1_delay smallint not null default 0,
  add column abp_rx1_dr_offset smallint not null default 0,
  add column abp_rx2_dr smallint not null default 0,
  add column abp_rx2_freq bigint not null default 0,
  add column class_b_timeout integer not null default 0,
  add column class_b_ping_slot_nb_k integer not null default 0,
  add column class_b_ping_slot_dr smallint not null default 0,
  add column class_b_ping_slot_freq bigint not null default 0,
  add column class_c_timeout integer not null default 0,
  add column is_relay boolean not null default false,
  add column is_relay_ed boolean not null default false,
  add column relay_ed_relay_only boolean not null default false,
  add column relay_enabled boolean not null default false,
  add column relay_cad_periodicity smallint not null default 0,
  add column relay_default_channel_index smallint not null default 0,
  add column relay_second_channel_freq bigint not null default 0,
  add column relay_second_channel_dr smallint not null default 0,
  add column relay_second_channel_ack_offset smallint not null default 0,
  add column relay_ed_activation_mode smallint not null default 0,
  add column relay_ed_smart_enable_level smallint not null default 0,
  add column relay_ed_back_off smallint not null default 0,
  add column relay_ed_uplink_limit_bucket_size smallint not null default 0,
  add column relay_ed_uplink_limit_reload_rate smallint not null default 0,
  add column relay_join_req_limit_reload_rate smallint not null default 0,
  add column relay_notify_limit_reload_rate smallint not null default 0,
  add column relay_global_uplink_limit_reload_rate smallint not null default 0,
  add column relay_overall_limit_reload_rate smallint not null default 0,
  add column relay_join_req_limit_bucket_size smallint not null default 0,
  add column relay_notify_limit_bucket_size smallint not null default 0,
  add column relay_global_uplink_limit_bucket_size smallint not null default 0,
  add column relay_overall_limit_bucket_size smallint not null default 0;

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

update device_profile
  set
    class_c_timeout = (class_c_params->'timeout')::integer
  where
    class_c_params is not null;

update device_profile
  set
    is_relay = (relay_params->'is_relay')::boolean,
    is_relay_ed = (relay_params->'is_relay_ed')::boolean,
    relay_ed_relay_only = (relay_params->'ed_relay_only')::boolean,
    relay_enabled = (relay_params->'relay_enabled')::boolean,
    relay_cad_periodicity = (relay_params->'relay_cad_periodicity')::smallint,
    relay_default_channel_index = (relay_params->'default_channel_index')::smallint,
    relay_second_channel_freq = (relay_params->'second_channel_freq')::bigint,
    relay_second_channel_dr = (relay_params->'second_channel_dr')::smallint,
    relay_second_channel_ack_offset = (relay_params->'second_channel_ack_offset')::smallint,
    relay_ed_activation_mode = (relay_params->'ed_activation_mode')::smallint,
    relay_ed_smart_enable_level = (relay_params->'ed_smart_enable_level')::smallint,
    relay_ed_back_off = (relay_params->'ed_back_off')::smallint,
    relay_ed_uplink_limit_bucket_size = (relay_params->'ed_uplink_limit_bucket_size')::smallint,
    relay_ed_uplink_limit_reload_rate = (relay_params->'ed_uplink_limit_reload_rate')::smallint,
    relay_join_req_limit_reload_rate = (relay_params->'relay_join_req_limit_reload_rate')::smallint,
    relay_notify_limit_reload_rate = (relay_params->'relay_notify_limit_reload_rate')::smallint,
    relay_global_uplink_limit_reload_rate = (relay_params->'relay_global_uplink_limit_reload_rate')::smallint,
    relay_overall_limit_reload_rate = (relay_params->'relay_overall_limit_reload_rate')::smallint,
    relay_join_req_limit_bucket_size = (relay_params->'relay_join_req_limit_bucket_size')::smallint,
    relay_notify_limit_bucket_size = (relay_params->'relay_notify_limit_bucket_size')::smallint,
    relay_global_uplink_limit_bucket_size = (relay_params->'relay_global_uplink_limit_bucket_size')::smallint,
    relay_overall_limit_bucket_size = (relay_params->'relay_overall_limit_bucket_size')::smallint
  where
    relay_params is not null;

alter table device_profile
  drop column abp_params,
  drop column class_b_params,
  drop column class_c_params,
  drop column relay_params,
  drop column app_layer_params;
