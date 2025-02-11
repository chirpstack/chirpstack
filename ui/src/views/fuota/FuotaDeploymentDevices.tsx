import { useState } from "react";

import { Space, Button, Popconfirm } from "antd";
import type { ColumnsType } from "antd/es/table";

import {
  ListFuotaDeploymentDevicesRequest,
  RemoveDevicesFromFuotaDeploymentRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import type {
  FuotaDeployment,
  ListFuotaDeploymentDevicesResponse,
  FuotaDeploymentDeviceListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import FuotaStore from "../../stores/FuotaStore";

interface IProps {
  fuotaDeployment: FuotaDeployment;
}

function FuotaDeploymentDevices(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [refreshKey, setRefreshKey] = useState<number>(0);

  const columns: ColumnsType<FuotaDeploymentDeviceListItem.AsObject> = [
    {
      title: "DevEUI",
      dataIndex: "devEui",
      key: "devEui",
      //       width: 250,
    },
  ];

  const onRowsSelectChange = (ids: string[]) => {
    setSelectedRowIds(ids);
  };

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    _orderBy: string | void,
    _orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListFuotaDeploymentDevicesRequest();
    req.setFuotaDeploymentId(props.fuotaDeployment.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    FuotaStore.listDevices(req, (resp: ListFuotaDeploymentDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const removeDevices = () => {
    const req = new RemoveDevicesFromFuotaDeploymentRequest();
    req.setFuotaDeploymentId(props.fuotaDeployment.getId());
    req.setDevEuisList(selectedRowIds);

    FuotaStore.removeDevices(req, () => {
      setRefreshKey(refreshKey + 1);
    });
  };

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <Space direction="horizontal" style={{ float: "right" }}>
        <Popconfirm
          title="Remove devices"
          description="Are you sure you want to remove the selected devices from the FUOTA deployment?"
          placement="left"
          onConfirm={removeDevices}
        >
          <Button disabled={selectedRowIds.length === 0}>Remove from FUOTA deployment</Button>
        </Popconfirm>
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

export default FuotaDeploymentDevices;
