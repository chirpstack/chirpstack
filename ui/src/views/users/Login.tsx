import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";
import { Row, Col, Card } from "antd";
import { Form, Input, Button } from "antd";

import SessionStore from "../../stores/SessionStore";
import InternalStore from "../../stores/InternalStore";
import { SettingsResponse, OpenIdConnectLoginRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

const layout = {
  labelCol: {
    span: 8,
  },
  wrapperCol: {
    span: 16,
  },
};

const tailLayout = {
  wrapperCol: {
    offset: 8,
    span: 16,
  },
};

interface LoginFormState {}

interface LoginFormValues {
  email: string;
  password: string;
}

class LoginForm extends Component<RouteComponentProps, LoginFormState> {
  onFinish = (values: LoginFormValues) => {
    SessionStore.login(values.email, values.password, () => {
      this.props.history.push("/");
    });
  };

  render() {
    return (
      <Row style={{ marginTop: "200px" }}>
        <Col span={8} offset={8}>
          <Card title="ChirpStack login">
            <Form {...layout} onFinish={this.onFinish}>
              <Form.Item
                label="Username / email"
                name="email"
                rules={[
                  {
                    required: true,
                    message: "Please enter your email / username!",
                  },
                ]}
              >
                <Input />
              </Form.Item>

              <Form.Item
                label="Password"
                name="password"
                rules={[
                  {
                    required: true,
                    message: "Please enter your password!",
                  },
                ]}
              >
                <Input.Password />
              </Form.Item>

              <Form.Item {...tailLayout}>
                <Button type="primary" htmlType="submit">
                  Submit
                </Button>
              </Form.Item>
            </Form>
          </Card>
        </Col>
      </Row>
    );
  }
}

interface OidcLoginProps {
  loginUrl: string;
  loginLabel: string;
}

class OidcLogin extends Component<OidcLoginProps> {
  render() {
    return (
      <Row style={{ marginTop: "200px" }}>
        <Col span={8} offset={8}>
          <Card title="ChirpStack login">
            <a href={this.props.loginUrl}>
              <Button type="primary">{this.props.loginLabel}</Button>
            </a>
          </Card>
        </Col>
      </Row>
    );
  }
}

interface LoginState {
  loaded: boolean;
  oidcEnabled: boolean;
  oidcLoginLabel: string;
  oidcLoginUrl: string;
}

class Login extends Component<RouteComponentProps, LoginState> {
  constructor(props: RouteComponentProps) {
    super(props);
    this.state = {
      loaded: false,
      oidcEnabled: false,
      oidcLoginLabel: "",
      oidcLoginUrl: "",
    };
  }

  componentDidMount() {
    SessionStore.logout(true, () => {});

    if (this.props.location.search === "") {
      InternalStore.settings((resp: SettingsResponse) => {
        this.setState({
          loaded: true,
          oidcEnabled: resp.getOpenidConnect()!.getEnabled(),
          oidcLoginLabel: resp.getOpenidConnect()!.getLoginLabel(),
          oidcLoginUrl: resp.getOpenidConnect()!.getLoginUrl(),
        });
      });
    } else {
      // Callback from OIDC provider.
      let q = new URLSearchParams(this.props.location.search);
      let req = new OpenIdConnectLoginRequest();
      req.setCode(q.get("code") || "");
      req.setState(q.get("state") || "");

      SessionStore.openIdConnectLogin(req, () => {
        this.props.history.push("/");
      });
    }
  }

  render() {
    if (!this.state.loaded) {
      return null;
    }

    if (this.state.oidcEnabled) {
      return <OidcLogin loginUrl={this.state.oidcLoginUrl} loginLabel={this.state.oidcLoginLabel} />;
    } else {
      return <LoginForm {...this.props} />;
    }
  }
}

export default Login;
