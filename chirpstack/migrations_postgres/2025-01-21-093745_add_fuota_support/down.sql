alter table device
    drop column app_layer_params;

alter table device_keys
    drop column gen_app_key;

drop table fuota_deployment_job;
drop table fuota_deployment_gateway;
drop table fuota_deployment_device;
drop table fuota_deployment;
