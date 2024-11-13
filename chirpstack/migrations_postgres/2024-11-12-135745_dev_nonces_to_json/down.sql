alter table device_keys
  alter column dev_nonces type int[] using '{}';
