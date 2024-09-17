alter table device_queue_item
  drop column expires_at;

alter table multicast_group_queue_item
  drop column expires_at;

