import React, { useState } from "react";

import moment from "moment";
import { Card, Button, Form, Input } from "antd";

import {
  Gateway,
  GenerateGatewayClientCertificateRequest,
  GenerateGatewayClientCertificateResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import GatewayStore from "../../stores/GatewayStore";

interface IProps {
  gateway: Gateway;
}

function GatewayCertificate(props: IProps) {
  const [certificate, setCertificate] = useState<GenerateGatewayClientCertificateResponse | undefined>(undefined);
  const [buttonDisabled, setButtonDisabled] = useState<boolean>(false);

  const requestCertificate = () => {
    setButtonDisabled(true);

    let req = new GenerateGatewayClientCertificateRequest();
    req.setGatewayId(props.gateway.getGatewayId());

    GatewayStore.generateClientCertificate(req, (resp: GenerateGatewayClientCertificateResponse) => {
      setCertificate(resp);
    });
  };

  const renderRequest = () => {
    return (
      <Card>
        <p>
          The gateway client-certificate can be used to connect the gateway in case a client-certificate is required for
          authentication. Please note that this feature might not be available in case it has not been configured in the
          server configuration. After the certificate has been generated, store the content of the files on your
          gateway.
          <strong>
            {" "}
            Please note the expiration of the certificate and make sure to renew the certificate on time!
          </strong>
        </p>
        <p>
          <Button onClick={requestCertificate} disabled={buttonDisabled}>
            Generate certificate
          </Button>
        </p>
      </Card>
    );
  };

  const renderResponse = () => {
    const cert = certificate!;

    const initial = {
      expiresAt: moment(cert.getExpiresAt()!.toDate()!).format("YYYY-MM-DD HH:mm:ss"),
      caCert: cert.getCaCert(),
      tlsCert: cert.getTlsCert(),
      tlsKey: cert.getTlsKey(),
    };

    return (
      <Form layout="vertical" initialValues={initial}>
        <Form.Item
          label="Expiration date"
          name="expiresAt"
          tooltip="The certificate expires at this date. Make sure to generate and configure a new certificate for your gateway before this expiration date."
        >
          <Input disabled style={{ cursor: "text", color: "black" }} />
        </Form.Item>
        <Form.Item
          label="CA certificate"
          name="caCert"
          tooltip="The CA certificate is to authenticate the certificate of the server. Store this as a text-file on your gateway, e.g. named 'ca.crt'."
        >
          <Input.TextArea autoSize disabled style={{ cursor: "text", fontFamily: "monospace", color: "black" }} />
        </Form.Item>
        <Form.Item
          label="TLS certificate"
          name="tlsCert"
          tooltip="Store this as a text-file on your gateway, e.g. named 'cert.crt'"
        >
          <Input.TextArea autoSize disabled style={{ cursor: "text", fontFamily: "monospace", color: "black" }} />
        </Form.Item>
        <Form.Item
          label="TLS key"
          name="tlsKey"
          tooltip="Store this as a text-file on your gateway, e.g. named 'cert.key'"
        >
          <Input.TextArea autoSize disabled style={{ cursor: "text", fontFamily: "monospace", color: "black" }} />
        </Form.Item>
      </Form>
    );
  };

  if (certificate !== undefined) {
    return renderResponse();
  }

  return renderRequest();
}

export default GatewayCertificate;
