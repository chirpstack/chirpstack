// @generated automatically by Diesel CLI.

diesel::table! {
    api_key (id) {
        id -> Text,
        created_at -> TimestamptzSqlite,
        name -> Text,
        is_admin -> Bool,
        tenant_id -> Nullable<Text>,
    }
}

diesel::table! {
    application (id) {
        id -> Text,
        tenant_id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        name -> Text,
        description -> Text,
        mqtt_tls_cert -> Nullable<Binary>,
        tags -> Text,
    }
}

diesel::table! {
    application_integration (application_id, kind) {
        application_id -> Text,
        kind -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        configuration -> Text,
    }
}

diesel::table! {
    device (dev_eui) {
        dev_eui -> Binary,
        application_id -> Text,
        device_profile_id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        last_seen_at -> Nullable<TimestamptzSqlite>,
        scheduler_run_after -> Nullable<TimestamptzSqlite>,
        name -> Text,
        description -> Text,
        external_power_source -> Bool,
        battery_level -> Nullable<Double>,
        margin -> Nullable<Integer>,
        dr -> Nullable<SmallInt>,
        latitude -> Nullable<Double>,
        longitude -> Nullable<Double>,
        altitude -> Nullable<Float>,
        dev_addr -> Nullable<Binary>,
        enabled_class -> Text,
        skip_fcnt_check -> Bool,
        is_disabled -> Bool,
        tags -> Text,
        variables -> Text,
        join_eui -> Binary,
        secondary_dev_addr -> Nullable<Binary>,
        device_session -> Nullable<Binary>,
    }
}

diesel::table! {
    device_keys (dev_eui) {
        dev_eui -> Binary,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        nwk_key -> Binary,
        app_key -> Binary,
        dev_nonces -> Text,
        join_nonce -> Integer,
    }
}

diesel::table! {
    device_profile (id) {
        id -> Text,
        tenant_id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        name -> Text,
        region -> Text,
        mac_version -> Text,
        reg_params_revision -> Text,
        adr_algorithm_id -> Text,
        payload_codec_runtime -> Text,
        uplink_interval -> Integer,
        device_status_req_interval -> Integer,
        supports_otaa -> Bool,
        supports_class_b -> Bool,
        supports_class_c -> Bool,
        class_b_timeout -> Integer,
        class_b_ping_slot_nb_k -> Integer,
        class_b_ping_slot_dr -> SmallInt,
        class_b_ping_slot_freq -> BigInt,
        class_c_timeout -> Integer,
        abp_rx1_delay -> SmallInt,
        abp_rx1_dr_offset -> SmallInt,
        abp_rx2_dr -> SmallInt,
        abp_rx2_freq -> BigInt,
        tags -> Text,
        payload_codec_script -> Text,
        flush_queue_on_activate -> Bool,
        description -> Text,
        measurements -> Text,
        auto_detect_measurements -> Bool,
        region_config_id -> Nullable<Text>,
        is_relay -> Bool,
        is_relay_ed -> Bool,
        relay_ed_relay_only -> Bool,
        relay_enabled -> Bool,
        relay_cad_periodicity -> SmallInt,
        relay_default_channel_index -> SmallInt,
        relay_second_channel_freq -> BigInt,
        relay_second_channel_dr -> SmallInt,
        relay_second_channel_ack_offset -> SmallInt,
        relay_ed_activation_mode -> SmallInt,
        relay_ed_smart_enable_level -> SmallInt,
        relay_ed_back_off -> SmallInt,
        relay_ed_uplink_limit_bucket_size -> SmallInt,
        relay_ed_uplink_limit_reload_rate -> SmallInt,
        relay_join_req_limit_reload_rate -> SmallInt,
        relay_notify_limit_reload_rate -> SmallInt,
        relay_global_uplink_limit_reload_rate -> SmallInt,
        relay_overall_limit_reload_rate -> SmallInt,
        relay_join_req_limit_bucket_size -> SmallInt,
        relay_notify_limit_bucket_size -> SmallInt,
        relay_global_uplink_limit_bucket_size -> SmallInt,
        relay_overall_limit_bucket_size -> SmallInt,
        allow_roaming -> Bool,
        rx1_delay -> SmallInt,
    }
}

