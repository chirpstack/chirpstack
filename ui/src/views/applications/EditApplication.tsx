import { useNavigate } from "react-router-dom";

import { Application, UpdateApplicationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../stores/ApplicationStore";
import ApplicationForm from "./ApplicationForm";
import SessionStore from "../../stores/SessionStore";

interface IProps {
  application: Application;
}

function EditApplication(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: Application) => {
    let req = new UpdateApplicationRequest();
    req.setApplication(obj);

    ApplicationStore.update(req, () => {
      navigate(`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}`);
    });
  };

  const disabled = !(
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.application.getTenantId()) ||
    SessionStore.isTenantDeviceAdmin(props.application.getTenantId())
  );

  return <ApplicationForm initialValues={props.application} disabled={disabled} onFinish={onFinish} />;
}

export default EditApplication;
