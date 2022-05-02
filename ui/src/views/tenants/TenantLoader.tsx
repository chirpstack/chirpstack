import React, { Component } from "react";
import { Route, Switch, RouteComponentProps } from "react-router-dom";

import { Tenant, GetTenantResponse } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantStore from "../../stores/TenantStore";
import SessionStore from "../../stores/SessionStore";

import TenantLayout from "./TenantLayout";
import ListTenantUsers from "./ListTenantUsers";
import CreateTenantUser from "./CreateTenantUser";
import EditTenantUser from "./EditTenantUser";
import ListTenantApiKeys from "../api-keys/ListTenantApiKeys";
import CreateTenantApiKey from "../api-keys/CreateTenantApiKey";
import ListDeviceProfiles from "../device-profiles/ListDeviceProfiles";
import CreateDeviceProfile from "../device-profiles/CreateDeviceProfile";
import EditDeviceProfile from "../device-profiles/EditDeviceProfile";
import ListGateways from "../gateways/ListGateways";
import CreateGateway from "../gateways/CreateGateway";
import GatewayLayout from "../gateways/GatewayLayout";
import ListApplications from "../applications/ListApplications";
import CreateApplication from "../applications/CreateApplication";
import ApplicationLoader from "../applications/ApplicationLoader";

interface IState {
  tenant?: Tenant;
  isAdmin: boolean;
  isTenantAdmin: boolean;
  isDeviceAdmin: boolean;
  isGatewayAdmin: boolean;
}

interface MatchParams {
  tenantId: string;
}

interface IProps extends RouteComponentProps<MatchParams> {}

class TenantLoader extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      isAdmin: false,
      isTenantAdmin: false,
      isDeviceAdmin: false,
      isGatewayAdmin: false,
    };
  }

  componentDidMount() {
    SessionStore.on("change", this.setIsAdmin);
    TenantStore.on("change", this.getTenant);

    this.getTenant();
    this.setIsAdmin();
  }

  componentWillUnmount() {
    SessionStore.removeListener("change", this.setIsAdmin);
    TenantStore.removeListener("change", this.getTenant);
  }

  componentDidUpdate(prevProps: RouteComponentProps<MatchParams>) {
    if (this.props.match.params.tenantId === prevProps.match.params.tenantId) {
      return;
    }

    this.getTenant();
    this.setIsAdmin();
  }

  getTenant = () => {
    const tenantId = this.props.match.params.tenantId;

    TenantStore.get(tenantId, (resp: GetTenantResponse) => {
      this.setState({
        tenant: resp.getTenant(),
      });
    });
  };

  setIsAdmin = () => {
    const tenantId = this.props.match.params.tenantId;

    this.setState({
      isAdmin: SessionStore.isAdmin(),
      isTenantAdmin: SessionStore.isAdmin() || SessionStore.isTenantAdmin(tenantId),
      isDeviceAdmin:
        SessionStore.isAdmin() || SessionStore.isTenantAdmin(tenantId) || SessionStore.isTenantDeviceAdmin(tenantId),
      isGatewayAdmin:
        SessionStore.isAdmin() || SessionStore.isTenantAdmin(tenantId) || SessionStore.isTenantGatewayAdmin(tenantId),
    });
  };

  render() {
    if (this.state.tenant === undefined) {
      return null;
    }

    const path = this.props.match.path;
    const tenant: Tenant = this.state.tenant!;

    return (
      <Switch>
        <Route
          exact
          path={`${path}/api-keys`}
          render={props => <ListTenantApiKeys tenant={tenant} isAdmin={this.state.isAdmin} {...props} />}
        />
        <Route
          exact
          path={`${path}/api-keys/create`}
          render={props => <CreateTenantApiKey tenant={tenant} {...props} />}
        />
        <Route
          exact
          path={`${path}/device-profiles`}
          render={props => <ListDeviceProfiles tenant={tenant} {...props} />}
        />
        <Route
          exact
          path={`${path}/device-profiles/create`}
          render={props => <CreateDeviceProfile tenant={tenant} {...props} />}
        />
        <Route
          exact
          path={`${path}/device-profiles/:deviceProfileId([\\w-]{36})/edit`}
          component={(props: any) => <EditDeviceProfile tenant={tenant} {...props} />}
        />
        <Route exact path={`${path}/users`} render={props => <ListTenantUsers tenant={tenant} {...props} />} />
        <Route exact path={`${path}/users/create`} render={props => <CreateTenantUser tenant={tenant} {...props} />} />
        <Route
          exact
          path={`${path}/users/:userId([\\w-]{36})/edit`}
          component={(props: any) => <EditTenantUser tenant={tenant} {...props} />}
        />
        <Route exact path={`${path}/gateways`} render={props => <ListGateways tenant={tenant} {...props} />} />
        <Route exact path={`${path}/gateways/create`} render={props => <CreateGateway tenant={tenant} {...props} />} />
        <Route
          path={`${path}/gateways/:gatewayId([a-f0-9]{16})`}
          component={(props: any) => <GatewayLayout tenant={tenant} {...props} />}
        />
        <Route exact path={`${path}/applications`} render={props => <ListApplications tenant={tenant} {...props} />} />
        <Route
          exact
          path={`${path}/applications/create`}
          render={props => <CreateApplication tenant={tenant} {...props} />}
        />
        <Route
          path={`${path}/applications/:applicationId([\\w-]{36})`}
          component={(props: any) => <ApplicationLoader tenant={tenant} {...props} />}
        />
        <Route path={`${path}`} render={props => <TenantLayout tenant={tenant} {...props} />} />
      </Switch>
    );
  }
}

export default TenantLoader;
