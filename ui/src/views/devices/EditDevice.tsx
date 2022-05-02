import React, { Component } from "react";
import { RouteComponentProps } from "react-router-dom";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { Device, UpdateDeviceRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import DeviceStore from "../../stores/DeviceStore";
import DeviceForm from "./DeviceForm";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
  application: Application;
  device: Device;
}

class EditDevice extends Component<IProps> {
  onFinish = (obj: Device) => {
    let req = new UpdateDeviceRequest();
    req.setDevice(obj);

    DeviceStore.update(req, () => {
      this.props.history.push(
        `/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}/devices/${obj.getDevEui()}`,
      );
    });
  };

  render() {
    return <DeviceForm initialValues={this.props.device} onFinish={this.onFinish} tenant={this.props.tenant} update />;
  }
}

export default EditDevice;
