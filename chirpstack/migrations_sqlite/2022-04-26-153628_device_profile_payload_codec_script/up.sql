alter table device_profile drop column payload_decoder_config;
alter table device_profile drop column payload_encoder_config;
alter table device_profile add column payload_codec_script text not null default '';

-- sqlite requires a complex procedure to remove the default of the column
-- note that this is available for any operation that modifies existing data on disk
-- see "simple procedure" in the second half of this section https://www.sqlite.org/lang_altertable.html#otheralter
-- an alternative could be creating a new column, moving the data in it, dropping the old and renaming the new to old
-- alter table device_profile alter column payload_codec_script drop default;
