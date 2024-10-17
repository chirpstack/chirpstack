import { useState, useEffect } from "react";
import { useNavigate, Link, useParams } from "react-router-dom";

import { Space, Breadcrumb, Card, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type {
  DeviceProfile,
  GetDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  GetDeviceProfileRequest,
  UpdateDeviceProfileRequest,
  DeleteDeviceProfileRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import DeviceProfileForm from "./DeviceProfileForm";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import SessionStore from "../../stores/SessionStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import Admin from "../../components/Admin";
import { useTitle } from "../helpers";

interface IProps {
  tenant: Tenant;
}

function EditDeviceProfile(props: IProps) {
  const navigate = useNavigate();
  const [deviceProfile, setDeviceProfile] = useState<DeviceProfile | undefined>(undefined);
  const { deviceProfileId } = useParams();
  useTitle("Tenants", props.tenant.getName(), "Device profiles", deviceProfile?.getName());

  useEffect(() => {
    const id = deviceProfileId!;
    const req = new GetDeviceProfileRequest();
    req.setId(id);

    DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
      setDeviceProfile(resp.getDeviceProfile());
    });
  }, [deviceProfileId]);

  const onFinish = (obj: DeviceProfile) => {
    const req = new UpdateDeviceProfileRequest();
    req.setDeviceProfile(obj);

    DeviceProfileStore.update(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/device-profiles`);
    });
  };

  const deleteDeviceProfile = () => {
    const req = new DeleteDeviceProfileRequest();
    req.setId(deviceProfileId!);

    DeviceProfileStore.delete(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/device-profiles`);
    });
  };

  const dp = deviceProfile;

  if (!dp) {
    return null;
  }

  const disabled = !(
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.tenant.getId()) ||
    SessionStore.isTenantDeviceAdmin(props.tenant.getId())
  );

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Building</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/device-profiles`}>Device profiles</Link>
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
          <Admin tenantId={props.tenant.getId()} isDeviceAdmin>
            <DeleteConfirm typ="device profile" confirm={dp.getName()} onConfirm={deleteDeviceProfile}>
              <Button danger type="primary">
                Delete device profile
              </Button>
            </DeleteConfirm>
          </Admin>,
        ]}
      />
      <Card>
        <DeviceProfileForm initialValues={dp} disabled={disabled} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default EditDeviceProfile;
