import React, { Component } from "react";
import { RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, PageHeader } from "antd";

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

interface IState {
  user?: User;
}

interface MatchParams {
  userId: string;
}

class EditUser extends Component<RouteComponentProps<MatchParams>, IState> {
  constructor(props: RouteComponentProps<MatchParams>) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.getUser(this.props.match.params.userId);
  }

  getUser = (id: string) => {
    let req = new GetUserRequest();
    req.setId(id);

    UserStore.get(req, (resp: GetUserResponse) => {
      this.setState({
        user: resp.getUser(),
      });
    });
  };

  onFinish = (obj: User, password: string) => {
    let req = new UpdateUserRequest();
    req.setUser(obj);

    UserStore.update(req, () => {
      this.props.history.push("/users");
    });
  };

  deleteUser = () => {
    if (!this.state.user) {
      return;
    }

    let req = new DeleteUserRequest();
    req.setId(this.state.user.getId());

    UserStore.delete(req, () => {
      this.props.history.push("/users");
    });
  };

  render() {
    const user = this.state.user;
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
            <DeleteConfirm typ="user" confirm={user.getEmail()} onConfirm={this.deleteUser}>
              <Button danger type="primary">
                Delete user
              </Button>
            </DeleteConfirm>,
          ]}
        />
        <Card>
          <UserForm initialValues={user} onFinish={this.onFinish} password={false} />
        </Card>
      </Space>
    );
  }
}

export default EditUser;
