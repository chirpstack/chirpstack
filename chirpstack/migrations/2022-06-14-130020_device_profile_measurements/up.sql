alter table device_profile
    add column measurements jsonb not null default '{}';

alter table device_profile_template
    add column measurements jsonb not null default '{}';

alter table device_profile
    alter column measurements drop default;

alter table device_profile_template
    alter column measurements drop default;
