alter table tenant
  add column dev_addr_prefixes text[] not null default '{}';

alter table tenant
  alter column dev_addr_prefixes drop default;

