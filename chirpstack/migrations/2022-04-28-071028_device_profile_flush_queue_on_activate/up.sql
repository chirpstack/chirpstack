alter table device_profile
    add column flush_queue_on_activate boolean not null default false;

alter table device_profile
    alter column flush_queue_on_activate drop default;
