alter table device_profile
    add column allow_roaming boolean not null default true;

alter table device_profile
    alter column allow_roaming drop default;
