import { Card } from "antd";
import { useNavigate, useParams } from "react-router-dom";
import { useState, useEffect } from "react";

import QubitroIntegrationForm from "./QubitroIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";
import { 
  QubitroIntegration,
  UpdateQubitroIntegrationRequest,
  GetQubitroIntegrationRequest,
  GetQubitroIntegrationResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

function EditQubitroIntegration() {
  const { applicationId } = useParams();
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<QubitroIntegration.AsObject | undefined>(undefined);

  useEffect(() => {
    const fetchIntegration = () => {
      const req = new GetQubitroIntegrationRequest();
      req.setApplicationId(applicationId!);

      ApplicationStore.getQubitroIntegration(req, (resp: GetQubitroIntegrationResponse) => {
        const integration = resp.getIntegration()!;
        setIntegration({
          applicationId: integration.getApplicationId(),
          projectId: integration.getProjectId(),
          webhookSigningKey: integration.getWebhookSigningKey(),
        });
      });
    };

    fetchIntegration();
  }, [applicationId]);

  const onFinish = async (obj: QubitroIntegration.AsObject) => {
    try {
      const integration = new QubitroIntegration();
      integration.setApplicationId(applicationId!);
      integration.setProjectId(obj.projectId);
      integration.setWebhookSigningKey(obj.webhookSigningKey);

      const request = new UpdateQubitroIntegrationRequest();
      request.setIntegration(integration);

      await new Promise<void>((resolve, reject) => {
        ApplicationStore.updateQubitroIntegration(request, () => {
          resolve();
        });
      });

      navigate(`/applications/${applicationId}/integrations`);
    } catch (e) {
      console.error(e);
    }
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update Qubitro integration">
      <QubitroIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditQubitroIntegration; 