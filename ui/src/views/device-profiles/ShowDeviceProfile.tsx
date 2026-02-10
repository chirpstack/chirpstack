import { useNavigate, useParams, Link } from "react-router-dom";

import { Space, Breadcrumb, Button, Card, Popconfirm } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { GetDeviceProfileRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  DeviceProfileDevice,
  DeviceProfileVendor,
  GetDeviceProfileResponse,
  DeviceProfile,
  DeleteDeviceProfileRequest,
  CreateDeviceProfileRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import type { CreateDeviceProfileResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import { useTitle } from "../helpers";
import { useEffect, useState } from "react";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import DeviceProfileForm from "./DeviceProfileForm";
import DeleteConfirm from "../../components/DeleteConfirm";
import sessionStore from "../../stores/SessionStore";

interface IProps {
  vendor: DeviceProfileVendor;
  device: DeviceProfileDevice;
}

function ShowDeviceProfile(props: IProps) {
  const navigate = useNavigate();
  const [deviceProfile, setDeviceProfile] = useState<DeviceProfile | undefined>(undefined);
  const { deviceProfileId } = useParams();

  useTitle([
    "Network Server",
    "Device Profiles",
    "Vendors",
    props.vendor.getName(),
    props.device.getName(),
    deviceProfile?.getName(),
  ]);

  useEffect(() => {
    const req = new GetDeviceProfileRequest();
    req.setId(deviceProfileId!);

    DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
      setDeviceProfile(resp.getDeviceProfile());
    });
  }, [deviceProfileId]);

  const deleteDeviceProfile = () => {
    const req = new DeleteDeviceProfileRequest();
    req.setId(deviceProfile!.getId());

    DeviceProfileStore.delete(req, () => {
      navigate(`/device-profiles/vendors/${props.vendor.getId()}/devices/${props.device.getId()}/profiles`);
    });
  };

  if (!deviceProfile) {
    return null;
  }

  const copyDeviceProfile = () => {
    const dp = deviceProfile.clone();
    dp.setId("");
    dp.setDeviceId("");
    dp.setTenantId(sessionStore.getTenantId());

    const req = new CreateDeviceProfileRequest();
    req.setDeviceProfile(dp);

    DeviceProfileStore.create(req, (resp: CreateDeviceProfileResponse) => {
      navigate(`/tenants/${sessionStore.getTenantId()}/device-profiles/${resp.getId()}/edit`);
    });
  };

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb
            items={[
              { title: "Network Server" },
              { title: "Device Profiles" },
              { title: <Link to={`/device-profiles/vendors`}>Vendors</Link> },
              {
                title: (
                  <Link to={`/device-profiles/vendors/${props.vendor.getId()}/devices`}>{props.vendor.getName()}</Link>
                ),
              },
              {
                title: (
                  <Link
                    to={`/device-profiles/vendors/${props.vendor.getId()}/devices/${props.device.getId()}/profiles`}
                  >
                    {props.device.getName()}
                  </Link>
                ),
              },
              { title: deviceProfile.getName() },
            ]}
          />
        )}
        title={deviceProfile.getName()}
        extra={[
          <Popconfirm
            placement="left"
            title="Copy device profile"
            description="This will copy the device profile to the selected tenant such that it can be modified. Would you like to proceed?"
            onConfirm={copyDeviceProfile}
          >
            <Button type="primary">Copy device profile</Button>
          </Popconfirm>,
          <DeleteConfirm typ="device profile" confirm={deviceProfile.getName()} onConfirm={deleteDeviceProfile}>
            <Button danger type="primary">
              Delete device profile
            </Button>
          </DeleteConfirm>,
        ]}
      />
      <Card>
        <DeviceProfileForm initialValues={deviceProfile} disabled onFinish={(_obj: DeviceProfile) => {}} />
      </Card>
    </Space>
  );
}

export default ShowDeviceProfile;
