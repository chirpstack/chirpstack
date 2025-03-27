use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

use super::{passthrough, Handler};
use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use rquickjs::{CatchResultExt, IntoJs};

use super::super::convert;
use super::super::js::vendor_base64_js;
use super::super::js::vendor_buffer;
use super::super::js::vendor_ieee754;

use crate::config;

pub struct Plugin {
    script: String,
    id: String,
    name: String,
}

impl Plugin {
    pub fn new(file_path: &str) -> Result<Self> {
        let script = fs::read_to_string(file_path).context("Read codec plugin")?;

        Plugin::from_string(script)
    }

    pub fn from_string(script: String) -> Result<Self> {
        let rt = rquickjs::Runtime::new()?;
        let ctx = rquickjs::Context::full(&rt)?;

        let (id, name) = ctx.with::<_, Result<(String, String)>>(|ctx| {
            let m = rquickjs::Module::declare(ctx, "script", script.clone())
                .context("Declare script")?;
            let (m, m_promise) = m.eval().context("Evaluate script")?;
            () = m_promise.finish()?;
            let id_func: rquickjs::Function = m.get("id").context("Get id function")?;
            let name_func: rquickjs::Function = m.get("name").context("Get name function")?;

            let id: String = id_func.call(()).context("Call id function")?;
            let name: String = name_func.call(()).context("Call name function")?;

            Ok((id, name))
        })?;

        let p = Plugin { script, id, name };

        Ok(p)
    }

    pub fn default() -> Result<Self> {
        let p = Plugin {
            script: passthrough::SCRIPT.to_string(),
            id: String::from("passthrough"),
            name: String::from("Passthrough"),
        };

        Ok(p)
    }
}

#[async_trait]
impl Handler for Plugin {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    async fn encode(
        &self,
        f_port: u8,
        variables: &HashMap<String, String>,
        obj: &prost_types::Struct,
    ) -> Result<Vec<u8>> {
        let conf = config::get();
        let max_run_ts = SystemTime::now() + conf.codec.js.max_execution_time;

        let resolver = rquickjs::loader::BuiltinResolver::default()
            .with_module("base64-js")
            .with_module("ieee754")
            .with_module("buffer");
        let loader = rquickjs::loader::BuiltinLoader::default()
            .with_module("base64-js", vendor_base64_js::SCRIPT)
            .with_module("ieee754", vendor_ieee754::SCRIPT)
            .with_module("buffer", vendor_buffer::SCRIPT);

        let rt = rquickjs::Runtime::new()?;
        rt.set_interrupt_handler(Some(Box::new(move || SystemTime::now() > max_run_ts)));
        rt.set_loader(resolver, loader);

        let ctx = rquickjs::Context::full(&rt)?;

        ctx.with::<_, Result<Vec<u8>>>(|ctx| {
            // We need to export the Buffer class, so it is correctly resolved
            // in called encode/decode functions
            let buff = rquickjs::Module::declare(
                ctx.clone(),
                "b",
                r#"
                import { Buffer } from "buffer";
                export { Buffer }
                "#,
            )
            .context("Declare script")?;
            let (buff, buff_promise) = buff
                .eval()
                .catch(&ctx)
                .map_err(|e| anyhow!("JS plugin error: {}", e))?;
            () = buff_promise.finish()?;
            let buff: rquickjs::Function = buff.get("Buffer")?;

            let m = rquickjs::Module::declare(ctx.clone(), "script", self.script.clone())
                .context("Declare script")?;
            let (m, m_promise) = m.eval().context("Evaluate script")?;
            () = m_promise.finish()?;
            let func: rquickjs::Function = m
                .get("encodeDownlink")
                .context("Get encodeDownlink function")?;

            let input = rquickjs::Object::new(ctx.clone())?;
            input.set("fPort", f_port.into_js(&ctx)?)?;
            input.set("variables", variables.into_js(&ctx)?)?;
            input.set("data", convert::struct_to_rquickjs(&ctx, obj))?;

            let globals = ctx.globals();
            globals.set("Buffer", buff)?;

            let res: rquickjs::Object = func
                .call((input,))
                .catch(&ctx)
                .map_err(|e| anyhow!("JS plugin error: {}", e))?;

            let errors: Result<Vec<String>, rquickjs::Error> = res.get("errors");
            if let Ok(errors) = errors {
                if !errors.is_empty() {
                    return Err(anyhow!(
                        "encodeDownlink returned errors: {}",
                        errors.join(", ")
                    ));
                }
            }

            // Directly into u8 can result into the following error:
            // Error converting from js 'float' into type 'i32'
            let v: Vec<f64> = res.get("bytes")?;
            let v: Vec<u8> = v.iter().map(|v| *v as u8).collect();

            Ok(v)
        })
    }

