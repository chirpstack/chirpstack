import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { User, CreateUserRequest, CreateUserResponse } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import UserForm from "./UserForm";
import UserStore from "../../stores/UserStore";

class CreateUser extends Component<RouteComponentProps> {
  onFinish = (obj: User, password: string) => {
    let req = new CreateUserRequest();
    req.setUser(obj);
    req.setPassword(password);

    UserStore.create(req, (resp: CreateUserResponse) => {
      this.props.history.push("/users");
    });
  };

  render() {
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
          <UserForm initialValues={user} onFinish={this.onFinish} password />
        </Card>
      </Space>
    );
  }
}

export default CreateUser;
