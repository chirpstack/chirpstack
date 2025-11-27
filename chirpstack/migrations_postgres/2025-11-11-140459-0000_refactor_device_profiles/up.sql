create table device_profile_vendor (
    id uuid primary key,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null,
    name text not null,
    vendor_id integer not null,
    ouis text[] not null,
    metadata jsonb not null
);

create index idx_device_profile_vendor_ouis on device_profile_vendor(ouis);
create index idx_device_profile_vendor_vendor_id on device_profile_vendor(vendor_id);

create table device_profile_device (
    id uuid primary key,
    vendor_id uuid not null references device_profile_vendor on delete cascade,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null,
    name varchar(100) not null,
    description text not null,
    metadata jsonb not null
);

create index idx_device_profile_device_vendor_id on device_profile_device(vendor_id);

alter table device_profile
    alter column tenant_id drop not null,
    add column device_id uuid references device_profile_device on delete cascade,
    add column firmware_version varchar(20) not null default '',
    add column vendor_profile_id integer not null default 0;

alter table device_profile
    alter column firmware_version drop default,
    alter column vendor_profile_id drop default;

create index idx_device_profile_device_id on device_profile(device_id);
