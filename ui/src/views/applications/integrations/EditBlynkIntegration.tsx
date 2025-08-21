import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type {
  Application,
  BlynkIntegration,
  GetBlynkIntegrationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  GetBlynkIntegrationRequest,
  UpdateBlynkIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import BlynkIntegrationForm from "./BlynkIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditBlynkIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<BlynkIntegration | undefined>(undefined);

  useEffect(() => {
    const req = new GetBlynkIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getBlynkIntegration(req, (resp: GetBlynkIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: BlynkIntegration) => {
    const req = new UpdateBlynkIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateBlynkIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update Blynk integration">
      <BlynkIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditBlynkIntegration;
