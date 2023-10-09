// This must return the name of the ADR algorithm.
export function name() {
  return "Example plugin";
}

// This must return the id of the ADR algorithm.
export function id() {
  return "example_id";
}

// This handles the ADR request.
//
// Input object example:
// {
//  regionConfigId: "eu868",
//  regionCommonName: "EU868",
//  devEui: "0102030405060708",
//  macVersion: "1.0.3",
//  regParamsRevision: "A",
//  adr: true,
//  dr: 1,
//  txPowerIndex: 0,
//  nbTrans: 1,
//  maxTxPowerIndex: 15,
//  requiredSnrForDr: -17.5,
//  installationMargin: 10,
//  minDr: 0,
//  maxDr: 5,
//  skipFCntCheck: false,
//  deviceVariables: {
//    "varA": "value1",
//    "varB": "value2",
//  },
//  uplinkHistory: [
//    {
//      "fCnt": 10,
//      "maxSnr": 7.5,
//      "maxRssi": -110,
//      "txPowerIndex": 0,
//      "gatewayCount": 3
//    }
//  ]
// }
//
// This function must return an object, example:
// {
//  dr: 2,
//  txPowerIndex: 1,
//  nbTrans: 1
// }
export function handle(req) {
  return {
    dr: req.dr,
    txPowerIndex: req.txPowerIndex,
    nbTrans: req.nbTrans
  };
}
