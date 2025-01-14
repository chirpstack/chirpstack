alter table device_profile add column abp_rx1_delay smallint not null default 0;
alter table device_profile add column abp_rx1_dr_offset smallint not null default 0;
alter table device_profile add column abp_rx2_dr smallint not null default 0;
alter table device_profile add column abp_rx2_freq bigint not null default 0;

update device_profile
  set
    abp_rx1_delay = abp_params->'rx1_delay',
    abp_rx1_dr_offset = abp_params->'rx1_dr_offset',
    abp_rx2_dr = abp_params->'rx2_dr',
    abp_rx2_freq = abp_params->'rx2_freq'
  where
    abp_params is not null;

alter table device_profile drop column abp_params;

