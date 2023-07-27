import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  GcpPubSubIntegration,
  CreateGcpPubSubIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import GcpPubSubIntegrationForm from "./GcpPubSubIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateGcpPubSubIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: GcpPubSubIntegration) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreateGcpPubSubIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createGcpPubSubIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new GcpPubSubIntegration();

  return (
    <Card title="Add GCP Pub/Sub integration">
      <GcpPubSubIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateGcpPubSubIntegration;
