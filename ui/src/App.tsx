import React, { Component } from "react";
import { Router, Route, Switch } from "react-router-dom";

import { Layout } from "antd";

import { User } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import Menu from "./components/Menu";
import Header from "./components/Header";

// dashboard
import Dashboard from "./views/dashboard/Dashboard";

// users
import Login from "./views/users/Login";

// tenants
import TenantRedirect from "./views/tenants/TenantRedirect";
import ListTenants from "./views/tenants/ListTenants";
import CreateTenant from "./views/tenants/CreateTenant";
import TenantLoader from "./views/tenants/TenantLoader";

// users
import ListUsers from "./views/users/ListUsers";
import CreateUser from "./views/users/CreateUser";
import EditUser from "./views/users/EditUser";
import ChangeUserPassword from "./views/users/ChangeUserPassword";

// api keys
import ListAdminApiKeys from "./views/api-keys/ListAdminApiKeys";
import CreateAdminApiKey from "./views/api-keys/CreateAdminApiKey";

// device-profile templates
import ListDeviceProfileTemplates from "./views/device-profile-templates/ListDeviceProfileTemplates";
import EditDeviceProfileTemplate from "./views/device-profile-templates/EditDeviceProfileTemplate";
import CreateDeviceProfileTemplate from "./views/device-profile-templates/CreateDeviceProfileTemplate";

// stores
import SessionStore from "./stores/SessionStore";

import history from "./history";

interface IProps {}

interface IState {
  user?: User;
}

class App extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      user: undefined,
    };
  }

  componentDidMount() {
    SessionStore.on("change", () => {
      this.setState({
        user: SessionStore.getUser(),
      });
    });

    this.setState({
      user: SessionStore.getUser(),
    });
  }

  render() {
    return (
      <Layout style={{ minHeight: "100vh" }}>
        <Router history={history}>
          <Switch>
            <Route exact path="/" component={TenantRedirect} />
            <Route exact path="/login" component={Login} />
            {this.state.user && (
              <Route>
                <Layout.Header className="layout-header">
                  <Header user={this.state.user} />
                </Layout.Header>
                <Layout className="layout">
                  <Layout.Sider width="300" theme="light" className="layout-menu">
                    <Menu />
                  </Layout.Sider>
                  <Layout.Content className="layout-content" style={{ padding: "24px 24px 24px" }}>
                    <Switch>
                      <Route exact path="/dashboard" component={Dashboard} />

                      <Route exact path="/tenants" component={ListTenants} />
                      <Route exact path="/tenants/create" component={CreateTenant} />
                      <Route path="/tenants/:tenantId([\w-]{36})" component={TenantLoader} />

                      <Route exact path="/users" component={ListUsers} />
                      <Route exact path="/users/create" component={CreateUser} />
                      <Route exact path="/users/:userId([\w-]{36})" component={EditUser} />
                      <Route exact path="/users/:userId([\w-]{36})/password" component={ChangeUserPassword} />

                      <Route exact path="/api-keys" component={ListAdminApiKeys} />
                      <Route exact path="/api-keys/create" component={CreateAdminApiKey} />

                      <Route exact path="/device-profile-templates" component={ListDeviceProfileTemplates} />
                      <Route exact path="/device-profile-templates/create" component={CreateDeviceProfileTemplate} />
                      <Route
                        exact
                        path="/device-profile-templates/:deviceProfileTemplateId([\w-]+)/edit"
                        component={EditDeviceProfileTemplate}
                      />
                    </Switch>
                  </Layout.Content>
                </Layout>
              </Route>
            )}
          </Switch>
        </Router>
      </Layout>
    );
  }
}

export default App;
