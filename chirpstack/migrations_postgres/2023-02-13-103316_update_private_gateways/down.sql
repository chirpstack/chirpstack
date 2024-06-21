alter table tenant
    drop column private_gateways_down;

alter table tenant
    rename column private_gateways_up to private_gateways;
