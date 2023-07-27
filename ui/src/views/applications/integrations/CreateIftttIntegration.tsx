import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  IftttIntegration,
  CreateIftttIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import IftttIntegrationForm from "./IftttIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  measurementKeys: string[];
}

function CreateIftttIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: IftttIntegration) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreateIftttIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createIftttIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new IftttIntegration();
  i.setUplinkValuesList(["", ""]);

  return (
    <Card title="Add IFTTT integration">
      <IftttIntegrationForm measurementKeys={props.measurementKeys} initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateIftttIntegration;
