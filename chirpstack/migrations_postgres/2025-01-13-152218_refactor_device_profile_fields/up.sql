alter table device_profile
  add column abp_params jsonb null,
  add column class_b_params jsonb null,
  add column class_c_params jsonb null,
  add column relay_params jsonb null,
  add column app_layer_params jsonb not null default '{}';

alter table device_profile
  alter column app_layer_params drop default;

update device_profile
  set abp_params = json_build_object(
    'rx1_delay', abp_rx1_delay,
    'rx1_dr_offset', abp_rx1_dr_offset,
    'rx2_dr', abp_rx2_dr,
    'rx2_freq', abp_rx2_freq)
  where supports_otaa = false;

update device_profile
  set class_b_params = json_build_object(
    'timeout', class_b_timeout,
    'ping_slot_nb_k', class_b_ping_slot_nb_k,
    'ping_slot_dr', class_b_ping_slot_dr,
    'ping_slot_freq', class_b_ping_slot_freq)
  where supports_class_b = true;

update device_profile
  set class_c_params = json_build_object(
    'timeout', class_c_timeout)
  where
    supports_class_c = true;

update device_profile
  set relay_params = json_build_object(
    'is_relay', is_relay,
    'is_relay_ed', is_relay_ed,
    'ed_relay_only', relay_ed_relay_only,
    'relay_enabled', relay_enabled,
    'relay_cad_periodicity', relay_cad_periodicity,
    'default_channel_index', relay_default_channel_index,
    'second_channel_freq', relay_second_channel_freq,
    'second_channel_dr', relay_second_channel_dr,
    'second_channel_ack_offset', relay_second_channel_ack_offset,
    'ed_activation_mode', relay_ed_activation_mode,
    'ed_smart_enable_level', relay_ed_smart_enable_level,
    'ed_back_off', relay_ed_back_off,
    'ed_uplink_limit_bucket_size', relay_ed_uplink_limit_bucket_size,
    'ed_uplink_limit_reload_rate', relay_ed_uplink_limit_reload_rate,
    'relay_join_req_limit_reload_rate', relay_join_req_limit_reload_rate,
    'relay_notify_limit_reload_rate', relay_notify_limit_reload_rate,
    'relay_global_uplink_limit_reload_rate', relay_global_uplink_limit_reload_rate,
    'relay_overall_limit_reload_rate', relay_overall_limit_reload_rate,
    'relay_join_req_limit_bucket_size', relay_join_req_limit_bucket_size,
    'relay_notify_limit_bucket_size', relay_notify_limit_bucket_size,
    'relay_global_uplink_limit_bucket_size', relay_global_uplink_limit_bucket_size,
    'relay_overall_limit_bucket_size', relay_overall_limit_bucket_size)
  where
    is_relay = true or is_relay_ed = true;

alter table device_profile
  drop column abp_rx1_delay,
  drop column abp_rx1_dr_offset,
  drop column abp_rx2_dr,
  drop column abp_rx2_freq,
  drop column class_b_timeout,
  drop column class_b_ping_slot_nb_k,
  drop column class_b_ping_slot_dr,
  drop column class_b_ping_slot_freq,
  drop column class_c_timeout,
  drop column is_relay,
  drop column is_relay_ed,
  drop column relay_ed_relay_only,
  drop column relay_enabled,
  drop column relay_cad_periodicity,
  drop column relay_default_channel_index,
  drop column relay_second_channel_freq,
  drop column relay_second_channel_dr,
  drop column relay_second_channel_ack_offset,
  drop column relay_ed_activation_mode,
  drop column relay_ed_smart_enable_level,
  drop column relay_ed_back_off,
  drop column relay_ed_uplink_limit_bucket_size,
  drop column relay_ed_uplink_limit_reload_rate,
  drop column relay_join_req_limit_reload_rate,
  drop column relay_notify_limit_reload_rate,
  drop column relay_global_uplink_limit_reload_rate,
  drop column relay_overall_limit_reload_rate,
  drop column relay_join_req_limit_bucket_size,
  drop column relay_notify_limit_bucket_size,
  drop column relay_global_uplink_limit_bucket_size,
  drop column relay_overall_limit_bucket_size;

