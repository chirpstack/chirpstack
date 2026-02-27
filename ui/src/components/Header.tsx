import { useState, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";

import { Button, Dropdown, Input, AutoComplete } from "antd";
import { UserOutlined, DownOutlined, QuestionOutlined } from "@ant-design/icons";

import type { User } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";
import type { SettingsResponse, GlobalSearchResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { GlobalSearchRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import InternalStore from "../stores/InternalStore";
import SessionStore from "../stores/SessionStore";
import { MenuProps } from "antd/lib";

const renderTitle = (title: string) => <span>{title}</span>;

const renderItem = (title: string, url: string) => ({
  value: title,
  url,
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

    const req = new GlobalSearchRequest();
    req.setLimit(20);
    req.setSearch(search);

    InternalStore.globalSearch(req, (resp: GlobalSearchResponse) => {
      setSearchResult(resp);
    });
  };

  // this type assertion is needed because of a bug in antd's AutoComplete typings
  const onSelect = (_: unknown, _option: (typeof options)[number]) => {
    const option = _option as unknown as ReturnType<typeof renderItem>;

    navigate(option.url);
  };

  const onLogout = () => {
    if (settings === undefined) {
      return;
    }

    const oidc = settings.getOpenidConnect()!;
    const oAuth2 = settings.getOauth2()!;

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

  const oidcEnabled = settings!.getOpenidConnect()!.getEnabled();
  const oAuth2Enabled = settings!.getOauth2()!.getEnabled();

  let menu: MenuProps = { items: [] };

  if (!(oidcEnabled || oAuth2Enabled)) {
    menu.items!.push({
      key: "change-pw",
      label: <Link to={`/users/${user.getId()}/password`}>Change password</Link>,
    });
  }

  menu.items!.push({
    key: "logout",
    label: "Logout",
    onClick: onLogout,
  });

  const options: {
    label: JSX.Element;
    options: ReturnType<typeof renderItem>[];
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
            classNames={{ popup: { root: "search-dropdown" } }}
            popupMatchSelectWidth={500}
            options={options}
            onSearch={onSearch}
            onSelect={onSelect}
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
          <Dropdown menu={menu} placement="bottomRight" trigger={["click"]}>
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
