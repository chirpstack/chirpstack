import React, { Component } from "react";

import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { StreamDeviceFramesRequest, LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import InternalStore from "../../stores/InternalStore";
import LogTable from "../../components/LogTable";

interface IProps {
  device: Device;
}

interface IState {
  frames: LogItem[];
  cancelFunc?: () => void;
}

class DeviceFrames extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      frames: [],
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
    let req = new StreamDeviceFramesRequest();
    req.setDevEui(this.props.device.getDevEui());

    let cancelFunc = InternalStore.streamDeviceFrames(req, this.onMessage);
    this.setState({
      cancelFunc: cancelFunc,
    });
  };

  onMessage = (l: LogItem) => {
    let frames = this.state.frames;

    if (frames.length === 0 || parseInt(l.getId().replace("-", "")) > parseInt(frames[0].getId().replace("-", ""))) {
      frames.unshift(l);
      this.setState({
        frames: frames,
      });
    }
  };

  render() {
    return <LogTable logs={this.state.frames} />;
  }
}

export default DeviceFrames;
