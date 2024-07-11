import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  AwsSnsIntegration,
  CreateAwsSnsIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import AwsSnsIntegrationForm from "./AwsSnsIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateAwsSnsIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: AwsSnsIntegration) => {
    obj.setApplicationId(props.application.getId());

    const req = new CreateAwsSnsIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createAwsSnsIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new AwsSnsIntegration();

  return (
    <Card title="Add AWS SNS integration">
      <AwsSnsIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateAwsSnsIntegration;
