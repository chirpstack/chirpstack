import { useEffect, useState } from "react";

import { Space, Button, Popover, Spin, Tag, Typography } from "antd";
import { LoadingOutlined } from "@ant-design/icons";
import type { ColumnsType } from "antd/es/table";

import type {
  MulticastGroup,
  MulticastGroupDeviceListItem,
  ListMulticastGroupDevicesResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  ListMulticastGroupDevicesRequest,
  MulticastGroupSetup,
  RemoveDeviceFromMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import { format_dt_from_secs } from "../helpers";

interface IProps {
  multicastGroup: MulticastGroup;
}

function ListMulticastGroupDevices(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [refreshKey, setRefreshKey] = useState<number>(0);
  const ts005Setup = props.multicastGroup.getSetup() === MulticastGroupSetup.TS005;

  useEffect(() => {
    if (!ts005Setup) {
      return;
    }

    const interval = setInterval(() => {
      setRefreshKey(v => v + 1);
    }, 10000);

    return () => clearInterval(interval);
  }, [ts005Setup]);

  const columns: ColumnsType<MulticastGroupDeviceListItem.AsObject> = [
    ...(ts005Setup
      ? [
          {
            title: "Status",
            key: "status",
            width: 130,
            render: (_text: string, record: MulticastGroupDeviceListItem.AsObject) => {
              if (record.errorMsg !== "") {
                return (
                  <Popover content={record.errorMsg} placement="right">
                    <Tag color="red">error</Tag>
                  </Popover>
                );
              } else if (record.pendingDelete) {
                return <Tag color="orange">pending delete</Tag>;
              } else if (record.mcSessionCompletedAt) {
                return <Tag color="green">session</Tag>;
              } else if (record.mcGroupSetupCompletedAt) {
                return <Tag color="blue">setup</Tag>;
              } else {
                return <Spin indicator={<LoadingOutlined spin />} size="small" />;
              }
            },
          },
        ]
      : []),
    {
      title: "Name",
      dataIndex: "deviceName",
      key: "deviceName",
    },
    {
      title: "DevEUI",
      dataIndex: "devEui",
      key: "devEui",
      width: 250,
      render: text => <Typography.Text code>{text}</Typography.Text>,
    },
    ...(ts005Setup
      ? [
          {
            title: "Mc. group ID",
            dataIndex: "mcGroupId",
            key: "mcGroupId",
            width: 120,
          },
          {
            title: "Mc. group setup completed at",
            key: "mcGroupSetupCompletedAt",
            render: (_text: string, record: MulticastGroupDeviceListItem.AsObject) =>
              format_dt_from_secs(record.mcGroupSetupCompletedAt?.seconds),
          },
          {
            title: "Mc. session completed at",
            key: "mcSessionCompletedAt",
            render: (_text: string, record: MulticastGroupDeviceListItem.AsObject) =>
              format_dt_from_secs(record.mcSessionCompletedAt?.seconds),
          },
        ]
      : []),
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
    const req = new ListMulticastGroupDevicesRequest();
    req.setMulticastGroupId(props.multicastGroup.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    MulticastGroupStore.listDevices(req, (resp: ListMulticastGroupDevicesResponse) => {
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

  return (
    <Space orientation="vertical" size="large" style={{ width: "100%" }}>
      <Space orientation="horizontal" style={{ float: "right" }}>
        <Button onClick={removeDevicesFromMulticastGroup} disabled={selectedRowIds.length === 0}>
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
