alter table multicast_group
  rename column class_b_ping_slot_period to class_b_ping_slot_nb_k;

--TODO
--update multicast_group set class_b_ping_slot_nb_k = coalesce(log(2, nullif(class_b_ping_slot_nb_k, 0) / 32), 0);
