import React, { useEffect, useState, useCallback } from "react";

import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { StreamDeviceEventsRequest, LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

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
    let req = new StreamDeviceEventsRequest();
    req.setDevEui(props.device.getDevEui());

    let cancelFunc = InternalStore.streamDeviceEvents(req, onMessage);

    return () => {
      cancelFunc();
    };
  }, [props, onMessage]);

  return <LogTable logs={events} />;
}

export default DeviceEvents;
