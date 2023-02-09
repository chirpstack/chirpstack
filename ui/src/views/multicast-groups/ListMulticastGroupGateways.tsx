import React, { Component } from "react";

import { Space, Button } from "antd";
import { ColumnsType } from "antd/es/table";

import {
  ListGatewaysRequest,
  ListGatewaysResponse,
  GatewayListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import {
  MulticastGroup,
  RemoveGatewayFromMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import GatewayStore from "../../stores/GatewayStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";

interface IProps {
  application: Application;
  multicastGroup: MulticastGroup;
}

interface IState {
  selectedRowIds: string[];
  refreshKey: number;
}

class ListMulticastGroupGateways extends Component<IProps, IState> {
  columns = (): ColumnsType<GatewayListItem.AsObject> => {
    return [
      {
        title: "Name",
        dataIndex: "name",
        key: "name",
      },
      {
        title: "Gateway ID",
        dataIndex: "gatewayId",
        key: "gatewayId",
        width: 250,
      },
    ];
  };

  constructor(props: IProps) {
    super(props);

    this.state = {
      selectedRowIds: [],
      refreshKey: 1,
    };
  }

  onRowsSelectChange = (ids: string[]) => {
    this.setState({
      selectedRowIds: ids,
    });
  };

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListGatewaysRequest();
    req.setTenantId(this.props.application.getTenantId());
    req.setMulticastGroupId(this.props.multicastGroup.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    GatewayStore.list(req, (resp: ListGatewaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  removeGatewaysFromMulticastGroup = () => {
    if (!window.confirm("Are you sure you want to remove the selected gateways from the multicast-group?")) {
      return;
    }

    let count = 0;
    let self = this;

    for (let gatewayId of this.state.selectedRowIds) {
      count++;

      let req = new RemoveGatewayFromMulticastGroupRequest();
      req.setMulticastGroupId(this.props.multicastGroup.getId());
      req.setGatewayId(gatewayId);

      let cbFunc = (cnt: number) => {
        return () => {
          if (cnt === self.state.selectedRowIds.length) {
            self.setState({
              refreshKey: self.state.refreshKey + 1,
            });
          }
        };
      };

      MulticastGroupStore.removeGateway(req, cbFunc(count));
    }
  };

  render() {
    return (
      <Space direction="vertical" size="large" style={{ width: "100%" }}>
        <Space direction="horizontal" style={{ float: "right" }}>
          <Button onClick={this.removeGatewaysFromMulticastGroup} disabled={this.state.selectedRowIds.length === 0}>
            Remove from multicast-group
          </Button>
        </Space>
        <DataTable
          columns={this.columns()}
          getPage={this.getPage}
          onRowsSelectChange={this.onRowsSelectChange}
          rowKey="gatewayId"
          refreshKey={this.state.refreshKey}
        />
      </Space>
    );
  }
}

export default ListMulticastGroupGateways;
