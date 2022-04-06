import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { DeviceProfile, CreateDeviceProfileRequest, CreateDeviceProfileResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import DeviceProfileForm from "./DeviceProfileForm";
import DeviceProfileStore from "../../stores/DeviceProfileStore";


interface IProps extends RouteComponentProps {
  tenant: Tenant,
}


class CreateDeviceProfile extends Component<IProps> {
  onFinish = (obj: DeviceProfile) => {
    obj.setTenantId(this.props.tenant.getId());

    let req = new CreateDeviceProfileRequest();
    req.setDeviceProfile(obj);

    DeviceProfileStore.create(req, (_resp: CreateDeviceProfileResponse) => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/device-profiles`);
    });
  }

  render() {
    const deviceProfile = new DeviceProfile(); 

    return(
      <Space direction="vertical" style={{width: "100%"}} size="large">
        <PageHeader
          breadcrumbRender={() => <Breadcrumb>
              <Breadcrumb.Item>
                <span>Tenants</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span><Link to={`/tenants/${this.props.tenant.getId()}`}>{this.props.tenant.getName()}</Link></span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span><Link to={`/tenants/${this.props.tenant.getId()}/device-profiles`}>Device profiles</Link></span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>}
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
