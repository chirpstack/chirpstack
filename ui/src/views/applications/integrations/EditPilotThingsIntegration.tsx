import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  PilotThingsIntegration,
  GetPilotThingsIntegrationRequest,
  GetPilotThingsIntegrationResponse,
  UpdatePilotThingsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import PilotThingsIntegrationForm from "./PilotThingsIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditPilotThingsIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<PilotThingsIntegration | undefined>(undefined);

  useEffect(() => {
    let req = new GetPilotThingsIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getPilotThingsIntegration(req, (resp: GetPilotThingsIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: PilotThingsIntegration) => {
    let req = new UpdatePilotThingsIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updatePilotThingsIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update Pilot Things integration">
      <PilotThingsIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditPilotThingsIntegration;
