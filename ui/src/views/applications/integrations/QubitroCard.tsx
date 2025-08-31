import { Card, Space } from "antd";
import { Link } from "react-router-dom";

import DeleteConfirm from "../../../components/DeleteConfirm";
import ApplicationStore from "../../../stores/ApplicationStore";
import { DeleteQubitroIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  add?: boolean;
  applicationId: string;
}

function QubitroCard({ add = false, applicationId }: IProps) {
  const deleteIntegration = () => {
    const req = new DeleteQubitroIntegrationRequest();
    req.setApplicationId(applicationId);
    
    ApplicationStore.deleteQubitroIntegration(req, () => {
      // Callback after successful deletion
    });
  };

  let actions: any[] = [];

  if (add) {
    actions = [<Link to={`/applications/${applicationId}/integrations/qubitro/create`}>Add</Link>];
  } else {
    actions = [
      <Link to={`/applications/${applicationId}/integrations/qubitro/edit`}>Edit</Link>,
      <DeleteConfirm typ="integration" confirm="delete" onConfirm={deleteIntegration} />,
    ];
  }

  return (
    <Card
      title="Qubitro"
      className="integration-card"
      cover={<img alt="Qubitro" src="/integrations/qubitro.png" style={{ padding: 1 }} />}
      actions={actions}
    >
      <Space direction="vertical">
        <p>
          The Qubitro integration forwards events to your Qubitro project using the Qubitro HTTP API.
        </p>
      </Space>
    </Card>
  );
}

export default QubitroCard; 