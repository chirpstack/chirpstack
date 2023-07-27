import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  HttpIntegration,
  GetHttpIntegrationRequest,
  GetHttpIntegrationResponse,
  UpdateHttpIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import HttpIntegrationForm from "./HttpIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditHttpIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<HttpIntegration | undefined>(undefined);

  useEffect(() => {
    let req = new GetHttpIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getHttpIntegration(req, (resp: GetHttpIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: HttpIntegration) => {
    let req = new UpdateHttpIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateHttpIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update HTTP integration">
      <HttpIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditHttpIntegration;
