import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";
import type { ColumnsType } from "antd/es/table";

import type {
  ListDeviceProfilesResponse,
  DeviceProfileVendor,
  DeviceProfileDevice,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  DeleteDeviceProfileDeviceRequest,
  ListDeviceProfilesRequest,
  type DeviceProfileListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import deviceProfileStore from "../../stores/DeviceProfileStore";
import { getEnumName, useTitle, formatMacVersion, formatRegParamsRevision } from "../helpers";
import DeleteConfirm from "../../components/DeleteConfirm";

interface IProps {
  vendor: DeviceProfileVendor;
  device: DeviceProfileDevice;
}

function ListVendorDeviceProfiles(props: IProps) {
  const navigate = useNavigate();
  useTitle("Network Server", "Device Profiles");

  const columns: ColumnsType<DeviceProfileListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (text, record) => (
        <Link
          to={`/device-profiles/vendors/${props.vendor.getId()}/devices/${props.device.getId()}/profiles/${record.id}`}
        >
          {text}
        </Link>
      ),
    },
    {
      title: "Firmware",
      dataIndex: "firmwareVersion",
      key: "firmwareVersion",
      width: 150,
    },
    {
      title: "Region",
      dataIndex: "region",
      key: "region",
      width: 150,
      render: (text, record) => {
        return getEnumName(Region, record.region);
      },
    },
    {
      title: "MAC version",
      dataIndex: "macVersion",
      key: "macVersion",
      width: 150,
      render: (text, record) => {
        return formatMacVersion(record.macVersion);
      },
    },
    {
      title: "Revision",
      dataIndex: "regParamsRevision",
      key: "regParamsRevision",
      width: 150,
      render: (text, record) => {
        return formatRegParamsRevision(record.regParamsRevision);
      },
    },
    {
      title: "Supports OTAA",
      dataIndex: "supportsOtaa",
      key: "supportsOtaa",
      width: 150,
      render: (text, record) => {
        if (record.supportsOtaa) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Supports Class-B",
      dataIndex: "supportsClassB",
      key: "supportsClassB",
      width: 150,
      render: (text, record) => {
        if (record.supportsClassB) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Supports Class-C",
      dataIndex: "supportsClassC",
      key: "supportsClassC",
      width: 150,
      render: (text, record) => {
        if (record.supportsClassC) {
          return "yes";
        } else {
          return "no";
        }
      },
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
    const req = new ListDeviceProfilesRequest();
    req.setLimit(limit);
    req.setOffset(offset);
    req.setDeviceId(props.device.getId());
    req.setGlobalOnly(true);

    deviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const deleteDevice = () => {
    const req = new DeleteDeviceProfileDeviceRequest();
    req.setId(props.device.getId());

    deviceProfileStore.deleteDevice(req, () => {
      navigate(`/device-profiles/vendors/${props.vendor.getId()}/devices`);
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
              {
                title: (
                  <Link to={`/device-profiles/vendors/${props.vendor.getId()}/devices`}>{props.vendor.getName()}</Link>
                ),
              },
              { title: props.device.getName() },
            ]}
          />
        )}
        title={props.device.getName()}
        extra={[
          <Space>
            <a href="https://github.com/chirpstack/chirpstack-device-profiles" target="_blank">
              <Button type="primary">Add device profile</Button>
            </a>
            <DeleteConfirm confirm={props.device.getName()} typ="device" onConfirm={deleteDevice}>
              <Button danger type="primary">
                Delete device
              </Button>
            </DeleteConfirm>
          </Space>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListVendorDeviceProfiles;
