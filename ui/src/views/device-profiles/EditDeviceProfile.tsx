import React, { Component } from "react";
import { RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, PageHeader } from "antd";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import {
  DeviceProfile,
  GetDeviceProfileRequest,
  GetDeviceProfileResponse,
  UpdateDeviceProfileRequest,
  DeleteDeviceProfileRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import DeviceProfileForm from "./DeviceProfileForm";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import SessionStore from "../../stores/SessionStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import Admin from "../../components/Admin";

interface IState {
  deviceProfile?: DeviceProfile;
}

interface MatchParams {
  deviceProfileId: string;
}

interface IProps extends RouteComponentProps<MatchParams> {
  tenant: Tenant;
}

class EditDeviceProfile extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.getDeviceProfile();
  }

  getDeviceProfile = () => {
    const id = this.props.match.params.deviceProfileId;
    let req = new GetDeviceProfileRequest();
    req.setId(id);

    DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
      this.setState({
        deviceProfile: resp.getDeviceProfile(),
      });
    });
  };

  onFinish = (obj: DeviceProfile) => {
    let req = new UpdateDeviceProfileRequest();
    req.setDeviceProfile(obj);

    DeviceProfileStore.update(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/device-profiles`);
    });
  };

  deleteDeviceProfile = () => {
    let req = new DeleteDeviceProfileRequest();
    req.setId(this.props.match.params.deviceProfileId);

    DeviceProfileStore.delete(req, () => {
      this.props.history.push(`/tenants/${this.props.tenant.getId()}/device-profiles`);
    });
  };

  render() {
    const dp = this.state.deviceProfile;

    if (!dp) {
      return null;
    }

    const disabled = !(
      SessionStore.isAdmin() ||
      SessionStore.isTenantAdmin(this.props.tenant.getId()) ||
      SessionStore.isTenantDeviceAdmin(this.props.tenant.getId())
    );

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
                <span>{dp.getName()}</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={dp.getName()}
          subTitle={`device profile id: ${dp.getId()}`}
          extra={[
            <Admin tenantId={this.props.tenant.getId()} isDeviceAdmin>
              <DeleteConfirm typ="device profile" confirm={dp.getName()} onConfirm={this.deleteDeviceProfile}>
                <Button danger type="primary">
                  Delete device profile
                </Button>
              </DeleteConfirm>
            </Admin>,
          ]}
        />
        <Card>
          <DeviceProfileForm initialValues={dp} disabled={disabled} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default EditDeviceProfile;
