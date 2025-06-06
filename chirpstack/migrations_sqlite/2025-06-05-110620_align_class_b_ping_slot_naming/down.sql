alter table fuota_deployment
    rename column multicast_class_b_ping_slot_periodicity to multicast_class_b_ping_slot_nb_k;

alter table multicast_group
    rename column class_b_ping_slot_periodicity to class_b_ping_slot_nb_k;

alter table device_profile_template
    rename column class_b_ping_slot_periodicity to class_b_ping_slot_nb_k;
