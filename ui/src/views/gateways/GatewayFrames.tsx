import React, { useState, useEffect } from "react";

import { Gateway } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { StreamGatewayFramesRequest, LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import InternalStore from "../../stores/InternalStore";
import LogTable from "../../components/LogTable";

interface IProps {
  gateway: Gateway;
}

function GatewayFrames(props: IProps) {
  const [frames, setFrames] = useState<LogItem[]>([]);

  useEffect(() => {
    const onMessage = (l: LogItem) => {
      setFrames(f => {
        if (f.length === 0 || parseInt(l.getId().replace("-", "")) > parseInt(f[0].getId().replace("-", ""))) {
          f.unshift(l);
        }
        return f;
      });
    };

    let req = new StreamGatewayFramesRequest();
    req.setGatewayId(props.gateway.getGatewayId());

    let cancelFunc = InternalStore.streamGatewayFrames(req, onMessage);

    return () => {
      cancelFunc();
    };
  }, [props]);

  return <LogTable logs={frames} />;
}

export default GatewayFrames;
