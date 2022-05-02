import React, { Component } from "react";

import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { StreamDeviceEventsRequest, LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import InternalStore from "../../stores/InternalStore";
import LogTable from "../../components/LogTable";

interface IProps {
  device: Device;
}

interface IState {
  events: LogItem[];
  cancelFunc?: () => void;
}

class DeviceEvents extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      events: [],
      cancelFunc: undefined,
    };
  }

  componentDidMount() {
    this.connectStream();
  }

  componentWillUnmount() {
    if (this.state.cancelFunc !== undefined) {
      this.state.cancelFunc();
    }
  }

  connectStream = () => {
    let req = new StreamDeviceEventsRequest();
    req.setDevEui(this.props.device.getDevEui());

    let cancelFunc = InternalStore.streamDeviceEvents(req, this.onMessage);
    this.setState({
      cancelFunc: cancelFunc,
    });
  };

  onMessage = (l: LogItem) => {
    let events = this.state.events;

    if (events.length === 0 || parseInt(l.getId().replace("-", "")) > parseInt(events[0].getId().replace("-", ""))) {
      events.unshift(l);
      this.setState({
        events: events,
      });
    }
  };

  render() {
    return <LogTable logs={this.state.events} />;
  }
}

export default DeviceEvents;
