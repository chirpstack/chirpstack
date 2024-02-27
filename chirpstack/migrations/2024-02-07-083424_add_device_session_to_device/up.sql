alter table device
    add column secondary_dev_addr bytea,
    add column device_session bytea;

create index idx_device_dev_addr on device (dev_addr);
create index idx_device_secondary_dev_addr on device (secondary_dev_addr);
