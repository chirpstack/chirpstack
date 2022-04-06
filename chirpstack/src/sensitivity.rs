pub fn calculate_sensitivity(bandwidth_hz: u32, noise_figure: f32, snr: f32) -> f32 {
    // see also: http://www.techplayon.com/lora-link-budget-sensitivity-calculations-example-explained/
    let log_bw = 10.0 * (bandwidth_hz as f32).log10();
    -174.0 + log_bw + (noise_figure + snr)
}

pub fn calculate_link_budget(bandwidth_hz: u32, noise_figure: f32, snr: f32, tx_power: f32) -> f32 {
    tx_power - calculate_sensitivity(bandwidth_hz, noise_figure, snr)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sensitivity() {
        let s = calculate_sensitivity(125000, 6.0, -20.0);
        assert_eq!(-137, s as isize);
    }

    #[test]
    fn test_link_budget() {
        let lb = calculate_link_budget(125000, 6.0, -20.0, 17.0);
        assert_eq!(154, lb as isize);
    }
}
