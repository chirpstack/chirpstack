import { Link, useNavigate } from "react-router-dom";

import { Space, Breadcrumb, Card } from "antd";
import { PageHeader } from "@ant-design/pro-layout";

import { MacVersion, RegParamsRevision } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import {
  DeviceProfile,
  CreateDeviceProfileRequest,
  CreateDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import DeviceProfileForm from "./DeviceProfileForm";
import DeviceProfileStore from "../../stores/DeviceProfileStore";

interface IProps {
  tenant: Tenant;
}

function CreateDeviceProfile(props: IProps) {
  const navigate = useNavigate();

  const onFinish = (obj: DeviceProfile) => {
    obj.setTenantId(props.tenant.getId());

    let req = new CreateDeviceProfileRequest();
    req.setDeviceProfile(obj);

    DeviceProfileStore.create(req, (_resp: CreateDeviceProfileResponse) => {
      navigate(`/tenants/${props.tenant.getId()}/device-profiles`);
    });
  };

  const codecScript = `// Decode uplink function.
  //
  // Input is an object with the following fields:
  // - bytes = Byte array containing the uplink payload, e.g. [255, 230, 255, 0]
  // - fPort = Uplink fPort.
  // - variables = Object containing the configured device variables.
  //
  // Output must be an object with the following fields:
  // - data = Object representing the decoded payload.
  function decodeUplink(input) {
    return {
      data: {
        temp: 22.5
      }
    };
  }
  
  // Encode downlink function.
  //
  // Input is an object with the following fields:
  // - data = Object representing the payload that must be encoded.
  // - variables = Object containing the configured device variables.
  //
  // Output must be an object with the following fields:
  // - bytes = Byte array containing the downlink payload.
  function encodeDownlink(input) {
    return {
      bytes: [225, 230, 255, 0]
    };
  }
  `;

  let deviceProfile = new DeviceProfile();
  deviceProfile.setPayloadCodecScript(codecScript);
  deviceProfile.setSupportsOtaa(true);
  deviceProfile.setUplinkInterval(3600);
  deviceProfile.setDeviceStatusReqInterval(1);
  deviceProfile.setAdrAlgorithmId("default");
  deviceProfile.setMacVersion(MacVersion.LORAWAN_1_0_3);
  deviceProfile.setRegParamsRevision(RegParamsRevision.A);
  deviceProfile.setFlushQueueOnActivate(true);
  deviceProfile.setAutoDetectMeasurements(true);

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
                <Link to={`/tenants/${props.tenant.getId()}/device-profiles`}>Device profiles</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>Add</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title="Add device profile"
      />
      <Card>
        <DeviceProfileForm initialValues={deviceProfile} onFinish={onFinish} />
      </Card>
    </Space>
  );
}

export default CreateDeviceProfile;
