import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  AzureServiceBusIntegration,
  GetAzureServiceBusIntegrationRequest,
  GetAzureServiceBusIntegrationResponse,
  UpdateAzureServiceBusIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import AzureServiceBusIntegrationForm from "./AzureServiceBusIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditAzureServiceBusIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<AzureServiceBusIntegration | undefined>(undefined);

  useEffect(() => {
    let req = new GetAzureServiceBusIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getAzureServiceBusIntegration(req, (resp: GetAzureServiceBusIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: AzureServiceBusIntegration) => {
    let req = new UpdateAzureServiceBusIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateAzureServiceBusIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update Azure Service-Bus integration">
      <AzureServiceBusIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditAzureServiceBusIntegration;
