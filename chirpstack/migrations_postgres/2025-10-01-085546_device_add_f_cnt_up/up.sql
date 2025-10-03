alter table device
    add column f_cnt_up bigint not null default 0;

alter table device
    alter column f_cnt_up drop default;
