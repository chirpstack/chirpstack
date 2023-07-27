import React, { useState, useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import {
  User,
  GetUserRequest,
  GetUserResponse,
  UpdateUserPasswordRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";
import UserStore from "../../stores/UserStore";
import PasswordForm from "./PasswordForm";

function ChangeUserPassword() {
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

  const onFinish = (pw: string) => {
    let req = new UpdateUserPasswordRequest();
    req.setUserId(userId!);
    req.setPassword(pw);

    UserStore.updatePassword(req, () => {
      navigate("/");
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
              <span>Users</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>{user.getEmail()}</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Change password</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={user.getEmail()}
        subTitle={`user id: ${user.getId()}`}
      />
      <Card>
        <PasswordForm onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default ChangeUserPassword;
