table! {
    api_key (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        name -> Varchar,
        is_admin -> Bool,
        tenant_id -> Nullable<Uuid>,
    }
}

table! {
    application (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        description -> Text,
        mqtt_tls_cert -> Nullable<Bytea>,
    }
}

table! {
    application_integration (application_id, kind) {
        application_id -> Uuid,
        kind -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        configuration -> Jsonb,
    }
}

table! {
    device (dev_eui) {
        dev_eui -> Bytea,
        application_id -> Uuid,
        device_profile_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_seen_at -> Nullable<Timestamptz>,
        scheduler_run_after -> Nullable<Timestamptz>,
        name -> Varchar,
        description -> Text,
        external_power_source -> Bool,
        battery_level -> Nullable<Numeric>,
        margin -> Nullable<Int4>,
        dr -> Nullable<Int2>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        altitude -> Nullable<Float4>,
        dev_addr -> Nullable<Bytea>,
        enabled_class -> Bpchar,
        skip_fcnt_check -> Bool,
        is_disabled -> Bool,
        tags -> Jsonb,
        variables -> Jsonb,
    }
}

table! {
    device_keys (dev_eui) {
        dev_eui -> Bytea,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        nwk_key -> Bytea,
        app_key -> Bytea,
        dev_nonces -> Array<Int4>,
        join_nonce -> Int4,
    }
}

table! {
    device_profile (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        region -> Varchar,
        mac_version -> Varchar,
        reg_params_revision -> Varchar,
        adr_algorithm_id -> Varchar,
        payload_codec_runtime -> Varchar,
        uplink_interval -> Int4,
        device_status_req_interval -> Int4,
        supports_otaa -> Bool,
        supports_class_b -> Bool,
        supports_class_c -> Bool,
        class_b_timeout -> Int4,
        class_b_ping_slot_period -> Int4,
        class_b_ping_slot_dr -> Int4,
        class_b_ping_slot_freq -> Int8,
        class_c_timeout -> Int4,
        abp_rx1_delay -> Int2,
        abp_rx1_dr_offset -> Int2,
        abp_rx2_dr -> Int2,
        abp_rx2_freq -> Int8,
        tags -> Jsonb,
        payload_codec_script -> Text,
        flush_queue_on_activate -> Bool,
    }
}

table! {
    device_queue_item (id) {
        id -> Uuid,
        dev_eui -> Bytea,
        created_at -> Timestamptz,
        f_port -> Int2,
        confirmed -> Bool,
        data -> Bytea,
        is_pending -> Bool,
        f_cnt_down -> Nullable<Int8>,
        timeout_after -> Nullable<Timestamptz>,
    }
}

table! {
    gateway (gateway_id) {
        gateway_id -> Bytea,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_seen_at -> Nullable<Timestamptz>,
        name -> Varchar,
        description -> Text,
        latitude -> Float8,
        longitude -> Float8,
        altitude -> Float4,
        stats_interval_secs -> Int4,
        tls_certificate -> Nullable<Bytea>,
        tags -> Jsonb,
        properties -> Jsonb,
    }
}

table! {
    multicast_group (id) {
        id -> Uuid,
        application_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        region -> Varchar,
        mc_addr -> Bytea,
        mc_nwk_s_key -> Bytea,
        mc_app_s_key -> Bytea,
        f_cnt -> Int8,
        group_type -> Bpchar,
        dr -> Int2,
        frequency -> Int8,
        class_b_ping_slot_period -> Int4,
    }
}

table! {
    multicast_group_device (multicast_group_id, dev_eui) {
        multicast_group_id -> Uuid,
        dev_eui -> Bytea,
        created_at -> Timestamptz,
    }
}

table! {
    multicast_group_queue_item (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        scheduler_run_after -> Timestamptz,
        multicast_group_id -> Uuid,
        gateway_id -> Bytea,
        f_cnt -> Int8,
        f_port -> Int2,
        data -> Bytea,
        emit_at_time_since_gps_epoch -> Nullable<Int8>,
    }
}

table! {
    tenant (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        description -> Text,
        can_have_gateways -> Bool,
        max_device_count -> Int4,
        max_gateway_count -> Int4,
        private_gateways -> Bool,
    }
}

table! {
    tenant_user (tenant_id, user_id) {
        tenant_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_admin -> Bool,
        is_device_admin -> Bool,
        is_gateway_admin -> Bool,
    }
}

table! {
    user (id) {
        id -> Uuid,
        external_id -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_admin -> Bool,
        is_active -> Bool,
        email -> Text,
        email_verified -> Bool,
        password_hash -> Varchar,
        note -> Text,
    }
}

joinable!(api_key -> tenant (tenant_id));
joinable!(application -> tenant (tenant_id));
joinable!(application_integration -> application (application_id));
joinable!(device -> application (application_id));
joinable!(device -> device_profile (device_profile_id));
joinable!(device_keys -> device (dev_eui));
joinable!(device_profile -> tenant (tenant_id));
joinable!(device_queue_item -> device (dev_eui));
joinable!(gateway -> tenant (tenant_id));
joinable!(multicast_group -> application (application_id));
joinable!(multicast_group_device -> device (dev_eui));
joinable!(multicast_group_device -> multicast_group (multicast_group_id));
joinable!(multicast_group_queue_item -> gateway (gateway_id));
joinable!(multicast_group_queue_item -> multicast_group (multicast_group_id));
joinable!(tenant_user -> tenant (tenant_id));
joinable!(tenant_user -> user (user_id));

allow_tables_to_appear_in_same_query!(
    api_key,
    application,
    application_integration,
    device,
    device_keys,
    device_profile,
    device_queue_item,
    gateway,
    multicast_group,
    multicast_group_device,
    multicast_group_queue_item,
    tenant,
    tenant_user,
    user,
);
