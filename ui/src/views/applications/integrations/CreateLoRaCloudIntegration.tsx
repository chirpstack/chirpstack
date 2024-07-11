import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  LoraCloudIntegration,
  LoraCloudModemGeolocationServices,
  CreateLoraCloudIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import LoRaCloudIntegrationForm from "./LoRaCloudIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function CreateLoRaCloudIntegration(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: LoraCloudIntegration) => {
    obj.setApplicationId(props.application.getId());

    const req = new CreateLoraCloudIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.createLoraCloudIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  const i = new LoraCloudIntegration();
  const mgs = new LoraCloudModemGeolocationServices();
  mgs.setModemEnabled(true);
  mgs.setForwardFPortsList([192, 197, 198, 199]);

  i.setModemGeolocationServices(mgs);

  return (
    <Card title="Add Semtech LoRa Cloud&trade; integration">
      <LoRaCloudIntegrationForm initialValues={i} onFinish={onFinish} />
    </Card>
  );
}

export default CreateLoRaCloudIntegration;
