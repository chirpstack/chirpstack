import { Link } from "react-router-dom";

import { Space, Breadcrumb, Button } from "antd";
import type { ColumnsType } from "antd/es/table";
import { PageHeader } from "@ant-design/pro-layout";

import type { ListUsersResponse, UserListItem } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";
import { ListUsersRequest } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";

import UserStore from "../../stores/UserStore";
import { useTitle } from "../helpers";

function ListUsers() {
  useTitle("Network Server", "Users");
  const columns: ColumnsType<UserListItem.AsObject> = [
    {
      title: "Email",
      dataIndex: "email",
      key: "email",
      render: (text, record) => <Link to={`/users/${record.id}`}>{text}</Link>,
    },
    {
      title: "Is admin",
      dataIndex: "isAdmin",
      key: "isAdmin",
      width: 150,
      render: (text, record) => {
        if (record.isAdmin) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Is active",
      dataIndex: "isActive",
      key: "isAdmin",
      width: 150,
      render: (text, record) => {
        if (record.isActive) {
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
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListUsersRequest();
    req.setLimit(limit);
    req.setOffset(offset);

    UserStore.list(req, (resp: ListUsersResponse) => {
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
              <span>Users</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Users"
        extra={[
          <Button type="primary">
            <Link to="/users/create">Add user</Link>
          </Button>,
        ]}
      />
      <DataTable columns={columns} getPage={getPage} rowKey="id" />
    </Space>
  );
}

export default ListUsers;
