// This must return the name of the codec plugin.
export function name() {
  return "Example plugin";
}

// This must return the id of the codec plugin.
export function id() {
  return "example_id";
}

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
      // temp: 22.5
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
    bytes: input.data // Passthrough
  };
}
