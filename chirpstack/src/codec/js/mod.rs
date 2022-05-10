use std::collections::HashMap;
use std::time::SystemTime;

use anyhow::Result;
use rquickjs::IntoJs;

use super::convert;
use crate::config;

mod vendor_base64_js;
mod vendor_buffer;
mod vendor_ieee754;

pub async fn decode(
    f_port: u8,
    variables: &HashMap<String, String>,
    decode_config: &str,
    b: &[u8],
) -> Result<pbjson_types::Struct> {
    let conf = config::get();
    let max_run_ts = SystemTime::now() + conf.codec.js.max_execution_time;

    let resolver = rquickjs::BuiltinResolver::default()
        .with_module("base64-js")
        .with_module("ieee754")
        .with_module("buffer");
    let loader = rquickjs::BuiltinLoader::default()
        .with_module("base64-js", vendor_base64_js::SCRIPT)
        .with_module("ieee754", vendor_ieee754::SCRIPT)
        .with_module("buffer", vendor_buffer::SCRIPT);

    let rt = rquickjs::Runtime::new()?;
    rt.set_interrupt_handler(Some(Box::new(move || SystemTime::now() > max_run_ts)));
    rt.set_loader(resolver, loader);

    let ctx = rquickjs::Context::full(&rt)?;

    let script = format!(
        r#"
        import {{ Buffer }} from "buffer";

        {}

        export {{ decodeUplink }};
        "#,
        decode_config
    );
    let b = b.to_vec();

    let out = ctx.with(|ctx| -> Result<pbjson_types::Struct> {
        let m = ctx.compile("script", script)?;
        let func: rquickjs::Function = m.get("decodeUplink")?;

        let input = rquickjs::Object::new(ctx)?;
        input.set("fPort", f_port.into_js(ctx)?)?;
        input.set("variables", variables.into_js(ctx)?)?;
        input.set("bytes", b.into_js(ctx)?)?;

        let res: rquickjs::Object = func.call((input,))?;
        Ok(convert::rquickjs_to_struct(&res))
    })?;

    let obj = out.fields.get("object").cloned().unwrap_or_default();
    if let Some(pbjson_types::value::Kind::StructValue(v)) = obj.kind {
        return Ok(v);
    }

    Err(anyhow!("decodeUplink did not return 'object'"))
}

pub async fn encode(
    f_port: u8,
    variables: &HashMap<String, String>,
    encode_config: &str,
    s: &prost_types::Struct,
) -> Result<Vec<u8>> {
    let conf = config::get();
    let max_run_ts = SystemTime::now() + conf.codec.js.max_execution_time;

    let resolver = rquickjs::BuiltinResolver::default()
        .with_module("base64-js")
        .with_module("ieee754")
        .with_module("buffer");
    let loader = rquickjs::BuiltinLoader::default()
        .with_module("base64-js", vendor_base64_js::SCRIPT)
        .with_module("ieee754", vendor_ieee754::SCRIPT)
        .with_module("buffer", vendor_buffer::SCRIPT);

    let rt = rquickjs::Runtime::new()?;
    rt.set_interrupt_handler(Some(Box::new(move || SystemTime::now() > max_run_ts)));
    rt.set_loader(resolver, loader);

    let ctx = rquickjs::Context::full(&rt)?;

    let script = format!(
        r#"
        import {{ Buffer }} from "buffer";

        {}

        export {{ encodeDownlink }};
        "#,
        encode_config,
    );

    ctx.with(|ctx| {
        let m = ctx.compile("script", script)?;
        let func: rquickjs::Function = m.get("encodeDownlink")?;

        let input = rquickjs::Object::new(ctx)?;
        input.set("fPort", f_port.into_js(ctx)?)?;
        input.set("variables", variables.into_js(ctx)?)?;
        input.set("data", convert::struct_to_rquickjs(ctx, s))?;

        let res: rquickjs::Object = func.call((input,))?;
        let v: Vec<u8> = res.get("bytes")?;
        Ok(v)
    })
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[tokio::test]
    pub async fn test_decode_timeout() {
        let decoder = r#"
            function decodeUplink(input) {
                while (true) {

                }
            }
        "#
        .to_string();

        let vars: HashMap<String, String> = HashMap::new();
        let out = decode(10, &vars, &decoder, &[0x01, 0x02, 0x03]).await;
        assert!(out.is_err());
    }

    #[tokio::test]
    pub async fn test_decode() {
        let decoder = r#"
            function decodeUplink(input) {
                var buff = new Buffer(input.bytes);

                return {
                    object: {
                        f_port: input.fPort,
                        variables: input.variables,
                        data_hex: buff.toString('hex'),
                        data: input.bytes
                    }
                };
            }
        "#
        .to_string();

        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("foo".into(), "bar".into());

        let out = decode(10, &vars, &decoder, &[0x01, 0x02, 0x03])
            .await
            .unwrap();

        let expected = pbjson_types::Struct {
            fields: [
                (
                    "f_port".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::NumberValue(10.0)),
                    },
                ),
                (
                    "variables".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StructValue(
                            pbjson_types::Struct {
                                fields: [(
                                    "foo".to_string(),
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::StringValue(
                                            "bar".to_string(),
                                        )),
                                    },
                                )]
                                .iter()
                                .cloned()
                                .collect(),
                            },
                        )),
                    },
                ),
                (
                    "data_hex".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StringValue("010203".to_string())),
                    },
                ),
                (
                    "data".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::ListValue(
                            pbjson_types::ListValue {
                                values: vec![
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::NumberValue(1.0)),
                                    },
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::NumberValue(2.0)),
                                    },
                                    pbjson_types::Value {
                                        kind: Some(pbjson_types::value::Kind::NumberValue(3.0)),
                                    },
                                ],
                            },
                        )),
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        assert_eq!(expected, out);
    }

    #[tokio::test]
    pub async fn test_encode_timeout() {
        let encoder = r#"
            function encodeDownlink(input) {
                while (true) {

                }
            }
        "#
        .to_string();

        let vars: HashMap<String, String> = HashMap::new();

        let input = prost_types::Struct {
            ..Default::default()
        };

        let out = encode(10, &vars, &encoder, &input).await;
        assert!(out.is_err());
    }

    #[tokio::test]
    pub async fn test_encode() {
        let encoder = r#"
            function encodeDownlink(input) {
                if (input.data.enabled) {
                    return {
                        bytes: [0x01] 
                    };
                } else {
                    return {
                        bytes: [0x02]
                    };
                }
            }
        "#
        .to_string();

        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("foo".into(), "bar".into());

        let mut input = prost_types::Struct::default();
        input.fields.insert(
            "enabled".to_string(),
            prost_types::Value {
                kind: Some(prost_types::value::Kind::BoolValue(true)),
            },
        );

        let out = encode(10, &vars, &encoder, &input).await.unwrap();
        assert_eq!(vec![1], out);
    }
}
