import { useState, useEffect } from "react";

import { Tag, Space, Button, Popconfirm, Spin, Typography, Popover } from "antd";
import { LoadingOutlined, ZoomInOutlined } from "@ant-design/icons";
import type { ColumnsType } from "antd/es/table";
import { format } from "date-fns";

import {
  ListFuotaDeploymentDevicesRequest,
  RemoveDevicesFromFuotaDeploymentRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import type {
  GetFuotaDeploymentResponse,
  ListFuotaDeploymentDevicesResponse,
  FuotaDeploymentDeviceListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import FuotaStore from "../../stores/FuotaStore";

interface IProps {
  getFuotaDeploymentResponse: GetFuotaDeploymentResponse;
}

function FuotaDeploymentDevices(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [refreshKey, setRefreshKey] = useState<number>(0);

  useEffect(() => {
    const interval = setInterval(() => {
      if (!props.getFuotaDeploymentResponse.getCompletedAt()) {
        setRefreshKey(refreshKey + 1);
      }
    }, 10000);

    return () => clearInterval(interval);
  }, [props.getFuotaDeploymentResponse, refreshKey]);

  const columns: ColumnsType<FuotaDeploymentDeviceListItem.AsObject> = [
    {
      title: "Status",
      key: "status",
      width: 100,
      render: (_text, record) => {
        if (record.errorMsg !== "") {
          return (
            <Popover content={record.errorMsg} placement="right">
              <Tag color="red">error</Tag>
            </Popover>
          );
        } else if (record.completedAt) {
          return <Tag color="green">ok</Tag>;
        } else if (props.getFuotaDeploymentResponse.getStartedAt()) {
          return <Spin indicator={<LoadingOutlined spin />} size="small" />;
        } else {
          return "";
        }
      },
    },
    {
      title: "DevEUI",
      dataIndex: "devEui",
      key: "devEui",
      width: 250,
      render: text => <Typography.Text code>{text}</Typography.Text>,
    },
    {
      title: "Mc. group setup completed at",
      key: "mcGroupSetupCompletedAt",
      render: (_text, record) => {
        if (record.mcGroupSetupCompletedAt !== undefined) {
          const ts = new Date(0);
          ts.setUTCSeconds(record.mcGroupSetupCompletedAt.seconds);
          return format(ts, "yyyy-MM-dd HH:mm:ss");
        }
      },
    },
    {
      title: "Frag. session setup completed at",
      key: "fragSessionSetupCompletedAt",
      render: (_text, record) => {
        if (record.fragSessionSetupCompletedAt !== undefined) {
          const ts = new Date(0);
          ts.setUTCSeconds(record.fragSessionSetupCompletedAt.seconds);
          return format(ts, "yyyy-MM-dd HH:mm:ss");
        }
      },
    },
    {
      title: "Mc. session completed at",
      key: "mcSessionCompletedAt",
      render: (_text, record) => {
        if (record.mcSessionCompletedAt !== undefined) {
          const ts = new Date(0);
          ts.setUTCSeconds(record.mcSessionCompletedAt.seconds);
          return format(ts, "yyyy-MM-dd HH:mm:ss");
        }
      },
    },
    {
      title: "Frag. status completed at",
      key: "fragStatusCompletedAt",
      render: (_text, record) => {
        if (record.fragStatusCompletedAt !== undefined) {
          const ts = new Date(0);
          ts.setUTCSeconds(record.fragStatusCompletedAt.seconds);
          return format(ts, "yyyy-MM-dd HH:mm:ss");
        }
      },
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
    req.setFuotaDeploymentId(props.getFuotaDeploymentResponse.getDeployment()!.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    FuotaStore.listDevices(req, (resp: ListFuotaDeploymentDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const removeDevices = () => {
    const req = new RemoveDevicesFromFuotaDeploymentRequest();
    req.setFuotaDeploymentId(props.getFuotaDeploymentResponse.getDeployment()!.getId());
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
