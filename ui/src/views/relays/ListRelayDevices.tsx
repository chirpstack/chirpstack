import { useState } from "react";

import { Space, Button } from "antd";
import type { ColumnsType } from "antd/es/table";

import type { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import type { ListRelayDevicesResponse, RelayDeviceListItem } from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";
import { ListRelayDevicesRequest, RemoveRelayDeviceRequest } from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
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

    for (const devEui of selectedRowIds) {
      count++;

      const req = new RemoveRelayDeviceRequest();
      req.setRelayDevEui(props.relayDevice.getDevEui());
      req.setDeviceDevEui(devEui);

      const cbFunc = (cnt: number) => {
        return () => {
          if (cnt === selectedRowIds.length) {
            setRefreshKey(refreshKey + 1);
          }
        };
      };

      RelayStore.removeDevice(req, cbFunc(count));
    }
  };

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListRelayDevicesRequest();
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
