import { useState } from "react";

import { Form, Input, Row, Col, Button, Tabs, Switch, Modal, Space, notification } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";
import { Scanner } from "@yudiel/react-qr-scanner";
import type { IDetectedBarcode } from "@yudiel/react-qr-scanner";

import { GetDeviceProfileByProfileIdRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  GetDeviceProfileByProfileIdResponse,
  DeviceProfile,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import { onFinishFailed } from "../helpers";
import EuiInput from "../../components/EuiInput";
import DeviceProfileSelect from "../../components/DeviceProfileSelect";
import DeviceProfileStore from "../../stores/DeviceProfileStore";

interface IProps {
  tenant: Tenant;
  initialValues: Device;
  onFinish: (obj: Device) => void;
  update?: boolean;
}

function DeviceForm(props: IProps) {
  const [form] = Form.useForm();
  const [showScanner, setShowScanner] = useState(false);
  const [deviceProfileId, setDeviceProfileId] = useState<string | null>(null);

  const onFinish = (values: Device.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    const d = new Device();

    d.setApplicationId(v.applicationId);
    d.setName(v.name);
    d.setDescription(v.description);
    d.setDevEui(v.devEui);
    d.setDeviceProfileId(v.deviceProfileId[v.deviceProfileId.length - 1]);
    d.setIsDisabled(v.isDisabled);
    d.setSkipFcntCheck(v.skipFcntCheck);
    d.setJoinEui(v.joinEui);

    // tags
    for (const elm of v.tagsMap) {
      d.getTagsMap().set(elm[0], elm[1]);
    }

    // variables
    for (const elm of v.variablesMap) {
      d.getVariablesMap().set(elm[0], elm[1]);
    }

    props.onFinish(d);
  };

  const onScannerError = (e: any) => {
    notification.error({
      message: "Error",
      description: e,
      duration: 3,
    });
  };

  const onScannerScan = (v: IDetectedBarcode[]) => {
    if (v.length === 0) {
      return;
    }
    setShowScanner(false);

    const barcode = v[0].rawValue;
    const barcodeBits = barcode.split(":");
    if (!barcode.startsWith("LW:D0") || barcodeBits.length < 5) {
      notification.error({
        message: "Error",
        description: "Invalid QR code",
        duration: 3,
      });
    }

    form.setFieldsValue({
      devEui: barcodeBits[3],
      joinEui: barcodeBits[2],
    });

    if (form.getFieldValue("name") === "") {
      form.setFieldValue("name", barcodeBits[3]);
    }

    const req = new GetDeviceProfileByProfileIdRequest();
    req.setVendorId(Number("0x" + barcodeBits[4].slice(0, 4)));
    req.setVendorProfileId(Number("0x" + barcodeBits[4].slice(4, 8)));
    DeviceProfileStore.getByProfileId(req, (resp: GetDeviceProfileByProfileIdResponse) => {
      const dp = resp.getDeviceProfile()!;
      setDeviceProfileId(dp.getId());
    });
  };

  return (
    <Form
      layout="vertical"
      initialValues={props.initialValues.toObject()}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
      form={form}
    >
      <Modal
        title="Scan QR-code"
        open={showScanner}
        onCancel={() => setShowScanner(false)}
        okButtonProps={{ style: { display: "none" } }}
      >
        <Space direction="vertical">
          <Scanner onScan={onScannerScan} onError={onScannerError} />
        </Space>
      </Modal>
      <Tabs
        tabBarExtraContent={
          <Button type="primary" disabled={props.update} onClick={() => setShowScanner(true)}>
            Scan QR-code
          </Button>
        }
      >
        <Tabs.TabPane tab="Device" key="1">
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input />
          </Form.Item>
          <Form.Item label="Description" name="description">
            <Input.TextArea />
          </Form.Item>
          <Row gutter={24}>
            <Col span={12}>
              <EuiInput label="Device EUI (EUI64)" name="devEui" disabled={props.update} required />
            </Col>
            <Col span={12}>
              <EuiInput
                label="Join EUI (EUI64)"
                name="joinEui"
                tooltip="The Join EUI will be automatically set / updated on OTAA. However, in some cases this field must be configured before OTAA (e.g. OTAA using a Relay)."
              />
            </Col>
          </Row>
          <DeviceProfileSelect
            label="Device profile"
            name="deviceProfileId"
            value={deviceProfileId || props.initialValues.getDeviceProfileId()}
            required
            tenant={props.tenant}
          />
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="Device is disabled"
                name="isDisabled"
                valuePropName="checked"
                tooltip="Received uplink frames and join-requests will be ignored."
              >
                <Switch />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item
                label="Disable frame-counter validation"
                name="skipFcntCheck"
                valuePropName="checked"
                tooltip="You must re-activate your device before this setting becomes effective. Note that disabling the frame-counter validation will compromise security as it allows replay-attacks."
              >
                <Switch />
              </Form.Item>
            </Col>
          </Row>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Tags" key="2">
          <Form.List name="tagsMap">
            {(fields, { add, remove }) => (
              <>
                {fields.map(({ key, name, ...restField }) => (
                  <Row gutter={24}>
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 0]}
                        rules={[{ required: true, message: "Please enter a key!" }]}
                      >
                        <Input placeholder="Key" />
                      </Form.Item>
                    </Col>
                    <Col span={16}>
                      <Form.Item
                        {...restField}
                        name={[name, 1]}
                        rules={[{ required: true, message: "Please enter a value!" }]}
                      >
                        <Input placeholder="Value" />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    Add tag
                  </Button>
                </Form.Item>
              </>
            )}
          </Form.List>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Variables" key="3">
          <Form.List name="variablesMap">
            {(fields, { add, remove }) => (
              <>
                {fields.map(({ key, name, ...restField }) => (
                  <Row gutter={24}>
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 0]}
                        rules={[{ required: true, message: "Please enter a key!" }]}
                      >
                        <Input placeholder="Key" />
                      </Form.Item>
                    </Col>
                    <Col span={16}>
                      <Form.Item
                        {...restField}
                        name={[name, 1]}
                        rules={[{ required: true, message: "Please enter a value!" }]}
                      >
                        <Input placeholder="Value" />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    Add variable
                  </Button>
                </Form.Item>
              </>
            )}
          </Form.List>
        </Tabs.TabPane>
      </Tabs>
      <Form.Item>
        <Button type="primary" htmlType="submit">
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default DeviceForm;
