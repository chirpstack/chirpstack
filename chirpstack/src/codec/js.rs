use std::collections::HashMap;
use std::time::SystemTime;

use anyhow::Result;
use rquickjs::IntoJs;

use super::convert;
use crate::config;

pub async fn decode(
    f_port: u8,
    variables: &HashMap<String, String>,
    decode_config: &str,
    b: &[u8],
) -> Result<pbjson_types::Struct> {
    let conf = config::get();
    let max_run_ts = SystemTime::now() + conf.codec.js.max_execution_time;

    let rt = rquickjs::Runtime::new().unwrap();
    rt.set_interrupt_handler(Some(Box::new(move || SystemTime::now() > max_run_ts)));

    let ctx = rquickjs::Context::full(&rt).unwrap();

    let script = decode_config.to_string();
    let b = b.to_vec();

    ctx.with(|ctx| {
        let m = ctx.compile("script", script).unwrap();
        let func: rquickjs::Function = m.get("Decode").unwrap();

        let input = rquickjs::Object::new(ctx).unwrap();
        input.set("f_port", f_port.into_js(ctx).unwrap()).unwrap();
        input
            .set("variables", variables.into_js(ctx).unwrap())
            .unwrap();
        input.set("data", b.into_js(ctx).unwrap()).unwrap();

        let res: rquickjs::Object = func.call((input,))?;
        Ok(convert::rquickjs_to_struct(&res))
    })
}

pub async fn encode(
    f_port: u8,
    variables: &HashMap<String, String>,
    encode_config: &str,
    s: &prost_types::Struct,
) -> Result<Vec<u8>> {
    let conf = config::get();
    let max_run_ts = SystemTime::now() + conf.codec.js.max_execution_time;

    let rt = rquickjs::Runtime::new().unwrap();
    rt.set_interrupt_handler(Some(Box::new(move || SystemTime::now() > max_run_ts)));

    let ctx = rquickjs::Context::full(&rt).unwrap();

    let script = encode_config.to_string();

    ctx.with(|ctx| {
        let m = ctx.compile("script", script).unwrap();
        let func: rquickjs::Function = m.get("Encode").unwrap();

        let input = rquickjs::Object::new(ctx).unwrap();
        input.set("f_port", f_port.into_js(ctx).unwrap()).unwrap();
        input
            .set("variables", variables.into_js(ctx).unwrap())
            .unwrap();
        input
            .set("object", convert::struct_to_rquickjs(ctx, s))
            .unwrap();

        let res: Vec<u8> = func.call((input,))?;
        Ok(res)
    })
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[tokio::test]
    pub async fn test_decode_timeout() {
        let decoder = r#"
            export function Decode(input) {
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
            export function Decode(input) {
                return {
                    f_port: input.f_port,
                    variables: input.variables,
                    data: input.data
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
            export function Encode(input) {
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
            export function Encode(input) {
                if (input.object.enabled) {
                    return [input.f_port, 0x01];
                } else {
                    return [input.f_port, 0x00];
                }
            }
        "#
        .to_string();

        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("foo".into(), "bar".into());

        let input = prost_types::Struct {
            fields: [(
                "enabled".to_string(),
                prost_types::Value {
                    kind: Some(prost_types::value::Kind::BoolValue(true)),
                },
            )]
            .iter()
            .cloned()
            .collect(),
        };

        let out = encode(10, &vars, &encoder, &input).await.unwrap();
        assert_eq!(vec![10, 1], out);
    }
}
