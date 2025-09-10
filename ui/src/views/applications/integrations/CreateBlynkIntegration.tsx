import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  BlynkIntegration,
  CreateBlynkIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import BlynkIngrationForm from "./BlynkIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateBlynkIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: BlynkIntegration) => {
    obj.setApplicationId(props.application.getId());

    const req = new CreateBlynkIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createBlynkIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new BlynkIntegration();

  return (
    <Card title="Add Blynk integration">
      <BlynkIngrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateBlynkIntegration;
