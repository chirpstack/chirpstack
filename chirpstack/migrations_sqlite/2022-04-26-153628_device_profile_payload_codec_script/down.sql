alter table device_profile drop column payload_codec_script;
alter table device_profile add column payload_decoder_config text not null default '';
alter table device_profile add column payload_encoder_config text not null default '';