diesel::table! {
    device_profile_template (id) {
        id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        name -> Text,
        description -> Text,
        vendor -> Text,
        firmware -> Text,
        region -> Text,
        mac_version -> Text,
        reg_params_revision -> Text,
        adr_algorithm_id -> Text,
        payload_codec_runtime -> Text,
        payload_codec_script -> Text,
        uplink_interval -> Integer,
        device_status_req_interval -> Integer,
        flush_queue_on_activate -> Bool,
        supports_otaa -> Bool,
        supports_class_b -> Bool,
        supports_class_c -> Bool,
        class_b_timeout -> Integer,
        class_b_ping_slot_nb_k -> Integer,
        class_b_ping_slot_dr -> SmallInt,
        class_b_ping_slot_freq -> BigInt,
        class_c_timeout -> Integer,
        abp_rx1_delay -> SmallInt,
        abp_rx1_dr_offset -> SmallInt,
        abp_rx2_dr -> SmallInt,
        abp_rx2_freq -> BigInt,
        tags -> Text,
        measurements -> Text,
        auto_detect_measurements -> Bool,
    }
}

diesel::table! {
    device_queue_item (id) {
        id -> Text,
        dev_eui -> Binary,
        created_at -> TimestamptzSqlite,
        f_port -> SmallInt,
        confirmed -> Bool,
        data -> Binary,
        is_pending -> Bool,
        f_cnt_down -> Nullable<BigInt>,
        timeout_after -> Nullable<TimestamptzSqlite>,
        is_encrypted -> Bool,
    }
}

diesel::table! {
    gateway (gateway_id) {
        gateway_id -> Binary,
        tenant_id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        last_seen_at -> Nullable<TimestamptzSqlite>,
        name -> Text,
        description -> Text,
        latitude -> Double,
        longitude -> Double,
        altitude -> Float,
        stats_interval_secs -> Integer,
        tls_certificate -> Nullable<Binary>,
        tags -> Text,
        properties -> Text,
    }
}

diesel::table! {
    multicast_group (id) {
        id -> Text,
        application_id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        name -> Text,
        region -> Text,
        mc_addr -> Binary,
        mc_nwk_s_key -> Binary,
        mc_app_s_key -> Binary,
        f_cnt -> BigInt,
        group_type -> Text,
        dr -> SmallInt,
        frequency -> BigInt,
        class_b_ping_slot_nb_k -> SmallInt,
        class_c_scheduling_type -> Text,
    }
}

diesel::table! {
    multicast_group_device (multicast_group_id, dev_eui) {
        multicast_group_id -> Text,
        dev_eui -> Binary,
        created_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    multicast_group_gateway (multicast_group_id, gateway_id) {
        multicast_group_id -> Text,
        gateway_id -> Binary,
        created_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    multicast_group_queue_item (id) {
        id -> Text,
        created_at -> TimestamptzSqlite,
        scheduler_run_after -> TimestamptzSqlite,
        multicast_group_id -> Text,
        gateway_id -> Binary,
        f_cnt -> BigInt,
        f_port -> SmallInt,
        data -> Binary,
        emit_at_time_since_gps_epoch -> Nullable<BigInt>,
    }
}

diesel::table! {
    relay_device (relay_dev_eui, dev_eui) {
        relay_dev_eui -> Binary,
        dev_eui -> Binary,
        created_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    tenant (id) {
        id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        name -> Text,
        description -> Text,
        can_have_gateways -> Bool,
        max_device_count -> Integer,
        max_gateway_count -> Integer,
        private_gateways_up -> Bool,
        private_gateways_down -> Bool,
        tags -> Text,
    }
}

diesel::table! {
    tenant_user (tenant_id, user_id) {
        tenant_id -> Text,
        user_id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        is_admin -> Bool,
        is_device_admin -> Bool,
        is_gateway_admin -> Bool,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        external_id -> Nullable<Text>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        is_admin -> Bool,
        is_active -> Bool,
        email -> Text,
        email_verified -> Bool,
        password_hash -> Text,
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
