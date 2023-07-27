import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  PilotThingsIntegration,
  CreatePilotThingsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import PilotThingsIntegrationForm from "./PilotThingsIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreatePilotThingsIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: PilotThingsIntegration) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreatePilotThingsIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createPilotThingsIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new PilotThingsIntegration();

  return (
    <Card title="Add Pilot Things integration">
      <PilotThingsIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreatePilotThingsIntegration;
