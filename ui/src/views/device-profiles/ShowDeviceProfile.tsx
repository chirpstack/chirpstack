import { useNavigate, useParams, Link } from "react-router-dom";

import { Space, Breadcrumb, Button, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { GetDeviceProfileRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  DeviceProfileDevice,
  DeviceProfileVendor,
  GetDeviceProfileResponse,
  DeviceProfile,
  DeleteDeviceProfileRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import { useTitle } from "../helpers";
import { useEffect, useState } from "react";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import DeviceProfileForm from "./DeviceProfileForm";
import DeleteConfirm from "../../components/DeleteConfirm";

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

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Network Server</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Device Profiles</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/device-profiles/vendors`}>Vendors</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/device-profiles/vendors/${props.vendor.getId()}/devices`}>{props.vendor.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/device-profiles/vendors/${props.vendor.getId()}/devices/${props.device.getId()}/profiles`}>
                  {props.device.getName()}
                </Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>{deviceProfile.getName()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={deviceProfile.getName()}
        extra={[
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
