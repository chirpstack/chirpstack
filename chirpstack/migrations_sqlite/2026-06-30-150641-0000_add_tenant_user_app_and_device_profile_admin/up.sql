create table tenant_user_application (
  user_id text not null references "user" on delete cascade,
  application_id text not null references application on delete cascade,
  created_at datetime not null,
  is_read_only boolean not null,
  primary key (user_id, application_id)
);

create table tenant_user_device_profile (
  user_id text not null references "user" on delete cascade,
  device_profile_id text not null references device_profile on delete cascade,
  created_at datetime not null,
  primary key (user_id, device_profile_id)
);
