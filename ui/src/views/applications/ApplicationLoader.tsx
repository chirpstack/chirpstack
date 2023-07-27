import React, { useState, useEffect } from "react";
import { Route, Routes, useParams } from "react-router-dom";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import {
  Application,
  GetApplicationRequest,
  GetApplicationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../stores/ApplicationStore";
import ApplicationLayout from "./ApplicationLayout";
import CreateDevice from "../devices/CreateDevice";
import DeviceLayout from "../devices/DeviceLayout";
import MulticastGroupLayout from "../multicast-groups/MulticastGroupLayout";
import CreateMulticastGroup from "../multicast-groups/CreateMulticastGroup";
import RelayLayout from "../relays/RelayLayout";

interface IProps {
  tenant: Tenant;
}

function ApplicationLoader(props: IProps) {
  const { applicationId } = useParams();
  const [application, setApplication] = useState<Application | undefined>(undefined);
  const [measurementKeys, setMeasurementKeys] = useState<string[]>([]);

  useEffect(() => {
    ApplicationStore.on("change", loadApplication);
    loadApplication();

    return () => {
      ApplicationStore.removeAllListeners("change");
    };
  }, [applicationId]);

  const loadApplication = () => {
    let req = new GetApplicationRequest();
    req.setId(applicationId!);

    ApplicationStore.get(req, (resp: GetApplicationResponse) => {
      setApplication(resp.getApplication());
      setMeasurementKeys(resp.getMeasurementKeysList());
    });
  };

  const app = application;
  if (!app) {
    return null;
  }

  const tenant = props.tenant;

  return (
    <Routes>
      <Route path="/devices/create" element={<CreateDevice tenant={tenant} application={app} />} />
      <Route path="/multicast-groups/create" element={<CreateMulticastGroup tenant={tenant} application={app} />} />
      <Route
        path="/multicast-groups/:multicastGroupId/*"
        element={<MulticastGroupLayout tenant={tenant} application={app} />}
      />
      <Route path="/devices/:devEui/*" element={<DeviceLayout tenant={tenant} application={app} />} />
      <Route path="/relays/:relayDevEui/*" element={<RelayLayout tenant={tenant} application={app} />} />
      <Route
        path="/*"
        element={<ApplicationLayout tenant={tenant} application={app} measurementKeys={measurementKeys} />}
      />
    </Routes>
  );
}

export default ApplicationLoader;
