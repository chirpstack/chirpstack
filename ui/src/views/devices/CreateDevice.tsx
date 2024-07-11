import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { CreateDeviceRequest, Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import type { GetDeviceProfileResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { GetDeviceProfileRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import DeviceForm from "./DeviceForm";
import DeviceStore from "../../stores/DeviceStore";
import DeviceProfileStore from "../../stores/DeviceProfileStore";

interface IProps {
  tenant: Tenant;
  application: Application;
}

function CreateDevice(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: Device) => {
    obj.setApplicationId(props.application.getId());

    const req = new CreateDeviceRequest();
    req.setDevice(obj);

    DeviceStore.create(req, () => {
      const req = new GetDeviceProfileRequest();
      req.setId(obj.getDeviceProfileId());

      DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
        const dp = resp.getDeviceProfile()!;
        if (dp.getSupportsOtaa()) {
          navigate(
            `/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/devices/${obj.getDevEui()}/keys`,
          );
        } else {
          navigate(
            `/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/devices/${obj.getDevEui()}`,
          );
        }
      });
    });
  };

  const device = new Device();
  device.setApplicationId(props.application.getId());

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Tenants</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications`}>Applications</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to={`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}`}>
                  {props.application.getName()}
                </Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Add device</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Add device"
      />
      <Card>
        <DeviceForm tenant={props.tenant} initialValues={device} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateDevice;
