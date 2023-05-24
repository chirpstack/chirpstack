import React, { Component } from "react";
import { withRouter, RouteComponentProps, Link } from "react-router-dom";

import { Menu, MenuProps } from "antd";
import {
  CloudOutlined,
  HomeOutlined,
  UserOutlined,
  DashboardOutlined,
  KeyOutlined,
  WifiOutlined,
  ControlOutlined,
  AppstoreOutlined,
  CompassOutlined,
} from "@ant-design/icons";

import {
  GetTenantResponse,
  ListTenantsRequest,
  ListTenantsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import Autocomplete, { OptionCallbackFunc, OptionsCallbackFunc } from "../components/Autocomplete";
import TenantStore from "../stores/TenantStore";
import SessionStore from "../stores/SessionStore";

interface IState {
  tenantId: string;
  selectedKey: string;
}

class SideMenu extends Component<RouteComponentProps, IState> {
  constructor(props: RouteComponentProps) {
    super(props);

    this.state = {
      tenantId: "",
      selectedKey: "ns-dashboard",
    };
  }

  componentDidMount() {
    SessionStore.on("tenant.change", this.setTenant);
    this.setTenant();
    this.parseLocation();
  }

  componentWillUnmount() {
    SessionStore.removeListener("tenant.change", this.setTenant);
  }

  componentDidUpdate(prevProps: RouteComponentProps) {
    if (this.props === prevProps) {
      return;
    }

    this.parseLocation();
  }

  setTenant = () => {
    this.setState({
      tenantId: SessionStore.getTenantId(),
    });
  };

  getTenantOptions = (search: string, fn: OptionsCallbackFunc) => {
    let req = new ListTenantsRequest();
    req.setSearch(search);
    req.setLimit(10);

    TenantStore.list(req, (resp: ListTenantsResponse) => {
      const options = resp.getResultList().map((o, i) => {
        return { label: o.getName(), value: o.getId() };
      });
      fn(options);
    });
  };

  getTenantOption = (id: string, fn: OptionCallbackFunc) => {
    TenantStore.get(id, (resp: GetTenantResponse) => {
      const tenant = resp.getTenant();
      if (tenant) {
        fn({ label: tenant.getName(), value: tenant.getId() });
      }
    });
  };

  onTenantSelect = (value: string) => {
    SessionStore.setTenantId(value);
    this.props.history.push(`/tenants/${value}`);
  };

  parseLocation = () => {
    const path = this.props.history.location.pathname;
    const tenantRe = /\/tenants\/([\w-]{36})/g;
    const match = tenantRe.exec(path);

    if (match !== null && this.state.tenantId !== match[1]) {
      SessionStore.setTenantId(match[1]);
    }

    // ns dashboard
    if (path === "/dashboard") {
      this.setState({ selectedKey: "ns-dashboard" });
    }

    // ns tenants
    if (/\/tenants(\/([\w-]{36}\/edit|create))?/g.exec(path)) {
      this.setState({ selectedKey: "ns-tenants" });
    }

    // ns tenants
    if (/\/users(\/([\w-]{36}\/edit|create))?/g.exec(path)) {
      this.setState({ selectedKey: "ns-users" });
    }

    // ns api keys
    if (/\/api-keys(\/([\w-]{36}\/edit|create))?/g.exec(path)) {
      this.setState({ selectedKey: "ns-api-keys" });
    }

    // ns device-profile templates
    if (/\/device-profile-templates(\/([\w-]{36}\/edit|create))?/g.exec(path)) {
      this.setState({ selectedKey: "ns-device-profile-templates" });
    }

    if (/\/regions.*/g.exec(path)) {
      this.setState({ selectedKey: "ns-regions" });
    }

    // tenant dashboard
    if (/\/tenants\/[\w-]{36}/g.exec(path)) {
      this.setState({ selectedKey: "tenant-dashboard" });
    }

    // tenant users
    if (/\/tenants\/[\w-]{36}\/users.*/g.exec(path)) {
      this.setState({ selectedKey: "tenant-users" });
    }

    // tenant api-keys
    if (/\/tenants\/[\w-]{36}\/api-keys.*/g.exec(path)) {
      this.setState({ selectedKey: "tenant-api-keys" });
    }

    // tenant device-profiles
    if (/\/tenants\/[\w-]{36}\/device-profiles.*/g.exec(path)) {
      this.setState({ selectedKey: "tenant-device-profiles" });
    }

    // tenant gateways
    if (/\/tenants\/[\w-]{36}\/gateways.*/g.exec(path)) {
      this.setState({ selectedKey: "tenant-gateways" });
    }

    // tenant applications
    if (/\/tenants\/[\w-]{36}\/applications.*/g.exec(path)) {
      this.setState({ selectedKey: "tenant-applications" });
    }
  };

  render() {
    const tenantId = this.state.tenantId;
    let items: MenuProps["items"] = [];

    if (SessionStore.isAdmin()) {
      items.push({
        key: "ns",
        label: "Network Server",
        icon: <CloudOutlined />,
        children: [
          { key: "ns-dashboard", icon: <DashboardOutlined />, label: <Link to="/dashboard">Dashboard</Link> },
          { key: "ns-tenants", icon: <HomeOutlined />, label: <Link to="/tenants">Tenants</Link> },
          { key: "ns-users", icon: <UserOutlined />, label: <Link to="/users">Users</Link> },
          { key: "ns-api-keys", icon: <KeyOutlined />, label: <Link to="/api-keys">API Keys</Link> },
          {
            key: "ns-device-profile-templates",
            icon: <ControlOutlined />,
            label: <Link to="/device-profile-templates">Device Profile Templates</Link>,
          },
          { key: "ns-regions", icon: <CompassOutlined />, label: <Link to="/regions">Regions</Link> },
        ],
      });
    } else {
      items.push({
        key: "ns",
        label: "Network Server",
        icon: <CloudOutlined />,
        children: [{ key: "ns-regions", icon: <CompassOutlined />, label: <Link to="/regions">Regions</Link> }],
      });
    }

    if (tenantId !== "") {
      items.push({
        key: "tenant",
        label: "Tenant",
        icon: <HomeOutlined />,
        children: [
          {
            key: "tenant-dashboard",
            icon: <DashboardOutlined />,
            label: <Link to={`/tenants/${tenantId}`}>Dashboard</Link>,
          },
          { key: "tenant-users", icon: <UserOutlined />, label: <Link to={`/tenants/${tenantId}/users`}>Users</Link> },
          {
            key: "tenant-api-keys",
            icon: <KeyOutlined />,
            label: <Link to={`/tenants/${tenantId}/api-keys`}>API Keys</Link>,
          },
          {
            key: "tenant-device-profiles",
            icon: <ControlOutlined />,
            label: <Link to={`/tenants/${tenantId}/device-profiles`}>Device Profiles</Link>,
          },
          {
            key: "tenant-gateways",
            icon: <WifiOutlined />,
            label: <Link to={`/tenants/${tenantId}/gateways`}>Gateways</Link>,
          },
          {
            key: "tenant-applications",
            icon: <AppstoreOutlined />,
            label: <Link to={`/tenants/${tenantId}/applications`}>Applications</Link>,
          },
        ],
      });
    }

    return (
      <div>
        <Autocomplete
          placeholder="Select tenant"
          className="organiation-select"
          getOption={this.getTenantOption}
          getOptions={this.getTenantOptions}
          onSelect={this.onTenantSelect}
          value={this.state.tenantId}
        />
        <Menu
          mode="inline"
          openKeys={["ns", "tenant"]}
          selectedKeys={[this.state.selectedKey]}
          expandIcon={<div></div>}
          items={items}
        />
      </div>
    );
  }
}

export default withRouter(SideMenu);
