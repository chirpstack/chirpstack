alter table device
    drop column join_eui;

drop table relay_device;

alter table device_profile drop column is_relay;
alter table device_profile drop column is_relay_ed;
alter table device_profile drop column relay_ed_relay_only;
alter table device_profile drop column relay_enabled;
alter table device_profile drop column relay_cad_periodicity;
alter table device_profile drop column relay_default_channel_index;
alter table device_profile drop column relay_second_channel_freq;
alter table device_profile drop column relay_second_channel_dr;
alter table device_profile drop column relay_second_channel_ack_offset;
alter table device_profile drop column relay_ed_activation_mode;
alter table device_profile drop column relay_ed_smart_enable_level;
alter table device_profile drop column relay_ed_back_off;
alter table device_profile drop column relay_ed_uplink_limit_bucket_size;
alter table device_profile drop column relay_ed_uplink_limit_reload_rate;
alter table device_profile drop column relay_join_req_limit_reload_rate;
alter table device_profile drop column relay_notify_limit_reload_rate;
alter table device_profile drop column relay_global_uplink_limit_reload_rate;
alter table device_profile drop column relay_overall_limit_reload_rate;
alter table device_profile drop column relay_join_req_limit_bucket_size;
alter table device_profile drop column relay_notify_limit_bucket_size;
alter table device_profile drop column relay_global_uplink_limit_bucket_size;
alter table device_profile drop column relay_overall_limit_bucket_size;
