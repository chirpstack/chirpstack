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
        app_layer_params -> Text,
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
        gen_app_key -> Binary,
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
        tags -> Text,
        payload_codec_script -> Text,
        flush_queue_on_activate -> Bool,
        description -> Text,
        measurements -> Text,
        auto_detect_measurements -> Bool,
        region_config_id -> Nullable<Text>,
        allow_roaming -> Bool,
        rx1_delay -> SmallInt,
        abp_params -> Nullable<Text>,
        class_b_params -> Nullable<Text>,
        class_c_params -> Nullable<Text>,
        relay_params -> Nullable<Text>,
        app_layer_params -> Text,
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
        class_b_ping_slot_periodicity -> Integer,
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
        expires_at -> Nullable<TimestamptzSqlite>,
    }
}

diesel::table! {
    fuota_deployment (id) {
        id -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        started_at -> Nullable<TimestamptzSqlite>,
        completed_at -> Nullable<TimestamptzSqlite>,
        name -> Text,
        application_id -> Text,
        device_profile_id -> Text,
        multicast_addr -> Binary,
        multicast_key -> Binary,
        multicast_group_type -> Text,
        multicast_class_c_scheduling_type -> Text,
        multicast_dr -> SmallInt,
        multicast_class_b_ping_slot_periodicity -> SmallInt,
        multicast_frequency -> BigInt,
        multicast_timeout -> SmallInt,
        multicast_session_start -> Nullable<TimestamptzSqlite>,
        multicast_session_end -> Nullable<TimestamptzSqlite>,
        unicast_max_retry_count -> SmallInt,
        fragmentation_fragment_size -> SmallInt,
        fragmentation_redundancy_percentage -> SmallInt,
        fragmentation_session_index -> SmallInt,
        fragmentation_matrix -> SmallInt,
        fragmentation_block_ack_delay -> SmallInt,
        fragmentation_descriptor -> Binary,
        request_fragmentation_session_status -> Text,
        payload -> Binary,
        on_complete_set_device_tags -> Text,
    }
}

diesel::table! {
    fuota_deployment_device (fuota_deployment_id, dev_eui) {
        fuota_deployment_id -> Text,
        dev_eui -> Binary,
        created_at -> TimestamptzSqlite,
        completed_at -> Nullable<TimestamptzSqlite>,
        mc_group_setup_completed_at -> Nullable<TimestamptzSqlite>,
        mc_session_completed_at -> Nullable<TimestamptzSqlite>,
        frag_session_setup_completed_at -> Nullable<TimestamptzSqlite>,
        frag_status_completed_at -> Nullable<TimestamptzSqlite>,
        error_msg -> Text,
    }
}

diesel::table! {
    fuota_deployment_gateway (fuota_deployment_id, gateway_id) {
        fuota_deployment_id -> Text,
        gateway_id -> Binary,
        created_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    fuota_deployment_job (fuota_deployment_id, job) {
        fuota_deployment_id -> Text,
        job -> Text,
        created_at -> TimestamptzSqlite,
        completed_at -> Nullable<TimestamptzSqlite>,
        max_retry_count -> SmallInt,
        attempt_count -> SmallInt,
        scheduler_run_after -> TimestamptzSqlite,
        warning_msg -> Text,
        error_msg -> Text,
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
        class_b_ping_slot_periodicity -> SmallInt,
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
        expires_at -> Nullable<TimestamptzSqlite>,
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
    relay_gateway (tenant_id, relay_id) {
        tenant_id -> Text,
        relay_id -> Binary,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        last_seen_at -> Nullable<TimestamptzSqlite>,
        name -> Text,
        description -> Text,
        stats_interval_secs -> Integer,
        region_config_id -> Text,
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
diesel::joinable!(fuota_deployment -> application (application_id));
diesel::joinable!(fuota_deployment -> device_profile (device_profile_id));
diesel::joinable!(fuota_deployment_device -> device (dev_eui));
diesel::joinable!(fuota_deployment_device -> fuota_deployment (fuota_deployment_id));
diesel::joinable!(fuota_deployment_gateway -> fuota_deployment (fuota_deployment_id));
diesel::joinable!(fuota_deployment_gateway -> gateway (gateway_id));
diesel::joinable!(fuota_deployment_job -> fuota_deployment (fuota_deployment_id));
diesel::joinable!(gateway -> tenant (tenant_id));
diesel::joinable!(multicast_group -> application (application_id));
diesel::joinable!(multicast_group_device -> device (dev_eui));
diesel::joinable!(multicast_group_device -> multicast_group (multicast_group_id));
diesel::joinable!(multicast_group_gateway -> gateway (gateway_id));
diesel::joinable!(multicast_group_gateway -> multicast_group (multicast_group_id));
diesel::joinable!(multicast_group_queue_item -> gateway (gateway_id));
diesel::joinable!(multicast_group_queue_item -> multicast_group (multicast_group_id));
diesel::joinable!(relay_gateway -> tenant (tenant_id));
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
    fuota_deployment,
    fuota_deployment_device,
    fuota_deployment_gateway,
    fuota_deployment_job,
    gateway,
    multicast_group,
    multicast_group_device,
    multicast_group_gateway,
    multicast_group_queue_item,
    relay_device,
    relay_gateway,
    tenant,
    tenant_user,
    user,
);
