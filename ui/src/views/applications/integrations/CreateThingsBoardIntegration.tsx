import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  ThingsBoardIntegration,
  CreateThingsBoardIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ThingsBoardIntegrationForm from "./ThingsBoardIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateThingsBoardIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: ThingsBoardIntegration) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreateThingsBoardIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createThingsBoardIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new ThingsBoardIntegration();

  return (
    <Card title="Add ThingsBoard integration">
      <ThingsBoardIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateThingsBoardIntegration;
