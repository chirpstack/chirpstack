import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import {
  Application,
  CreateApplicationRequest,
  CreateApplicationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationForm from "./ApplicationForm";
import ApplicationStore from "../../stores/ApplicationStore";

interface IProps {
  tenant: Tenant;
}

function CreateApplication(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: Application) => {
    obj.setTenantId(props.tenant.getId());

    let req = new CreateApplicationRequest();
    req.setApplication(obj);

    ApplicationStore.create(req, (resp: CreateApplicationResponse) => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${resp.getId()}`);
    });
  };

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
                <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications`}>Applications</Link>
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
        <ApplicationForm initialValues={app} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateApplication;
