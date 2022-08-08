import React, { Component } from "react";
import { notification } from "antd";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { Tenant, TenantUser } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { User } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import TenantUserForm from "./TenantUserForm";

// stores
import SessionStore from "../../stores/SessionStore";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
}

interface IState {
  user?: User;
}

class CreateTenantUser extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      user: undefined,
    };
  }

  componentDidMount() {
    SessionStore.on("change", () => {
      this.setState({
        user: SessionStore.getUser(),
      });
    });

    this.setState({
      user: SessionStore.getUser(),
    });
  }

  onFinish = async (obj: TenantUser) => {
    // Highjack the request to login server
    if (this.state.user) {
      const res = await fetch("http://localhost:4000/api/invite_user", {
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify({
          email: obj.getEmail(),
          tenant_id: this.props.tenant.getId(),
          tenant_name: this.props.tenant.getName(),
          admin: obj.getIsAdmin(),
          inviter_id: this.state.user.getId(),
        }),
      });

      if (res.status === 201) {
        notification.success({
          message: "User invited",
          duration: 3,
        });

        this.props.history.push(`/tenants/${this.props.tenant.getId()}/users`);
      } else {
        // TODO figure out why error msg doesn't come in
        notification.error({
          message: "Unable to invite user, please try again",
          duration: 3,
        });
      }
    }
  };

  render() {
    const tu = new TenantUser();

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <PageHeader
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Tenants</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}`}>{this.props.tenant.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/users`}>Tenant users</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add tenant user"
        />
        <Card>
          <TenantUserForm initialValues={tu} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateTenantUser;
