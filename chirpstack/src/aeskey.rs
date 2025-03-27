use rand::RngCore;

use lrwn::AES128Key;

pub fn get_random_aes_key() -> AES128Key {
    let mut rng = rand::rng();
    let mut key: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut key);
    AES128Key::from_bytes(key)
}
