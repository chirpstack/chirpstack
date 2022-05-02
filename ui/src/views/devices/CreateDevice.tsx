import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { CreateDeviceRequest, Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import {
  GetDeviceProfileRequest,
  GetDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import DeviceForm from "./DeviceForm";
import DeviceStore from "../../stores/DeviceStore";
import DeviceProfileStore from "../../stores/DeviceProfileStore";

interface IProps extends RouteComponentProps {
  tenant: Tenant;
  application: Application;
}

class CreateDevice extends Component<IProps> {
  onFinish = (obj: Device) => {
    obj.setApplicationId(this.props.application.getId());

    let req = new CreateDeviceRequest();
    req.setDevice(obj);

    DeviceStore.create(req, () => {
      let req = new GetDeviceProfileRequest();
      req.setId(obj.getDeviceProfileId());

      DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
        let dp = resp.getDeviceProfile()!;
        if (dp.getSupportsOtaa()) {
          this.props.history.push(
            `/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}/devices/${obj.getDevEui()}/keys`,
          );
        } else {
          this.props.history.push(
            `/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}/devices/${obj.getDevEui()}`,
          );
        }
      });
    });
  };

  render() {
    let device = new Device();
    device.setApplicationId(this.props.application.getId());

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
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications`}>Applications</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}/applications/${this.props.application.getId()}`}>
                    {this.props.application.getName()}
                  </Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Add device</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add device"
        />
        <Card>
          <DeviceForm tenant={this.props.tenant} initialValues={device} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateDevice;
