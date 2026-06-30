create table tenant_user_application (
  user_id uuid not null references "user" on delete cascade,
  application_id uuid not null references application on delete cascade,
  created_at timestamp with time zone not null,
  is_read_only boolean not null,
  primary key (user_id, application_id)
);

create table tenant_user_device_profile (
  user_id uuid not null references "user" on delete cascade,
  device_profile_id uuid not null references device_profile on delete cascade,
  created_at timestamp with time zone not null,
  primary key (user_id, device_profile_id)
);
