alter table device_profile
    add column auto_detect_measurements boolean not null default true;

alter table device_profile_template
    add column auto_detect_measurements boolean not null default true;

alter table device_profile
    alter column auto_detect_measurements drop default;

alter table device_profile_template
    alter column auto_detect_measurements drop default;
