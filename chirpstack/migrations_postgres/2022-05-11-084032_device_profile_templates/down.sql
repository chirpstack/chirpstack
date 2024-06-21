alter table device_profile
    alter column class_b_ping_slot_dr type integer,
    drop column description;

drop table device_profile_template;
