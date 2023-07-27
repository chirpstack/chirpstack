import React, { useState, useEffect } from "react";

import { useParams, Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card, Button } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import {
  DeviceProfileTemplate,
  GetDeviceProfileTemplateRequest,
  GetDeviceProfileTemplateResponse,
  UpdateDeviceProfileTemplateRequest,
  DeleteDeviceProfileTemplateRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";

import DeviceProfileTemplateForm from "./DeviceProfileTemplateForm";
import DeviceProfileTemplateStore from "../../stores/DeviceProfileTemplateStore";
import DeleteConfirm from "../../components/DeleteConfirm";

function EditDeviceProfileTemplate() {
  const navigate = useNavigate();
  const [deviceProfileTemplate, setDeviceProfileTemplate] = useState<DeviceProfileTemplate | undefined>(undefined);
  const { deviceProfileTemplateId } = useParams();

  useEffect(() => {
    const id = deviceProfileTemplateId!;
    let req = new GetDeviceProfileTemplateRequest();
    req.setId(id);

    DeviceProfileTemplateStore.get(req, (resp: GetDeviceProfileTemplateResponse) => {
      setDeviceProfileTemplate(resp.getDeviceProfileTemplate());
    });
  }, [deviceProfileTemplateId]);

  const onFinish = (obj: DeviceProfileTemplate) => {
    let req = new UpdateDeviceProfileTemplateRequest();
    req.setDeviceProfileTemplate(obj);

    DeviceProfileTemplateStore.update(req, () => {
      navigate(`/device-profile-templates`);
    });
  };

  const deleteDeviceProfileTemplate = () => {
    let req = new DeleteDeviceProfileTemplateRequest();
    req.setId(deviceProfileTemplateId!);

    DeviceProfileTemplateStore.delete(req, () => {
      navigate(`/device-profile-templates`);
    });
  };

  const dp = deviceProfileTemplate;

  if (!dp) {
    return null;
  }

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Network Server</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/device-profile-templates`}>Device-profile templates</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>{dp.getName()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={dp.getName()}
        subTitle={`device-profile template id: ${dp.getId()}`}
        extra={[
          <DeleteConfirm typ="device-profile template" confirm={dp.getName()} onConfirm={deleteDeviceProfileTemplate}>
            <Button danger type="primary">
              Delete device-profile template
            </Button>
          </DeleteConfirm>,
        ]}
      />
      <Card>
        <DeviceProfileTemplateForm initialValues={dp} update={true} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default EditDeviceProfileTemplate;
