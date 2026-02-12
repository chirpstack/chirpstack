alter table device_profile
    add column supported_uplink_data_rates smallint[] not null default '{}';

alter table device_profile
    alter column supported_uplink_data_rates drop default;
