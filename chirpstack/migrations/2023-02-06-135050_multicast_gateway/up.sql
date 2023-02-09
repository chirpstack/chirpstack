create table multicast_group_gateway (
    multicast_group_id uuid not null references multicast_group on delete cascade,
    gateway_id bytea not null references gateway on delete cascade,
    created_at timestamp with time zone not null,
    primary key (multicast_group_id, gateway_id)
);

alter table multicast_group 
    add column class_c_scheduling_type varchar(20) not null default 'DELAY';

alter table multicast_group 
    alter column class_c_scheduling_type drop default;
