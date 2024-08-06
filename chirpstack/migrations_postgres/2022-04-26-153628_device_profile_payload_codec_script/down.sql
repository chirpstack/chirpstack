alter table device_profile
    drop column payload_codec_script,
    add column payload_decoder_config text not null default '',
    add column payload_encoder_config text not null default '';

alter table device_profile
    alter column payload_decoder_config drop default,
    alter column payload_decoder_config drop default;
