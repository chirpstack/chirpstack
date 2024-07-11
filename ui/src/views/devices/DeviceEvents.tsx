import { useEffect, useState, useCallback } from "react";

import type { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import type { LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { StreamDeviceEventsRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import InternalStore from "../../stores/InternalStore";
import LogTable from "../../components/LogTable";

interface IProps {
  device: Device;
}

function DeviceEvents(props: IProps) {
  const [events, setEvents] = useState<LogItem[]>([]);

  const onMessage = useCallback((l: LogItem) => {
    setEvents(e => {
      if (e.length === 0 || parseInt(l.getId().replace("-", "")) > parseInt(e[0].getId().replace("-", ""))) {
        return [l, ...e];
      }
      return e;
    });
  }, []);

  useEffect(() => {
    const req = new StreamDeviceEventsRequest();
    req.setDevEui(props.device.getDevEui());

    const cancelFunc = InternalStore.streamDeviceEvents(req, onMessage);

    return () => {
      cancelFunc();
    };
  }, [props, onMessage]);

  return <LogTable logs={events} />;
}

export default DeviceEvents;
