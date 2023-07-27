import React, { useState } from "react";
import { Link } from "react-router-dom";

import { DeleteOutlined } from "@ant-design/icons";
import { Space, Breadcrumb, Button } from "antd";
import { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import {
  ListApiKeysRequest,
  ListApiKeysResponse,
  DeleteApiKeyRequest,
  ApiKey,
} from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import InternalStore from "../../stores/InternalStore";
import DeleteConfirm from "../../components/DeleteConfirm";

function ListAdminApiKeys() {
  const [refreshKey, setRefreshKey] = useState<number>(1);

  const columns: ColumnsType<ApiKey.AsObject> = [
    {
      title: "ID",
      dataIndex: "id",
      key: "id",
      width: 400,
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
    },
    {
      title: "Action",
      dataIndex: "id",
      key: "action",
      width: 100,
      render: (text, record) => (
        <DeleteConfirm typ="API key" confirm={record.name} onConfirm={deleteApiKey(record.id)}>
          <Button shape="circle" icon={<DeleteOutlined />} />
        </DeleteConfirm>
      ),
    },
  ];

  const deleteApiKey = (id: string): (() => void) => {
    return () => {
      let req = new DeleteApiKeyRequest();
      req.setId(id);

      InternalStore.deleteApiKey(req, () => {
        // trigger a data-table reload
        setRefreshKey(refreshKey + 1);
      });
    };
  };

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListApiKeysRequest();
    req.setLimit(limit);
    req.setOffset(offset);
    req.setIsAdmin(true);

    InternalStore.listApiKeys(req, (resp: ListApiKeysResponse) => {
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
              <span>API keys</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="API keys"
        extra={[
          <Button type="primary">
            <Link to="/api-keys/create">Add API key</Link>
          </Button>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" refreshKey={refreshKey} />
    </Space>
  );
}

export default ListAdminApiKeys;
