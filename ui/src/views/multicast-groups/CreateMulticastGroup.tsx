import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type { CreateMulticastGroupResponse } from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  CreateMulticastGroupRequest,
  MulticastGroup,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import MulticastGroupForm from "./MulticastGroupForm";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import { useTitle } from "../helpers";

interface IProps {
  tenant: Tenant;
  application: Application;
}

function CreateMulticastGroup(props: IProps) {
  const navigate = useNavigate();
  useTitle("Tenants", props.tenant.getName(), "Applications", props.application.getName(), "Add multicast-group");

  const onFinish = (obj: MulticastGroup) => {
    obj.setApplicationId(props.application.getId());

    const req = new CreateMulticastGroupRequest();
    req.setMulticastGroup(obj);

    MulticastGroupStore.create(req, (resp: CreateMulticastGroupResponse) => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/multicast-groups`);
    });
  };

  const multicastGroup = new MulticastGroup();
  multicastGroup.setApplicationId(props.application.getId());

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Building</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications`}>Applications</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}`}>
                  {props.application.getName()}
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
        <MulticastGroupForm initialValues={multicastGroup} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateMulticastGroup;
