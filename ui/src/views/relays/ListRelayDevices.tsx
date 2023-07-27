import React, { useState } from "react";

import { Space, Button } from "antd";
import { ColumnsType } from "antd/es/table";

import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
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

function ListRelayDevices(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [refreshKey, setRefreshKey] = useState<number>(0);

  const columns: ColumnsType<RelayDeviceListItem.AsObject> = [
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

  const onRowsSelectChange = (ids: string[]) => {
    setSelectedRowIds(ids);
  };

  const removeRelayDevices = () => {
    if (!window.confirm("Are you sure you want to remove the selected devices from the relay?")) {
      return;
    }

    let count = 0;

    for (let devEui of selectedRowIds) {
      count++;

      let req = new RemoveRelayDeviceRequest();
      req.setRelayDevEui(props.relayDevice.getDevEui());
      req.setDeviceDevEui(devEui);

      let cbFunc = (cnt: number) => {
        return () => {
          if (cnt === selectedRowIds.length) {
            setRefreshKey(refreshKey + 1);
          }
        };
      };

      RelayStore.removeDevice(req, cbFunc(count));
    }
  };

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListRelayDevicesRequest();
    req.setRelayDevEui(props.relayDevice.getDevEui());
    req.setLimit(limit);
    req.setOffset(offset);

    RelayStore.listDevices(req, (resp: ListRelayDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <Space direction="horizontal" style={{ float: "right" }}>
        <Button onClick={removeRelayDevices} disabled={selectedRowIds.length === 0}>
          Remove remove from relay
        </Button>
      </Space>
      <DataTable
        columns={columns}
        getPage={getPage}
        onRowsSelectChange={onRowsSelectChange}
        rowKey="devEui"
        refreshKey={refreshKey}
      />
    </Space>
  );
}

export default ListRelayDevices;
