drop index idx_device_dev_addr;
drop index idx_device_secondary_dev_addr;

alter table device drop column secondary_dev_addr;
alter table device drop column device_session;
