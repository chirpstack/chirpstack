import { useState, useEffect, useCallback } from "react";

import type { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import type { LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { StreamDeviceFramesRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import InternalStore from "../../stores/InternalStore";
import LogTable from "../../components/LogTable";

interface IProps {
  device: Device;
}

function DeviceFrames(props: IProps) {
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
    const req = new StreamDeviceFramesRequest();
    req.setDevEui(props.device.getDevEui());
    return InternalStore.streamDeviceFrames(req, onMessage);
  }, [props, onMessage]);

  return <LogTable logs={frames} />;
}

export default DeviceFrames;
