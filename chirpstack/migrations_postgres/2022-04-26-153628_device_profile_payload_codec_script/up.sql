alter table device_profile
    drop column payload_decoder_config,
    drop column payload_encoder_config,
    add column payload_codec_script text not null default '';

alter table device_profile
    alter column payload_codec_script drop default;
