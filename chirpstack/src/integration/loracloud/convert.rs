pub fn serde_json_to_pb_json(val: &serde_json::Value) -> pbjson_types::Struct {
    // Initial value must be an object.
    if let serde_json::Value::Object(_) = val {
        if let Some(pbjson_types::value::Kind::StructValue(v)) = _serde_json_to_pb_json(val) {
            return v;
        }
    }

    Default::default()
}

fn _serde_json_to_pb_json(val: &serde_json::Value) -> Option<pbjson_types::value::Kind> {
    match val {
        serde_json::Value::Null => None,
        serde_json::Value::Bool(v) => Some(pbjson_types::value::Kind::BoolValue(*v)),
        serde_json::Value::Number(v) => {
            if v.is_f64() {
                Some(pbjson_types::value::Kind::NumberValue(v.as_f64().unwrap()))
            } else if v.is_i64() {
                Some(pbjson_types::value::Kind::NumberValue(
                    v.as_i64().unwrap() as f64
                ))
            } else if v.is_u64() {
                Some(pbjson_types::value::Kind::NumberValue(
                    v.as_u64().unwrap() as f64
                ))
            } else {
                None
            }
        }
        serde_json::Value::String(v) => Some(pbjson_types::value::Kind::StringValue(v.clone())),
        serde_json::Value::Array(v) => Some(pbjson_types::value::Kind::ListValue(
            pbjson_types::ListValue {
                values: v
                    .iter()
                    .map(|v| pbjson_types::Value {
                        kind: _serde_json_to_pb_json(v),
                    })
                    .collect(),
            },
        )),
        serde_json::Value::Object(v) => Some(pbjson_types::value::Kind::StructValue(
            pbjson_types::Struct {
                fields: v
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            pbjson_types::Value {
                                kind: _serde_json_to_pb_json(v),
                            },
                        )
                    })
                    .collect(),
            },
        )),
    }
}
