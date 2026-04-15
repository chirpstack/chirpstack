alter table multicast_group
    add column mc_key blob;

alter table multicast_group_device
    add column mc_group_id smallint;

alter table multicast_group_device
    add column mc_group_setup_completed_at datetime;

alter table multicast_group_device
    add column mc_session_completed_at datetime;

alter table multicast_group_device
    add column error_msg text not null default '';

alter table multicast_group_device
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
        mgd.rowid as mgd_rowid,
        cast(
            row_number() over (
                partition by mgd.dev_eui
                order by fd.started_at, mgd.created_at, mgd.multicast_group_id
            ) - 1 as integer
        ) as mc_group_id
    from multicast_group_device mgd
    join fuota_deployment fd
        on fd.id = mgd.multicast_group_id
    where fd.started_at is not null
      and fd.completed_at is null
)
update multicast_group_device
set mc_group_id = (
    select afg.mc_group_id
    from active_fuota_groups afg
    where afg.mgd_rowid = multicast_group_device.rowid
)
where rowid in (
    select mgd_rowid
    from active_fuota_groups
);

create unique index idx_multicast_group_device_dev_eui_mc_group_id
    on multicast_group_device (dev_eui, mc_group_id);
