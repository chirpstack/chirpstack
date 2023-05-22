use anyhow::Result;

pub fn encode_freq(freq: u32) -> Result<[u8; 3]> {
    let mut freq = freq;
    // Support LoRaWAN 2.4GHz, in which case the stepping is 200Hz:
    // See Frequency Encoding in MAC Commands
    // https://lora-developers.semtech.com/documentation/tech-papers-and-guides/physical-layer-proposal-2.4ghz/
    if freq >= 2400000000 {
        freq /= 2;
    }

    if freq / 100 >= (1 << 24) {
        return Err(anyhow!("max freq value is 2^24 - 1"));
    }
    if freq % 100 != 0 {
        return Err(anyhow!("freq must be multiple of 100"));
    }

    let mut b = [0; 3];
    b[0..3].copy_from_slice(&(freq / 100).to_le_bytes()[0..3]);
    Ok(b)
}

pub fn decode_freq(b: &[u8]) -> Result<u32> {
    if b.len() != 3 {
        return Err(anyhow!("3 bytes expected for frequency"));
    }
    let mut freq_b: [u8; 4] = [0; 4];
    freq_b[0..3].copy_from_slice(&b[0..3]);
    let mut freq = u32::from_le_bytes(freq_b);

    if freq >= 12000000 {
        // 2.4GHz frequency
        freq *= 200
    } else {
        freq *= 100
    }

    Ok(freq)
}
