import { useNavigate } from "react-router-dom";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  MulticastGroup,
  UpdateMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import MulticastGroupStore from "../../stores/MulticastGroupStore";
import MulticastGroupForm from "./MulticastGroupForm";
import SessionStore from "../../stores/SessionStore";

interface IProps {
  multicastGroup: MulticastGroup;
  application: Application;
}

function EditMulticastGroup(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: MulticastGroup) => {
    let req = new UpdateMulticastGroupRequest();
    req.setMulticastGroup(obj);

    MulticastGroupStore.update(req, () => {
      navigate(`../${props.multicastGroup.getId()}`);
    });
  };

  let disabled = !(
    SessionStore.isAdmin() ||
    SessionStore.isTenantAdmin(props.application.getTenantId()) ||
    SessionStore.isTenantDeviceAdmin(props.application.getTenantId())
  );

  return <MulticastGroupForm initialValues={props.multicastGroup} disabled={disabled} onFinish={onFinish} />;
}

export default EditMulticastGroup;
