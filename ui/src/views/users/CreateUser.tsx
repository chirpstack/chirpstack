import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { User, CreateUserRequest, CreateUserResponse } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import UserForm from "./UserForm";
import UserStore from "../../stores/UserStore";

function CreateUser() {
  const navigate = useNavigate();

  const onFinish = (obj: User, password: string) => {
    let req = new CreateUserRequest();
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
              <span>Add</span>
            </Breadcrumb.Item>
          </Breadcrumb>
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
