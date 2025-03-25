use std::fs;
use std::collections::HashMap;
use std::time::SystemTime;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use super::{Handler, passthrough};

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
        let rt = rquickjs::Runtime::new()?;
        let ctx = rquickjs::Context::full(&rt)?;
        let script = fs::read_to_string(file_path).context("Read codec plugin")?;

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
            name: String::from("Passthrough") 
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

    async fn encode(&self, f_port: u8, variables: &HashMap<String, String>, obj: &prost_types::Struct) -> Result<Vec<u8>> {
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
                .map_err(|e| anyhow!("JS error: {}", e))?;
            () = buff_promise.finish()?;
            let buff: rquickjs::Function = buff.get("Buffer")?;

            let m = rquickjs::Module::declare(ctx.clone(), "script", self.script.clone())
                .context("Declare script")?;
            let (m, m_promise) = m.eval().context("Evaluate script")?;
            () = m_promise.finish()?;
            let func: rquickjs::Function = m.get("encodeDownlink").context("Get encodeDownlink function")?;

            let input = rquickjs::Object::new(ctx.clone())?;
            input.set("fPort", f_port.into_js(&ctx)?)?;
            input.set("variables", variables.into_js(&ctx)?)?;
            input.set("data", convert::struct_to_rquickjs(&ctx, obj))?;

            let globals = ctx.globals();
            globals.set("Buffer", buff)?;

            let res: rquickjs::Object = func
                .call((input,))
                .catch(&ctx)
                .map_err(|e| anyhow!("JS error: {}", e))?;

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

    async fn decode(&self, recv_time: DateTime<Utc>, f_port: u8, variables: &HashMap<String, String>, b: &[u8]) -> Result<pbjson_types::Struct> {
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

        ctx.with::<_, Result<pbjson_types::Struct>>(|ctx| {
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
                .map_err(|e| anyhow!("JS error: {}", e))?;
            () = buff_promise.finish()?;
            let buff: rquickjs::Function = buff.get("Buffer")?;

            let m = rquickjs::Module::declare(ctx.clone(), "script", self.script.clone())
                .context("Declare script")?;
            let (m, m_promise) = m.eval().context("Evaluate script")?;
            () = m_promise.finish()?;
            let func: rquickjs::Function = m.get("decodeUplink").context("Get decodeUplink function")?;

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
                .map_err(|e| anyhow!("JS error: {}", e))?;

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
        })
    }
}

// TODO: add tests
