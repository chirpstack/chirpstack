import React, { useEffect, useState } from "react";
import { useNavigate, useParams, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import {
  User,
  GetUserRequest,
  GetUserResponse,
  UpdateUserRequest,
  DeleteUserRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import UserForm from "./UserForm";
import UserStore from "../../stores/UserStore";
import DeleteConfirm from "../../components/DeleteConfirm";

function EditUser() {
  const navigate = useNavigate();
  const { userId } = useParams();
  const [user, setUser] = useState<User | undefined>(undefined);

  useEffect(() => {
    let req = new GetUserRequest();
    req.setId(userId!);

    UserStore.get(req, (resp: GetUserResponse) => {
      setUser(resp.getUser());
    });
  }, [userId]);

  const onFinish = (obj: User, password: string) => {
    let req = new UpdateUserRequest();
    req.setUser(obj);

    UserStore.update(req, () => {
      navigate("/users");
    });
  };

  const deleteUser = () => {
    if (!user) {
      return;
    }

    let req = new DeleteUserRequest();
    req.setId(user.getId());

    UserStore.delete(req, () => {
      navigate("/users");
    });
  };

  if (!user) {
    return null;
  }

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Network-server</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to="/users">Users</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>{user.getEmail()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={user.getEmail()}
        subTitle={`user id: ${user.getId()}`}
        extra={[
          <Button>
            <Link to={`/users/${user.getId()}/password`}>Change password</Link>
          </Button>,
          <DeleteConfirm typ="user" confirm={user.getEmail()} onConfirm={deleteUser}>
            <Button danger type="primary">
              Delete user
            </Button>
          </DeleteConfirm>,
        ]}
      />
      <Card>
        <UserForm initialValues={user} onFinish={onFinish} password={false} />
      </Card>
    </Space>
  );
}

export default EditUser;
