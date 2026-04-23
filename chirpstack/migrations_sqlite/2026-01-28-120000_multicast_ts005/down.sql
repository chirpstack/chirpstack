drop index if exists idx_multicast_group_device_dev_eui_mc_group_id;

alter table multicast_group_device
    drop column pending_delete;

alter table multicast_group_device
    drop column error_msg;

alter table multicast_group_device
    drop column mc_session_completed_at;

alter table multicast_group_device
    drop column mc_group_setup_completed_at;

alter table multicast_group_device
    drop column mc_group_id;

alter table multicast_group
    drop column mc_key;