    async fn decode(
        &self,
        recv_time: DateTime<Utc>,
        f_port: u8,
        variables: &HashMap<String, String>,
        b: &[u8],
    ) -> Result<pbjson_types::Struct> {
        let conf = config::get();
        let max_run_ts = SystemTime::now() + conf.codec.js.max_execution_time;

        let resolver = rquickjs::loader::BuiltinResolver::default()
            .with_module("base64-js")
            .with_module("ieee754")
            .with_module("buffer");
        let loader = rquickjs::loader::BuiltinLoader::default()
            .with_module("base64-js", vendor_base64_js::SCRIPT)
            .with_module("ieee754", vendor_ieee754::SCRIPT)
            .with_module("buffer", vendor_buffer::SCRIPT);

        let rt = rquickjs::Runtime::new()?;
        rt.set_interrupt_handler(Some(Box::new(move || SystemTime::now() > max_run_ts)));
        rt.set_loader(resolver, loader);

        let ctx = rquickjs::Context::full(&rt)?;

        let out = ctx.with(|ctx| -> Result<pbjson_types::Struct> {
            // We need to export the Buffer class, so it is correctly resolved
            // in called encode/decode functions
            let buff = rquickjs::Module::declare(
                ctx.clone(),
                "b",
                r#"
                import { Buffer } from "buffer";
                export { Buffer }
                "#,
            )
            .context("Declare script")?;
            let (buff, buff_promise) = buff
                .eval()
                .catch(&ctx)
                .map_err(|e| anyhow!("JS plugin error: {}", e))?;
            () = buff_promise.finish()?;
            let buff: rquickjs::Function = buff.get("Buffer")?;

            let m = rquickjs::Module::declare(ctx.clone(), "script", self.script.clone())
                .context("Declare script")?;
            let (m, m_promise) = m.eval().context("Evaluate script")?;
            () = m_promise.finish()?;
            let func: rquickjs::Function =
                m.get("decodeUplink").context("Get decodeUplink function")?;

            let input = rquickjs::Object::new(ctx.clone())?;
            input.set("bytes", b.into_js(&ctx)?)?;
            input.set("fPort", f_port.into_js(&ctx)?)?;
            input.set("recvTime", recv_time.into_js(&ctx)?)?;
            input.set("variables", variables.into_js(&ctx)?)?;

            let globals = ctx.globals();
            globals.set("Buffer", buff)?;

            let res: rquickjs::Object = func
                .call((input,))
                .catch(&ctx)
                .map_err(|e| anyhow!("JS plugin error: {}", e))?;

            let errors: Result<Vec<String>, rquickjs::Error> = res.get("errors");
            if let Ok(errors) = errors {
                if !errors.is_empty() {
                    return Err(anyhow!(
                        "decodeUplink returned errors: {}",
                        errors.join(", ")
                    ));
                }
            }

            Ok(convert::rquickjs_to_struct(&res))
        })?;

        let data = out.fields.get("data").cloned().unwrap_or_default();
        if let Some(pbjson_types::value::Kind::StructValue(v)) = data.kind {
            return Ok(v);
        }

        Err(anyhow!("decodeUplink did not return 'data'"))
    }
}

pub mod test {
    use super::*;
    use chrono::TimeZone;
    use chrono::Utc;

    #[tokio::test]
    async fn test_plugin() {
        let p = Plugin::new("../examples/codec_plugins/plugin_skeleton.js").unwrap();

        assert_eq!("Example plugin", p.get_name());
        assert_eq!("example_id", p.get_id());
    }

