alter table device_profile add column is_relay boolean not null default false;
alter table device_profile add column is_relay_ed boolean not null default false;
alter table device_profile add column relay_ed_relay_only boolean not null default false;
alter table device_profile add column relay_enabled boolean not null default false;
alter table device_profile add column relay_cad_periodicity smallint not null default 0;
alter table device_profile add column relay_default_channel_index smallint not null default 0;
alter table device_profile add column relay_second_channel_freq bigint not null default 0;
alter table device_profile add column relay_second_channel_dr smallint not null default 0;
alter table device_profile add column relay_second_channel_ack_offset smallint not null default 0;
alter table device_profile add column relay_ed_activation_mode smallint not null default 0;
alter table device_profile add column relay_ed_smart_enable_level smallint not null default 0;
alter table device_profile add column relay_ed_back_off smallint not null default 0;
alter table device_profile add column relay_ed_uplink_limit_bucket_size smallint not null default 0;
alter table device_profile add column relay_ed_uplink_limit_reload_rate smallint not null default 0;
alter table device_profile add column relay_join_req_limit_reload_rate smallint not null default 0;
alter table device_profile add column relay_notify_limit_reload_rate smallint not null default 0;
alter table device_profile add column relay_global_uplink_limit_reload_rate smallint not null default 0;
alter table device_profile add column relay_overall_limit_reload_rate smallint not null default 0;
alter table device_profile add column relay_join_req_limit_bucket_size smallint not null default 0;
alter table device_profile add column relay_notify_limit_bucket_size smallint not null default 0;
alter table device_profile add column relay_global_uplink_limit_bucket_size smallint not null default 0;
alter table device_profile add column relay_overall_limit_bucket_size smallint not null default 0;

-- sqlite no drop default
-- NOTE: there is a patern of:
-- 1. add column not null default X
-- 2. column drop default
-- Step 1 is here to "backfill" the DB with a default
-- For sqlite, we could just create the table with the default still...
-- or create the table with the final state as we don't need old migrations
-- alter table device_profile
--     alter column is_relay drop default,
--     alter column is_relay_ed drop default,
--     alter column relay_ed_relay_only drop default,
--     alter column relay_enabled drop default,
--     alter column relay_cad_periodicity drop default,
--     alter column relay_default_channel_index drop default,
--     alter column relay_second_channel_freq drop default,
--     alter column relay_second_channel_dr drop default,
--     alter column relay_second_channel_ack_offset drop default,
--     alter column relay_ed_activation_mode drop default,
--     alter column relay_ed_smart_enable_level drop default,
--     alter column relay_ed_back_off drop default,
--     alter column relay_ed_uplink_limit_bucket_size drop default,
--     alter column relay_ed_uplink_limit_reload_rate drop default,
--     alter column relay_join_req_limit_reload_rate drop default,
--     alter column relay_notify_limit_reload_rate drop default,
--     alter column relay_global_uplink_limit_reload_rate drop default,
--     alter column relay_overall_limit_reload_rate drop default,
--     alter column relay_join_req_limit_bucket_size drop default,
--     alter column relay_notify_limit_bucket_size drop default,
--     alter column relay_global_uplink_limit_bucket_size drop default,
--     alter column relay_overall_limit_bucket_size drop default;

create table relay_device (
    relay_dev_eui blob not null references device on delete cascade,
    dev_eui blob not null references device on delete cascade,
    created_at datetime not null,
    primary key (relay_dev_eui, dev_eui)
);

alter table device
    add column join_eui blob not null default x'00000000';

-- sqlite no drop default
-- alter table device
--     alter column join_eui drop default;
