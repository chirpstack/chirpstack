import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { CreateFuotaDeploymentRequest, FuotaDeployment } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import type { CreateFuotaDeploymentResponse } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import { useTitle } from "../helpers";
import FuotaDeploymentForm from "./FuotaDeploymentForm";
import FuotaStore from "../../stores/FuotaStore";

interface IProps {
  tenant: Tenant;
  application: Application;
}

function CreateFuotaDeployment(props: IProps) {
  const navigate = useNavigate();
  useTitle("Tenants", props.tenant.getName(), "Applications", props.application.getName(), "New FUOTA deployment");

  const onFinish = (obj: FuotaDeployment) => {
    obj.setApplicationId(props.application.getId());

    const req = new CreateFuotaDeploymentRequest();
    req.setDeployment(obj);

    FuotaStore.createDeployment(req, (resp: CreateFuotaDeploymentResponse) => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/fuota`);
    });
  };

  const fuotaDeployment = new FuotaDeployment();
  fuotaDeployment.setApplicationId(props.application.getId());
  fuotaDeployment.setCalculateFragmentationFragmentSize(true);
  fuotaDeployment.setCalculateMulticastTimeout(true);

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
              <span>New FUOTA deployment</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="New FUOTA deployment"
      />
      <Card>
        <FuotaDeploymentForm tenant={props.tenant} initialValues={fuotaDeployment} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateFuotaDeployment;