    #[tokio::test]
    pub async fn test_decode_timeout() {
        let script = r#"
            export function id() {
                return "test_decode_timeout";
            }

            export function name() {
                return "test_decode_timeout";
            }
            
            export function decodeUplink(input) {
                while (true) {

                }
            }
        "#
        .to_string();

        let p = Plugin::from_string(script).unwrap();

        let vars: HashMap<String, String> = HashMap::new();
        let out = p.decode(Utc::now(), 10, &vars, &[0x01, 0x02, 0x03]).await;

        assert!(out.is_err());
    }

    #[tokio::test]
    pub async fn test_decode_error() {
        let script = r#"
            export function id() {
                return "test_decode_error";
            }

            export function name() {
                return "test_decode_error";
            }

            export function decodeUplink(input) {
                return foo;
            }
        "#
        .to_string();

        let p = Plugin::from_string(script).unwrap();

        let vars: HashMap<String, String> = HashMap::new();
        let out = p.decode(Utc::now(), 10, &vars, &[0x01, 0x02, 0x03]).await;

        assert_eq!(
            "JS plugin error: Error: foo is not defined\n    at decodeUplink (script:10:1)\n",
            out.err().unwrap().to_string()
        );
    }

    #[tokio::test]
    pub async fn test_decode() {
        let recv_time = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();

        let script = r#"
            export function id() {
                return "test_decode";
            }

            export function name() {
                return "test_decode";
            }

            export function decodeUplink(input) {
                var buff = new Buffer(input.bytes);

                return {
                    data: {
                        f_port: input.fPort,
                        variables: input.variables,
                        data_hex: buff.toString('hex'),
                        data: input.bytes,
                        recv_time: input.recvTime.toString()
                    }
                };
            }
        "#
        .to_string();

        let p = Plugin::from_string(script).unwrap();

        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("foo".into(), "bar".into());

        let out = p
            .decode(recv_time, 10, &vars, &[0x01, 0x02, 0x03])
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
                (
                    "recv_time".to_string(),
                    pbjson_types::Value {
                        kind: Some(pbjson_types::value::Kind::StringValue(
                            "Tue Jul 08 2014 09:10:11 GMT+0000".to_string(),
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
        let script = r#"
            export function id() {
                return "test_encode_timeout";
            }

            export function name() {
                return "test_encode_timeout";
            }

            export function encodeDownlink(input) {
                while (true) {

                }
            }
        "#
        .to_string();

        let p = Plugin::from_string(script).unwrap();

        let vars: HashMap<String, String> = HashMap::new();

        let input = prost_types::Struct {
            ..Default::default()
        };

        let out = p.encode(10, &vars, &input).await;
        assert!(out.is_err());
    }

    #[tokio::test]
    pub async fn test_encode_error() {
        let script = r#"
            export function id() {
                return "test_encode_error";
            }

            export function name() {
                return "test_encode_error";
            }

            export function encodeDownlink(input) {
                return foo;
            }
        "#
        .to_string();

        let p = Plugin::from_string(script).unwrap();

        let vars: HashMap<String, String> = HashMap::new();

        let input = prost_types::Struct {
            ..Default::default()
        };

        let out = p.encode(10, &vars, &input).await;
        assert_eq!(
            "JS plugin error: Error: foo is not defined\n    at encodeDownlink (script:10:1)\n",
            out.err().unwrap().to_string()
        );
    }

    #[tokio::test]
    pub async fn test_encode() {
        let script = r#"
            export function id() {
                return "test_encode";
            }

            export function name() {
                return "test_encode";
            }

            export function encodeDownlink(input) {
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

        let p = Plugin::from_string(script).unwrap();

        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("foo".into(), "bar".into());

        let mut input = prost_types::Struct::default();
        input.fields.insert(
            "enabled".to_string(),
            prost_types::Value {
                kind: Some(prost_types::value::Kind::BoolValue(true)),
            },
        );

        let out = p.encode(10, &vars, &input).await.unwrap();
        assert_eq!(vec![1], out);
    }
}
