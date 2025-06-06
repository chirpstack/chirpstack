alter table device_profile_template
    rename column class_b_ping_slot_nb_k to class_b_ping_slot_periodicity;

alter table multicast_group
    rename column class_b_ping_slot_nb_k to class_b_ping_slot_periodicity;

alter table fuota_deployment
    rename column multicast_class_b_ping_slot_nb_k to multicast_class_b_ping_slot_periodicity;
