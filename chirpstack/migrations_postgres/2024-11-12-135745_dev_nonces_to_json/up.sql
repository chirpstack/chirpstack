alter table device_keys
  alter column dev_nonces type jsonb using jsonb_build_object('0000000000000000', dev_nonces);

update device_keys
  set dev_nonces = jsonb_build_object(encode(device.join_eui, 'hex'), dev_nonces->'0000000000000000')
from device
  where device.dev_eui = device_keys.dev_eui;
