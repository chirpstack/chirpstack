import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  InfluxDbIntegration,
  CreateInfluxDbIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import InfluxDbIntegrationForm from "./InfluxDbIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateInfluxDbIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: InfluxDbIntegration) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreateInfluxDbIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createInfluxDbIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new InfluxDbIntegration();

  return (
    <Card title="Add InfluxDB integration">
      <InfluxDbIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateInfluxDbIntegration;
