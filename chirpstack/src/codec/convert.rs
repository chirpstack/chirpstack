pub fn rquickjs_to_struct(val: &rquickjs::Value) -> pbjson_types::Struct {
    if val.type_of() == rquickjs::Type::Object {
        if let Some(pbjson_types::value::Kind::StructValue(v)) = _rquickjs_to_struct_val(val) {
            return v;
        }
    }

    Default::default()
}

fn _rquickjs_to_struct_val(val: &rquickjs::Value) -> Option<pbjson_types::value::Kind> {
    match val.type_of() {
        rquickjs::Type::Bool => Some(pbjson_types::value::Kind::BoolValue(val.as_bool().unwrap())),
        rquickjs::Type::Int => Some(pbjson_types::value::Kind::NumberValue(
            val.as_int().unwrap().into(),
        )),
        rquickjs::Type::Float => Some(pbjson_types::value::Kind::NumberValue(
            val.as_float().unwrap().into(),
        )),
        rquickjs::Type::String => Some(pbjson_types::value::Kind::StringValue(
            val.as_string().unwrap().to_string().unwrap(),
        )),
        rquickjs::Type::Array => Some(pbjson_types::value::Kind::ListValue(
            pbjson_types::ListValue {
                values: val
                    .as_array()
                    .unwrap()
                    .iter::<rquickjs::Value>()
                    .map(|v| pbjson_types::Value {
                        kind: _rquickjs_to_struct_val(&v.unwrap()),
                    })
                    .collect(),
            },
        )),
        rquickjs::Type::Object => Some(pbjson_types::value::Kind::StructValue(
            pbjson_types::Struct {
                fields: val
                    .as_object()
                    .unwrap()
                    .clone()
                    .into_iter()
                    .map(|i| {
                        let (k, v) = i.unwrap();
                        (
                            k.to_string().unwrap(),
                            pbjson_types::Value {
                                kind: _rquickjs_to_struct_val(&v),
                            },
                        )
                    })
                    .collect(),
            },
        )),
        _ => None,
    }
}

pub fn struct_to_rquickjs<'js>(
    ctx: rquickjs::Ctx<'js>,
    obj: &prost_types::Struct,
) -> rquickjs::Object<'js> {
    let out = rquickjs::Object::new(ctx).unwrap();

    for (k, v) in &obj.fields {
        out.set(k, _struct_to_rquickjs(ctx, v)).unwrap();
    }

    out
}

fn _struct_to_rquickjs<'js>(
    ctx: rquickjs::Ctx<'js>,
    val: &prost_types::Value,
) -> rquickjs::Value<'js> {
    match &val.kind {
        None => rquickjs::Value::new_null(ctx),
        Some(val) => match val {
            prost_types::value::Kind::NullValue(_) => rquickjs::Value::new_null(ctx),
            prost_types::value::Kind::NumberValue(v) => rquickjs::Value::new_float(ctx, *v),
            prost_types::value::Kind::StringValue(v) => {
                rquickjs::Value::from_string(rquickjs::String::from_str(ctx, v).unwrap())
            }
            prost_types::value::Kind::BoolValue(v) => rquickjs::Value::new_bool(ctx, *v),
            prost_types::value::Kind::StructValue(v) => {
                let out = rquickjs::Object::new(ctx).unwrap();
                for (k, v) in &v.fields {
                    out.set(k, _struct_to_rquickjs(ctx, v)).unwrap();
                }
                rquickjs::Value::from_object(out)
            }
            prost_types::value::Kind::ListValue(v) => {
                let out = rquickjs::Array::new(ctx).unwrap();
                for (i, v) in v.values.iter().enumerate() {
                    out.set(i, _struct_to_rquickjs(ctx, v)).unwrap();
                }
                rquickjs::Value::from_array(out)
            }
        },
    }
}
