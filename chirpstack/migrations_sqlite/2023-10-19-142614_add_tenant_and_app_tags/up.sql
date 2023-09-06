alter table tenant
    add column tags text not null default '{}';

create index idx_tenant_tags on tenant (tags);

alter table application
    add column tags text not null default '{}';

create index idx_application_tags on application (tags);
