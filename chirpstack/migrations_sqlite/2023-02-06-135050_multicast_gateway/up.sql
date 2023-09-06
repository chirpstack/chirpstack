create table multicast_group_gateway (
    multicast_group_id text not null references multicast_group on delete cascade,
    gateway_id blob not null references gateway on delete cascade,
    created_at datetime not null,
    primary key (multicast_group_id, gateway_id)
);

alter table multicast_group 
    add column class_c_scheduling_type varchar(20) not null default 'DELAY';

-- sqlite: no drop default
-- alter table multicast_group 
--     alter column class_c_scheduling_type drop default;
