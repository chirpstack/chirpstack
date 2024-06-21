alter table device_queue_item
    add column is_encrypted boolean default false not null;

alter table device_queue_item
    alter column is_encrypted drop default;
