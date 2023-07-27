import React, { useEffect, useState } from "react";
import { Link } from "react-router-dom";

import { Space, Breadcrumb, Table } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { ListRegionsResponse, RegionListItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import { getEnumName } from "../helpers";
import InternalStore from "../../stores/InternalStore";

function ListRegions() {
  const [regions, setRegions] = useState<ListRegionsResponse | undefined>(undefined);

  useEffect(() => {
    InternalStore.listRegions((resp: ListRegionsResponse) => {
      setRegions(resp);
    });
  }, []);

  let items: RegionListItem.AsObject[] = [];

  if (regions !== undefined) {
    items = regions.getRegionsList().map((r, i) => r.toObject());
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
        loading={regions === undefined}
        pagination={false}
        dataSource={items}
        columns={[
          {
            title: "ID",
            dataIndex: "id",
            key: "id",
            render: (text, record) => <Link to={`/regions/${text}`}>{text}</Link>,
          },
          {
            title: "Region",
            dataIndex: "region",
            key: "region",
            render: (text, record) => {
              return getEnumName(Region, record.region);
            },
          },
          {
            title: "Description",
            dataIndex: "description",
            key: "description",
          },
        ]}
      />
    </Space>
  );
}

export default ListRegions;
