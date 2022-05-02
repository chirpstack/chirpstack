import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Space, Button } from "antd";
import { ColumnsType } from "antd/es/table";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import {
  MulticastGroupType,
  ListMulticastGroupsRequest,
  ListMulticastGroupsResponse,
  MulticastGroupListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import { getEnumName } from "../helpers";
import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import Admin from "../../components/Admin";

interface IProps {
  application: Application;
}

class ListMulticastGroups extends Component<IProps> {
  columns = (): ColumnsType<MulticastGroupListItem.AsObject> => {
    return [
      {
        title: "Name",
        dataIndex: "name",
        key: "name",
        render: (text, record) => (
          <Link
            to={`/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/multicast-groups/${
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
  };

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListMulticastGroupsRequest();
    req.setApplicationId(this.props.application.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    MulticastGroupStore.list(req, (resp: ListMulticastGroupsResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  render() {
    return (
      <Space direction="vertical" size="large" style={{ width: "100%" }}>
        <Admin tenantId={this.props.application.getTenantId()} isDeviceAdmin>
          <Button type="primary" style={{ float: "right" }}>
            <Link
              to={`/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/multicast-groups/create`}
            >
              Add multicast-group
            </Link>
          </Button>
        </Admin>
        <DataTable columns={this.columns()} getPage={this.getPage} rowKey="id" />
      </Space>
    );
  }
}

export default ListMulticastGroups;
