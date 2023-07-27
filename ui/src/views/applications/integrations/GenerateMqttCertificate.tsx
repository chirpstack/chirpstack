import React, { useState } from "react";

import moment from "moment";
import { Card, Button, Form, Input } from "antd";

import {
  Application,
  GenerateMqttIntegrationClientCertificateRequest,
  GenerateMqttIntegrationClientCertificateResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function GenerateMqttCertificate(props: IProps) {
  const [certificate, setCertificate] = useState<GenerateMqttIntegrationClientCertificateResponse | undefined>(
    undefined,
  );
  const [buttonDisabled, setButtonDisabled] = useState<boolean>(false);

  const requestCertificate = () => {
    setButtonDisabled(true);

    let req = new GenerateMqttIntegrationClientCertificateRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.generateMqttIntegrationClientCertificate(
      req,
      (resp: GenerateMqttIntegrationClientCertificateResponse) => {
        setCertificate(resp);
      },
    );
  };

  const renderRequest = () => {
    return (
      <Card>
        <p>
          If required by the network, the MQTT client needs to be configured with a client certificate in order to
          connect to the MQTT broker to receive device data. The generated certificate is application specific.
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
          tooltip="The CA certificate is to authenticate the certificate of the server. Store this as a text-file, e.g. named 'ca.crt'."
        >
          <Input.TextArea autoSize disabled style={{ cursor: "text", fontFamily: "monospace", color: "black" }} />
        </Form.Item>
        <Form.Item label="TLS certificate" name="tlsCert" tooltip="Store this as a text-file, e.g. named 'cert.crt'">
          <Input.TextArea autoSize disabled style={{ cursor: "text", fontFamily: "monospace", color: "black" }} />
        </Form.Item>
        <Form.Item label="TLS key" name="tlsKey" tooltip="Store this as a text-file, e.g. named 'cert.key'">
          <Input.TextArea autoSize disabled style={{ cursor: "text", fontFamily: "monospace", color: "black" }} />
        </Form.Item>
      </Form>
    );
  };

  let content = renderRequest();

  if (certificate !== undefined) {
    content = renderResponse();
  }

  return <Card title="Generate MQTT certificate">{content}</Card>;
}

export default GenerateMqttCertificate;
