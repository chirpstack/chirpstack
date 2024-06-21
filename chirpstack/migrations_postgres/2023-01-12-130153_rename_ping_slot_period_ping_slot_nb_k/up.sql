alter table device_profile
    rename column class_b_ping_slot_period to class_b_ping_slot_nb_k;

alter table device_profile_template
    rename column class_b_ping_slot_period to class_b_ping_slot_nb_k;
