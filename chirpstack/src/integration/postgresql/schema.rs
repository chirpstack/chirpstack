table! {
    event_ack (queue_item_id) {
        queue_item_id -> Uuid,
        deduplication_id -> Uuid,
        time -> Timestamptz,
        tenant_id -> Uuid,
        tenant_name -> Text,
        application_id -> Uuid,
        application_name -> Text,
        device_profile_id -> Uuid,
        device_profile_name -> Text,
        device_name -> Text,
        dev_eui -> Bpchar,
        tags -> Jsonb,
        acknowledged -> Bool,
        f_cnt_down -> Int8,
    }
}

table! {
    event_integration (deduplication_id) {
        deduplication_id -> Uuid,
        time -> Timestamptz,
        tenant_id -> Uuid,
        tenant_name -> Text,
        application_id -> Uuid,
        application_name -> Text,
        device_profile_id -> Uuid,
        device_profile_name -> Text,
        device_name -> Text,
        dev_eui -> Bpchar,
        tags -> Jsonb,
        integration_name -> Text,
        event_type -> Text,
        object -> Jsonb,
    }
}

table! {
    event_join (deduplication_id) {
        deduplication_id -> Uuid,
        time -> Timestamptz,
        tenant_id -> Uuid,
        tenant_name -> Text,
        application_id -> Uuid,
        application_name -> Text,
        device_profile_id -> Uuid,
        device_profile_name -> Text,
        device_name -> Text,
        dev_eui -> Bpchar,
        tags -> Jsonb,
        dev_addr -> Bpchar,
    }
}

table! {
    event_location (deduplication_id) {
        deduplication_id -> Uuid,
        time -> Timestamptz,
        tenant_id -> Uuid,
        tenant_name -> Text,
        application_id -> Uuid,
        application_name -> Text,
        device_profile_id -> Uuid,
        device_profile_name -> Text,
        device_name -> Text,
        dev_eui -> Bpchar,
        tags -> Jsonb,
        latitude -> Float8,
        longitude -> Float8,
        altitude -> Float8,
        source -> Text,
        accuracy -> Float4,
    }
}

table! {
    event_log (id) {
        id -> Int8,
        time -> Timestamptz,
        tenant_id -> Uuid,
        tenant_name -> Text,
        application_id -> Uuid,
        application_name -> Text,
        device_profile_id -> Uuid,
        device_profile_name -> Text,
        device_name -> Text,
        dev_eui -> Bpchar,
        tags -> Jsonb,
        level -> Text,
        code -> Text,
        description -> Text,
        context -> Jsonb,
    }
}

table! {
    event_status (deduplication_id) {
        deduplication_id -> Uuid,
        time -> Timestamptz,
        tenant_id -> Uuid,
        tenant_name -> Text,
        application_id -> Uuid,
        application_name -> Text,
        device_profile_id -> Uuid,
        device_profile_name -> Text,
        device_name -> Text,
        dev_eui -> Bpchar,
        tags -> Jsonb,
        margin -> Int2,
        external_power_source -> Bool,
        battery_level_unavailable -> Bool,
        battery_level -> Float4,
    }
}

table! {
    event_tx_ack (queue_item_id) {
        queue_item_id -> Uuid,
        downlink_id -> Int8,
        time -> Timestamptz,
        tenant_id -> Uuid,
        tenant_name -> Text,
        application_id -> Uuid,
        application_name -> Text,
        device_profile_id -> Uuid,
        device_profile_name -> Text,
        device_name -> Text,
        dev_eui -> Bpchar,
        tags -> Jsonb,
        f_cnt_down -> Int8,
        gateway_id -> Bpchar,
        tx_info -> Jsonb,
    }
}

table! {
    event_up (deduplication_id) {
        deduplication_id -> Uuid,
        time -> Timestamptz,
        tenant_id -> Uuid,
        tenant_name -> Text,
        application_id -> Uuid,
        application_name -> Text,
        device_profile_id -> Uuid,
        device_profile_name -> Text,
        device_name -> Text,
        dev_eui -> Bpchar,
        tags -> Jsonb,
        dev_addr -> Bpchar,
        adr -> Bool,
        dr -> Int2,
        f_cnt -> Int8,
        f_port -> Int2,
        confirmed -> Bool,
        data -> Bytea,
        object -> Jsonb,
        rx_info -> Jsonb,
        tx_info -> Jsonb,
    }
}

allow_tables_to_appear_in_same_query!(
    event_ack,
    event_integration,
    event_join,
    event_location,
    event_log,
    event_status,
    event_tx_ack,
    event_up,
);
