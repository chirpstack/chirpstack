import { notification } from "antd";
import { MacVersion, RegParamsRevision } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

export function formatMacVersion(m: MacVersion) {
  switch (m) {
    case MacVersion.LORAWAN_1_0_0:
      return "LoRaWAN 1.0.0";
    case MacVersion.LORAWAN_1_0_1:
      return "LoRaWAN 1.0.1";
    case MacVersion.LORAWAN_1_0_2:
      return "LoRaWAN 1.0.2";
    case MacVersion.LORAWAN_1_0_3:
      return "LoRaWAN 1.0.3";
    case MacVersion.LORAWAN_1_0_4:
      return "LoRaWAN 1.0.4";
    case MacVersion.LORAWAN_1_1_0:
      return "LoRaWAN 1.1.0";
  }

  return "";
}

export function formatRegParamsRevision(r: RegParamsRevision) {
  switch (r) {
    case RegParamsRevision.A:
      return "A";
    case RegParamsRevision.B:
      return "B";
    case RegParamsRevision.RP002_1_0_0:
      return "RP002-1.0.0";
    case RegParamsRevision.RP002_1_0_1:
      return "RP002-1.0.1";
    case RegParamsRevision.RP002_1_0_2:
      return "RP002-1.0.2";
    case RegParamsRevision.RP002_1_0_3:
      return "RP002-1.0.3";
  }

  return "";
}

export function getEnumName(enums: { [key: number]: string }, index: number) {
  for (const [k, v] of Object.entries(enums)) {
    // This is weird. 'typeof v' returns 'number', but 'v === 0' errors
    // that v (string) can't be compared to number.
    let vUnknown = v as unknown;
    let vNumber = vUnknown as number;
    if (vNumber === index) {
      return k;
    }
  }
  return "";
}

export function onFinishFailed() {
  notification.error({
    message: "Validation errors",
    description: "Please inspect input fields for errors",
    duration: 3,
  });
};
