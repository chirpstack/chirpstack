import React, { Component } from "react";
import { Link } from "react-router-dom";

import { Space, Breadcrumb, PageHeader, Table } from "antd";

import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import {
  ListRegionsResponse,
  RegionListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import { getEnumName } from "../helpers";
import InternalStore from "../../stores/InternalStore";

interface IProps {}

interface IState {
  regions?: ListRegionsResponse;
}


class ListRegions extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {};
  }

  componentDidMount() {
    InternalStore.listRegions((resp: ListRegionsResponse) => {
      this.setState({
        regions: resp,
      });
    });
  }

  render() {
    let items: RegionListItem.AsObject[] = [];

    if (this.state.regions !== undefined) {
      items = this.state.regions.getRegionsList().map((r, i) => r.toObject());
    }

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <PageHeader
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Network Server</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Regions</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Regions"
        />
        <Table
          loading={this.state.regions === undefined}
          pagination={false}
          dataSource={items}
          columns={[
            {
              title: "Name",
              dataIndex: "name",
              key: "name",
              render: (text, record) => (
                <Link to={`/regions/${text}`}>{text}</Link>
              ),
            },
            {
              title: "Region",
              dataIndex: "region",
              key: "region",
              render: (text, record) => {
                return getEnumName(Region, record.region);
              },
            },
          ]}
        />
      </Space>
    );
  }
}

export default ListRegions;
