import React, { useState, useEffect } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";

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

function SideMenu() {
  const [tenantId, setTenantId] = useState<string>("");
  const [selectedKey, setSelectedKey] = useState<string>("");

  const location = useLocation();
  const navigate = useNavigate();

  const setTenant = () => {
    setTenantId(SessionStore.getTenantId());
  };

  const getTenantOptions = (search: string, fn: OptionsCallbackFunc) => {
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

  const getTenantOption = (id: string, fn: OptionCallbackFunc) => {
    TenantStore.get(id, (resp: GetTenantResponse) => {
      const tenant = resp.getTenant();
      if (tenant) {
        fn({ label: tenant.getName(), value: tenant.getId() });
      }
    });
  };

  const onTenantSelect = (value: string) => {
    SessionStore.setTenantId(value);
    navigate(`/tenants/${value}`);
  };

  const parseLocation = () => {
    const path = location.pathname;
    const tenantRe = /\/tenants\/([\w-]{36})/g;
    const match = tenantRe.exec(path);

    if (match !== null && tenantId !== match[1]) {
      SessionStore.setTenantId(match[1]);
    }

    // ns dashboard
    if (path === "/dashboard") {
      setSelectedKey("ns-dashboard");
    }

    // ns tenants
    if (/\/tenants(\/([\w-]{36}\/edit|create))?/g.exec(path)) {
      setSelectedKey("ns-tenants");
    }

    // ns tenants
    if (/\/users(\/([\w-]{36}\/edit|create))?/g.exec(path)) {
      setSelectedKey("ns-users");
    }

    // ns api keys
    if (/\/api-keys(\/([\w-]{36}\/edit|create))?/g.exec(path)) {
      setSelectedKey("ns-api-keys");
    }

    // ns device-profile templates
    if (/\/device-profile-templates(\/([\w-]{36}\/edit|create))?/g.exec(path)) {
      setSelectedKey("ns-device-profile-templates");
    }

    if (/\/regions.*/g.exec(path)) {
      setSelectedKey("ns-regions");
    }

    // tenant dashboard
    if (/\/tenants\/[\w-]{36}/g.exec(path)) {
      setSelectedKey("tenant-dashboard");
    }

    // tenant users
    if (/\/tenants\/[\w-]{36}\/users.*/g.exec(path)) {
      setSelectedKey("tenant-users");
    }

    // tenant api-keys
    if (/\/tenants\/[\w-]{36}\/api-keys.*/g.exec(path)) {
      setSelectedKey("tenant-api-keys");
    }

    // tenant device-profiles
    if (/\/tenants\/[\w-]{36}\/device-profiles.*/g.exec(path)) {
      setSelectedKey("tenant-device-profiles");
    }

    // tenant gateways
    if (/\/tenants\/[\w-]{36}\/gateways.*/g.exec(path)) {
      setSelectedKey("tenant-gateways");
    }

    // tenant applications
    if (/\/tenants\/[\w-]{36}\/applications.*/g.exec(path)) {
      setSelectedKey("tenant-applications");
    }
  };

  useEffect(() => {
    SessionStore.on("tenant.change", setTenant);
    setTenant();
    parseLocation();

    return () => {
      SessionStore.removeListener("tenant.change", setTenant);
    };
  }, []);

  useEffect(() => {
    parseLocation();
  }, [location]);

  let items: MenuProps["items"] = [];

  if (SessionStore.isAdmin()) {
    items.push({
      key: "ns",
      label: "Network Server",
      icon: <CloudOutlined />,
      children: [
        {
          key: "ns-dashboard",
          icon: <DashboardOutlined />,
          label: <Link to="/dashboard">Dashboard</Link>,
        },
        {
          key: "ns-tenants",
          icon: <HomeOutlined />,
          label: <Link to="/tenants">Tenants</Link>,
        },
        {
          key: "ns-users",
          icon: <UserOutlined />,
          label: <Link to="/users">Users</Link>,
        },
        {
          key: "ns-api-keys",
          icon: <KeyOutlined />,
          label: <Link to="/api-keys">API Keys</Link>,
        },
        {
          key: "ns-device-profile-templates",
          icon: <ControlOutlined />,
          label: <Link to="/device-profile-templates">Device Profile Templates</Link>,
        },
        {
          key: "ns-regions",
          icon: <CompassOutlined />,
          label: <Link to="/regions">Regions</Link>,
        },
      ],
    });
  } else {
    items.push({
      key: "ns",
      label: "Network Server",
      icon: <CloudOutlined />,
      children: [
        {
          key: "ns-regions",
          icon: <CompassOutlined />,
          label: <Link to="/regions">Regions</Link>,
        },
      ],
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
        {
          key: "tenant-users",
          icon: <UserOutlined />,
          label: <Link to={`/tenants/${tenantId}/users`}>Users</Link>,
        },
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
        getOption={getTenantOption}
        getOptions={getTenantOptions}
        onSelect={onTenantSelect}
        value={tenantId}
      />
      <Menu
        mode="inline"
        openKeys={["ns", "tenant"]}
        selectedKeys={[selectedKey]}
        expandIcon={<div></div>}
        items={items}
      />
    </div>
  );
}

export default SideMenu;
