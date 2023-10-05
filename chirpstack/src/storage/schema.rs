// @generated automatically by Diesel CLI.

diesel::table! {
    api_key (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        is_admin -> Bool,
        tenant_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    application (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        mqtt_tls_cert -> Nullable<Bytea>,
    }
}

diesel::table! {
    application_integration (application_id, kind) {
        application_id -> Uuid,
        #[max_length = 20]
        kind -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        configuration -> Jsonb,
    }
}

diesel::table! {
    device (dev_eui) {
        dev_eui -> Bytea,
        application_id -> Uuid,
        device_profile_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_seen_at -> Nullable<Timestamptz>,
        scheduler_run_after -> Nullable<Timestamptz>,
        #[max_length = 100]
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
        #[max_length = 1]
        enabled_class -> Bpchar,
        skip_fcnt_check -> Bool,
        is_disabled -> Bool,
        tags -> Jsonb,
        variables -> Jsonb,
        join_eui -> Bytea,
    }
}

diesel::table! {
    device_keys (dev_eui) {
        dev_eui -> Bytea,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        nwk_key -> Bytea,
        app_key -> Bytea,
        dev_nonces -> Array<Nullable<Int4>>,
        join_nonce -> Int4,
    }
}

diesel::table! {
    device_profile (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 10]
        region -> Varchar,
        #[max_length = 10]
        mac_version -> Varchar,
        #[max_length = 20]
        reg_params_revision -> Varchar,
        #[max_length = 100]
        adr_algorithm_id -> Varchar,
        #[max_length = 20]
        payload_codec_runtime -> Varchar,
        uplink_interval -> Int4,
        device_status_req_interval -> Int4,
        supports_otaa -> Bool,
        supports_class_b -> Bool,
        supports_class_c -> Bool,
        class_b_timeout -> Int4,
        class_b_ping_slot_nb_k -> Int4,
        class_b_ping_slot_dr -> Int2,
        class_b_ping_slot_freq -> Int8,
        class_c_timeout -> Int4,
        abp_rx1_delay -> Int2,
        abp_rx1_dr_offset -> Int2,
        abp_rx2_dr -> Int2,
        abp_rx2_freq -> Int8,
        tags -> Jsonb,
        payload_codec_script -> Text,
        flush_queue_on_activate -> Bool,
        description -> Text,
        measurements -> Jsonb,
        auto_detect_measurements -> Bool,
        #[max_length = 100]
        region_config_id -> Nullable<Varchar>,
        is_relay -> Bool,
        is_relay_ed -> Bool,
        relay_ed_relay_only -> Bool,
        relay_enabled -> Bool,
        relay_cad_periodicity -> Int2,
        relay_default_channel_index -> Int2,
        relay_second_channel_freq -> Int8,
        relay_second_channel_dr -> Int2,
        relay_second_channel_ack_offset -> Int2,
        relay_ed_activation_mode -> Int2,
        relay_ed_smart_enable_level -> Int2,
        relay_ed_back_off -> Int2,
        relay_ed_uplink_limit_bucket_size -> Int2,
        relay_ed_uplink_limit_reload_rate -> Int2,
        relay_join_req_limit_reload_rate -> Int2,
        relay_notify_limit_reload_rate -> Int2,
        relay_global_uplink_limit_reload_rate -> Int2,
        relay_overall_limit_reload_rate -> Int2,
        relay_join_req_limit_bucket_size -> Int2,
        relay_notify_limit_bucket_size -> Int2,
        relay_global_uplink_limit_bucket_size -> Int2,
        relay_overall_limit_bucket_size -> Int2,
    }
}

