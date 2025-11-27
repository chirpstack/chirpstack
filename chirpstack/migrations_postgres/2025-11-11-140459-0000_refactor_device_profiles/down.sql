alter table device_profile
    drop column firmware_version,
    drop column vendor_profile_id,
    drop column device_id;

delete from device_profile where tenant_id is null;

alter table device_profile
    alter column tenant_id set not null;

drop table device_profile_device;

drop table device_profile_vendor;
