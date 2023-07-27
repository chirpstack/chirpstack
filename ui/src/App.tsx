import React, { useState } from "react";
import { Router, Routes, Route } from "react-router-dom";

import { Layout } from "antd";

import { User } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import Header from "./components/Header";
import Menu from "./components/Menu";

// dashboard
import Dashboard from "./views/dashboard/Dashboard";

// users
import Login from "./views/users/Login";
import ListUsers from "./views/users/ListUsers";
import CreateUser from "./views/users/CreateUser";
import EditUser from "./views/users/EditUser";
import ChangeUserPassword from "./views/users/ChangeUserPassword";

// tenants
import TenantRedirect from "./views/tenants/TenantRedirect";
import ListTenants from "./views/tenants/ListTenants";
import CreateTenant from "./views/tenants/CreateTenant";
import TenantLoader from "./views/tenants/TenantLoader";

// api keys
import ListAdminApiKeys from "./views/api-keys/ListAdminApiKeys";
import CreateAdminApiKey from "./views/api-keys/CreateAdminApiKey";

// device-profile templates
import ListDeviceProfileTemplates from "./views/device-profile-templates/ListDeviceProfileTemplates";
import CreateDeviceProfileTemplate from "./views/device-profile-templates/CreateDeviceProfileTemplate";
import EditDeviceProfileTemplate from "./views/device-profile-templates/EditDeviceProfileTemplate";

// regions
import ListRegions from "./views/regions/ListRegions";
import RegionDetails from "./views/regions/RegionDetails";

// stores
import SessionStore from "./stores/SessionStore";

import history from "./history";

const CustomRouter = ({ history, ...props }: any) => {
  const [state, setState] = useState({
    action: history.action,
    location: history.location,
  });

  React.useLayoutEffect(() => history.listen(setState), [history]);

  return <Router {...props} location={state.location} navigationType={state.action} navigator={history} />;
};

function App() {
  const [user, setUser] = useState<User | undefined>(SessionStore.getUser());
  SessionStore.on("change", () => {
    setUser(SessionStore.getUser());
  });

  return (
    <Layout style={{ minHeight: "100vh" }}>
      <CustomRouter history={history}>
        <Routes>
          <Route path="/" element={<TenantRedirect />} />
          <Route path="/login" element={<Login />} />
        </Routes>

        {user && (
          <div>
            <Layout.Header className="layout-header">
              <Header user={user} />
            </Layout.Header>
            <Layout className="layout">
              <Layout.Sider width="300" theme="light" className="layout-menu">
                <Menu />
              </Layout.Sider>
              <Layout.Content className="layout-content" style={{ padding: "24px 24px 24px" }}>
                <Routes>
                  <Route path="/dashboard" element={<Dashboard />} />
                  <Route path="/tenants" element={<ListTenants />} />
                  <Route path="/tenants/create" element={<CreateTenant />} />
                  <Route path="/tenants/:tenantId/*" element={<TenantLoader />} />

                  <Route path="/users" element={<ListUsers />} />
                  <Route path="/users/create" element={<CreateUser />} />
                  <Route path="/users/:userId" element={<EditUser />} />
                  <Route path="/users/:userId/password" element={<ChangeUserPassword />} />

                  <Route path="/api-keys" element={<ListAdminApiKeys />} />
                  <Route path="/api-keys/create" element={<CreateAdminApiKey />} />

                  <Route path="/device-profile-templates" element={<ListDeviceProfileTemplates />} />
                  <Route path="/device-profile-templates/create" element={<CreateDeviceProfileTemplate />} />
                  <Route
                    path="/device-profile-templates/:deviceProfileTemplateId/edit"
                    element={<EditDeviceProfileTemplate />}
                  />

                  <Route path="/regions" element={<ListRegions />} />
                  <Route path="/regions/:id" element={<RegionDetails />} />
                </Routes>
              </Layout.Content>
            </Layout>
          </div>
        )}
      </CustomRouter>
    </Layout>
  );
}

export default App;
