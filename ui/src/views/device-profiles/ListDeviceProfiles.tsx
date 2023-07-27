import { Link } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import {
  ListDeviceProfilesRequest,
  ListDeviceProfilesResponse,
  DeviceProfileListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

import { getEnumName, formatMacVersion, formatRegParamsRevision } from "../helpers";
import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import Admin from "../../components/Admin";

interface IProps {
  tenant: Tenant;
}

function ListDeviceProfiles(props: IProps) {
  const columns: ColumnsType<DeviceProfileListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (text, record) => (
        <Link to={`/tenants/${props.tenant.getId()}/device-profiles/${record.id}/edit`}>{text}</Link>
      ),
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

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListDeviceProfilesRequest();
    req.setTenantId(props.tenant.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    DeviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
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
              <span>Tenants</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Device profiles</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Device profiles"
        extra={[
          <Admin tenantId={props.tenant.getId()} isDeviceAdmin>
            <Button type="primary">
              <Link to={`/tenants/${props.tenant.getId()}/device-profiles/create`}>Add device profile</Link>
            </Button>
          </Admin>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListDeviceProfiles;
