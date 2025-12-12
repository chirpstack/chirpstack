alter table device_profile
  add column supported_uplink_data_rates text not null default '[]';

