alter table tenant
  add column dev_addr_prefixes text not null default '[]';
