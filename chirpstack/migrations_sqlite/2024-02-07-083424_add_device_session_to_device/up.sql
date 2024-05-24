alter table device add column secondary_dev_addr blob;
alter table device add column device_session blob;

create index idx_device_dev_addr on device (dev_addr);
create index idx_device_secondary_dev_addr on device (secondary_dev_addr);
