use anyhow::Result;

use crate::applayer::PayloadCodec;

pub enum Cid {
    PackageVersionReq,
    PackageVersionAns,
    AppTimeReq,
    AppTimeAns,
    DeviceAppTimePeropdicityReq,
    DeviceAppTimePeropdicityAns,
    ForceDeviceResyncReq,
}

impl Cid {
    pub fn from_u8(uplink: bool, value: u8) -> Result<Cid> {
        Ok(match uplink {
            true => match value {
                0x00 => Cid::PackageVersionAns,
                0x01 => Cid::AppTimeReq,
                0x02 => Cid::DeviceAppTimePeropdicityAns,
                _ => return Err(anyhow!("Invalid CID: {}", value)),
            },
            false => match value {
                0x00 => Cid::PackageVersionReq,
                0x01 => Cid::AppTimeAns,
                0x02 => Cid::DeviceAppTimePeropdicityReq,
                0x03 => Cid::ForceDeviceResyncReq,
                _ => return Err(anyhow!("Invalid CID: {}", value)),
            },
        })
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Cid::PackageVersionReq | Cid::PackageVersionAns => 0x00,
            Cid::AppTimeReq | Cid::AppTimeAns => 0x01,
            Cid::DeviceAppTimePeropdicityReq | Cid::DeviceAppTimePeropdicityAns => 0x02,
            Cid::ForceDeviceResyncReq => 0x03,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Payload {
    PackageVersionReq,
    PackageVersionAns(PackageVersionAnsPayload),
    AppTimeReq(AppTimeReqPayload),
    AppTimeAns(AppTimeAnsPayload),
    DeviceAppTimePeropdicityReq(DeviceAppTimePeriodicityReqPayload),
    DeviceAppTimePeropdicityAns(DeviceAppTimePeriodicityAnsPayload),
    ForceDeviceResyncReq(ForceDeviceResyncReqPayload),
}

impl Payload {
    pub fn cid(&self) -> Cid {
        match self {
            Self::PackageVersionReq => Cid::PackageVersionReq,
            Self::PackageVersionAns(_) => Cid::PackageVersionAns,
            Self::AppTimeReq(_) => Cid::AppTimeReq,
            Self::AppTimeAns(_) => Cid::AppTimeAns,
            Self::DeviceAppTimePeropdicityReq(_) => Cid::DeviceAppTimePeropdicityReq,
            Self::DeviceAppTimePeropdicityAns(_) => Cid::DeviceAppTimePeropdicityAns,
            Self::ForceDeviceResyncReq(_) => Cid::ForceDeviceResyncReq,
        }
    }

    pub fn from_slice(uplink: bool, b: &[u8]) -> Result<Self> {
        if b.is_empty() {
            return Err(anyhow!("At least one byte is expected"));
        }

        let cid = Cid::from_u8(uplink, b[0])?;

        Ok(match cid {
            Cid::PackageVersionReq => Payload::PackageVersionReq,
            Cid::PackageVersionAns => {
                Payload::PackageVersionAns(PackageVersionAnsPayload::decode(&b[1..])?)
            }
            Cid::AppTimeReq => Payload::AppTimeReq(AppTimeReqPayload::decode(&b[1..])?),
            Cid::AppTimeAns => Payload::AppTimeAns(AppTimeAnsPayload::decode(&b[1..])?),
            Cid::DeviceAppTimePeropdicityReq => Payload::DeviceAppTimePeropdicityReq(
                DeviceAppTimePeriodicityReqPayload::decode(&b[1..])?,
            ),
            Cid::DeviceAppTimePeropdicityAns => Payload::DeviceAppTimePeropdicityAns(
                DeviceAppTimePeriodicityAnsPayload::decode(&b[1..])?,
            ),
            Cid::ForceDeviceResyncReq => {
                Payload::ForceDeviceResyncReq(ForceDeviceResyncReqPayload::decode(&b[1..])?)
            }
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut out = vec![self.cid().to_u8()];

        match self {
            Self::PackageVersionReq => {}
            Self::PackageVersionAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::AppTimeReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::AppTimeAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::DeviceAppTimePeropdicityReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::DeviceAppTimePeropdicityAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::ForceDeviceResyncReq(pl) => out.extend_from_slice(&pl.encode()?),
        };

        Ok(out)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PackageVersionAnsPayload {
    pub package_identifier: u8,
    pub package_version: u8,
}

impl PayloadCodec for PackageVersionAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 2 {
            return Err(anyhow!("Expected 2 bytes"));
        }

        Ok(PackageVersionAnsPayload {
            package_identifier: b[0],
            package_version: b[1],
        })
    }
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![self.package_identifier, self.package_version])
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AppTimeReqPayload {
    pub device_time: u32,
    pub param: AppTimeReqPayloadParam,
}

impl PayloadCodec for AppTimeReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 5 {
            return Err(anyhow!("Expected 5 bytes"));
        }

        Ok(AppTimeReqPayload {
            device_time: {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&b[0..4]);
                u32::from_le_bytes(bytes)
            },
            param: AppTimeReqPayloadParam {
                token_req: b[4] & 0x0f,
                ans_required: b[4] & 0x10 != 0,
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.param.token_req > 15 {
            return Err(anyhow!("Max token_req value is 15"));
        }

        let mut b = vec![0; 5];
        b[0..4].copy_from_slice(&self.device_time.to_le_bytes());
        b[4] = self.param.token_req;

        if self.param.ans_required {
            b[4] |= 0x10;
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AppTimeReqPayloadParam {
    pub token_req: u8,
    pub ans_required: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AppTimeAnsPayload {
    pub time_correction: i32,
    pub param: AppTimeAnsPayloadParam,
}

impl PayloadCodec for AppTimeAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 5 {
            return Err(anyhow!("Expected 5 bytes"));
        }

        Ok(AppTimeAnsPayload {
            time_correction: {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&b[0..4]);
                i32::from_le_bytes(bytes)
            },
            param: AppTimeAnsPayloadParam {
                token_ans: b[4] & 0x0f,
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.param.token_ans > 15 {
            return Err(anyhow!("Max token_ans value is 15"));
        }

        let mut b = vec![0; 5];
        b[0..4].copy_from_slice(&self.time_correction.to_le_bytes());
        b[4] = self.param.token_ans;

        Ok(b)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AppTimeAnsPayloadParam {
    pub token_ans: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeviceAppTimePeriodicityReqPayload {
    pub period: u8,
}

impl PayloadCodec for DeviceAppTimePeriodicityReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(DeviceAppTimePeriodicityReqPayload {
            period: b[0] & 0x0f,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.period > 15 {
            return Err(anyhow!("Max period value is 15"));
        }

        Ok(vec![self.period])
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeviceAppTimePeriodicityAnsPayload {
    pub status: DeviceAppTimePeriodicityAnsPayloadStatus,
    pub time: u32,
}

impl PayloadCodec for DeviceAppTimePeriodicityAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 5 {
            return Err(anyhow!("Expected 5 bytes"));
        }

        Ok(DeviceAppTimePeriodicityAnsPayload {
            status: DeviceAppTimePeriodicityAnsPayloadStatus {
                not_supported: b[0] & 0x01 != 0,
            },
            time: {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&b[1..5]);
                u32::from_le_bytes(bytes)
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut b = vec![0; 5];
        if self.status.not_supported {
            b[0] |= 0x01;
        }

        b[1..5].copy_from_slice(&self.time.to_le_bytes());
        Ok(b)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeviceAppTimePeriodicityAnsPayloadStatus {
    pub not_supported: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForceDeviceResyncReqPayload {
    pub force_conf: ForceDeviceResyncReqPayloadForceConf,
}

impl PayloadCodec for ForceDeviceResyncReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(ForceDeviceResyncReqPayload {
            force_conf: ForceDeviceResyncReqPayloadForceConf {
                nb_transmissions: b[0] & 0x07,
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.force_conf.nb_transmissions > 7 {
            return Err(anyhow!("Max nb_transmissions is 7"));
        }

        Ok(vec![self.force_conf.nb_transmissions])
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForceDeviceResyncReqPayloadForceConf {
    pub nb_transmissions: u8,
}

#[cfg(test)]
mod test {
    use super::*;

    struct CommandTest {
        name: String,
        uplink: bool,
        command: Payload,
        bytes: Vec<u8>,
        expected_error: Option<String>,
    }

    #[test]
    fn test_package_version_req() {
        let encode_tests = [CommandTest {
            name: "encode PackageVersionReq".into(),
            uplink: false,
            command: Payload::PackageVersionReq,
            bytes: vec![0x00],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode PackageVersionReq".into(),
            uplink: false,
            command: Payload::PackageVersionReq,
            bytes: vec![0x00],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_package_version_ans() {
        let encode_tests = [CommandTest {
            name: "encode PackageVersionAns".into(),
            uplink: true,
            command: Payload::PackageVersionAns(PackageVersionAnsPayload {
                package_identifier: 1,
                package_version: 1,
            }),
            bytes: vec![0x00, 0x01, 0x01],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode PackageVersionAns".into(),
            uplink: true,
            command: Payload::PackageVersionAns(PackageVersionAnsPayload {
                package_identifier: 1,
                package_version: 1,
            }),
            bytes: vec![0x00, 0x01, 0x01],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_app_time_req() {
        let encode_tests = [CommandTest {
            name: "encode AppTimeReq".into(),
            uplink: true,
            command: Payload::AppTimeReq(AppTimeReqPayload {
                device_time: 1024,
                param: AppTimeReqPayloadParam {
                    token_req: 15,
                    ans_required: true,
                },
            }),
            bytes: vec![0x01, 0x00, 0x04, 0x00, 0x00, 0x1f],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode AppTimeReq".into(),
            uplink: true,
            command: Payload::AppTimeReq(AppTimeReqPayload {
                device_time: 1024,
                param: AppTimeReqPayloadParam {
                    token_req: 15,
                    ans_required: true,
                },
            }),
            bytes: vec![0x01, 0x00, 0x04, 0x00, 0x00, 0x1f],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_app_time_ans() {
        let encode_tests = [CommandTest {
            name: "encode AppTimeAns".into(),
            uplink: false,
            command: Payload::AppTimeAns(AppTimeAnsPayload {
                time_correction: 1024,
                param: AppTimeAnsPayloadParam { token_ans: 15 },
            }),
            bytes: vec![0x01, 0x00, 0x04, 0x00, 0x00, 0x0f],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode AppTimeAns".into(),
            uplink: false,
            command: Payload::AppTimeAns(AppTimeAnsPayload {
                time_correction: 1024,
                param: AppTimeAnsPayloadParam { token_ans: 15 },
            }),
            bytes: vec![0x01, 0x00, 0x04, 0x00, 0x00, 0x0f],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_device_app_time_periodicity_req() {
        let encode_tests = [CommandTest {
            name: "encode DeviceAppTimePeropdicityReq".into(),
            uplink: false,
            command: Payload::DeviceAppTimePeropdicityReq(DeviceAppTimePeriodicityReqPayload {
                period: 15,
            }),
            bytes: vec![0x02, 0x0f],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode DeviceAppTimePeropdicityReq".into(),
            uplink: false,
            command: Payload::DeviceAppTimePeropdicityReq(DeviceAppTimePeriodicityReqPayload {
                period: 15,
            }),
            bytes: vec![0x02, 0x0f],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_device_app_time_periodicity_ans() {
        let encode_tests = [CommandTest {
            name: "encode DeviceAppTimePeropdicityAns".into(),
            uplink: true,
            command: Payload::DeviceAppTimePeropdicityAns(DeviceAppTimePeriodicityAnsPayload {
                status: DeviceAppTimePeriodicityAnsPayloadStatus {
                    not_supported: true,
                },
                time: 1024,
            }),
            bytes: vec![0x02, 0x01, 0x00, 0x04, 0x00, 0x00],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode DeviceAppTimePeropdicityAns".into(),
            uplink: true,
            command: Payload::DeviceAppTimePeropdicityAns(DeviceAppTimePeriodicityAnsPayload {
                status: DeviceAppTimePeriodicityAnsPayloadStatus {
                    not_supported: true,
                },
                time: 1024,
            }),
            bytes: vec![0x02, 0x01, 0x00, 0x04, 0x00, 0x00],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_force_device_resync_req() {
        let encode_tests = [CommandTest {
            name: "encode ForceDeviceResyncReq".into(),
            uplink: false,
            command: Payload::ForceDeviceResyncReq(ForceDeviceResyncReqPayload {
                force_conf: ForceDeviceResyncReqPayloadForceConf {
                    nb_transmissions: 7,
                },
            }),
            bytes: vec![0x03, 0x07],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode ForceDeviceResyncReq".into(),
            uplink: false,
            command: Payload::ForceDeviceResyncReq(ForceDeviceResyncReqPayload {
                force_conf: ForceDeviceResyncReqPayloadForceConf {
                    nb_transmissions: 7,
                },
            }),
            bytes: vec![0x03, 0x07],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    fn run_tests_encode(tests: &[CommandTest]) {
        for tst in tests {
            println!("> {}", tst.name);
            let resp = tst.command.to_vec();
            if let Some(e) = &tst.expected_error {
                assert!(resp.is_err());
                assert_eq!(e, &resp.err().unwrap().to_string());
            } else {
                assert_eq!(tst.bytes, resp.unwrap());
            }
        }
    }

    fn run_tests_decode(tests: &[CommandTest]) {
        for tst in tests {
            println!("> {}", tst.name);
            let resp = Payload::from_slice(tst.uplink, &tst.bytes);
            if let Some(e) = &tst.expected_error {
                assert!(resp.is_err());
                assert_eq!(e, &resp.err().unwrap().to_string());
            } else {
                assert_eq!(tst.command, resp.unwrap());
            }
        }
    }
}
