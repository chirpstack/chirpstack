use std::fs;

use anyhow::{Context, Result};
use async_trait::async_trait;
use super::{Handler, passthrough};

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

    fn get_script(&self) -> String {
        self.script.clone()
    }
}