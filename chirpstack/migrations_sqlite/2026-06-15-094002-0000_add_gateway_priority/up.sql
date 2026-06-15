alter table gateway
  add column downlink_priority smallint not null default 10;
