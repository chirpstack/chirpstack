use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::{Aes128, Block};
use anyhow::Result;

use crate::{AES128Key, NetID, EUI64};

/// For LoRaWAN 1.0: SNwkSIntKey = NwkSEncKey = FNwkSIntKey = NwkSKey
pub fn get_f_nwk_s_int_key(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x01, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}

pub fn get_app_s_key(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x02, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}

pub fn get_s_nwk_s_int_key(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x03, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}

pub fn get_nwk_s_enc_key(
    opt_neg: bool,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    get_s_key(
        opt_neg, 0x04, nwk_key, net_id, join_eui, join_nonce, dev_nonce,
    )
}

pub fn get_js_enc_key(dev_eui: &EUI64, nwk_key: &AES128Key) -> Result<AES128Key> {
    get_js_key(0x05, dev_eui, nwk_key)
}

pub fn get_js_int_key(dev_eui: &EUI64, nwk_key: &AES128Key) -> Result<AES128Key> {
    get_js_key(0x06, dev_eui, nwk_key)
}

/// Note: For LoRaWAN 1.0.x, use the NwkSKey as nwk_s_enc_key.
pub fn get_root_wor_s_key(nwk_s_enc_key: &AES128Key) -> Result<AES128Key> {
    let key_bytes = nwk_s_enc_key.to_bytes();
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut b: [u8; 16] = [0; 16];
    b[0] = 0x01;

    let block = Block::from_mut_slice(&mut b);
    cipher.encrypt_block(block);
    Ok(AES128Key::from_slice(block)?)
}

fn get_s_key(
    opt_neg: bool,
    typ: u8,
    nwk_key: &AES128Key,
    net_id: &NetID,
    join_eui: &EUI64,
    join_nonce: u32,
    dev_nonce: u16,
) -> Result<AES128Key> {
    let key_bytes = nwk_key.to_bytes();
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut b: [u8; 16] = [0; 16];

    b[0] = typ;
    if opt_neg {
        b[1..4].clone_from_slice(&join_nonce.to_le_bytes()[0..3]);
        b[4..12].clone_from_slice(&join_eui.to_le_bytes());
        b[12..14].clone_from_slice(&dev_nonce.to_le_bytes()[0..2]);
    } else {
        b[1..4].clone_from_slice(&join_nonce.to_le_bytes()[0..3]);
        b[4..7].clone_from_slice(&net_id.to_le_bytes());
        b[7..9].clone_from_slice(&dev_nonce.to_le_bytes()[0..2]);
    }

    let block = Block::from_mut_slice(&mut b);
    cipher.encrypt_block(block);

    Ok(AES128Key::from_slice(block)?)
}

fn get_js_key(typ: u8, dev_eui: &EUI64, nwk_key: &AES128Key) -> Result<AES128Key> {
    let key_bytes = nwk_key.to_bytes();
    let key = GenericArray::from_slice(&key_bytes);
    let cipher = Aes128::new(key);

    let mut b: [u8; 16] = [0; 16];
    b[0] = typ;
    b[1..9].clone_from_slice(&dev_eui.to_le_bytes());

    let block = Block::from_mut_slice(&mut b);
    cipher.encrypt_block(block);

    Ok(AES128Key::from_slice(block)?)
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn nwk_key() -> AES128Key {
        AES128Key::from_bytes([
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
            0x07, 0x08,
        ])
    }

    fn app_key() -> AES128Key {
        AES128Key::from_bytes([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ])
    }

    fn join_eui() -> EUI64 {
        EUI64::from_be_bytes([0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])
    }

    fn join_nonce() -> u32 {
        65536
    }

    fn dev_nonce() -> u16 {
        258
    }

    fn net_id() -> NetID {
        NetID::from_be_bytes([0x01, 0x02, 0x03])
    }

    #[test]
    fn lorawan_1_0() {
        let nwk_s_key = get_f_nwk_s_int_key(
            false,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let app_s_key = get_app_s_key(
            false,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let root_wor_s_key = get_root_wor_s_key(&nwk_s_key).unwrap();

        assert_eq!(
            AES128Key::from_bytes([
                223, 83, 195, 95, 48, 52, 204, 206, 208, 255, 53, 76, 112, 222, 4, 223,
            ]),
            nwk_s_key
        );

        assert_eq!(
            AES128Key::from_bytes([
                146, 123, 156, 145, 17, 131, 207, 254, 76, 178, 255, 75, 117, 84, 95, 109
            ]),
            app_s_key
        );

        assert_eq!(
            AES128Key::from_bytes([
                0x60, 0xf8, 0xac, 0xd9, 0xde, 0x2c, 0xc5, 0x06, 0xfb, 0x06, 0x63, 0x94, 0x08, 0xfe,
                0x57, 0x4a
            ]),
            root_wor_s_key
        );
    }

    #[test]
    fn lorawan_1_1() {
        let app_s_key = get_app_s_key(
            true,
            &app_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let f_nwk_s_int_key = get_f_nwk_s_int_key(
            true,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let s_nwk_s_int_key = get_s_nwk_s_int_key(
            true,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        let nwk_s_enc_key = get_nwk_s_enc_key(
            true,
            &nwk_key(),
            &net_id(),
            &join_eui(),
            join_nonce(),
            dev_nonce(),
        )
        .unwrap();

        assert_eq!(
            AES128Key::from_bytes([
                1, 98, 18, 21, 209, 202, 8, 254, 191, 12, 96, 44, 194, 173, 144, 250
            ]),
            app_s_key,
        );

        assert_eq!(
            AES128Key::from_bytes([
                83, 127, 138, 174, 137, 108, 121, 224, 21, 209, 2, 208, 98, 134, 53, 78
            ]),
            f_nwk_s_int_key,
        );

        assert_eq!(
            AES128Key::from_bytes([
                88, 148, 152, 153, 48, 146, 207, 219, 95, 210, 224, 42, 199, 81, 11, 241
            ]),
            s_nwk_s_int_key,
        );

        assert_eq!(
            AES128Key::from_bytes([
                152, 152, 40, 60, 79, 102, 235, 108, 111, 213, 22, 88, 130, 4, 108, 64
            ]),
            nwk_s_enc_key,
        );
    }
}
