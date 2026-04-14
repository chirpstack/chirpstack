alter table multicast_group
    add column mc_key bytea;

alter table multicast_group_device
    add column mc_group_id smallint,
    add column mc_group_setup_completed_at timestamp with time zone,
    add column mc_session_completed_at timestamp with time zone,
    add column error_msg text not null default '',
    add column pending_delete boolean not null default false;

create unique index idx_multicast_group_device_dev_eui_mc_group_id
    on multicast_group_device (dev_eui, mc_group_id);
