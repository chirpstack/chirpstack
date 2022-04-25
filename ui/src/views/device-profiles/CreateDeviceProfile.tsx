import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import {
  DeviceProfile,
  CreateDeviceProfileRequest,
  CreateDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import DeviceProfileForm from "./DeviceProfileForm";
import DeviceProfileStore from "../../stores/DeviceProfileStore";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
}

class CreateDeviceProfile extends Component<IProps> {
  onFinish = (obj: DeviceProfile) => {
    obj.setTenantId(this.props.tenant.getId());

    let req = new CreateDeviceProfileRequest();
    req.setDeviceProfile(obj);

    DeviceProfileStore.create(req, (_resp: CreateDeviceProfileResponse) => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/device-profiles`);
    });
  };

  render() {
    const encodeScript = `// Encode encodes the given object into an array of bytes.
//
// Input is an object with the following fields:
// - f_port = LoRaWAN fPort
// - variables = Device variables
// - object = Input object, e.g. {"temperature": 22.5}
//
// This function must return an array of bytes, e.g. [225, 230, 255, 0]
export function Encode(input) {
  return [];
}`;

    const decodeScript = `// Decode decodes an array of bytes into an object.
//
// Input is an object with the following fields:
// - f_port = LoRaWAN fPort
// - variables = Device variables
// - data = Input byte array, e.g. [225, 230, 255, 0]
//
// This function must return an object, e.g. {"temperature": 22.5}
export function Decode(input) {
  return {};
}`;

    let deviceProfile = new DeviceProfile();
    deviceProfile.setPayloadEncoderConfig(encodeScript);
    deviceProfile.setPayloadDecoderConfig(decodeScript);

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <PageHeader
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Tenants</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}`}>{this.props.tenant.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/device-profiles`}>Device profiles</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add device profile"
        />
        <Card>
          <DeviceProfileForm initialValues={deviceProfile} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateDeviceProfile;
