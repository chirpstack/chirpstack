import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type {
  Application,
  LoraCloudIntegration,
  GetLoraCloudIntegrationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  GetLoraCloudIntegrationRequest,
  UpdateLoraCloudIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import LoRaCloudIntegrationForm from "./LoRaCloudIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditLoRaCloudIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<LoraCloudIntegration | undefined>(undefined);

  useEffect(() => {
    const req = new GetLoraCloudIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getLoraCloudIntegration(req, (resp: GetLoraCloudIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: LoraCloudIntegration) => {
    const req = new UpdateLoraCloudIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateLoraCloudIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update Semtech LoRa Cloud&trade; integration">
      <LoRaCloudIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditLoRaCloudIntegration;
