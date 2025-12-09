drop index idx_device_profile_device_id;

alter table device_profile drop column vendor_profile_id;
alter table device_profile drop column firmware_version;
alter table device_profile drop column device_id;

delete from device_profile where tenant_id is null;

pragma foreign_keys = off;
create table device_profile_new (
  id text not null primary key,
  tenant_id text not null references tenant on delete cascade,
  created_at datetime not null,
  updated_at datetime not null,
  name varchar(100) not null,
  region varchar(10) not null,
  mac_version varchar(10) not null,
  reg_params_revision varchar(20) not null,
  adr_algorithm_id varchar(100) not null,
  payload_codec_runtime varchar(20) not null,
  uplink_interval integer not null,
  device_status_req_interval integer not null,
  supports_otaa boolean not null,
  supports_class_b boolean not null,
  supports_class_c boolean not null,
  tags text not null,
  payload_codec_script text not null,
  flush_queue_on_activate boolean not null,
  description text not null,
  measurements text not null,
  auto_detect_measurements boolean not null,
  region_config_id varchar(100) null,
  allow_roaming boolean not null,
  rx1_delay smallint not null,
  abp_params text null,
  class_b_params text null,
  class_c_params text null,
  relay_params text null,
  app_layer_params text not null
);
insert into device_profile_new select * from device_profile;
drop table device_profile;
alter table device_profile_new rename to device_profile;
pragma foreign_keys = on;

drop table device_profile_device;
drop table device_profile_vendor;
