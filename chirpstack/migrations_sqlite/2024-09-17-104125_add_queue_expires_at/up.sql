alter table multicast_group_queue_item
  add column expires_at datetime null;

alter table device_queue_item
  add column expires_at datetime null;
