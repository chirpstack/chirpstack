import React, { Component } from "react";

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

interface IState {
  certificate?: GenerateMqttIntegrationClientCertificateResponse;
  buttonDisabled: boolean;
}

class GenerateMqttCertificate extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      certificate: undefined,
      buttonDisabled: false,
    };
  }

  requestCertificate = () => {
    this.setState({
      buttonDisabled: true,
    });

    let req = new GenerateMqttIntegrationClientCertificateRequest();
    req.setApplicationId(this.props.application.getId());

    ApplicationStore.generateMqttIntegrationClientCertificate(
      req,
      (resp: GenerateMqttIntegrationClientCertificateResponse) => {
        this.setState({
          certificate: resp,
        });
      },
    );
  };

  renderRequest = () => {
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
          <Button onClick={this.requestCertificate} disabled={this.state.buttonDisabled}>
            Generate certificate
          </Button>
        </p>
      </Card>
    );
  };

  renderResponse = () => {
    const certificate = this.state.certificate!;

    const initial = {
      expiresAt: moment(certificate.getExpiresAt()!.toDate()!).format("YYYY-MM-DD HH:mm:ss"),
      caCert: certificate.getCaCert(),
      tlsCert: certificate.getTlsCert(),
      tlsKey: certificate.getTlsKey(),
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

  render() {
    let content = this.renderRequest();

    if (this.state.certificate !== undefined) {
      content = this.renderResponse();
    }

    return <Card title="Generate MQTT certificate">{content}</Card>;
  }
}

export default GenerateMqttCertificate;
