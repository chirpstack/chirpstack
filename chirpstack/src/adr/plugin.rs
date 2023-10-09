use std::fs;

use anyhow::{Context, Result};
use async_trait::async_trait;

use super::{Handler, Request, Response};

pub struct Plugin {
    script: String,
    id: String,
    name: String,
}

impl Plugin {
    pub fn new(file_path: &str) -> Result<Self> {
        let rt = rquickjs::Runtime::new()?;
        let ctx = rquickjs::Context::full(&rt)?;
        let script = fs::read_to_string(file_path).context("Read ADR plugin")?;

        let (id, name) = ctx.with::<_, Result<(String, String)>>(|ctx| {
            let m = ctx
                .compile("script", script.clone())
                .context("Compile script")?;
            let id_func: rquickjs::Function = m.get("id").context("Get id function")?;
            let name_func: rquickjs::Function = m.get("name").context("Get name function")?;

            let id: String = id_func.call(()).context("Call id function")?;
            let name: String = name_func.call(()).context("Call name function")?;

            Ok((id, name))
        })?;

        let p = Plugin { script, id, name };

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

    async fn handle(&self, req: &Request) -> Result<Response> {
        let rt = rquickjs::Runtime::new()?;
        let ctx = rquickjs::Context::full(&rt)?;

        ctx.with::<_, Result<Response>>(|ctx| {
            let m = ctx
                .compile("script", self.script.clone())
                .context("Compile script")?;
            let func: rquickjs::Function = m.get("handle").context("Get handle function")?;

            let device_variables = rquickjs::Object::new(ctx)?;
            for (k, v) in &req.device_variables {
                device_variables.set(k, v)?;
            }

            let input = rquickjs::Object::new(ctx)?;
            input.set("regionConfigId", req.region_config_id.clone())?;
            input.set("regionCommonName", req.region_common_name.to_string())?;
            input.set("devEui", req.dev_eui.to_string())?;
            input.set("macVersion", req.mac_version.to_string())?;
            input.set("regParamsRevision", req.reg_params_revision.to_string())?;
            input.set("adr", req.adr)?;
            input.set("dr", req.dr)?;
            input.set("txPowerIndex", req.tx_power_index)?;
            input.set("nbTrans", req.nb_trans)?;
            input.set("maxTxPowerIndex", req.max_tx_power_index)?;
            input.set("requiredSnrForDr", req.required_snr_for_dr)?;
            input.set("installationMargin", req.installation_margin)?;
            input.set("minDr", req.min_dr)?;
            input.set("maxDr", req.max_dr)?;
            input.set("deviceVariables", device_variables)?;

            let mut uplink_history: Vec<rquickjs::Object> = Vec::new();

            for uh in &req.uplink_history {
                let obj = rquickjs::Object::new(ctx)?;
                obj.set("fCnt", uh.f_cnt)?;
                obj.set("maxSnr", uh.max_snr)?;
                obj.set("maxRssi", uh.max_rssi)?;
                obj.set("txPowerIndex", uh.tx_power_index)?;
                obj.set("gatewayCount", uh.gateway_count)?;
                uplink_history.push(obj);
            }

            input.set("uplinkHistory", uplink_history)?;

            let res: rquickjs::Object = func.call((input,)).context("Call handle function")?;

            Ok(Response {
                dr: res.get("dr").context("Get dr response")?,
                tx_power_index: res
                    .get("txPowerIndex")
                    .context("Get txPowerIndex response")?,
                nb_trans: res.get("nbTrans").context("Get nbTrans response")?,
            })
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use lrwn::EUI64;

    #[tokio::test]
    async fn test_plugin() {
        let p = Plugin::new("../examples/adr_plugins/plugin_skeleton.js").unwrap();

        assert_eq!("Example plugin", p.get_name());
        assert_eq!("example_id", p.get_id());

        let req = Request {
            region_config_id: "eu868".into(),
            region_common_name: lrwn::region::CommonName::EU868,
            dev_eui: EUI64::from_be_bytes([1, 2, 3, 4, 5, 6, 7, 8]),
            mac_version: lrwn::region::MacVersion::LORAWAN_1_0_3,
            reg_params_revision: lrwn::region::Revision::A,
            adr: true,
            dr: 3,
            tx_power_index: 0,
            nb_trans: 1,
            max_tx_power_index: 15,
            required_snr_for_dr: -15.0,
            installation_margin: 10.0,
            min_dr: 0,
            max_dr: 5,
            uplink_history: vec![],
            skip_f_cnt_check: false,
            device_variables: Default::default(),
        };

        let resp = p.handle(&req).await.unwrap();
        assert_eq!(
            Response {
                dr: 3,
                tx_power_index: 0,
                nb_trans: 1,
            },
            resp
        );
    }
}
