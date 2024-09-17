alter table multicast_group_queue_item
  add column expires_at timestamp with time zone null;

alter table device_queue_item
  add column expires_at timestamp with time zone null;

