import React, { Component } from "react";
import { Link, RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, Card, PageHeader } from "antd";

import { MacVersion, RegParamsRevision } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import {
  DeviceProfileTemplate,
  CreateDeviceProfileTemplateRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";

import DeviceProfileTemplateForm from "./DeviceProfileTemplateForm";
import DeviceProfileTemplateStore from "../../stores/DeviceProfileTemplateStore";

class CreateDeviceProfileTemplate extends Component<RouteComponentProps> {
  onFinish = (obj: DeviceProfileTemplate) => {
    let req = new CreateDeviceProfileTemplateRequest();
    req.setDeviceProfileTemplate(obj);

    DeviceProfileTemplateStore.create(req, () => {
      this.props.history.push(`/device-profile-templates`);
    });
  };

  render() {
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

    let deviceProfileTemplate = new DeviceProfileTemplate();
    deviceProfileTemplate.setPayloadCodecScript(codecScript);
    deviceProfileTemplate.setSupportsOtaa(true);
    deviceProfileTemplate.setUplinkInterval(3600);
    deviceProfileTemplate.setDeviceStatusReqInterval(1);
    deviceProfileTemplate.setAdrAlgorithmId("default");
    deviceProfileTemplate.setMacVersion(MacVersion.LORAWAN_1_0_3);
    deviceProfileTemplate.setRegParamsRevision(RegParamsRevision.A);
    deviceProfileTemplate.setFlushQueueOnActivate(true);

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
                <span>Add</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title="Add device-profile template"
        />
        <Card>
          <DeviceProfileTemplateForm initialValues={deviceProfileTemplate} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default CreateDeviceProfileTemplate;
