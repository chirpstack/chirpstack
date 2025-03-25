// passthrough default codec
// Performs nothing with the uplinks, and passthrough downlinks

pub const SCRIPT: &str = r#"
/**
 * Decode uplink function
 * 
 * @param {object} input
 * @param {number[]} input.bytes Byte array containing the uplink payload, e.g. [255, 230, 255, 0]
 * @param {number} input.fPort Uplink fPort.
 * @param {Record<string, string>} input.variables Object containing the configured device variables.
 * 
 * @returns {{data: object}} Object representing the decoded payload.
 */
export function decodeUplink(input) {
  return {
    data: {
      // Empty object
    }
  };
}

/**
 * Encode downlink function.
 * 
 * @param {object} input
 * @param {object} input.data Object representing the payload that must be encoded.
 * @param {Record<string, string>} input.variables Object containing the configured device variables.
 * 
 * @returns {{bytes: number[]}} Byte array containing the downlink payload.
 */
export function encodeDownlink(input) {
  return {
    bytes: input.data, // Passthrough
  };
}
"#;
