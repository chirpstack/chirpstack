alter table multicast_group
  rename column class_b_ping_slot_period to class_b_ping_slot_nb_k;

update multicast_group set class_b_ping_slot_nb_k = log(2, class_b_ping_slot_nb_k / 32);

alter table multicast_group
  alter column class_b_ping_slot_nb_k type smallint;
  
