import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  AzureServiceBusIntegration,
  CreateAzureServiceBusIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import AzureServiceBusIntegrationForm from "./AzureServiceBusIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateAzureServiceBusIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: AzureServiceBusIntegration) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreateAzureServiceBusIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createAzureServiceBusIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new AzureServiceBusIntegration();

  return (
    <Card title="Add Azure Service-Bus integration">
      <AzureServiceBusIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateAzureServiceBusIntegration;
