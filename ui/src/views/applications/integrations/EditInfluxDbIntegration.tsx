import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type {
  Application,
  InfluxDbIntegration,
  GetInfluxDbIntegrationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  GetInfluxDbIntegrationRequest,
  UpdateInfluxDbIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import InfluxDbIntegrationForm from "./InfluxDbIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditInfluxDbIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<InfluxDbIntegration | undefined>(undefined);

  useEffect(() => {
    const req = new GetInfluxDbIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getInfluxDbIntegration(req, (resp: GetInfluxDbIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: InfluxDbIntegration) => {
    const req = new UpdateInfluxDbIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateInfluxDbIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update InfluxDB integration">
      <InfluxDbIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditInfluxDbIntegration;
