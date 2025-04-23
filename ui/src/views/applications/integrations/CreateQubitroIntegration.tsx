import { Card } from "antd";
import { useNavigate } from "react-router-dom";

import { Application, QubitroIntegration, CreateQubitroIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import QubitroIntegrationForm from "./QubitroIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateQubitroIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: QubitroIntegration) => {
    obj.setApplicationId(props.application.getId());

    const req = new CreateQubitroIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createQubitroIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new QubitroIntegration();

  return (
    <Card title="Add Qubitro integration">
      <QubitroIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateQubitroIntegration; 