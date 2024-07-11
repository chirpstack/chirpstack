import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type {
  Application,
  AwsSnsIntegration,
  GetAwsSnsIntegrationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  GetAwsSnsIntegrationRequest,
  UpdateAwsSnsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import AwsSnsIntegrationForm from "./AwsSnsIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditAwsSnsIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<AwsSnsIntegration | undefined>(undefined);

  useEffect(() => {
    const req = new GetAwsSnsIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getAwsSnsIntegration(req, (resp: GetAwsSnsIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: AwsSnsIntegration) => {
    const req = new UpdateAwsSnsIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateAwsSnsIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update AWS SNS integration">
      <AwsSnsIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditAwsSnsIntegration;
