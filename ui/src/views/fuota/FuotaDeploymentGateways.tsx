import { useState } from "react";

import { Space, Button, Popconfirm, Typography } from "antd";
import type { ColumnsType } from "antd/es/table";

import {
  ListFuotaDeploymentGatewaysRequest,
  RemoveGatewaysFromFuotaDeploymentRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import type {
  FuotaDeployment,
  ListFuotaDeploymentGatewaysResponse,
  FuotaDeploymentGatewayListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import FuotaStore from "../../stores/FuotaStore";

interface IProps {
  fuotaDeployment: FuotaDeployment;
}

function FuotaDeploymentGateways(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [refreshKey, setRefreshKey] = useState<number>(0);

  const columns: ColumnsType<FuotaDeploymentGatewayListItem.AsObject> = [
    {
      title: "Gateway ID",
      dataIndex: "gatewayId",
      key: "gatewayId",
      render: text => <Typography.Text code>{text}</Typography.Text>,
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
    const req = new ListFuotaDeploymentGatewaysRequest();
    req.setFuotaDeploymentId(props.fuotaDeployment.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    FuotaStore.listGateways(req, (resp: ListFuotaDeploymentGatewaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const removeGateways = () => {
    const req = new RemoveGatewaysFromFuotaDeploymentRequest();
    req.setFuotaDeploymentId(props.fuotaDeployment.getId());
    req.setGatewayIdsList(selectedRowIds);

    FuotaStore.removeGateways(req, () => {
      setRefreshKey(refreshKey + 1);
    });
  };

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <Space direction="horizontal" style={{ float: "right" }}>
        <Popconfirm
          title="Remove gateways"
          description="Are you sure you want to remove the selected gateways from the FUOTA deployment?"
          placement="left"
          onConfirm={removeGateways}
        >
          <Button disabled={selectedRowIds.length === 0}>Remove from FUOTA deployment</Button>
        </Popconfirm>
      </Space>
      <DataTable
        columns={columns}
        getPage={getPage}
        onRowsSelectChange={onRowsSelectChange}
        rowKey="gatewayId"
        refreshKey={refreshKey}
      />
    </Space>
  );
}

export default FuotaDeploymentGateways;
