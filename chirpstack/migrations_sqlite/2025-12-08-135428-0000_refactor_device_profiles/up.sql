create table device_profile_vendor (
  id text not null primary key,
  created_at datetime not null,
  updated_at datetime not null,
  name text not null,
  vendor_id integer not null,
  ouis text not null,
  metadata text not null
);

create index idx_device_profile_vendor_name_trgm on device_profile_vendor (name);
create index idx_device_profile_vendor_ouis on device_profile_vendor (ouis);
create index idx_device_profile_vendor_vendor_id on device_profile_vendor (vendor_id);

create table device_profile_device (
  id text not null primary key,
  vendor_id text not null references device_profile_vendor on delete cascade,
  created_at datetime not null,
  updated_at datetime not null,
  name varchar(100) not null,
  description text not null,
  metadata text not null
);

create index idx_device_profile_device_vendor_id on device_profile_device (vendor_id);
create index idx_device_profile_device_name_trgm on device_profile_device (name);


-- We can not alter the tenant_id column to drop the 'not null'.
-- Therefore, we create a new table where tenant_id is nullable and then copy 
-- over the data.
pragma foreign_keys = off;
create table device_profile_new (
  id text not null primary key,
  tenant_id text null references tenant on delete cascade,
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

alter table device_profile
  add column device_id text null references device_profile_device on delete cascade;
alter table device_profile
  add column firmware_version varchar(20) not null default '';
alter table device_profile
  add column vendor_profile_id integer not null default 0;

create index idx_device_profile_device_id on device_profile (device_id);
