import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

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

import AesKeyInput from "../../components/AesKeyInput";
import DeviceStore from "../../stores/DeviceStore";

interface FormProps {
  initialValues: DeviceKeys;
  onFinish: (obj: DeviceKeys) => void;
}

class LW10DeviceKeysForm extends Component<FormProps> {
  formRef = React.createRef<any>();

  onFinish = (values: DeviceKeys.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let dk = new DeviceKeys();

    dk.setDevEui(v.devEui);
    // NOTE: this is not an error! In the LoRaWAN 1.1 specs, the what was previously
    // the AppKey has been renamed to the NwkKey and a new value AppKey was added.
    dk.setNwkKey(v.nwkKey);

    this.props.onFinish(dk);
  };

  render() {
    return (
      <Form
        layout="vertical"
        initialValues={this.props.initialValues.toObject()}
        onFinish={this.onFinish}
        ref={this.formRef}
      >
        <AesKeyInput
          label="Application key"
          name="nwkKey"
          tooltip="For LoRaWAN 1.0 devices. In case your device supports LoRaWAN 1.1, update the device-profile first."
          value={this.props.initialValues.getNwkKey()}
          formRef={this.formRef}
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
}

class LW11DeviceKeysForm extends Component<FormProps> {
  formRef = React.createRef<any>();

  onFinish = (values: DeviceKeys.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let dk = new DeviceKeys();

    dk.setDevEui(v.devEui);
    dk.setAppKey(v.appKey);
    dk.setNwkKey(v.nwkKey);

    this.props.onFinish(dk);
  };

  render() {
    return (
      <Form
        layout="vertical"
        initialValues={this.props.initialValues.toObject()}
        onFinish={this.onFinish}
        ref={this.formRef}
      >
        <AesKeyInput
          label="Application key"
          tooltip="For LoRaWAN 1.1 devices. In case your device does not support LoRaWAN 1.1, update the device-profile first."
          name="appKey"
          value={this.props.initialValues.getAppKey()}
          formRef={this.formRef}
          required
        />
        <AesKeyInput
          label="Network key"
          tooltip="For LoRaWAN 1.1 devices. In case your device does not support LoRaWAN 1.1, update the device-profile first."
          name="nwkKey"
          value={this.props.initialValues.getNwkKey()}
          formRef={this.formRef}
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
}

interface IProps extends RouteComponentProps {
  tenant: Tenant;
  application: Application;
  device: Device;
  deviceProfile: DeviceProfile;
}

interface IState {
  deviceKeys?: DeviceKeys;
  deviceKeysRequested: boolean;
}

class SetDeviceKeys extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      deviceKeysRequested: false,
    };
  }

  componentDidMount() {
    this.getDeviceKeys();
  }

  getDeviceKeys = () => {
    let req = new GetDeviceKeysRequest();
    req.setDevEui(this.props.device.getDevEui());

    DeviceStore.getKeys(req, (resp?: GetDeviceKeysResponse) => {
      if (resp) {
        this.setState({
          deviceKeys: resp.getDeviceKeys(),
          deviceKeysRequested: true,
        });
      } else {
        this.setState({
          deviceKeysRequested: true,
        });
      }
    });
  };

  onFinish = (obj: DeviceKeys) => {
    if (this.state.deviceKeys) {
      // this is an update
      let req = new UpdateDeviceKeysRequest();
      req.setDeviceKeys(obj);

      DeviceStore.updateKeys(req, () => {
        this.props.history.push(
          `/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}/devices/${this.props.device.getDevEui()}`,
        );
      });
    } else {
      // this is a create
      let req = new CreateDeviceKeysRequest();
      obj.setDevEui(this.props.device.getDevEui());
      req.setDeviceKeys(obj);

      DeviceStore.createKeys(req, () => {
        this.props.history.push(
          `/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}/devices/${this.props.device.getDevEui()}`,
        );
      });
    }
  };

  flushDevNonces = () => {
    let req = new FlushDevNoncesRequest();
    req.setDevEui(this.props.device.getDevEui());
    DeviceStore.flushDevNonces(req, () => {});
  };

  render() {
    if (!this.state.deviceKeysRequested) {
      return null;
    }

    const macVersion = this.props.deviceProfile.getMacVersion();
    const lw11 = macVersion === MacVersion.LORAWAN_1_1_0;

    let initialValues = new DeviceKeys();
    if (this.state.deviceKeys) {
      initialValues = this.state.deviceKeys;
    }

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        {this.state.deviceKeys && (
          <div style={{ float: "right" }}>
            <Popconfirm
              placement="left"
              title="Are you sure you want to flush all device-nonces that have been used during previous OTAA activations?"
              onConfirm={this.flushDevNonces}
            >
              <Button>Flush OTAA device nonces</Button>
            </Popconfirm>
          </div>
        )}
        {!lw11 && <LW10DeviceKeysForm initialValues={initialValues} onFinish={this.onFinish} />}
        {lw11 && <LW11DeviceKeysForm initialValues={initialValues} onFinish={this.onFinish} />}
      </Space>
    );
  }
}

export default SetDeviceKeys;
