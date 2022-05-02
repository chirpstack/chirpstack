import React, { Component } from "react";
import { RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, PageHeader } from "antd";

import {
  Tenant,
  TenantUser,
  GetTenantUserRequest,
  GetTenantUserResponse,
  UpdateTenantUserRequest,
  DeleteTenantUserRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantUserForm from "./TenantUserForm";
import TenantStore from "../../stores/TenantStore";
import SessionStore from "../../stores/SessionStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import Admin from "../../components/Admin";

interface IState {
  tenantUser?: TenantUser;
}

interface MatchParams {
  userId: string;
}

interface IProps extends RouteComponentProps<MatchParams> {
  tenant: Tenant;
}

class EditTenantUser extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.getTenantUser();
  }

  getTenantUser = () => {
    const id = this.props.match.params.userId;
    let req = new GetTenantUserRequest();
    req.setTenantId(this.props.tenant.getId());
    req.setUserId(id);

    TenantStore.getUser(req, (resp: GetTenantUserResponse) => {
      this.setState({
        tenantUser: resp.getTenantUser(),
      });
    });
  };

  onFinish = (obj: TenantUser) => {
    let req = new UpdateTenantUserRequest();
    req.setTenantUser(obj);

    TenantStore.updateUser(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/users`);
    });
  };

  deleteTenantUser = () => {
    let req = new DeleteTenantUserRequest();
    req.setTenantId(this.props.tenant.getId());
    req.setUserId(this.props.match.params.userId);

    TenantStore.deleteUser(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/users`);
    });
  };

  render() {
    const tu = this.state.tenantUser;

    if (!tu) {
      return null;
    }

    const disabled = !(SessionStore.isAdmin() || SessionStore.isTenantAdmin(this.props.tenant.getId()));

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
                <span>{tu.getEmail()}</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={tu.getEmail()}
          subTitle={`user id: ${tu.getUserId()}`}
          extra={[
            <Admin tenantId={this.props.tenant.getId()} isTenantAdmin>
              <DeleteConfirm typ="tenant user" confirm={tu.getEmail()} onConfirm={this.deleteTenantUser}>
                <Button danger type="primary">
                  Delete tenant user
                </Button>
              </DeleteConfirm>
            </Admin>,
          ]}
        />
        <Card>
          <TenantUserForm initialValues={tu} onFinish={this.onFinish} disabled={disabled} disableEmail />
        </Card>
      </Space>
    );
  }
}

export default EditTenantUser;
