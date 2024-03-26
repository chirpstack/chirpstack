alter table multicast_group
  alter column class_b_ping_slot_nb_k type integer;

update multicast_group set class_b_ping_slot_nb_k = pow(2, class_b_ping_slot_nb_k) * 32;

alter table multicast_group
  rename column class_b_ping_slot_nb_k to class_b_ping_slot_period;

