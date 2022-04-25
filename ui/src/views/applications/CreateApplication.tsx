import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import {
  Application,
  CreateApplicationRequest,
  CreateApplicationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationForm from "./ApplicationForm";
import ApplicationStore from "../../stores/ApplicationStore";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
}

class CreateApplication extends Component<IProps> {
  onFinish = (obj: Application) => {
    obj.setTenantId(this.props.tenant.getId());

    let req = new CreateApplicationRequest();
    req.setApplication(obj);

    ApplicationStore.create(req, (resp: CreateApplicationResponse) => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/applications/${resp.getId()}`);
    });
  };

  render() {
    const app = new Application();

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
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications`}>Applications</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add application"
        />
        <Card>
          <ApplicationForm initialValues={app} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateApplication;
