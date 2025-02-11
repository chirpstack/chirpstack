import { Link } from "react-router-dom";

import { Space, Button } from "antd";
import type { ColumnsType } from "antd/es/table";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import type {
  ListMulticastGroupsResponse,
  MulticastGroupListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  MulticastGroupType,
  ListMulticastGroupsRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import { getEnumName } from "../helpers";
import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import Admin from "../../components/Admin";

interface IProps {
  application: Application;
}

function ListMulticastGroups(props: IProps) {
  const columns: ColumnsType<MulticastGroupListItem.AsObject> = [
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      render: (text, record) => (
        <Link
          to={`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/multicast-groups/${
            record.id
          }`}
        >
          {text}
        </Link>
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
      title: "Group type",
      dataIndex: "groupType",
      key: "groupType",
      width: 150,
      render: (text, record) => {
        return getEnumName(MulticastGroupType, record.groupType);
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
    const req = new ListMulticastGroupsRequest();
    req.setApplicationId(props.application.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    MulticastGroupStore.list(req, (resp: ListMulticastGroupsResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <Admin tenantId={props.application.getTenantId()} isDeviceAdmin>
        <Button type="primary" style={{ float: "right" }}>
          <Link
            to={`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/multicast-groups/create`}
          >
            Add multicast-group
          </Link>
        </Button>
      </Admin>
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListMulticastGroups;
