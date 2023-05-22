import React, { Component } from "react";
import { Route, Switch, RouteComponentProps } from "react-router-dom";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import {
  Application,
  GetApplicationRequest,
  GetApplicationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../stores/ApplicationStore";
import ApplicationLayout from "./ApplicationLayout";
import CreateDevice from "../devices/CreateDevice";
import DeviceLayout from "../devices/DeviceLayout";
import MulticastGroupLayout from "../multicast-groups/MulticastGroupLayout";
import CreateMulticastGroup from "../multicast-groups/CreateMulticastGroup";
import RelayLayout from "../relays/RelayLayout";

interface MatchParams {
  applicationId: string;
}

interface IProps extends RouteComponentProps<MatchParams> {
  tenant: Tenant;
}

interface IState {
  application?: Application;
  measurementKeys: string[];
}

class ApplicationLoader extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      measurementKeys: [],
    };
  }

  componentDidMount() {
    this.getApplication();
  }

  getApplication = () => {
    let req = new GetApplicationRequest();
    req.setId(this.props.match.params.applicationId);

    ApplicationStore.get(req, (resp: GetApplicationResponse) => {
      this.setState({
        application: resp.getApplication(),
        measurementKeys: resp.getMeasurementKeysList(),
      });
    });
  };

  render() {
    const app = this.state.application;
    if (!app) {
      return null;
    }

    const path = this.props.match.path;
    const tenant = this.props.tenant;

    return (
      <Switch>
        <Route
          exact
          path={`${path}/devices/create`}
          render={props => <CreateDevice tenant={tenant} application={app} {...props} />}
        />
        <Route
          exact
          path={`${path}/multicast-groups/create`}
          render={props => <CreateMulticastGroup tenant={tenant} application={app} {...props} />}
        />
        <Route
          path={`${path}/multicast-groups/:multicastGroupId([\\w-]{36})`}
          render={(props: any) => <MulticastGroupLayout tenant={tenant} application={app} {...props} />}
        />
        <Route
          path={`${path}/devices/:devEui([0-9a-f]{16})`}
          component={(props: any) => <DeviceLayout tenant={tenant} application={app} {...props} />}
        />
        <Route
          path={`${path}/relays/:relayDevEui([0-9a-f]{16})`}
          component={(props: any) => <RelayLayout tenant={tenant} application={app} {...props} />}
        />
        <Route
          path={path}
          render={props => (
            <ApplicationLayout
              tenant={tenant}
              application={app}
              measurementKeys={this.state.measurementKeys}
              {...props}
            />
          )}
        />
      </Switch>
    );
  }
}

export default ApplicationLoader;
