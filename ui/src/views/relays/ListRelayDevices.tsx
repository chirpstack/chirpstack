import React, { Component } from "react";

import { Space, Button } from "antd";
import { ColumnsType } from "antd/es/table";

import {
  Device,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import {
  ListRelayDevicesRequest,
  ListRelayDevicesResponse,
  RelayDeviceListItem,
  RemoveRelayDeviceRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import RelayStore from "../../stores/RelayStore";

interface IProps {
  relayDevice: Device;
}

interface IState {
  selectedRowIds: string[];
  refreshKey: number;
}

class ListRelayDevices extends Component<IProps, IState> {
  columns = (): ColumnsType<RelayDeviceListItem.AsObject> => {
    return [
      {
        title: "Name",
        dataIndex: "name",
        key: "name",
      },
      {
        title: "DevEUI",
        dataIndex: "devEui",
        key: "devEui",
        width: 250,
      },
    ];
  }

  constructor(props: IProps) {
    super(props);

    this.state = {
      selectedRowIds: [],
      refreshKey: 1,
    };
  }

  onRowsSelectChange = (ids: string[]) => {
    this.setState({
      selectedRowIds: ids,
    });
  }

  removeRelayDevices = () => {
    if (!window.confirm("Are you sure you want to remove the selected devices from the relay?")) {
      return;
    }

    let count = 0;
    let self = this;

    for (let devEui of this.state.selectedRowIds) {
      count++;

      let req = new RemoveRelayDeviceRequest();
      req.setRelayDevEui(this.props.relayDevice.getDevEui());
      req.setDeviceDevEui(devEui);

      let cbFunc = (cnt: number) => {
        return () => {
          if (cnt === self.state.selectedRowIds.length) {
            self.setState({
              refreshKey: self.state.refreshKey + 1,
            });
          }
        }
      };

      RelayStore.removeDevice(req, cbFunc(count));
    }
  }

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListRelayDevicesRequest();
    req.setRelayDevEui(this.props.relayDevice.getDevEui());
    req.setLimit(limit);
    req.setOffset(offset);

    RelayStore.listDevices(req, (resp: ListRelayDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  }

  render() {
    return(
      <Space direction="vertical" size="large" style={{ width: "100%" }}>
        <Space direction="horizontal" style={{ float: "right" }}>
          <Button onClick={this.removeRelayDevices} disabled={this.state.selectedRowIds.length === 0}>
            Remove remove from relay
          </Button>
        </Space>
        <DataTable
          columns={this.columns()}
          getPage={this.getPage}
          onRowsSelectChange={this.onRowsSelectChange}
          rowKey="devEui"
          refreshKey={this.state.refreshKey}
        />
      </Space>
    );
  }
}

export default ListRelayDevices;
