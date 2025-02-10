use anyhow::Result;

use crate::applayer::PayloadCodec;

pub enum Cid {
    PackageVersionReq,
    PackageVersionAns,
    FragSessionStatusReq,
    FragSessionStatusAns,
    FragSessionSetupReq,
    FragSessionSetupAns,
    FragSessionDeleteReq,
    FragSessionDeleteAns,
    DataFragment,
}

impl Cid {
    pub fn from_u8(uplink: bool, value: u8) -> Result<Self> {
        Ok(match uplink {
            true => match value {
                0x00 => Cid::PackageVersionAns,
                0x01 => Cid::FragSessionStatusAns,
                0x02 => Cid::FragSessionSetupAns,
                0x03 => Cid::FragSessionDeleteAns,
                _ => return Err(anyhow!("Invalid CID: {}", value)),
            },
            false => match value {
                0x00 => Cid::PackageVersionReq,
                0x01 => Cid::FragSessionStatusReq,
                0x02 => Cid::FragSessionSetupReq,
                0x03 => Cid::FragSessionDeleteReq,
                0x08 => Cid::DataFragment,
                _ => return Err(anyhow!("Invalid CID: {}", value)),
            },
        })
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Cid::PackageVersionReq | Cid::PackageVersionAns => 0x00,
            Cid::FragSessionStatusReq | Cid::FragSessionStatusAns => 0x01,
            Cid::FragSessionSetupReq | Cid::FragSessionSetupAns => 0x02,
            Cid::FragSessionDeleteReq | Cid::FragSessionDeleteAns => 0x03,
            Cid::DataFragment => 0x08,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Payload {
    PackageVersionReq,
    PackageVersionAns(PackageVersionAnsPayload),
    FragSessionStatusReq(FragSessionStatusReqPayload),
    FragSessionStatusAns(FragSessionStatusAnsPayload),
    FragSessionSetupReq(FragSessionSetupReqPayload),
    FragSessionSetupAns(FragSessionSetupAnsPayload),
    FragSessionDeleteReq(FragSessionDeleteReqPayload),
    FragSessionDeleteAns(FragSessionDeleteAnsPayload),
    DataFragment(DataFragmentPayload),
}

impl Payload {
    pub fn cid(&self) -> Cid {
        match self {
            Self::PackageVersionReq => Cid::PackageVersionReq,
            Self::PackageVersionAns(_) => Cid::PackageVersionAns,
            Self::FragSessionStatusReq(_) => Cid::FragSessionStatusReq,
            Self::FragSessionStatusAns(_) => Cid::FragSessionStatusAns,
            Self::FragSessionSetupReq(_) => Cid::FragSessionSetupReq,
            Self::FragSessionSetupAns(_) => Cid::FragSessionSetupAns,
            Self::FragSessionDeleteReq(_) => Cid::FragSessionDeleteReq,
            Self::FragSessionDeleteAns(_) => Cid::FragSessionDeleteAns,
            Self::DataFragment(_) => Cid::DataFragment,
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
            Cid::FragSessionStatusReq => {
                Payload::FragSessionStatusReq(FragSessionStatusReqPayload::decode(&b[1..])?)
            }
            Cid::FragSessionStatusAns => {
                Payload::FragSessionStatusAns(FragSessionStatusAnsPayload::decode(&b[1..])?)
            }
            Cid::FragSessionSetupReq => {
                Payload::FragSessionSetupReq(FragSessionSetupReqPayload::decode(&b[1..])?)
            }
            Cid::FragSessionSetupAns => {
                Payload::FragSessionSetupAns(FragSessionSetupAnsPayload::decode(&b[1..])?)
            }
            Cid::FragSessionDeleteReq => {
                Payload::FragSessionDeleteReq(FragSessionDeleteReqPayload::decode(&b[1..])?)
            }
            Cid::FragSessionDeleteAns => {
                Payload::FragSessionDeleteAns(FragSessionDeleteAnsPayload::decode(&b[1..])?)
            }
            Cid::DataFragment => Payload::DataFragment(DataFragmentPayload::decode(&b[1..])?),
        })
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut out = vec![self.cid().to_u8()];

        match self {
            Self::PackageVersionReq => {}
            Self::PackageVersionAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::FragSessionStatusReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::FragSessionStatusAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::FragSessionSetupReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::FragSessionSetupAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::FragSessionDeleteReq(pl) => out.extend_from_slice(&pl.encode()?),
            Self::FragSessionDeleteAns(pl) => out.extend_from_slice(&pl.encode()?),
            Self::DataFragment(pl) => out.extend_from_slice(&pl.encode()?),
        }

        Ok(out)
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct FragSessionStatusReqPayload {
    pub participants: bool,
    pub frag_index: u8,
}

impl PayloadCodec for FragSessionStatusReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(FragSessionStatusReqPayload {
            participants: b[0] & 0x01 != 0,
            frag_index: (b[0] >> 1) & 0x03,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.frag_index > 3 {
            return Err(anyhow!("Max frag_index value is 3"));
        }

        let mut b = vec![self.frag_index << 1];
        if self.participants {
            b[0] |= 0x01;
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct FragSessionStatusAnsPayload {
    pub received_and_index: FragSessionStatusAnsPayloadReceivedAndIndex,
    pub missing_frag: u8,
    pub status: FragSessionStatusAnsPayloadStatus,
}

impl PayloadCodec for FragSessionStatusAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 4 {
            return Err(anyhow!("Expected 4 bytes"));
        }

        Ok(FragSessionStatusAnsPayload {
            received_and_index: FragSessionStatusAnsPayloadReceivedAndIndex {
                nb_frag_received: {
                    let mut bytes = [0; 2];
                    bytes.copy_from_slice(&b[0..2]);
                    u16::from_le_bytes(bytes) & 0x3fff
                },
                frag_index: b[1] >> 6,
            },
            missing_frag: b[2],
            status: FragSessionStatusAnsPayloadStatus {
                not_enough_matrix_memory: b[3] & 0x01 != 0,
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.received_and_index.nb_frag_received > 16383 {
            return Err(anyhow!("Max nb_frag_received value us 16383"));
        }

        if self.received_and_index.frag_index > 3 {
            return Err(anyhow!("Max frag_index value is 3"));
        }

        let mut b = Vec::with_capacity(4);
        b.extend_from_slice(&self.received_and_index.nb_frag_received.to_le_bytes());
        b[1] |= self.received_and_index.frag_index << 6;

        b.push(self.missing_frag);

        b.push(0x00);
        if self.status.not_enough_matrix_memory {
            b[3] |= 0x01;
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct FragSessionStatusAnsPayloadStatus {
    pub not_enough_matrix_memory: bool,
}

#[derive(Debug, PartialEq)]
pub struct FragSessionStatusAnsPayloadReceivedAndIndex {
    pub nb_frag_received: u16,
    pub frag_index: u8,
}

#[derive(Debug, PartialEq)]
pub struct FragSessionSetupReqPayload {
    pub frag_session: FragSessionSetuReqPayloadFragSession,
    pub nb_frag: u16,
    pub frag_size: u8,
    pub control: FragSessionSetuReqPayloadControl,
    pub padding: u8,
    pub descriptor: [u8; 4],
}

impl PayloadCodec for FragSessionSetupReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 10 {
            return Err(anyhow!("Expected 10 bytes"));
        }

        Ok(FragSessionSetupReqPayload {
            frag_session: FragSessionSetuReqPayloadFragSession {
                mc_group_bit_mask: {
                    let mut mask = [false; 4];
                    for (i, v) in mask.iter_mut().enumerate() {
                        *v = b[0] & 1 << i != 0;
                    }
                    mask
                },
                frag_index: (b[0] >> 4) & 0x03,
            },
            nb_frag: {
                let mut bytes = [0; 2];
                bytes.copy_from_slice(&b[1..3]);
                u16::from_le_bytes(bytes)
            },
            frag_size: b[3],
            control: FragSessionSetuReqPayloadControl {
                block_ack_delay: b[4] & 0x07,
                fragmentation_matrix: (b[4] >> 3) & 0x07,
            },
            padding: b[5],
            descriptor: {
                let mut bytes = [0; 4];
                bytes.copy_from_slice(&b[6..10]);
                bytes
            },
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.frag_session.frag_index > 3 {
            return Err(anyhow!("Max frag_index value is 3"));
        }

        if self.control.block_ack_delay > 7 {
            return Err(anyhow!("Max block_ack_delay value is 7"));
        }

        if self.control.fragmentation_matrix > 7 {
            return Err(anyhow!("Max fragmentation_matrix value is 7"));
        }

        let mut b = Vec::with_capacity(10);

        b.push(self.frag_session.frag_index << 4);
        for (i, v) in self.frag_session.mc_group_bit_mask.iter().enumerate() {
            if *v {
                b[0] |= 1 << i;
            }
        }
        b.extend_from_slice(&self.nb_frag.to_le_bytes());
        b.push(self.frag_size);
        b.push(self.control.block_ack_delay | (self.control.fragmentation_matrix << 3));
        b.push(self.padding);
        b.extend_from_slice(&self.descriptor);

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct FragSessionSetuReqPayloadFragSession {
    pub mc_group_bit_mask: [bool; 4],
    pub frag_index: u8,
}

#[derive(Debug, PartialEq)]
pub struct FragSessionSetuReqPayloadControl {
    pub block_ack_delay: u8,
    pub fragmentation_matrix: u8,
}

#[derive(Debug, PartialEq)]
pub struct FragSessionSetupAnsPayload {
    pub encoding_unsupported: bool,
    pub not_enough_memory: bool,
    pub frag_session_index_not_supported: bool,
    pub wrong_descriptor: bool,
    pub frag_index: u8,
}

impl PayloadCodec for FragSessionSetupAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(FragSessionSetupAnsPayload {
            encoding_unsupported: b[0] & 0x01 != 0,
            not_enough_memory: b[0] & 0x02 != 0,
            frag_session_index_not_supported: b[0] & 0x04 != 0,
            wrong_descriptor: b[0] & 0x08 != 0,
            frag_index: (b[0] >> 6),
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.frag_index > 3 {
            return Err(anyhow!("Max frag_index value is 3"));
        }

        let mut b = vec![self.frag_index << 6];
        if self.encoding_unsupported {
            b[0] |= 0x01;
        }
        if self.not_enough_memory {
            b[0] |= 0x02;
        }
        if self.frag_session_index_not_supported {
            b[0] |= 0x04;
        }
        if self.wrong_descriptor {
            b[0] |= 0x08;
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct FragSessionDeleteReqPayload {
    pub frag_index: u8,
}

impl PayloadCodec for FragSessionDeleteReqPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(FragSessionDeleteReqPayload {
            frag_index: b[0] & 0x03,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.frag_index > 3 {
            return Err(anyhow!("Max frag_index value is 3"));
        }

        Ok(vec![self.frag_index])
    }
}

#[derive(Debug, PartialEq)]
pub struct FragSessionDeleteAnsPayload {
    pub frag_index: u8,
    pub session_does_not_exist: bool,
}

impl PayloadCodec for FragSessionDeleteAnsPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() != 1 {
            return Err(anyhow!("Expected 1 byte"));
        }

        Ok(FragSessionDeleteAnsPayload {
            frag_index: b[0] & 0x03,
            session_does_not_exist: b[0] & 0x04 != 0,
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.frag_index > 3 {
            return Err(anyhow!("Max frag_index value is 3"));
        }

        let mut b = vec![self.frag_index];
        if self.session_does_not_exist {
            b[0] |= 0x04;
        }

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct DataFragmentPayload {
    pub index_and_n: DataFragmentPayloadIndexAndN,
    pub data: Vec<u8>,
}

impl PayloadCodec for DataFragmentPayload {
    fn decode(b: &[u8]) -> Result<Self> {
        if b.len() < 2 {
            return Err(anyhow!("At least 2 bytes expected"));
        }

        Ok(DataFragmentPayload {
            index_and_n: DataFragmentPayloadIndexAndN {
                n: {
                    let mut bytes = [0; 2];
                    bytes.copy_from_slice(&b[0..2]);
                    u16::from_le_bytes(bytes) & 0x3fff
                },
                frag_index: b[1] >> 6,
            },
            data: b[2..].to_vec(),
        })
    }

    fn encode(&self) -> Result<Vec<u8>> {
        if self.index_and_n.n > 16383 {
            return Err(anyhow!("Max n value us 16383"));
        }

        if self.index_and_n.frag_index > 3 {
            return Err(anyhow!("Max frag_index value is 3"));
        }

        let mut b = Vec::with_capacity(2 + self.data.len());
        b.extend_from_slice(&self.index_and_n.n.to_le_bytes());
        b[1] |= self.index_and_n.frag_index << 6;
        b.extend_from_slice(&self.data);

        Ok(b)
    }
}

#[derive(Debug, PartialEq)]
pub struct DataFragmentPayloadIndexAndN {
    pub n: u16,
    pub frag_index: u8,
}

// Encode the given slice of bytes to fragments including forward error correction.
// This is based on the proposed FEC code from the Fragmented Data Block Transport over
// LoRaWAN recommendation.
pub fn encode(payload: &[u8], fragment_size: usize, redundancy: usize) -> Result<Vec<Vec<u8>>> {
    if payload.len() % fragment_size != 0 {
        return Err(anyhow!("Payload size must be a multiple of fragment_size"));
    }

    // fragment the data into rows
    let mut data_rows: Vec<Vec<u8>> = payload.chunks(fragment_size).map(|v| v.to_vec()).collect();
    let w = data_rows.len();

    for y in 0..redundancy {
        let mut s = vec![0; fragment_size];
        let a = matrix_line(y + 1, w);

        for x in 0..w {
            if a[x] == 1 {
                for (m, s_val) in s.iter_mut().enumerate() {
                    *s_val ^= data_rows[x][m];
                }
            }
        }

        data_rows.push(s);
    }

    Ok(data_rows)
}

fn prbs23(x: usize) -> usize {
    let b0 = x & 1;
    let b1 = (x & 32) / 32;
    (x / 2) + (b0 ^ b1) * (1 << 22)
}

fn is_power_2(num: usize) -> bool {
    num != 0 && (num & (num - 1)) == 0
}

fn matrix_line(n: usize, m: usize) -> Vec<usize> {
    let mut line = vec![0; m];

    let mm = if is_power_2(m) { 1 } else { 0 };

    let mut x = 1 + (1001 * n);

    for _nb_coeff in 0..(m / 2) {
        let mut r = 1 << 16;
        while r >= m {
            x = prbs23(x);
            r = x % (m + mm);
        }
        line[r] = 1;
    }

    line
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
    fn test_frag_session_status_req() {
        let encode_tests = [CommandTest {
            name: "encode FragSessionStatusReq".into(),
            uplink: false,
            command: Payload::FragSessionStatusReq(FragSessionStatusReqPayload {
                participants: true,
                frag_index: 2,
            }),
            bytes: vec![0x01, 0x05],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode FragSessionStatusReq".into(),
            uplink: false,
            command: Payload::FragSessionStatusReq(FragSessionStatusReqPayload {
                participants: true,
                frag_index: 2,
            }),
            bytes: vec![0x01, 0x05],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_frag_session_status_ans() {
        let encode_tests = [CommandTest {
            name: "encode FragSessionStatusAns".into(),
            uplink: true,
            command: Payload::FragSessionStatusAns(FragSessionStatusAnsPayload {
                received_and_index: FragSessionStatusAnsPayloadReceivedAndIndex {
                    nb_frag_received: 1024,
                    frag_index: 3,
                },
                missing_frag: 128,
                status: FragSessionStatusAnsPayloadStatus {
                    not_enough_matrix_memory: true,
                },
            }),
            bytes: vec![0x01, 0x00, 0xc4, 0x80, 0x01],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode FragSessionStatusAns".into(),
            uplink: true,
            command: Payload::FragSessionStatusAns(FragSessionStatusAnsPayload {
                received_and_index: FragSessionStatusAnsPayloadReceivedAndIndex {
                    nb_frag_received: 1024,
                    frag_index: 3,
                },
                missing_frag: 128,
                status: FragSessionStatusAnsPayloadStatus {
                    not_enough_matrix_memory: true,
                },
            }),
            bytes: vec![0x01, 0x00, 0xc4, 0x80, 0x01],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_frag_session_setup_req() {
        let encode_tests = [CommandTest {
            name: "encode FragSessionSetupReq".into(),
            uplink: false,
            command: Payload::FragSessionSetupReq(FragSessionSetupReqPayload {
                frag_session: FragSessionSetuReqPayloadFragSession {
                    mc_group_bit_mask: [true, false, false, false],
                    frag_index: 3,
                },
                nb_frag: 1024,
                frag_size: 128,
                control: FragSessionSetuReqPayloadControl {
                    block_ack_delay: 5,
                    fragmentation_matrix: 1,
                },
                padding: 64,
                descriptor: [0x01, 0x02, 0x03, 0x04],
            }),
            bytes: vec![
                0x02, 0x31, 0x00, 0x04, 0x80, 0x0d, 0x40, 0x01, 0x02, 0x03, 0x04,
            ],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "encode FragSessionSetupReq".into(),
            uplink: false,
            command: Payload::FragSessionSetupReq(FragSessionSetupReqPayload {
                frag_session: FragSessionSetuReqPayloadFragSession {
                    mc_group_bit_mask: [true, false, false, false],
                    frag_index: 3,
                },
                nb_frag: 1024,
                frag_size: 128,
                control: FragSessionSetuReqPayloadControl {
                    block_ack_delay: 5,
                    fragmentation_matrix: 1,
                },
                padding: 64,
                descriptor: [0x01, 0x02, 0x03, 0x04],
            }),
            bytes: vec![
                0x02, 0x31, 0x00, 0x04, 0x80, 0x0d, 0x40, 0x01, 0x02, 0x03, 0x04,
            ],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_frag_session_setup_ans() {
        let encode_tests = [CommandTest {
            name: "encode FragSessionSetupAns".into(),
            uplink: true,
            command: Payload::FragSessionSetupAns(FragSessionSetupAnsPayload {
                encoding_unsupported: true,
                not_enough_memory: true,
                frag_session_index_not_supported: false,
                wrong_descriptor: true,
                frag_index: 2,
            }),
            bytes: vec![0x02, 0x8B],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode FragSessionSetupAns".into(),
            uplink: true,
            command: Payload::FragSessionSetupAns(FragSessionSetupAnsPayload {
                encoding_unsupported: true,
                not_enough_memory: true,
                frag_session_index_not_supported: false,
                wrong_descriptor: true,
                frag_index: 2,
            }),
            bytes: vec![0x02, 0x8B],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_frag_session_delete_req() {
        let encode_tests = [CommandTest {
            name: "encode FragSessionDelete".into(),
            uplink: false,
            command: Payload::FragSessionDeleteReq(FragSessionDeleteReqPayload { frag_index: 3 }),
            bytes: vec![0x03, 0x03],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode FragSessionDelete".into(),
            uplink: false,
            command: Payload::FragSessionDeleteReq(FragSessionDeleteReqPayload { frag_index: 3 }),
            bytes: vec![0x03, 0x03],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_frag_session_delete_ans() {
        let encode_tests = [CommandTest {
            name: "encode FragSessionDeleteAns".into(),
            uplink: true,
            command: Payload::FragSessionDeleteAns(FragSessionDeleteAnsPayload {
                frag_index: 3,
                session_does_not_exist: true,
            }),
            bytes: vec![0x03, 0x07],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode FragSessionDeleteAns".into(),
            uplink: true,
            command: Payload::FragSessionDeleteAns(FragSessionDeleteAnsPayload {
                frag_index: 3,
                session_does_not_exist: true,
            }),
            bytes: vec![0x03, 0x07],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_data_fragment() {
        let encode_tests = [CommandTest {
            name: "encode DataFragment".into(),
            uplink: false,
            command: Payload::DataFragment(DataFragmentPayload {
                index_and_n: DataFragmentPayloadIndexAndN {
                    n: 1024,
                    frag_index: 2,
                },
                data: vec![0x01, 0x02, 0x03, 0x04],
            }),
            bytes: vec![0x08, 0x00, 0x84, 0x01, 0x02, 0x03, 0x04],
            expected_error: None,
        }];

        let decode_tests = [CommandTest {
            name: "decode DataFragment".into(),
            uplink: false,
            command: Payload::DataFragment(DataFragmentPayload {
                index_and_n: DataFragmentPayloadIndexAndN {
                    n: 1024,
                    frag_index: 2,
                },
                data: vec![0x01, 0x02, 0x03, 0x04],
            }),
            bytes: vec![0x08, 0x00, 0x84, 0x01, 0x02, 0x03, 0x04],
            expected_error: None,
        }];

        run_tests_encode(&encode_tests);
        run_tests_decode(&decode_tests);
    }

    #[test]
    fn test_encode() {
        struct EncodeTest {
            name: String,
            data: Vec<u8>,
            fragment_size: usize,
            redundancy: usize,
            expected_fragments: Vec<Vec<u8>>,
            expected_error: Option<String>,
        }

        let mut data = vec![0; 100];
        for (i, v) in data.iter_mut().enumerate() {
            *v = i as u8;
        }

        let tests = [
            EncodeTest {
                name: "invalid fragment size".into(),
                data: vec![0; 5],
                fragment_size: 10,
                redundancy: 0,
                expected_fragments: vec![],
                expected_error: Some("Payload size must be a multiple of fragment_size".into()),
            },
            EncodeTest {
                name: "fragment size 10, redundancy 10".into(),
                data: data.clone(),
                fragment_size: 10,
                redundancy: 10,
                expected_fragments: vec![
                    vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9],
                    vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x13],
                    vec![0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d],
                    vec![0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27],
                    vec![0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31],
                    vec![0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b],
                    vec![0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45],
                    vec![0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f],
                    vec![0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59],
                    vec![0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63],
                    vec![0x26, 0x26, 0x22, 0x22, 0x2e, 0x2e, 0x22, 0x22, 0x26, 0x26],
                    vec![0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x6a, 0x6b, 0x7c, 0x7d],
                    vec![0x5c, 0x5d, 0x6e, 0x6f, 0x10, 0x11, 0x2, 0x3, 0x4, 0x5],
                    vec![0x36, 0x36, 0x32, 0x32, 0x3e, 0x3e, 0x22, 0x22, 0x36, 0x36],
                    vec![0x3a, 0x3a, 0xe, 0xe, 0xa, 0xa, 0x6, 0x6, 0xa, 0xa],
                    vec![0xe, 0xe, 0x32, 0x32, 0x36, 0x36, 0x22, 0x22, 0x3e, 0x3e],
                    vec![0x2, 0x2, 0xe, 0xe, 0x72, 0x72, 0x76, 0x76, 0x62, 0x62],
                    vec![0x1e, 0x1e, 0x1a, 0x1a, 0x66, 0x66, 0x5a, 0x5a, 0x4e, 0x4e],
                    vec![0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x30, 0x31, 0x22, 0x23],
                    vec![0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x18, 0x19, 0xa, 0xb],
                ],
                expected_error: None,
            },
            EncodeTest {
                name: "fragment size 10, redundancy 5".into(),
                data: data.clone(),
                fragment_size: 10,
                redundancy: 5,
                expected_fragments: vec![
                    vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9],
                    vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x13],
                    vec![0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d],
                    vec![0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27],
                    vec![0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31],
                    vec![0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b],
                    vec![0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45],
                    vec![0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f],
                    vec![0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59],
                    vec![0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63],
                    vec![0x26, 0x26, 0x22, 0x22, 0x2e, 0x2e, 0x22, 0x22, 0x26, 0x26],
                    vec![0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x6a, 0x6b, 0x7c, 0x7d],
                    vec![0x5c, 0x5d, 0x6e, 0x6f, 0x10, 0x11, 0x2, 0x3, 0x4, 0x5],
                    vec![0x36, 0x36, 0x32, 0x32, 0x3e, 0x3e, 0x22, 0x22, 0x36, 0x36],
                    vec![0x3a, 0x3a, 0xe, 0xe, 0xa, 0xa, 0x6, 0x6, 0xa, 0xa],
                ],
                expected_error: None,
            },
        ];

        for tst in &tests {
            println!("> {}", tst.name);

            let res = encode(&tst.data, tst.fragment_size, tst.redundancy);
            if let Some(e) = &tst.expected_error {
                assert_eq!(e, &res.err().unwrap().to_string());
            } else {
                assert_eq!(tst.expected_fragments, res.unwrap());
            }
        }
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
