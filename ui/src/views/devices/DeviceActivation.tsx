import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

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

import { onFinishFailed } from "../helpers";

interface FormProps {
  disabled: boolean;
  initialValues: DeviceActivationPb;
  device: Device;
  onFinish: (obj: DeviceActivationPb) => void;
}

function LW10DeviceActivationForm(props: FormProps) {
  const [form] = Form.useForm();

  const onFinish = (values: DeviceActivationPb.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let da = new DeviceActivationPb();

    da.setDevAddr(v.devAddr);
    da.setAppSKey(v.appSKey);
    da.setNwkSEncKey(v.nwkSEncKey);
    da.setSNwkSIntKey(v.nwkSEncKey);
    da.setFNwkSIntKey(v.nwkSEncKey);
    da.setFCntUp(v.fCntUp);
    da.setAFCntDown(v.nFCntDown);
    da.setNFCntDown(v.nFCntDown);

    props.onFinish(da);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed} form={form}>
      <DevAddrInput
        label="Device address"
        name="devAddr"
        value={props.initialValues.getDevAddr()}
        devEui={props.device.getDevEui()}
        required
      />
      <AesKeyInput
        label="Network session key (LoRaWAN 1.0)"
        name="nwkSEncKey"
        value={props.initialValues.getNwkSEncKey()}
        required
      />
      <AesKeyInput
        label="Application session key (LoRaWAN 1.0)"
        name="appSKey"
        value={props.initialValues.getAppSKey()}
        required
      />
      <Row gutter={24}>
        <Col span={6}>
          <Form.Item label="Uplink frame-counter" name="fCntUp">
            <InputNumber min={0} />
          </Form.Item>
        </Col>
        <Col span={6}>
          <Form.Item label="Downlink frame-counter" name="nFCntDown">
            <InputNumber min={0} />
          </Form.Item>
        </Col>
      </Row>
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          (Re)activate device
        </Button>
      </Form.Item>
    </Form>
  );
}

function LW11DeviceActivationForm(props: FormProps) {
  const [form] = Form.useForm();

  const onFinish = (values: DeviceActivationPb.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let da = new DeviceActivationPb();

    da.setDevAddr(v.devAddr);
    da.setAppSKey(v.appSKey);
    da.setNwkSEncKey(v.nwkSEncKey);
    da.setSNwkSIntKey(v.sNwkSIntKey);
    da.setFNwkSIntKey(v.fNwkSIntKey);
    da.setFCntUp(v.fCntUp);
    da.setAFCntDown(v.aFCntDown);
    da.setNFCntDown(v.nFCntDown);

    props.onFinish(da);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed} form={form}>
      <DevAddrInput
        label="Device address"
        name="devAddr"
        value={props.initialValues.getDevAddr()}
        devEui={props.device.getDevEui()}
        required
      />
      <AesKeyInput
        label="Network session encryption key"
        name="nwkSEncKey"
        value={props.initialValues.getNwkSEncKey()}
        required
      />
      <AesKeyInput
        label="Serving network session integrity key"
        name="sNwkSIntKey"
        value={props.initialValues.getSNwkSIntKey()}
        required
      />
      <AesKeyInput
        label="Forwarding network session integrity key"
        name="fNwkSIntKey"
        value={props.initialValues.getFNwkSIntKey()}
        required
      />
      <AesKeyInput label="Application session key" name="appSKey" value={props.initialValues.getAppSKey()} required />
      <Row gutter={24}>
        <Col span={6}>
          <Form.Item label="Uplink frame-counter" name="fCntUp">
            <InputNumber min={0} />
          </Form.Item>
        </Col>
        <Col span={6}>
          <Form.Item label="Downlink frame-counter (network)" name="nFCntDown">
            <InputNumber min={0} />
          </Form.Item>
        </Col>
        <Col span={6}>
          <Form.Item label="Downlink frame-counter (application)" name="aFCntDown">
            <InputNumber min={0} />
          </Form.Item>
        </Col>
      </Row>
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          (Re)activate device
        </Button>
      </Form.Item>
    </Form>
  );
}

interface IProps {
  tenant: Tenant;
  application: Application;
  device: Device;
  deviceProfile: DeviceProfile;
}

function DeviceActivation(props: IProps) {
  const navigate = useNavigate();
  const [deviceActivation, setDeviceActivation] = useState<DeviceActivationPb | undefined>(undefined);
  const [deviceActivationRequested, setDeviceActivationRequested] = useState<boolean>(false);

  useEffect(() => {
    let req = new GetDeviceActivationRequest();
    req.setDevEui(props.device.getDevEui());

    DeviceStore.getActivation(req, (resp: GetDeviceActivationResponse) => {
      setDeviceActivation(resp.getDeviceActivation());
      setDeviceActivationRequested(true);
    });
  }, [props]);

  const onFinish = (obj: DeviceActivationPb) => {
    let req = new ActivateDeviceRequest();
    obj.setDevEui(props.device.getDevEui());
    req.setDeviceActivation(obj);

    DeviceStore.activate(req, () => {
      navigate(
        `/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/devices/${props.device.getDevEui()}`,
      );
    });
  };

  if (!deviceActivationRequested) {
    return null;
  }

  if (!deviceActivation && props.deviceProfile.getSupportsOtaa()) {
    return <Alert type="info" showIcon message="This device has not (yet) been activated." />;
  }

  let macVersion = props.deviceProfile.getMacVersion();
  const lw11 = macVersion === MacVersion.LORAWAN_1_1_0;

  let initialValues = new DeviceActivationPb();
  if (deviceActivation) {
    initialValues = deviceActivation;
  }

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      {!lw11 && (
        <LW10DeviceActivationForm
          initialValues={initialValues}
          device={props.device}
          onFinish={onFinish}
          disabled={props.deviceProfile.getSupportsOtaa()}
        />
      )}
      {lw11 && (
        <LW11DeviceActivationForm
          initialValues={initialValues}
          device={props.device}
          onFinish={onFinish}
          disabled={props.deviceProfile.getSupportsOtaa()}
        />
      )}
    </Space>
  );
}

export default DeviceActivation;
