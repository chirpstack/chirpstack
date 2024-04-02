alter table multicast_group
  rename column class_b_ping_slot_period to class_b_ping_slot_nb_k;

update multicast_group set class_b_ping_slot_nb_k = coalesce(log(2, nullif(class_b_ping_slot_nb_k, 0) / 32), 0);

alter table multicast_group
  alter column class_b_ping_slot_nb_k type smallint;
  
