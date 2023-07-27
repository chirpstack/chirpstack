import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  ThingsBoardIntegration,
  GetThingsBoardIntegrationRequest,
  GetThingsBoardIntegrationResponse,
  UpdateThingsBoardIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ThingsBoardIntegrationForm from "./ThingsBoardIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditThingsBoardIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<ThingsBoardIntegration | undefined>(undefined);

  useEffect(() => {
    let req = new GetThingsBoardIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getThingsBoardIntegration(req, (resp: GetThingsBoardIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: ThingsBoardIntegration) => {
    let req = new UpdateThingsBoardIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateThingsBoardIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update ThingsBoard integration">
      <ThingsBoardIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditThingsBoardIntegration;
