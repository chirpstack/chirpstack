import React, { Component } from "react";
import { Route, Switch, RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, PageHeader, Menu } from "antd";

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

interface IProps extends RouteComponentProps {
  tenant: Tenant;
  application: Application;
  measurementKeys: string[];
}

class ApplicationLayout extends Component<IProps> {
  deleteApplication = () => {
    let req = new DeleteApplicationRequest();
    req.setId(this.props.application.getId());

    ApplicationStore.delete(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/applications`);
    });
  };

  render() {
    const tenant = this.props.tenant;
    const app = this.props.application;

    if (!app) {
      return null;
    }

    const path = this.props.history.location.pathname;
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
                  <Link to={`/tenants/${this.props.tenant.getId()}`}>{this.props.tenant.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications`}>Applications</Link>
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
            <Admin tenantId={this.props.tenant.getId()} isDeviceAdmin>
              <DeleteConfirm confirm={app.getName()} typ="application" onConfirm={this.deleteApplication}>
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
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups`}>
                Multicast groups
              </Link>
            </Menu.Item>
            <Menu.Item key="relay">
              <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/relays`}>
                Relays
              </Link>
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
          <Switch>
            <Route exact path={this.props.match.path} render={props => <ListDevices application={app} {...props} />} />
            <Route
              exact
              path={`${this.props.match.path}/edit`}
              render={props => <EditApplication application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations`}
              render={props => <ListIntegrations application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/multicast-groups`}
              render={props => <ListMulticastGroups application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/relays`}
              render={props => <ListRelays application={app} {...props} />}
            />

            <Route
              exact
              path={`${this.props.match.path}/integrations/http/create`}
              render={props => <CreateHttpIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/http/edit`}
              render={props => <EditHttpIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/aws-sns/create`}
              render={props => <CreateAwsSnsIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/aws-sns/edit`}
              render={props => <EditAwsSnsIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/azure-service-bus/create`}
              render={props => <CreateAzureServiceBusIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/azure-service-bus/edit`}
              render={props => <EditAzureServiceBusIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/gcp-pub-sub/create`}
              render={props => <CreateGcpPubSubIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/gcp-pub-sub/edit`}
              render={props => <EditGcpPubSubIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/influxdb/create`}
              render={props => <CreateInfluxDbIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/influxdb/edit`}
              render={props => <EditInfluxDbIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/mydevices/create`}
              render={props => <CreateMyDevicesIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/mydevices/edit`}
              render={props => <EditMyDevicesIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/pilot-things/create`}
              render={props => <CreatePilotThingsIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/pilot-things/edit`}
              render={props => <EditPilotThingsIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/loracloud/create`}
              render={props => <CreateLoRaCloudIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/loracloud/edit`}
              render={props => <EditLoRaCloudIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/thingsboard/create`}
              render={props => <CreateThingsBoardIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/thingsboard/edit`}
              render={props => <EditThingsBoardIntegration application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/mqtt/certificate`}
              render={props => <GenerateMqttCertificate application={app} {...props} />}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/ifttt/create`}
              render={props => (
                <CreateIftttIntegration application={app} measurementKeys={this.props.measurementKeys} {...props} />
              )}
            />
            <Route
              exact
              path={`${this.props.match.path}/integrations/ifttt/edit`}
              render={props => (
                <EditIftttIntegration application={app} measurementKeys={this.props.measurementKeys} {...props} />
              )}
            />
          </Switch>
        </Card>
      </Space>
    );
  }
}

export default ApplicationLayout;
