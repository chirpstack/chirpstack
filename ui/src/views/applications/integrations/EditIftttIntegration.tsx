import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type {
  Application,
  IftttIntegration,
  GetIftttIntegrationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  GetIftttIntegrationRequest,
  UpdateIftttIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import IftttIntegrationForm from "./IftttIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  measurementKeys: string[];
}

function EditIftttIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<IftttIntegration | undefined>(undefined);

  useEffect(() => {
    const req = new GetIftttIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getIftttIntegration(req, (resp: GetIftttIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: IftttIntegration) => {
    const req = new UpdateIftttIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateIftttIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update IFTTT integration">
      <IftttIntegrationForm measurementKeys={props.measurementKeys} initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditIftttIntegration;
