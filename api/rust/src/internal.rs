include!(concat!(env!("OUT_DIR"), "/internal/internal.rs"));
#[cfg(feature = "json")]
include!(concat!(env!("OUT_DIR"), "/internal/internal.serde.rs"));

impl DeviceSession {
    pub fn get_a_f_cnt_down(&self) -> u32 {
        if self.mac_version().to_string().starts_with("1.0") {
            // LoRaWAN 1.0
            self.n_f_cnt_down
        } else {
            // LoRaWAN 1.1
            self.a_f_cnt_down
        }
    }

    pub fn set_a_f_cnt_down(&mut self, f_cnt: u32) {
        if self.mac_version().to_string().starts_with("1.0") {
            // LoRaWAN 1.0
            self.n_f_cnt_down = f_cnt;
        } else {
            // LoRaWAN 1.1
            self.a_f_cnt_down = f_cnt;
        }
    }
}
