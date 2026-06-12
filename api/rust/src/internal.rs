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

    pub fn append_gateway_rx_info_history(
        &mut self,
        item: GatewayRxInfoHistory,
        max_history: usize,
    ) {
        if self.gateway_rx_info_history.len() >= max_history {
            let start_index = self.gateway_rx_info_history.len() - max_history + 1;
            self.gateway_rx_info_history = self.gateway_rx_info_history[start_index..].to_vec();
        }

        self.gateway_rx_info_history.push(item);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_append_gateway_rx_info_history() {
        let mut ds = DeviceSession::default();

        for _i in 0..10 {
            ds.append_gateway_rx_info_history(
                GatewayRxInfoHistory {
                    dr: 0,
                    items: vec![],
                },
                8,
            );
        }

        assert_eq!(8, ds.gateway_rx_info_history.len());
    }
}
