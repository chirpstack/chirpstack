import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { Card } from "antd";

import {
  Application,
  MyDevicesIntegration,
  GetMyDevicesIntegrationRequest,
  GetMyDevicesIntegrationResponse,
  UpdateMyDevicesIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import MyDevicesIntegrationForm from "./MyDevicesIntegrationForm";
import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
}

function EditMyDevicesIntegration(props: IProps) {
  const navigate = useNavigate();
  const [integration, setIntegration] = useState<MyDevicesIntegration | undefined>(undefined);

  useEffect(() => {
    let req = new GetMyDevicesIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.getMyDevicesIntegration(req, (resp: GetMyDevicesIntegrationResponse) => {
      setIntegration(resp.getIntegration());
    });
  }, [props]);

  const onFinish = (obj: MyDevicesIntegration) => {
    let req = new UpdateMyDevicesIntegrationRequest();
    req.setIntegration(obj);

    ApplicationStore.updateMyDevicesIntegration(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/integrations`);
    });
  };

  if (integration === undefined) {
    return null;
  }

  return (
    <Card title="Update myDevices integration">
      <MyDevicesIntegrationForm initialValues={integration} onFinish={onFinish} />
    </Card>
  );
}

export default EditMyDevicesIntegration;
