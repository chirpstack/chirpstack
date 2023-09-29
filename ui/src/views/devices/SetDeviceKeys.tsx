import React, { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { Form, Button, Space, Popconfirm } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { DeviceProfile } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  DeviceKeys,
  GetDeviceKeysRequest,
  GetDeviceKeysResponse,
  CreateDeviceKeysRequest,
  UpdateDeviceKeysRequest,
  FlushDevNoncesRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { MacVersion } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";

import { onFinishFailed } from "../helpers";
import AesKeyInput from "../../components/AesKeyInput";
import DeviceStore from "../../stores/DeviceStore";

interface FormProps {
  initialValues: DeviceKeys;
  onFinish: (obj: DeviceKeys) => void;
}

function LW10DeviceKeysForm(props: FormProps) {
  const [form] = Form.useForm();

  const onFinish = (values: DeviceKeys.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let dk = new DeviceKeys();

    dk.setDevEui(v.devEui);
    // NOTE: this is not an error! In the LoRaWAN 1.1 specs, the what was previously
    // the AppKey has been renamed to the NwkKey and a new value AppKey was added.
    dk.setNwkKey(v.nwkKey);

    props.onFinish(dk);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed} form={form}>
      <AesKeyInput
        label="Application key"
        name="nwkKey"
        tooltip="For LoRaWAN 1.0 devices. In case your device supports LoRaWAN 1.1, update the device-profile first."
        value={props.initialValues.getNwkKey()}
        required
      />
      <Form.Item>
        <Button type="primary" htmlType="submit">
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

function LW11DeviceKeysForm(props: FormProps) {
  const [form] = Form.useForm();

  const onFinish = (values: DeviceKeys.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let dk = new DeviceKeys();

    dk.setDevEui(v.devEui);
    dk.setAppKey(v.appKey);
    dk.setNwkKey(v.nwkKey);

    props.onFinish(dk);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed} form={form}>
      <AesKeyInput
        label="Application key"
        tooltip="For LoRaWAN 1.1 devices. In case your device does not support LoRaWAN 1.1, update the device-profile first."
        name="appKey"
        value={props.initialValues.getAppKey()}
        required
      />
      <AesKeyInput
        label="Network key"
        tooltip="For LoRaWAN 1.1 devices. In case your device does not support LoRaWAN 1.1, update the device-profile first."
        name="nwkKey"
        value={props.initialValues.getNwkKey()}
        required
      />
      <Form.Item>
        <Button type="primary" htmlType="submit">
          Submit
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

function SetDeviceKeys(props: IProps) {
  const navigate = useNavigate();
  const [deviceKeys, setDeviceKeys] = useState<DeviceKeys | undefined>(undefined);
  const [deviceKeysRequested, setDeviceKeysRequested] = useState<boolean>(false);

  useEffect(() => {
    let req = new GetDeviceKeysRequest();
    req.setDevEui(props.device.getDevEui());

    DeviceStore.getKeys(req, (resp?: GetDeviceKeysResponse) => {
      if (resp) {
        setDeviceKeys(resp.getDeviceKeys());
        setDeviceKeysRequested(true);
      } else {
        setDeviceKeysRequested(true);
      }
    });
  }, [props]);

  const onFinish = (obj: DeviceKeys) => {
    if (deviceKeys) {
      // this is an update
      let req = new UpdateDeviceKeysRequest();
      req.setDeviceKeys(obj);

      DeviceStore.updateKeys(req, () => {
        navigate(
          `/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/devices/${props.device.getDevEui()}`,
        );
      });
    } else {
      // this is a create
      let req = new CreateDeviceKeysRequest();
      obj.setDevEui(props.device.getDevEui());
      req.setDeviceKeys(obj);

      DeviceStore.createKeys(req, () => {
        navigate(
          `/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/devices/${props.device.getDevEui()}`,
        );
      });
    }
  };

  const flushDevNonces = () => {
    let req = new FlushDevNoncesRequest();
    req.setDevEui(props.device.getDevEui());
    DeviceStore.flushDevNonces(req, () => { });
  };

  if (!deviceKeysRequested) {
    return null;
  }

  const macVersion = props.deviceProfile.getMacVersion();
  const lw11 = macVersion === MacVersion.LORAWAN_1_1_0;

  let initialValues = new DeviceKeys();
  if (deviceKeys) {
    initialValues = deviceKeys;
  }

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      {deviceKeys && (
        <div style={{ float: "right" }}>
          <Popconfirm
            placement="left"
            title="Are you sure you want to flush all device-nonces that have been used during previous OTAA activations?"
            onConfirm={flushDevNonces}
          >
            <Button>Flush OTAA device nonces</Button>
          </Popconfirm>
        </div>
      )}
      {!lw11 && <LW10DeviceKeysForm initialValues={initialValues} onFinish={onFinish} />}
      {lw11 && <LW11DeviceKeysForm initialValues={initialValues} onFinish={onFinish} />}
    </Space>
  );
}

export default SetDeviceKeys;
