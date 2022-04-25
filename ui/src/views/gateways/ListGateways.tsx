import React, { Component } from "react";
import { Link } from "react-router-dom";

import moment from "moment";
import { Space, Breadcrumb, Button, PageHeader } from "antd";
import { ColumnsType } from "antd/es/table";

import {
  ListGatewaysRequest,
  ListGatewaysResponse,
  GatewayListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import GatewayStore from "../../stores/GatewayStore";
import Admin from "../../components/Admin";

interface IProps {
  tenant: Tenant;
}

class ListGateways extends Component<IProps> {
  columns = (): ColumnsType<GatewayListItem.AsObject> => {
    return [
      {
        title: "Last seen",
        dataIndex: "lastSeenAt",
        key: "lastSeenAt",
        width: 250,
        render: (text, record) => {
          if (record.lastSeenAt !== undefined) {
            let ts = new Date(0);
            ts.setUTCSeconds(record.lastSeenAt.seconds);
            return moment(ts).format("YYYY-MM-DD HH:mm:ss");
          }
          return "Never";
        },
      },
      {
        title: "Gateway ID",
        dataIndex: "gatewayId",
        key: "gatewayId",
        width: 250,
        render: (text, record) => (
          <Link to={`/tenants/${this.props.tenant.getId()}/gateways/${record.gatewayId}`}>{text}</Link>
        ),
      },
      {
        title: "Name",
        dataIndex: "name",
        key: "name",
      },
      {
        title: "Region name",
        dataIndex: "propertiesMap",
        key: "regionName",
        width: 150,
        render: (text, record) => {
          for (const [k, v] of record.propertiesMap) {
            if (k === "region_name") {
              return v;
            }
          }

          return "";
        },
      },
      {
        title: "Region common-name",
        dataIndex: "propertiesMap",
        key: "regionCommonName",
        width: 250,
        render: (text, record) => {
          for (const [k, v] of record.propertiesMap) {
            if (k === "region_common_name") {
              return v;
            }
          }

          return "";
        },
      },
    ];
  };

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListGatewaysRequest();
    req.setTenantId(this.props.tenant.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    GatewayStore.list(req, (resp: ListGatewaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  render() {
    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <PageHeader
          title="Gateways"
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Tenants</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}`}>{this.props.tenant.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Gateways</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          extra={[
            <Admin tenantId={this.props.tenant.getId()} isGatewayAdmin>
              <Button type="primary">
                <Link to={`/tenants/${this.props.tenant.getId()}/gateways/create`}>Add gateway</Link>
              </Button>
            </Admin>,
          ]}
        />
        <DataTable columns={this.columns()} getPage={this.getPage} rowKey="gatewayId" />
      </Space>
    );
  }
}

export default ListGateways;
