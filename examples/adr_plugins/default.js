export function name() {
  return "JS example for default ADR algorithm";
}

export function id() {
  return "js_example_default";
}

export function handle(req) {
  let resp = {
    dr: req.dr,
    txPowerIndex: req.txPowerIndex,
    nbTrans: req.nbTrans,
  };

  if (!req.adr) {
    return resp;
  }

  if (req.dr > req.maxDr) {
    resp.dr = req.maxDr;
  }

  // Set the new Nb Trans.
  resp.nbTrans = getNbTrans(req.nbTrans, getPacketLossPercentage(req));

  // Calculate the number of steps.
  let snrMax = getMaxSnr(req);
  let snrMargin = snrMax - req.requiredSnrForDr - req.installationMargin;
  let nStep = Math.floor(snrMargin / 3);

  // In case of negative steps the ADR algorithm will increase the TxPower
  // if possible. To avoid up / down / up / down TxPower changes, wait until
  // we have at least the required number of uplink history elements.
  if (nStep < 0 && getHistoryCount(req) != requiredHistoryCount()) {
    return resp;
  }

  let [desiredTxPowerIndex, desiredDr] = getIdealTxPowerIndexAndDr(
    nStep,
    resp.txPowerIndex,
    resp.dr,
    req.maxTxPowerIndex,
    req.maxDr,
  );

  resp.dr = desiredDr;
  resp.txPowerIndex = desiredTxPowerIndex;

  return resp;
}

function getIdealTxPowerIndexAndDr(nbStep, txPowerIndex, dr, maxTxPowerIndex, maxDr) {
  while (nbStep !== 0) {
    if (nbStep > 0) {
      if (dr < maxDr) {
        // Increase the DR.
        dr++;
      } else if (txPowerIndex < maxTxPowerIndex) {
        // Decrease the Tx Power.
        txPowerIndex++;
      }
      nbStep--;
    } else {
      // Incease the TxPower.
      if (txPowerIndex > 0) {
        txPowerIndex--;
      }
      nbStep++;
    }
  }

  return [txPowerIndex, dr];
}

function requiredHistoryCount() {
  return 20;
}

function getHistoryCount(req) {
  let count = 0;
  for (let uh of req.uplinkHistory) {
    if (uh.txPowerIndex === req.txPowerIndex) {
      count++;
    }
  }

  return count;
}

function getMaxSnr(req) {
  let maxSnr = -999.0;

  for (let uh of req.uplinkHistory) {
    if (uh.maxSnr > maxSnr) {
      maxSnr = uh.maxSnr;
    }
  }

  return maxSnr;
}

function getNbTrans(currentNbTrans, pktLossRate) {
  const pktLossTable = [
    [1, 1, 2],
    [1, 2, 3],
    [2, 3, 3],
    [3, 3, 3],
  ];

  if (currentNbTrans < 1) {
    currentNbTrans = 1;
  }
  if (currentNbTrans > 3) {
    currentNbTrans = 3;
  }

  const nbTransIndex = currentNbTrans - 1;

  if (pktLossRate < 5.0) {
    return pktLossTable[0][nbTransIndex];
  } else if (pktLossRate < 10.0) {
    return pktLossTable[1][nbTransIndex];
  } else if (pktLossRate < 30.0) {
    return pktLossTable[2][nbTransIndex];
  }

  return pktLossTable[3][nbTransIndex];
}

function getPacketLossPercentage(req) {
  if (req.uplinkHistory.length < requiredHistoryCount()) {
    return 0.0;
  }

  let lostPackets = 0;
  let previousFCnt = req.uplinkHistory[0].fCnt;

  for (let uh of req.uplinkHistory.slice(1)) {
    lostPackets += uh.fCnt - previousFCnt - 1;
    previousFCnt = uh.fCnt;
  }

  return lostPackets / req.uplinkHistory.length * 100.0;
}
