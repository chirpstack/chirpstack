import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  HttpIntegration,
  CreateHttpIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import HttpIntegrationForm from "./HttpIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateHttpIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: HttpIntegration) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreateHttpIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createHttpIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new HttpIntegration();

  return (
    <Card title="Add HTTP integration">
      <HttpIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateHttpIntegration;
