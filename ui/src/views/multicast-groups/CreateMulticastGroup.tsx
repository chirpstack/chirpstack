import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  CreateMulticastGroupRequest,
  CreateMulticastGroupResponse,
  MulticastGroup,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import MulticastGroupForm from "./MulticastGroupForm";
import MulticastGroupStore from "../../stores/MulticastGroupStore";

interface IProps {
  tenant: Tenant;
  application: Application;
}

function CreateMulticastGroup(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: MulticastGroup) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreateMulticastGroupRequest();
    req.setMulticastGroup(obj);

    MulticastGroupStore.create(req, (resp: CreateMulticastGroupResponse) => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/multicast-groups`);
    });
  };

  let multicastGroup = new MulticastGroup();
  multicastGroup.setApplicationId(props.application.getId());

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
