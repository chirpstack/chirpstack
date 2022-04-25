import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  CreateMulticastGroupRequest,
  CreateMulticastGroupResponse,
  MulticastGroup,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import MulticastGroupForm from "./MulticastGroupForm";
import MulticastGroupStore from "../../stores/MulticastGroupStore";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
  application: Application;
}

class CreateMulticastGroup extends Component<IProps> {
  onFinish = (obj: MulticastGroup) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateMulticastGroupRequest();
    req.setMulticastGroup(obj);

    MulticastGroupStore.create(req, (resp: CreateMulticastGroupResponse) => {
      this.props.history.push(
        `/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}/multicast-groups`,
      );
    });
  };

  render() {
    let multicastGroup = new MulticastGroup();
    multicastGroup.setApplicationId(this.props.application.getId());

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
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}`}>
                    {this.props.application.getName()}
                  </Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add multicast-group</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add multicast-group"
        />
        <Card>
          <MulticastGroupForm initialValues={multicastGroup} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateMulticastGroup;
