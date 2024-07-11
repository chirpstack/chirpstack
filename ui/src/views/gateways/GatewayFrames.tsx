import { useState, useEffect, useCallback } from "react";

import type { Gateway } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import type { LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { StreamGatewayFramesRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import InternalStore from "../../stores/InternalStore";
import LogTable from "../../components/LogTable";

interface IProps {
  gateway: Gateway;
}

function GatewayFrames(props: IProps) {
  const [frames, setFrames] = useState<LogItem[]>([]);

  const onMessage = useCallback((l: LogItem) => {
    setFrames(f => {
      if (f.length === 0 || parseInt(l.getId().replace("-", "")) > parseInt(f[0].getId().replace("-", ""))) {
        return [l, ...f];
      }
      return f;
    });
  }, []);

  useEffect(() => {
    const req = new StreamGatewayFramesRequest();
    req.setGatewayId(props.gateway.getGatewayId());
    return InternalStore.streamGatewayFrames(req, onMessage);
  }, [props, onMessage]);

  return <LogTable logs={frames} />;
}

export default GatewayFrames;
