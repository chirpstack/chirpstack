alter table tenant
    add column tags jsonb not null default '{}';

alter table tenant
    alter column tags drop default;

create index idx_tenant_tags on tenant using gin (tags);

alter table application
    add column tags jsonb not null default '{}';

alter table application
    alter column tags drop default;

create index idx_application_tags on application using gin (tags);