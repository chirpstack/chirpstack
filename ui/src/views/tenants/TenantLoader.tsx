import React, { useState, useEffect } from "react";
import { useParams, Routes, Route } from "react-router";

import { Tenant, GetTenantResponse } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import TenantStore from "../../stores/TenantStore";
import SessionStore from "../../stores/SessionStore";

import TenantLayout from "./TenantLayout";
import ListTenantUsers from "./ListTenantUsers";
import CreateTenantUser from "./CreateTenantUser";
import EditTenantUser from "./EditTenantUser";

import ListTenantApiKeys from "../api-keys/ListTenantApiKeys";
import CreateTenantApiKey from "../api-keys/CreateTenantApiKey";

import ListDeviceProfiles from "../device-profiles/ListDeviceProfiles";
import CreateDeviceProfile from "../device-profiles/CreateDeviceProfile";
import EditDeviceProfile from "../device-profiles/EditDeviceProfile";

import ListGateways from "../gateways/ListGateways";
import CreateGateway from "../gateways/CreateGateway";
import GatewayLayout from "../gateways/GatewayLayout";

import ListApplications from "../applications/ListApplications";
import CreateApplication from "../applications/CreateApplication";
import ApplicationLoader from "../applications/ApplicationLoader";

function TenantLoader() {
  const [tenant, setTenant] = useState<Tenant | undefined>(undefined);
  const { tenantId } = useParams();

  useEffect(() => {
    const getTenant = () => {
      TenantStore.get(tenantId!, (resp: GetTenantResponse) => {
        setTenant(resp.getTenant());
      });
    };

    TenantStore.on("change", getTenant);

    getTenant();

    return () => {
      SessionStore.removeListener("change", getTenant);
    };
  }, [tenantId]);

  if (tenant === undefined) {
    return null;
  }

  return (
    <Routes>
      <Route path="/users" element={<ListTenantUsers tenant={tenant} />} />
      <Route path="/users/create" element={<CreateTenantUser tenant={tenant} />} />
      <Route path="/users/:userId/edit" element={<EditTenantUser tenant={tenant} />} />

      <Route path="/api-keys" element={<ListTenantApiKeys tenant={tenant} />} />
      <Route path="/api-keys/create" element={<CreateTenantApiKey tenant={tenant} />} />

      <Route path="/device-profiles" element={<ListDeviceProfiles tenant={tenant} />} />
      <Route path="/device-profiles/create" element={<CreateDeviceProfile tenant={tenant} />} />
      <Route path="/device-profiles/:deviceProfileId/edit" element={<EditDeviceProfile tenant={tenant} />} />

      <Route path="/gateways" element={<ListGateways tenant={tenant} />} />
      <Route path="/gateways/create" element={<CreateGateway tenant={tenant} />} />
      <Route path="/gateways/:gatewayId/*" element={<GatewayLayout tenant={tenant} />} />

      <Route path="/applications" element={<ListApplications tenant={tenant} />} />
      <Route path="/applications/create" element={<CreateApplication tenant={tenant} />} />
      <Route path="/applications/:applicationId/*" element={<ApplicationLoader tenant={tenant} />} />

      <Route path="/*" element={<TenantLayout tenant={tenant} />} />
    </Routes>
  );
}

export default TenantLoader;
