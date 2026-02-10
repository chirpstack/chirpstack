import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";
import type { ColumnsType } from "antd/es/table";

import type {
  DeviceProfileDeviceListItem,
  ListDeviceProfileDevicesResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  ListDeviceProfileDevicesRequest,
  DeleteDeviceProfileVendorRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import type { DeviceProfileVendor } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import deviceProfileStore from "../../stores/DeviceProfileStore";
import { useTitle } from "../helpers";
import DeleteConfirm from "../../components/DeleteConfirm";

interface IProps {
  vendor: DeviceProfileVendor;
}

function ListDeviceProfileDevices(props: IProps) {
  const navigate = useNavigate();
  useTitle(["Network Server", "Device Profiles", "Vendors", props.vendor.getName()]);

  const columns: ColumnsType<DeviceProfileDeviceListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (text, record) => (
        <Link to={`/device-profiles/vendors/${props.vendor.getId()}/devices/${record.id}/profiles`}>{text}</Link>
      ),
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
    const req = new ListDeviceProfileDevicesRequest();
    req.setVendorId(props.vendor.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    deviceProfileStore.listDevices(req, (resp: ListDeviceProfileDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const deleteVendor = () => {
    const req = new DeleteDeviceProfileVendorRequest();
    req.setId(props.vendor.getId());

    deviceProfileStore.deleteVendor(req, () => {
      navigate("/device-profiles/vendors");
    });
  };

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb
            items={[
              { title: "Network Server" },
              { title: "Device Profiles" },
              { title: <Link to={`/device-profiles/vendors`}>Vendors</Link> },
              { title: props.vendor.getName() },
            ]}
          />
        )}
        title={props.vendor.getName()}
        extra={[
          <Space key="add-device-delete-vendor">
            <a href="https://github.com/chirpstack/chirpstack-device-profiles" target="_blank">
              <Button type="primary">Add device</Button>
            </a>
            <DeleteConfirm confirm={props.vendor.getName()} typ="vendor" onConfirm={deleteVendor}>
              <Button danger type="primary">
                Delete vendor
              </Button>
            </DeleteConfirm>
          </Space>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListDeviceProfileDevices;
