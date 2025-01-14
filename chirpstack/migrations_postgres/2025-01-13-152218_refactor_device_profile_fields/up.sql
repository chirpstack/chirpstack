alter table device_profile
  add column abp_params jsonb null;

update device_profile
  set abp_params = json_build_object(
    'rx1_delay', abp_rx1_delay,
    'rx1_dr_offset', abp_rx1_dr_offset,
    'rx2_dr', abp_rx2_dr,
    'rx2_freq', abp_rx2_freq)
  where supports_otaa = false;

alter table device_profile
  drop column abp_rx1_delay,
  drop column abp_rx1_dr_offset,
  drop column abp_rx2_dr,
  drop column abp_rx2_freq;

