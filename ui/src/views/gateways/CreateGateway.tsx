import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Gateway, CreateGatewayRequest } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import GatewayForm from "./GatewayForm";
import GatewayStore from "../../stores/GatewayStore";

interface IProps {
  tenant: Tenant;
}

function CreateGateway(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: Gateway) => {
    obj.setTenantId(props.tenant.getId());

    let req = new CreateGatewayRequest();
    req.setGateway(obj);

    GatewayStore.create(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/gateways/${obj.getGatewayId()}`);
    });
  };

  let gateway = new Gateway();
  gateway.setStatsInterval(30);

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        title="Add gateway"
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
                <Link to={`/tenants/${props.tenant.getId()}/gateways`}>Gateways</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Add</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
      />
      <Card>
        <GatewayForm initialValues={gateway} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateGateway;
