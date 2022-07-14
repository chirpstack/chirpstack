import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Space, Form, Button, Row, Col, InputNumber, Alert } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  Device,
  GetDeviceActivationRequest,
  GetDeviceActivationResponse,
  DeviceActivation as DeviceActivationPb,
  ActivateDeviceRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { MacVersion } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { DeviceProfile } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import AesKeyInput from "../../components/AesKeyInput";
import DevAddrInput from "../../components/DevAddrInput";
import DeviceStore from "../../stores/DeviceStore";

interface FormProps {
  disabled: boolean;
  initialValues: DeviceActivationPb;
  device: Device;
  onFinish: (obj: DeviceActivationPb) => void;
}

class LW10DeviceActivationForm extends Component<FormProps> {
  formRef = React.createRef<any>();

  onFinish = (values: DeviceActivationPb.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let da = new DeviceActivationPb();

    da.setDevAddr(v.devAddr);
    da.setAppSKey(v.appSKey);
    da.setNwkSEncKey(v.nwkSEncKey);
    da.setSNwkSIntKey(v.nwkSEncKey);
    da.setFNwkSIntKey(v.nwkSEncKey);
    da.setFCntUp(v.fCntUp);
    da.setAFCntDown(v.nFCntDown);
    da.setNFCntDown(v.nFCntDown);

    this.props.onFinish(da);
  };

  render() {
    return (
      <Form
        layout="vertical"
        initialValues={this.props.initialValues.toObject()}
        onFinish={this.onFinish}
        ref={this.formRef}
      >
        <DevAddrInput
          label="Device address"
          name="devAddr"
          value={this.props.initialValues.getDevAddr()}
          devEui={this.props.device.getDevEui()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <AesKeyInput
          label="Network session key (LoRaWAN 1.0)"
          name="nwkSEncKey"
          value={this.props.initialValues.getNwkSEncKey()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <AesKeyInput
          label="Application session key (LoRaWAN 1.0)"
          name="appSKey"
          value={this.props.initialValues.getAppSKey()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <Row gutter={24}>
          <Col span={6}>
            <Form.Item label="Uplink frame-counter" name="fCntUp">
              <InputNumber min={0} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="Downlink frame-counter" name="nFCntDown">
              <InputNumber min={0} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
        </Row>
        <Form.Item>
          <Button type="primary" htmlType="submit" disabled={this.props.disabled}>
            (Re)activate device
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

class LW11DeviceActivationForm extends Component<FormProps> {
  formRef = React.createRef<any>();

  onFinish = (values: DeviceActivationPb.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let da = new DeviceActivationPb();

    da.setDevAddr(v.devAddr);
    da.setAppSKey(v.appSKey);
    da.setNwkSEncKey(v.nwkSEncKey);
    da.setSNwkSIntKey(v.sNwkSIntKey);
    da.setFNwkSIntKey(v.fNwkSIntKey);
    da.setFCntUp(v.fCntUp);
    da.setAFCntDown(v.aFCntDown);
    da.setNFCntDown(v.nFCntDown);

    this.props.onFinish(da);
  };

  render() {
    return (
      <Form
        layout="vertical"
        initialValues={this.props.initialValues.toObject()}
        onFinish={this.onFinish}
        ref={this.formRef}
      >
        <DevAddrInput
          label="Device address"
          name="devAddr"
          value={this.props.initialValues.getDevAddr()}
          devEui={this.props.device.getDevEui()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <AesKeyInput
          label="Network session encryption key"
          name="nwkSEncKey"
          value={this.props.initialValues.getNwkSEncKey()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <AesKeyInput
          label="Serving network session integrity key"
          name="sNwkSIntKey"
          value={this.props.initialValues.getSNwkSIntKey()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <AesKeyInput
          label="Forwarding network session integrity key"
          name="fNwkSIntKey"
          value={this.props.initialValues.getFNwkSIntKey()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <AesKeyInput
          label="Application session key"
          name="appSKey"
          value={this.props.initialValues.getAppSKey()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <Row gutter={24}>
          <Col span={6}>
            <Form.Item label="Uplink frame-counter" name="fCntUp">
              <InputNumber min={0} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="Downlink frame-counter (network)" name="nFCntDown">
              <InputNumber min={0} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="Downlink frame-counter (application)" name="aFCntDown">
              <InputNumber min={0} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
        </Row>
        <Form.Item>
          <Button type="primary" htmlType="submit" disabled={this.props.disabled}>
            (Re)activate device
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

interface IProps extends RouteComponentProps {
  tenant: Tenant;
  application: Application;
  device: Device;
  deviceProfile: DeviceProfile;
}

interface IState {
  deviceActivation?: DeviceActivationPb;
  deviceActivationRequested: boolean;
}

class DeviceActivation extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      deviceActivationRequested: false,
    };
  }

  componentDidMount() {
    let req = new GetDeviceActivationRequest();
    req.setDevEui(this.props.device.getDevEui());

    DeviceStore.getActivation(req, (resp: GetDeviceActivationResponse) => {
      this.setState({
        deviceActivation: resp.getDeviceActivation(),
        deviceActivationRequested: true,
      });
    });
  }

  onFinish = (obj: DeviceActivationPb) => {
    let req = new ActivateDeviceRequest();
    obj.setDevEui(this.props.device.getDevEui());
    req.setDeviceActivation(obj);

    DeviceStore.activate(req, () => {
      this.props.history.push(
        `/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}/devices/${this.props.device.getDevEui()}`,
      );
    });
  };

  render() {
    if (!this.state.deviceActivationRequested) {
      return null;
    }

    if (!this.state.deviceActivation && this.props.deviceProfile.getSupportsOtaa()) {
      return <Alert type="info" showIcon message="This device has not (yet) been activated." />;
    }

    let macVersion = this.props.deviceProfile.getMacVersion();
    const lw11 = macVersion === MacVersion.LORAWAN_1_1_0;

    let initialValues = new DeviceActivationPb();
    if (this.state.deviceActivation) {
      initialValues = this.state.deviceActivation;
    }

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        {!lw11 && (
          <LW10DeviceActivationForm
            initialValues={initialValues}
            device={this.props.device}
            onFinish={this.onFinish}
            disabled={this.props.deviceProfile.getSupportsOtaa()}
          />
        )}
        {lw11 && (
          <LW11DeviceActivationForm
            initialValues={initialValues}
            device={this.props.device}
            onFinish={this.onFinish}
            disabled={this.props.deviceProfile.getSupportsOtaa()}
          />
        )}
      </Space>
    );
  }
}

export default DeviceActivation;
