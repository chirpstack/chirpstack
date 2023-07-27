import { Route, Routes, Link, useNavigate, useLocation } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, Menu } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application, DeleteApplicationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../stores/ApplicationStore";
import SessionStore from "../../stores/SessionStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import ListDevices from "../devices/ListDevices";
import ListRelays from "../relays/ListRelays";
import EditApplication from "./EditApplication";
import ListIntegrations from "./ListIntegrations";
import ListMulticastGroups from "../multicast-groups/ListMulticastGroups";
import Admin from "../../components/Admin";

import CreateHttpIntegration from "./integrations/CreateHttpIntegration";
import EditHttpIntegration from "./integrations/EditHttpIntegration";
import CreateAwsSnsIntegration from "./integrations/CreateAwsSnsIntegration";
import EditAwsSnsIntegration from "./integrations/EditAwsSnsIntegration";
import CreateAzureServiceBusIntegration from "./integrations/CreateAzureServiceBusIntegration";
import EditAzureServiceBusIntegration from "./integrations/EditAzureServiceBusIntegration";
import CreateGcpPubSubIntegration from "./integrations/CreateGcpPubSubIntegration";
import EditGcpPubSubIntegration from "./integrations/EditGcpPubSubIntegration";
import CreateInfluxDbIntegration from "./integrations/CreateInfluxDbIntegration";
import EditInfluxDbIntegration from "./integrations/EditInfluxDbIntegration";
import CreateMyDevicesIntegration from "./integrations/CreateMyDevicesIntegration";
import EditMyDevicesIntegration from "./integrations/EditMyDevicesIntegration";
import CreatePilotThingsIntegration from "./integrations/CreatePilotThingsIntegration";
import EditPilotThingsIntegration from "./integrations/EditPilotThingsIntegration";
import CreateLoRaCloudIntegration from "./integrations/CreateLoRaCloudIntegration";
import EditLoRaCloudIntegration from "./integrations/EditLoRaCloudIntegration";
import CreateThingsBoardIntegration from "./integrations/CreateThingsBoardIntegration";
import EditThingsBoardIntegration from "./integrations/EditThingsBoardIntegration";
import GenerateMqttCertificate from "./integrations/GenerateMqttCertificate";
import CreateIftttIntegration from "./integrations/CreateIftttIntegration";
import EditIftttIntegration from "./integrations/EditIftttIntegration";

interface IProps {
  tenant: Tenant;
  application: Application;
  measurementKeys: string[];
}

function ApplicationLayout(props: IProps) {
  const navigate = useNavigate();
  const location = useLocation();

  const deleteApplication = () => {
    let req = new DeleteApplicationRequest();
    req.setId(props.application.getId());

    ApplicationStore.delete(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/applications`);
    });
  };

  const tenant = props.tenant;
  const app = props.application;

  if (!app) {
    return null;
  }

  const path = location.pathname;
  let tab = "devices";

  if (path.endsWith("/multicast-groups")) {
    tab = "mg";
  }
  if (path.endsWith("/relays")) {
    tab = "relay";
  }
  if (path.endsWith("/edit")) {
    tab = "edit";
  }
  if (path.match(/.*\/integrations.*/g)) {
    tab = "integrations";
  }

  const showIntegrations =
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(tenant.getId()) ||
    SessionStore.isTenantDeviceAdmin(tenant.getId());

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
              <span>{app.getName()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={app.getName()}
        subTitle={`application id: ${app.getId()}`}
        extra={[
          <Admin tenantId={props.tenant.getId()} isDeviceAdmin>
            <DeleteConfirm confirm={app.getName()} typ="application" onConfirm={deleteApplication}>
              <Button danger type="primary">
                Delete application
              </Button>
            </DeleteConfirm>
          </Admin>,
        ]}
      />
      <Card>
        <Menu mode="horizontal" selectedKeys={[tab]} style={{ marginBottom: 24 }}>
          <Menu.Item key="devices">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}`}>Devices</Link>
          </Menu.Item>
          <Menu.Item key="mg">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups`}>Multicast groups</Link>
          </Menu.Item>
          <Menu.Item key="relay">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/relays`}>Relays</Link>
          </Menu.Item>
          <Menu.Item key="edit">
            <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/edit`}>Application configuration</Link>
          </Menu.Item>
          {showIntegrations && (
            <Menu.Item key="integrations">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/integrations`}>Integrations</Link>
            </Menu.Item>
          )}
        </Menu>
        <Routes>
          <Route path="/" element={<ListDevices application={app} />} />
          <Route path="/edit" element={<EditApplication application={app} />} />
          <Route path="/integrations" element={<ListIntegrations application={app} />} />
          <Route path="/multicast-groups" element={<ListMulticastGroups application={app} />} />
          <Route path="/relays" element={<ListRelays application={app} />} />

          <Route path="/integrations/http/create" element={<CreateHttpIntegration application={app} />} />
          <Route path="/integrations/http/edit" element={<EditHttpIntegration application={app} />} />

          <Route path="/integrations/http/create" element={<CreateHttpIntegration application={app} />} />
          <Route path="/integrations/http/edit" element={<EditHttpIntegration application={app} />} />
          <Route path="/integrations/aws-sns/create" element={<CreateAwsSnsIntegration application={app} />} />
          <Route path="/integrations/aws-sns/edit" element={<EditAwsSnsIntegration application={app} />} />
          <Route
            path="/integrations/azure-service-bus/create"
            element={<CreateAzureServiceBusIntegration application={app} />}
          />
          <Route
            path="/integrations/azure-service-bus/edit"
            element={<EditAzureServiceBusIntegration application={app} />}
          />
          <Route path="/integrations/gcp-pub-sub/create" element={<CreateGcpPubSubIntegration application={app} />} />
          <Route path="/integrations/gcp-pub-sub/edit" element={<EditGcpPubSubIntegration application={app} />} />
          <Route path="/integrations/influxdb/create" element={<CreateInfluxDbIntegration application={app} />} />
          <Route path="/integrations/influxdb/edit" element={<EditInfluxDbIntegration application={app} />} />
          <Route path="/integrations/mydevices/create" element={<CreateMyDevicesIntegration application={app} />} />
          <Route path="/integrations/mydevices/edit" element={<EditMyDevicesIntegration application={app} />} />
          <Route
            path="/integrations/pilot-things/create"
            element={<CreatePilotThingsIntegration application={app} />}
          />
          <Route path="/integrations/pilot-things/edit" element={<EditPilotThingsIntegration application={app} />} />
          <Route path="/integrations/loracloud/create" element={<CreateLoRaCloudIntegration application={app} />} />
          <Route path="/integrations/loracloud/edit" element={<EditLoRaCloudIntegration application={app} />} />
          <Route path="/integrations/thingsboard/create" element={<CreateThingsBoardIntegration application={app} />} />
          <Route path="/integrations/thingsboard/edit" element={<EditThingsBoardIntegration application={app} />} />
          <Route path="/integrations/mqtt/certificate" element={<GenerateMqttCertificate application={app} />} />
          <Route
            path="/integrations/ifttt/create"
            element={<CreateIftttIntegration application={app} measurementKeys={props.measurementKeys} />}
          />
          <Route
            path="/integrations/ifttt/edit"
            element={<EditIftttIntegration application={app} measurementKeys={props.measurementKeys} />}
          />
        </Routes>
      </Card>
    </Space>
  );
}

export default ApplicationLayout;