diesel::table! {
    device_profile_template (id) {
        id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        #[max_length = 100]
        vendor -> Varchar,
        #[max_length = 100]
        firmware -> Varchar,
        #[max_length = 10]
        region -> Varchar,
        #[max_length = 10]
        mac_version -> Varchar,
        #[max_length = 20]
        reg_params_revision -> Varchar,
        #[max_length = 100]
        adr_algorithm_id -> Varchar,
        #[max_length = 20]
        payload_codec_runtime -> Varchar,
        payload_codec_script -> Text,
        uplink_interval -> Int4,
        device_status_req_interval -> Int4,
        flush_queue_on_activate -> Bool,
        supports_otaa -> Bool,
        supports_class_b -> Bool,
        supports_class_c -> Bool,
        class_b_timeout -> Int4,
        class_b_ping_slot_nb_k -> Int4,
        class_b_ping_slot_dr -> Int2,
        class_b_ping_slot_freq -> Int8,
        class_c_timeout -> Int4,
        abp_rx1_delay -> Int2,
        abp_rx1_dr_offset -> Int2,
        abp_rx2_dr -> Int2,
        abp_rx2_freq -> Int8,
        tags -> Jsonb,
        measurements -> Jsonb,
        auto_detect_measurements -> Bool,
    }
}

diesel::table! {
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
        is_encrypted -> Bool,
    }
}

diesel::table! {
    gateway (gateway_id) {
        gateway_id -> Bytea,
        tenant_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_seen_at -> Nullable<Timestamptz>,
        #[max_length = 100]
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

diesel::table! {
    multicast_group (id) {
        id -> Uuid,
        application_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 10]
        region -> Varchar,
        mc_addr -> Bytea,
        mc_nwk_s_key -> Bytea,
        mc_app_s_key -> Bytea,
        f_cnt -> Int8,
        #[max_length = 1]
        group_type -> Bpchar,
        dr -> Int2,
        frequency -> Int8,
        class_b_ping_slot_period -> Int4,
        #[max_length = 20]
        class_c_scheduling_type -> Varchar,
    }
}

diesel::table! {
    multicast_group_device (multicast_group_id, dev_eui) {
        multicast_group_id -> Uuid,
        dev_eui -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    multicast_group_gateway (multicast_group_id, gateway_id) {
        multicast_group_id -> Uuid,
        gateway_id -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
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

diesel::table! {
    relay_device (relay_dev_eui, dev_eui) {
        relay_dev_eui -> Bytea,
        dev_eui -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    tenant (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        can_have_gateways -> Bool,
        max_device_count -> Int4,
        max_gateway_count -> Int4,
        private_gateways_up -> Bool,
        private_gateways_down -> Bool,
    }
}

diesel::table! {
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

diesel::table! {
    user (id) {
        id -> Uuid,
        external_id -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_admin -> Bool,
        is_active -> Bool,
        email -> Text,
        email_verified -> Bool,
        #[max_length = 200]
        password_hash -> Varchar,
        note -> Text,
    }
}

diesel::joinable!(api_key -> tenant (tenant_id));
diesel::joinable!(application -> tenant (tenant_id));
diesel::joinable!(application_integration -> application (application_id));
diesel::joinable!(device -> application (application_id));
diesel::joinable!(device -> device_profile (device_profile_id));
diesel::joinable!(device_keys -> device (dev_eui));
diesel::joinable!(device_profile -> tenant (tenant_id));
diesel::joinable!(device_queue_item -> device (dev_eui));
diesel::joinable!(gateway -> tenant (tenant_id));
diesel::joinable!(multicast_group -> application (application_id));
diesel::joinable!(multicast_group_device -> device (dev_eui));
diesel::joinable!(multicast_group_device -> multicast_group (multicast_group_id));
diesel::joinable!(multicast_group_gateway -> gateway (gateway_id));
diesel::joinable!(multicast_group_gateway -> multicast_group (multicast_group_id));
diesel::joinable!(multicast_group_queue_item -> gateway (gateway_id));
diesel::joinable!(multicast_group_queue_item -> multicast_group (multicast_group_id));
diesel::joinable!(tenant_user -> tenant (tenant_id));
diesel::joinable!(tenant_user -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_key,
    application,
    application_integration,
    device,
    device_keys,
    device_profile,
    device_profile_template,
    device_queue_item,
    gateway,
    multicast_group,
    multicast_group_device,
    multicast_group_gateway,
    multicast_group_queue_item,
    relay_device,
    tenant,
    tenant_user,
    user,
);
