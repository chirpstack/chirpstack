alter table tenant
    rename column private_gateways to private_gateways_up;

alter table tenant
    add column private_gateways_down boolean not null default false;

alter table tenant
    alter column private_gateways_down drop default;
