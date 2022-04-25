import React, { Component } from "react";

import { Space, Button } from "antd";
import { ColumnsType } from "antd/es/table";

import {
  ListDevicesRequest,
  ListDevicesResponse,
  DeviceListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import {
  MulticastGroup,
  RemoveDeviceFromMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import DeviceStore from "../../stores/DeviceStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";

interface IProps {
  multicastGroup: MulticastGroup;
}

interface IState {
  selectedRowIds: string[];
  refreshKey: number;
}

class ListMulticastGroupDevices extends Component<IProps, IState> {
  columns = (): ColumnsType<DeviceListItem.AsObject> => {
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
  };

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
  };

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListDevicesRequest();
    req.setApplicationId(this.props.multicastGroup.getApplicationId());
    req.setMulticastGroupId(this.props.multicastGroup.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    DeviceStore.list(req, (resp: ListDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  removeDevicesFromMulticastGroup = () => {
    if (!window.confirm("Are you sure you want to remove the selected devices from the multicast-group?")) {
      return;
    }

    let count = 0;
    let self = this;

    for (let devEui of this.state.selectedRowIds) {
      count++;

      let req = new RemoveDeviceFromMulticastGroupRequest();
      req.setMulticastGroupId(this.props.multicastGroup.getId());
      req.setDevEui(devEui);

      let cbFunc = (cnt: number) => {
        return () => {
          if (cnt === self.state.selectedRowIds.length) {
            self.setState({
              refreshKey: self.state.refreshKey + 1,
            });
          }
        };
      };

      MulticastGroupStore.removeDevice(req, cbFunc(count));
    }
  };

  render() {
    return (
      <Space direction="vertical" size="large" style={{ width: "100%" }}>
        <Space direction="horizontal" style={{ float: "right" }}>
          <Button onClick={this.removeDevicesFromMulticastGroup} disabled={this.state.selectedRowIds.length === 0}>
            Remove from multicast-group
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

export default ListMulticastGroupDevices;
