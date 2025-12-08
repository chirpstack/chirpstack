import { useState, useEffect } from "react";
import { useParams, Routes, Route } from "react-router";

import { GetDeviceProfileDeviceRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import type {
  DeviceProfileVendor,
  DeviceProfileDevice,
  GetDeviceProfileDeviceResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import deviceProfileStore from "../../stores/DeviceProfileStore";

import ListVendorDeviceProfiles from "./ListVendorDeviceProfiles";
import ShowDeviceProfile from "./ShowDeviceProfile";

interface IProps {
  vendor: DeviceProfileVendor;
}

function DeviceLoader(props: IProps) {
  const { deviceId } = useParams();
  const [device, setDevice] = useState<DeviceProfileDevice | undefined>(undefined);

  useEffect(() => {
    const req = new GetDeviceProfileDeviceRequest();
    req.setId(deviceId!);

    deviceProfileStore.getDevice(req, (resp: GetDeviceProfileDeviceResponse) => {
      setDevice(resp.getDevice());
    });
  }, [deviceId]);

  if (!device) {
    return null;
  }

  return (
    <Routes>
      <Route path="/profiles" element={<ListVendorDeviceProfiles vendor={props.vendor} device={device} />} />
      <Route path="/profiles/:deviceProfileId" element={<ShowDeviceProfile vendor={props.vendor} device={device} />} />
    </Routes>
  );
}

export default DeviceLoader;
