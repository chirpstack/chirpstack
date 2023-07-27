import React, { useState, useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";

import { Row, Col, Card } from "antd";
import { Form, Input, Button } from "antd";

import { SettingsResponse, OpenIdConnectLoginRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

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
  const [oidcLoginLabel, setOidcLoginLabel] = useState<string>("");
  const [oidcLoginUrl, setOidcLoginUrl] = useState<string>("");

  useEffect(() => {
    SessionStore.logout(true, () => {});

    if (location.search === "") {
      InternalStore.settings((resp: SettingsResponse) => {
        setLoaded(true);
        setOidcEnabled(resp.getOpenidConnect()!.getEnabled());
        setOidcLoginLabel(resp.getOpenidConnect()!.getLoginLabel());
        setOidcLoginUrl(resp.getOpenidConnect()!.getLoginUrl());
      });
    } else {
      // Callback from OIDC provider.
      let q = new URLSearchParams(location.search);
      let req = new OpenIdConnectLoginRequest();
      req.setCode(q.get("code") || "");
      req.setState(q.get("state") || "");

      SessionStore.openIdConnectLogin(req, () => {
        navigate("/");
      });
    }
  }, [location, navigate]);

  if (!loaded) {
    return null;
  }

  if (oidcEnabled) {
    return <OidcLogin loginUrl={oidcLoginUrl} loginLabel={oidcLoginLabel} />;
  } else {
    return <LoginForm />;
  }
}

export default Login;
