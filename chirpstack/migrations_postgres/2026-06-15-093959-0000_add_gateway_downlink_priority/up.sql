alter table gateway
  add column downlink_priority smallint not null default 10;

alter table gateway
  alter column downlink_priority drop default;
