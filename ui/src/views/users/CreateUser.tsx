import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { CreateUserResponse } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";
import { User, CreateUserRequest } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import UserForm from "./UserForm";
import UserStore from "../../stores/UserStore";
import { useTitle } from "../helpers";

function CreateUser() {
  useTitle("Network Server", "Users", "Add");
  const navigate = useNavigate();

  const onFinish = (obj: User, password: string) => {
    const req = new CreateUserRequest();
    req.setUser(obj);
    req.setPassword(password);

    UserStore.create(req, (resp: CreateUserResponse) => {
      navigate("/users");
    });
  };

  const user = new User();

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb
            items={[{ title: "Network Server" }, { title: <Link to="/users">Users</Link> }, { title: "Add" }]}
          />
        )}
        title="Add user"
      />
      <Card>
        <UserForm initialValues={user} onFinish={onFinish} password />
      </Card>
    </Space>
  );
}

export default CreateUser;
