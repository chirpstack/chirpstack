import React, { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";

import { Button, Menu, Dropdown, Input, AutoComplete } from "antd";
import { UserOutlined, DownOutlined, QuestionOutlined } from "@ant-design/icons";

import { User } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";
import {
  SettingsResponse,
  GlobalSearchRequest,
  GlobalSearchResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import InternalStore from "../stores/InternalStore";
import SessionStore from "../stores/SessionStore";

const renderTitle = (title: string) => <span>{title}</span>;

const renderItem = (title: string, url: string) => ({
  value: title,
  label: <Link to={url}>{title}</Link>,
});

function Header({ user }: { user: User }) {
  const navigate = useNavigate();

  const [settings, setSettings] = useState<SettingsResponse | undefined>(undefined);
  const [searchResult, setSearchResult] = useState<GlobalSearchResponse | undefined>(undefined);

  useEffect(() => {
    InternalStore.settings((resp: SettingsResponse) => {
      setSettings(resp);
    });
  }, [user]);

  const onSearch = (search: string) => {
    if (search.length < 3) {
      return;
    }

    let req = new GlobalSearchRequest();
    req.setLimit(20);
    req.setSearch(search);

    InternalStore.globalSearch(req, (resp: GlobalSearchResponse) => {
      setSearchResult(resp);
    });
  };

  const onLogout = () => {
    if (settings === undefined) {
      return;
    }

    let oidc = settings.getOpenidConnect()!;
    let oAuth2 = settings.getOauth2()!;

    if (oidc.getEnabled() && oidc.getLogoutUrl() !== "") {
      SessionStore.logout(false, () => {
        window.location.replace(oidc.getLogoutUrl());
      });
    } else if (oAuth2.getEnabled() && oAuth2.getLogoutUrl() !== "") {
      SessionStore.logout(false, () => {
        window.location.replace(oAuth2.getLogoutUrl());
      });
    } else {
      SessionStore.logout(true, () => {
        navigate("/login");
      });
    }
  };

  if (settings === undefined) {
    return null;
  }

  let oidcEnabled = settings!.getOpenidConnect()!.getEnabled();
  let oAuth2Enabled = settings!.getOauth2()!.getEnabled();

  const menu = (
    <Menu>
      {!(oidcEnabled || oAuth2Enabled) && (
        <Menu.Item>
          <Link to={`/users/${user.getId()}/password`}>Change password</Link>
        </Menu.Item>
      )}
      <Menu.Item onClick={onLogout}>Logout</Menu.Item>
    </Menu>
  );

  let options: {
    label: any;
    options: any[];
  }[] = [
    {
      label: renderTitle("Tenants"),
      options: [],
    },
    {
      label: renderTitle("Gateways"),
      options: [],
    },
    {
      label: renderTitle("Applications"),
      options: [],
    },
    {
      label: renderTitle("Devices"),
      options: [],
    },
  ];

  if (searchResult !== undefined) {
    for (const res of searchResult.getResultList()) {
      if (res.getKind() === "tenant") {
        options[0].options.push(renderItem(res.getTenantName(), `/tenants/${res.getTenantId()}`));
      }

      if (res.getKind() === "gateway") {
        options[1].options.push(
          renderItem(res.getGatewayName(), `/tenants/${res.getTenantId()}/gateways/${res.getGatewayId()}`),
        );
      }

      if (res.getKind() === "application") {
        options[2].options.push(
          renderItem(res.getApplicationName(), `/tenants/${res.getTenantId()}/applications/${res.getApplicationId()}`),
        );
      }

      if (res.getKind() === "device") {
        options[3].options.push(
          renderItem(
            res.getDeviceName(),
            `/tenants/${res.getTenantId()}/applications/${res.getApplicationId()}/devices/${res.getDeviceDevEui()}`,
          ),
        );
      }
    }
  }

  return (
    <div>
      <img className="logo" alt="ChirpStack" src="/logo.png" />
      <div className="actions">
        <div className="search">
          <AutoComplete
            dropdownClassName="search-dropdown"
            dropdownMatchSelectWidth={500}
            options={options}
            onSearch={onSearch}
          >
            <Input.Search placeholder="Search..." style={{ width: 500, marginTop: -5 }} />
          </AutoComplete>
        </div>
        <div className="help">
          <a href="https://www.chirpstack.io" target="_blank" rel="noreferrer">
            <Button icon={<QuestionOutlined />} />
          </a>
        </div>
        <div className="user">
          <Dropdown overlay={menu} placement="bottomRight" trigger={["click"]}>
            <Button type="primary" icon={<UserOutlined />}>
              {user.getEmail()} <DownOutlined />
            </Button>
          </Dropdown>
        </div>
      </div>
    </div>
  );
}

export default Header;
