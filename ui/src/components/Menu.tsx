import { useState, useEffect, useCallback } from "react";
import { Link, useLocation, useNavigate } from "react-router-dom";

import type { MenuProps } from "antd";
import { Menu, Typography } from "antd";
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
  RadarChartOutlined,
} from "@ant-design/icons";

import type { GetTenantResponse, ListTenantsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { ListTenantsRequest } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import type { GetVersionResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import type { OptionCallbackFunc, OptionsCallbackFunc } from "../components/Autocomplete";
import Autocomplete from "../components/Autocomplete";
import Admin from "../components/Admin";
import TenantStore from "../stores/TenantStore";
import SessionStore from "../stores/SessionStore";
import InternalStore from "../stores/InternalStore";

function SideMenu() {
  const [tenantId, setTenantId] = useState<string>("");
  const [selectedKey, setSelectedKey] = useState<string>("");
  const [version, setVersion] = useState<string>("");

  const location = useLocation();
  const navigate = useNavigate();

  const setTenant = () => {
    setTenantId(SessionStore.getTenantId());
  };

  const getTenantOptions = (search: string, fn: OptionsCallbackFunc) => {
    const req = new ListTenantsRequest();
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

  const parseLocation = useCallback(() => {
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

    // tenant gateway-mesh
    if (/\/tenants\/[\w-]{36}\/gateways\/mesh.*/g.exec(path)) {
      setSelectedKey("tenant-gateways-mesh");
    }

    // tenant applications
    if (/\/tenants\/[\w-]{36}\/applications.*/g.exec(path)) {
      setSelectedKey("tenant-applications");
    }
  }, [location.pathname, tenantId]);

  useEffect(() => {
    SessionStore.on("tenant.change", setTenant);
    setTenant();
    parseLocation();

    if (SessionStore.isAdmin()) {
      InternalStore.getVersion((resp: GetVersionResponse) => {
        setVersion(resp.getVersion());
      });
    }

    return () => {
      SessionStore.removeListener("tenant.change", setTenant);
    };
  }, [parseLocation]);

  useEffect(() => {
    parseLocation();
  }, [location, parseLocation]);

  const items: MenuProps["items"] = [];

  if (SessionStore.isAdmin()) {
    items.push();
  } else {
    items.push();
  }

  if (tenantId !== "") {
    items.push({
      key: "tenant",
      label: "Building",
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
          key: "ns-device-profile-templates",
          icon: <ControlOutlined />,
          label: <Link to="/device-profile-templates">Device Profile Templates</Link>,
        },
        {
          key: "tenant-device-profiles",
          icon: <ControlOutlined />,
          label: <Link to={`/tenants/${tenantId}/device-profiles`}>Device Profiles</Link>,
        },
        {
          key: "ns-regions",
          icon: <CompassOutlined />,
          label: <Link to="/regions">Regions</Link>,
        },
        {
          key: "tenant-gateways",
          icon: <WifiOutlined />,
          label: <Link to={`/tenants/${tenantId}/gateways`}>Gateways</Link>,
        },
        {
          key: "tenant-gateways-mesh",
          icon: <RadarChartOutlined />,
          label: <Link to={`/tenants/${tenantId}/gateways/mesh/relays`}>Gateway Mesh</Link>,
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
        className="tenant-select"
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
      <Admin>
        <Typography.Text type="secondary" className="version">
          Version: v{version}
        </Typography.Text>
      </Admin>
    </div>
  );
}

export default SideMenu;
