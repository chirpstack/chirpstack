alter table device_queue_item
    add column is_encrypted boolean default false not null;
