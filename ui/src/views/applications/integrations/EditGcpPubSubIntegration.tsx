import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type {
  Application,
  GcpPubSubIntegration,
  GetGcpPubSubIntegrationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  GetGcpPubSubIntegrationRequest,
  UpdateGcpPubSubIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import GcpPubSubIntegrationForm from "./GcpPubSubIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditGcpPubSubIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<GcpPubSubIntegration | undefined>(undefined);

  useEffect(() => {
    const req = new GetGcpPubSubIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getGcpPubSubIntegration(req, (resp: GetGcpPubSubIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: GcpPubSubIntegration) => {
    const req = new UpdateGcpPubSubIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateGcpPubSubIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update GCP Pub/Sub integration">
      <GcpPubSubIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditGcpPubSubIntegration;
