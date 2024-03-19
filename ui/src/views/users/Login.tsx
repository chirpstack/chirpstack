import React, { useState, useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import { Row, Col, Card } from "antd";
import { Form, Input, Button } from "antd";

import {
  SettingsResponse,
  OpenIdConnectLoginRequest,
  OAuth2LoginRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import SessionStore from "../../stores/SessionStore";
import InternalStore from "../../stores/InternalStore";

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

interface LoginFormValues {
  email: string;
  password: string;
}

interface OidcLoginProps {
  loginUrl: string;
  loginLabel: string;
}

interface OAuth2LoginProps {
  loginUrl: string;
  loginLabel: string;
}

function OidcLogin({ loginUrl, loginLabel }: OidcLoginProps) {
  return (
    <Row style={{ marginTop: "200px" }}>
      <Col span={8} offset={8}>
        <Card title="ChirpStack login">
          <a href={loginUrl}>
            <Button type="primary">{loginLabel}</Button>
          </a>
        </Card>
      </Col>
    </Row>
  );
}

function OAuth2Login({ loginUrl, loginLabel }: OAuth2LoginProps) {
  return (
    <Row style={{ marginTop: "200px" }}>
      <Col span={8} offset={8}>
        <Card title="ChirpStack login">
          <a href={loginUrl}>
            <Button type="primary">{loginLabel}</Button>
          </a>
        </Card>
      </Col>
    </Row>
  );
}

function LoginForm() {
  const navigate = useNavigate();

  const onFinish = (values: LoginFormValues) => {
    SessionStore.login(values.email, values.password, () => {
      navigate("/");
    });
  };

  return (
    <Row style={{ marginTop: "200px" }}>
      <Col span={8} offset={8}>
        <Card title="ChirpStack login">
          <Form {...layout} onFinish={onFinish}>
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

function Login() {
  const location = useLocation();
  const navigate = useNavigate();

  const [loaded, setLoaded] = useState<boolean>(false);
  const [oidcEnabled, setOidcEnabled] = useState<boolean>(false);
  const [oAuth2Enabled, setOAuth2Enabled] = useState<boolean>(false);
  const [oidcLoginLabel, setOidcLoginLabel] = useState<string>("");
  const [oidcLoginUrl, setOidcLoginUrl] = useState<string>("");
  const [oAuth2LoginLabel, setOAuth2LoginLabel] = useState<string>("");
  const [oAuth2LoginUrl, setOAuth2LoginUrl] = useState<string>("");

  useEffect(() => {
    SessionStore.logout(true, () => {});

    InternalStore.settings((resp: SettingsResponse) => {
      const oidc = resp.getOpenidConnect()!;
      const oAuth2 = resp.getOauth2()!;

      setOidcEnabled(oidc.getEnabled());
      setOidcLoginLabel(oidc.getLoginLabel());
      setOidcLoginUrl(oidc.getLoginUrl());

      setOAuth2Enabled(oAuth2.getEnabled());
      setOAuth2LoginLabel(oAuth2.getLoginLabel());
      setOAuth2LoginUrl(oAuth2.getLoginUrl());

      if (location.search !== "") {
        // Callback from OIDC or OAuth2 provider.
        let q = new URLSearchParams(location.search);

        if (oidc.getEnabled()) {
          let req = new OpenIdConnectLoginRequest();
          req.setCode(q.get("code") || "");
          req.setState(q.get("state") || "");

          SessionStore.openIdConnectLogin(req, () => {
            navigate("/");
          });
        } else if (oAuth2.getEnabled()) {
          let req = new OAuth2LoginRequest();
          req.setCode(q.get("code") || "");
          req.setState(q.get("state") || "");

          SessionStore.oAuth2Login(req, () => {
            navigate("/");
          });
        }
      } else {
        if (oidc.getEnabled() && oidc.getLoginRedirect()) {
          window.location.replace(oidc.getLoginUrl());
        } else if (oAuth2.getEnabled() && oAuth2.getLoginRedirect()) {
          window.location.replace(oAuth2.getLoginUrl());
        } else {
          setLoaded(true);
        }
      }
    });
  }, [location, navigate]);

  if (!loaded) {
    return null;
  }

  if (oidcEnabled) {
    return <OidcLogin loginUrl={oidcLoginUrl} loginLabel={oidcLoginLabel} />;
  } else if (oAuth2Enabled) {
    return <OAuth2Login loginUrl={oAuth2LoginUrl} loginLabel={oAuth2LoginLabel} />;
  } else {
    return <LoginForm />;
  }
}

export default Login;
