import { useState } from "react";
import { Link } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";
import type { ColumnsType } from "antd/es/table";

import type {
  ListDeviceProfileVendorsResponse,
  DeviceProfileVendorListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { ListDeviceProfileVendorsRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import deviceProfileStore from "../../stores/DeviceProfileStore";
import { useTitle } from "../helpers";

function ListDeviceProfileVendors() {
  useTitle("Network Server", "Device Profiles", "Vendors");
  const [refreshKey, setRefreshKey] = useState<number>(1);

  const columns: ColumnsType<DeviceProfileVendorListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (text, record) => <Link to={`/device-profiles/vendors/${record.id}/devices`}>{text}</Link>,
    },
  ];

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    _orderBy: string | void,
    _orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListDeviceProfileVendorsRequest();
    req.setLimit(limit);
    req.setOffset(offset);

    deviceProfileStore.listVendors(req, (resp: ListDeviceProfileVendorsResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb items={[{ title: "Network Server" }, { title: "Device Profiles" }, { title: "Vendors" }]} />
        )}
        title="Vendors"
        extra={[
          <a href="https://github.com/chirpstack/chirpstack-device-profiles" target="_blank">
            <Button type="primary">Add vendor</Button>
          </a>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" refreshKey={refreshKey} />
    </Space>
  );
}

export default ListDeviceProfileVendors;
