alter table device_profile add column abp_params text null;
alter table device_profile add column class_b_params text null;

update device_profile
  set abp_params = json_object(
    'rx1_delay', abp_rx1_delay,
    'rx1_dr_offset', abp_rx1_dr_offset,
    'rx2_dr', abp_rx2_dr,
    'rx2_freq', abp_rx2_freq)
  where supports_otaa = false;

update device_profile
  set class_b_params = json_object(
    'timeout', class_b_timeout,
    'ping_slot_nb_k', class_b_ping_slot_nb_k,
    'ping_slot_dr', class_b_ping_slot_dr,
    'ping_slot_freq', class_b_ping_slot_freq)
  where supports_class_b = false;

alter table device_profile drop column abp_rx1_delay;
alter table device_profile drop column abp_rx1_dr_offset;
alter table device_profile drop column abp_rx2_dr;
alter table device_profile drop column abp_rx2_freq;

alter table device_profile drop column class_b_timeout;
alter table device_profile drop column class_b_ping_slot_nb_k;
alter table device_profile drop column class_b_ping_slot_dr;
alter table device_profile drop column class_b_ping_slot_freq;

