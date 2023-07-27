import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  AwsSnsIntegration,
  GetAwsSnsIntegrationRequest,
  GetAwsSnsIntegrationResponse,
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
    let req = new GetAwsSnsIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getAwsSnsIntegration(req, (resp: GetAwsSnsIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: AwsSnsIntegration) => {
    let req = new UpdateAwsSnsIntegrationRequest();
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
