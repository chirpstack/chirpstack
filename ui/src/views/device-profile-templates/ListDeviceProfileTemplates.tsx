import { Link } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import type { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import type {
  ListDeviceProfileTemplatesResponse,
  DeviceProfileTemplateListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";
import { ListDeviceProfileTemplatesRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";
import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

import { getEnumName } from "../helpers";
import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import DeviceProfileTemplateStore from "../../stores/DeviceProfileTemplateStore";
import { useTitle } from "../helpers";

function ListDeviceProfileTemplates() {
  useTitle("Network Server", "Device-profile templates");
  const columns: ColumnsType<DeviceProfileTemplateListItem.AsObject> = [
    {
      title: "Vendor",
      dataIndex: "vendor",
      key: "vendor",
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (text, record) => <Link to={`/device-profile-templates/${record.id}/edit`}>{text}</Link>,
    },
    {
      title: "Firmware",
      dataIndex: "firmware",
      key: "firmware",
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
  ];

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListDeviceProfileTemplatesRequest();
    req.setLimit(limit);
    req.setOffset(offset);

    DeviceProfileTemplateStore.list(req, (resp: ListDeviceProfileTemplatesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Network Server</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Device-profile templates</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Device-profile templates"
        extra={[
          <Button type="primary">
            <Link to={`/device-profile-templates/create`}>Add device-profile template</Link>
          </Button>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListDeviceProfileTemplates;
