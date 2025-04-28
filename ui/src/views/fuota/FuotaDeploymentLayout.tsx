import { useState, useEffect } from "react";
import { Route, Routes, useNavigate, useParams, useLocation, Link } from "react-router-dom";

import { format } from "date-fns";
import { Space, Breadcrumb, Card, Button, Menu, Popconfirm, Descriptions, DescriptionsProps } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { GetFuotaDeploymentResponse, FuotaDeployment } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import {
  GetFuotaDeploymentRequest,
  DeleteFuotaDeploymentRequest,
  StartFuotaDeploymentRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import FuotaStore from "../../stores/FuotaStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import Admin from "../../components/Admin";
import { useTitle } from "../helpers";
import EditFuotaDeployment from "./EditFuotaDeployment";
import FuotaDeploymentDevices from "./FuotaDeploymentDevices";
import FuotaDeploymentGateways from "./FuotaDeploymentGateways";
import FuotaDeploymentDashboard from "./FuotaDeploymentDashboard";

interface IProps {
  tenant: Tenant;
  application: Application;
}

function FuotaDeploymentLayout(props: IProps) {
  const { fuotaDeploymentId } = useParams();
  const navigate = useNavigate();
  const location = useLocation();
  const [getFuotaDeploymentResponse, setGetFuotaDeploymentResponse] = useState<GetFuotaDeploymentResponse | undefined>(
    undefined,
  );
  const [fuotaDeployment, setFuotaDeployment] = useState<FuotaDeployment | undefined>(undefined);

  useTitle(
    "Tenants",
    props.tenant.getName(),
    "Applications",
    props.application.getName(),
    "FUOTA deployments",
    fuotaDeployment?.getName(),
  );

  useEffect(() => {
    const getDeployment = () => {
      const req = new GetFuotaDeploymentRequest();
      req.setId(fuotaDeploymentId!);

      FuotaStore.getDeployment(req, (resp: GetFuotaDeploymentResponse) => {
        setFuotaDeployment(resp.getDeployment());
        setGetFuotaDeploymentResponse(resp);
      });
    };

    FuotaStore.on("updateDeployment", getDeployment);
    getDeployment();

    return () => {
      FuotaStore.removeListener("updateDeployment", getDeployment);
    };
  }, [fuotaDeploymentId]);

  const startFuotaDeployment = () => {
    const req = new StartFuotaDeploymentRequest();
    req.setId(fuotaDeploymentId!);
    FuotaStore.startDeployment(req, () => {});
  };

  const deleteFuotaDeployment = () => {
    const req = new DeleteFuotaDeploymentRequest();
    req.setId(fuotaDeploymentId!);

    FuotaStore.deleteDeployment(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/fuota`);
    });
  };

  const tenant = props.tenant;
  const app = props.application;
  const d = fuotaDeployment;

  if (!d || !getFuotaDeploymentResponse) {
    return null;
  }

  let tab = "dashboard";
  const path = location.pathname;
  if (path.endsWith("edit")) {
    tab = "edit";
  }
  if (path.endsWith("gateways")) {
    tab = "gateways";
  }
  if (path.endsWith("devices")) {
    tab = "devices";
  }

  let descriptionsItems: DescriptionsProps["items"] = [];
  let startedAt = getFuotaDeploymentResponse.getStartedAt();
  let completedAt = getFuotaDeploymentResponse.getCompletedAt();

  if (startedAt) {
    const ts = new Date(0);
    ts.setUTCSeconds(startedAt.getSeconds());

    descriptionsItems.push({
      key: "startedAt",
      label: "Started at",
      children: <p>{format(ts, "yyyy-MM-dd HH:mm:ss")}</p>,
    });
  }

  if (completedAt) {
    const ts = new Date(0);
    ts.setUTCSeconds(completedAt.getSeconds());

    descriptionsItems.push({
      key: "completedAt",
      label: "Completed at",
      children: <p>{format(ts, "yyyy-MM-dd HH:mm:ss")}</p>,
    });
  }

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
                <Link to={`/tenants/${props.tenant.getId()}/applications/${app.getId()}`}>{app.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications/${app.getId()}/fuota`}>FUOTA deployments</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>{d.getName()}</Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={d.getName()}
        subTitle={`FUOTA deployment id: ${d.getId()}`}
        extra={[
          <Admin tenantId={tenant.getId()} isDeviceAdmin>
            <Space direction="horizontal" style={{ float: "right" }}>
              <Popconfirm
                placement="left"
                title="Start deployment"
                description="Are you sure you want to start the deploymen? Once started, you will not be able to make changes."
                onConfirm={startFuotaDeployment}
              >
                <Button type="primary" disabled={getFuotaDeploymentResponse.getStartedAt() !== undefined}>
                  Start deployment
                </Button>
              </Popconfirm>
              <DeleteConfirm typ="FUOTA deployment" confirm={d.getName()} onConfirm={deleteFuotaDeployment}>
                <Button danger type="primary">
                  Delete FUOTA deployment
                </Button>
              </DeleteConfirm>
            </Space>
          </Admin>,
        ]}
      >
        <Descriptions items={descriptionsItems} />
      </PageHeader>
      <Card>
        <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
          <Menu.Item key="dashboard">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/fuota/${d.getId()}`}>Dashboard</Link>
          </Menu.Item>
          <Menu.Item key="edit">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/fuota/${d.getId()}/edit`}>
              Configuration
            </Link>
          </Menu.Item>
          <Menu.Item key="devices">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/fuota/${d.getId()}/devices`}>
              Devices
            </Link>
          </Menu.Item>
          <Menu.Item key="gateways">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/fuota/${d.getId()}/gateways`}>
              Gateways
            </Link>
          </Menu.Item>
        </Menu>
        <Routes>
          <Route
            path="/"
            element={<FuotaDeploymentDashboard getFuotaDeploymentResponse={getFuotaDeploymentResponse} />}
          />
          <Route
            path="/edit"
            element={
              <EditFuotaDeployment
                fuotaDeployment={d}
                application={app}
                tenant={tenant}
                disabled={getFuotaDeploymentResponse.getStartedAt() !== undefined}
              />
            }
          />
          <Route
            path="/devices"
            element={<FuotaDeploymentDevices getFuotaDeploymentResponse={getFuotaDeploymentResponse} />}
          />
          <Route path="/gateways" element={<FuotaDeploymentGateways fuotaDeployment={d} />} />
        </Routes>
      </Card>
    </Space>
  );
}

export default FuotaDeploymentLayout;
