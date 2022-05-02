import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import {
  User,
  GetUserRequest,
  GetUserResponse,
  UpdateUserPasswordRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";
import UserStore from "../../stores/UserStore";
import PasswordForm from "./PasswordForm";

interface MatchParams {
  userId: string;
}

interface IState {
  user?: User;
}

class ChangeUserPassword extends Component<RouteComponentProps<MatchParams>, IState> {
  constructor(props: RouteComponentProps<MatchParams>) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    let req = new GetUserRequest();
    req.setId(this.props.match.params.userId);

    UserStore.get(req, (resp: GetUserResponse) => {
      this.setState({
        user: resp.getUser(),
      });
    });
  }

  onFinish = (pw: string) => {
    let req = new UpdateUserPasswordRequest();
    req.setUserId(this.props.match.params.userId);
    req.setPassword(pw);

    UserStore.updatePassword(req, () => {
      this.props.history.push("/");
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
          <PasswordForm onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default ChangeUserPassword;
