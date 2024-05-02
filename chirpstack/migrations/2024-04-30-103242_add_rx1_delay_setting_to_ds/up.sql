alter table device_profile
  add column rx1_delay smallint not null default 0;

alter table device_profile
  alter column rx1_delay drop default;
