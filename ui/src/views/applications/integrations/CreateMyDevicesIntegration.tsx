import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  MyDevicesIntegration,
  CreateMyDevicesIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import MyDevicesIntegrationForm from "./MyDevicesIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateMyDevicesIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: MyDevicesIntegration) => {
    obj.setApplicationId(props.application.getId());

    let req = new CreateMyDevicesIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createMyDevicesIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new MyDevicesIntegration();

  return (
    <Card title="Add myDevices integration">
      <MyDevicesIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateMyDevicesIntegration;
