import { Card } from "antd";
import { useNavigate, useParams } from "react-router-dom";

import QubitroIntegrationForm from "./QubitroIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";
import { QubitroIntegration, CreateQubitroIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

function CreateQubitroIntegration() {
  const { applicationId } = useParams();
  const navigate = useNavigate();

  const onFinish = async (obj: QubitroIntegration.AsObject) => {
    try {
      const integration = new QubitroIntegration();
      integration.setApplicationId(applicationId!);
      integration.setProjectId(obj.projectId);
      integration.setWebhookSigningKey(obj.webhookSigningKey);

      const request = new CreateQubitroIntegrationRequest();
      request.setIntegration(integration);

      await new Promise<void>((resolve, reject) => {
        ApplicationStore.createQubitroIntegration(request, () => {
          resolve();
        });
      });

      navigate(`/applications/${applicationId}/integrations`);
    } catch (e) {
      console.error(e);
    }
  };

  const initialValues: QubitroIntegration.AsObject = {
    applicationId: applicationId!,
    projectId: "",
    webhookSigningKey: "",
  };

  return (
    <Card title="Add Qubitro integration">
      <QubitroIntegrationForm initialValues={initialValues} onFinish={onFinish} />
    </Card>
  );
}

export default CreateQubitroIntegration; 