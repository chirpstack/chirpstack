import React, { Component } from "react";
import { Link, withRouter, RouteComponentProps } from "react-router-dom";

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

interface IProps extends RouteComponentProps {
  user: User;
}

interface IState {
  searchResult?: GlobalSearchResponse;
  settings?: SettingsResponse;
}

const renderTitle = (title: string) => <span>{title}</span>;

const renderItem = (title: string, url: string) => ({
  value: title,
  label: <Link to={url}>{title}</Link>,
});

class Header extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {};
  }

  componentDidMount() {
    InternalStore.settings((resp: SettingsResponse) => {
      this.setState({
        settings: resp,
      });
    });
  }

  onSearch = (search: string) => {
    if (search.length < 3) {
      return;
    }

    let req = new GlobalSearchRequest();
    req.setLimit(20);
    req.setSearch(search);

    InternalStore.globalSearch(req, (resp: GlobalSearchResponse) => {
      this.setState({
        searchResult: resp,
      });
    });
  };

  onLogout = () => {
    let settings = this.state.settings;
    if (settings === undefined) {
      return;
    }

    let oidc = settings.getOpenidConnect()!;

    if (!oidc.getEnabled() || oidc.getLogoutUrl() === "") {
      SessionStore.logout(true, () => {
        this.props.history.push("/login");
      });
    } else {
      SessionStore.logout(false, () => {
        window.location.assign(oidc.getLogoutUrl());
      });
    }
  };

  render() {
    if (this.state.settings === undefined) {
      return null;
    }

    let oidcEnabled = this.state.settings!.getOpenidConnect()!.getEnabled();

    const menu = (
      <Menu>
        {!oidcEnabled && (
          <Menu.Item>
            <Link to={`/users/${this.props.user.getId()}/password`}>Change password</Link>
          </Menu.Item>
        )}
        <Menu.Item onClick={this.onLogout}>Logout</Menu.Item>
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

    if (this.state.searchResult !== undefined) {
      for (const res of this.state.searchResult.getResultList()) {
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
            renderItem(
              res.getApplicationName(),
              `/tenants/${res.getTenantId()}/applications/${res.getApplicationId()}`,
            ),
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
              onSearch={this.onSearch}
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
                {this.props.user.getEmail()} <DownOutlined />
              </Button>
            </Dropdown>
          </div>
        </div>
      </div>
    );
  }
}

export default withRouter(Header);
