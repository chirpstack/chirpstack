alter table device_profile
  add column abp_params jsonb null,
  add column class_b_params jsonb null,
  add column class_c_params jsonb null;

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

alter table device_profile
  drop column abp_rx1_delay,
  drop column abp_rx1_dr_offset,
  drop column abp_rx2_dr,
  drop column abp_rx2_freq,
  drop column class_b_timeout,
  drop column class_b_ping_slot_nb_k,
  drop column class_b_ping_slot_dr,
  drop column class_b_ping_slot_freq,
  drop column class_c_timeout;

