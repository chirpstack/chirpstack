alter table device_profile
    add column codec_plugin_id varchar(100) not null default '';

alter table device_profile_template
    add column codec_plugin_id varchar(100) not null default '';