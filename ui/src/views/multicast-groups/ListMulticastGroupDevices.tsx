import { useState } from "react";

import { Space, Button } from "antd";
import type { ColumnsType } from "antd/es/table";

import type { ListDevicesResponse, DeviceListItem } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { ListDevicesRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import type { MulticastGroup } from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/multicast_tenant_pb";
import { RemoveDeviceFromMulticastGroupRequest } from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import DeviceStore from "../../stores/DeviceStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import SessionStore from "../../stores/SessionStore";

interface IProps {
  tenant: Tenant;
  multicastGroup: MulticastGroup;
}

function ListMulticastGroupDevices(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [refreshKey, setRefreshKey] = useState<number>(0);

  const columns: ColumnsType<DeviceListItem.AsObject> = [
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

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListDevicesRequest();
    req.setApplicationId(props.multicastGroup.getApplicationId());
    req.setMulticastGroupId(props.multicastGroup.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    DeviceStore.list(req, (resp: ListDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const removeDevicesFromMulticastGroup = () => {
    if (!window.confirm("Are you sure you want to remove the selected devices from the multicast-group?")) {
      return;
    }

    let count = 0;

    for (const devEui of selectedRowIds) {
      count++;

      const req = new RemoveDeviceFromMulticastGroupRequest();
      req.setMulticastGroupId(props.multicastGroup.getId());
      req.setDevEui(devEui);

      const cbFunc = (cnt: number) => {
        return () => {
          if (cnt === selectedRowIds.length) {
            setRefreshKey(refreshKey + 1);
          }
        };
      };

      MulticastGroupStore.removeDevice(req, cbFunc(count));
    }
  };

  const disabled = !(
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.tenant.getId()) ||
    SessionStore.isTenantDeviceAdmin(props.tenant.getId()) ||
    SessionStore.isApplicationAdmin(props.multicastGroup.getApplicationId())
  );

  return (
    <Space orientation="vertical" size="large" style={{ width: "100%" }}>
      <Space orientation="horizontal" style={{ float: "right" }}>
        <Button onClick={removeDevicesFromMulticastGroup} disabled={selectedRowIds.length === 0 || disabled}>
          Remove from multicast-group
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

export default ListMulticastGroupDevices;
