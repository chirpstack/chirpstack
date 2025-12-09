import { useState, useEffect } from "react";
import { useParams, Routes, Route } from "react-router";

import { GetDeviceProfileVendorRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import type {
  DeviceProfileVendor,
  GetDeviceProfileVendorResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import deviceProfileStore from "../../stores/DeviceProfileStore";

import ListDevices from "./ListDevices";
import DeviceLoader from "./DeviceLoader";

function VendorLoader() {
  const { vendorId } = useParams();
  const [vendor, setVendor] = useState<DeviceProfileVendor | undefined>(undefined);

  useEffect(() => {
    const req = new GetDeviceProfileVendorRequest();
    req.setId(vendorId!);

    deviceProfileStore.getVendor(req, (resp: GetDeviceProfileVendorResponse) => {
      setVendor(resp.getVendor());
    });
  }, [vendorId]);

  if (!vendor) {
    return null;
  }

  return (
    <Routes>
      <Route path="/devices" element={<ListDevices vendor={vendor} />} />
      <Route path="/devices/:deviceId/*" element={<DeviceLoader vendor={vendor} />} />
    </Routes>
  );
}

export default VendorLoader;
