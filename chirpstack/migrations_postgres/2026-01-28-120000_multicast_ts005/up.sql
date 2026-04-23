alter table multicast_group
    add column mc_key bytea;

alter table multicast_group_device
    add column mc_group_id smallint,
    add column mc_group_setup_completed_at timestamp with time zone,
    add column mc_session_completed_at timestamp with time zone,
    add column error_msg text not null default '',
    add column pending_delete boolean not null default false;

create temporary table _multicast_ts005_guard (
    slot_count integer not null check (slot_count <= 4)
);

insert into _multicast_ts005_guard (slot_count)
select count(*)
from multicast_group_device mgd
join fuota_deployment fd
    on fd.id = mgd.multicast_group_id
where fd.started_at is not null
  and fd.completed_at is null
group by mgd.dev_eui;

drop table _multicast_ts005_guard;

with active_fuota_groups as (
    select
        mgd.multicast_group_id,
        mgd.dev_eui,
        cast(
            row_number() over (
                partition by mgd.dev_eui
                order by fd.started_at, mgd.created_at, mgd.multicast_group_id
            ) - 1 as smallint
        ) as mc_group_id
    from multicast_group_device mgd
    join fuota_deployment fd
        on fd.id = mgd.multicast_group_id
    where fd.started_at is not null
      and fd.completed_at is null
)
update multicast_group_device mgd
set mc_group_id = afg.mc_group_id
from active_fuota_groups afg
where mgd.multicast_group_id = afg.multicast_group_id
  and mgd.dev_eui = afg.dev_eui;

create unique index idx_multicast_group_device_dev_eui_mc_group_id
    on multicast_group_device (dev_eui, mc_group_id);
