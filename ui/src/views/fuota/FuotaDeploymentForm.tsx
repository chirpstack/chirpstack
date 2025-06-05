import { useEffect, useState } from "react";

import { Tabs, Form, Input, InputNumber, Select, Row, Col, Button, Upload, UploadFile, Switch } from "antd";
import { UploadOutlined, MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { FuotaDeployment, RequestFragmentationSessionStatus } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import type {
  ListDeviceProfilesResponse,
  GetDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  ListDeviceProfilesRequest,
  GetDeviceProfileRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  MulticastGroupType,
  MulticastGroupSchedulingType,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import { onFinishFailed, getEnumName } from "../helpers";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import AutocompleteInput from "../../components/AutocompleteInput";
import type { OptionsCallbackFunc, OptionCallbackFunc } from "../../components/Autocomplete";

interface IProps {
  tenant: Tenant;
  initialValues: FuotaDeployment;
  onFinish: (obj: FuotaDeployment) => void;
  disabled?: boolean;
  update?: boolean;
}

function FuotaDeploymentForm(props: IProps) {
  const [form] = Form.useForm();
  const [calculateMulticastTimeout, setCalculateMulticastTimeout] = useState<boolean>(
    props.initialValues.getCalculateMulticastTimeout(),
  );
  const [calculateFragmentationFragmentSize, setCalculateFragmentationFragmentSize] = useState<boolean>(
    props.initialValues.getCalculateFragmentationFragmentSize(),
  );
  const [isMulticastClassB, setIsMulticastClassB] = useState<boolean>(
    props.initialValues.getMulticastGroupType() === MulticastGroupType.CLASS_B,
  );
  const [fileList, setFileList] = useState<UploadFile[]>([]);

  useEffect(() => {
    if (props.initialValues.getPayload().length != 0) {
      setFileList([
        {
          uid: "-1",
          name: `firmware.bin (${props.initialValues.getPayload().length} bytes)`,
          status: "done",
        },
      ]);
    }
  }, [props.initialValues]);

  const onFinish = (values: FuotaDeployment.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    const d = new FuotaDeployment();

    d.setId(v.id);
    d.setApplicationId(v.applicationId);

    d.setName(v.name);
    d.setDeviceProfileId(v.deviceProfileId);
    d.setUnicastMaxRetryCount(v.unicastMaxRetryCount);
    d.setMulticastGroupType(v.multicastGroupType);
    d.setMulticastClassBPingSlotPeriodicity(v.multicastClassBPingSlotPeriodicity);
    d.setMulticastClassCSchedulingType(v.multicastClassCSchedulingType);
    d.setMulticastDr(v.multicastDr);
    d.setMulticastFrequency(v.multicastFrequency);
    d.setFragmentationRedundancyPercentage(v.fragmentationRedundancyPercentage);
    d.setRequestFragmentationSessionStatus(v.requestFragmentationSessionStatus);
    d.setCalculateMulticastTimeout(v.calculateMulticastTimeout);
    d.setMulticastTimeout(v.multicastTimeout);
    d.setCalculateFragmentationFragmentSize(v.calculateFragmentationFragmentSize);
    d.setFragmentationFragmentSize(v.fragmentationFragmentSize);
    d.setPayload(v.payload);

    // on complete set device tags
    for (const elm of v.onCompleteSetDeviceTagsMap) {
      d.getOnCompleteSetDeviceTagsMap().set(elm[0], elm[1]);
    }

    props.onFinish(d);
  };

  const onMulticastGroupTypeChange = (groupType: MulticastGroupType) => {
    setIsMulticastClassB(groupType == MulticastGroupType.CLASS_B);
  };

  const getDeviceProfileOptions = (search: string, fn: OptionsCallbackFunc) => {
    const req = new ListDeviceProfilesRequest();
    req.setTenantId(props.tenant.getId());
    req.setSearch(search);
    req.setLimit(10);

    DeviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
      const options = resp.getResultList().map((o, i) => {
        return { label: o.getName(), value: o.getId() };
      });

      fn(options);
    });
  };

  const getDeviceProfileOption = (id: string, fn: OptionCallbackFunc) => {
    const req = new GetDeviceProfileRequest();
    req.setId(id);

    DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
      const dp = resp.getDeviceProfile();
      if (dp) {
        fn({ label: dp.getName(), value: dp.getId() });
      }
    });
  };

  const beforeUpload = (file: UploadFile) => {
    const reader = new FileReader();
    reader.onload = e => {
      if (e.target !== null && e.target.result !== null && typeof e.target.result !== "string") {
        const arrayBuffer = e.target.result;
        const bytes = new Uint8Array(arrayBuffer);

        setFileList([
          {
            uid: "-1",
            name: `firmware.bin (${bytes.length} bytes)`,
            status: "done",
          },
        ]);

        form.setFieldsValue({
          payload: bytes,
        });
      }
    };
    reader.readAsArrayBuffer(file as unknown as Blob);
    return false;
  };

  const onRemoveUpload = () => {
    setFileList([]);
    form.setFieldsValue({
      payload: new Uint8Array(),
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
      <Tabs>
        <Tabs.TabPane tab="Deployment" key="1">
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input disabled={props.disabled} />
          </Form.Item>
          <Row gutter={24}>
            <Col span={16}>
              <AutocompleteInput
                label="Device profile"
                name="deviceProfileId"
                getOption={getDeviceProfileOption}
                getOptions={getDeviceProfileOptions}
                disabled={props.disabled || props.update}
                required
              />
            </Col>
            <Col span={8}>
              <Form.Item
                label="Unicast retry count (max)"
                name="unicastMaxRetryCount"
                tooltip="This defines how many times ChirpStack will retry unicast commands in case not acknowledged by the end-device."
                required
              >
                <InputNumber min={0} max={5} disabled={props.disabled} style={{ width: "100%" }} />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={8}>
              <Form.Item
                label="Multicast group-type"
                name="multicastGroupType"
                tooltip="The multicast-group type defines the way how multicast frames are scheduled by the network-server."
                rules={[{ required: true, message: "Please select a multicast group-type!" }]}
              >
                <Select onChange={onMulticastGroupTypeChange} disabled={props.disabled}>
                  <Select.Option value={MulticastGroupType.CLASS_C}>Class-C</Select.Option>
                  <Select.Option value={MulticastGroupType.CLASS_B}>Class-B</Select.Option>
                </Select>
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item label="Class-B ping-slot periodicity" name="multicastClassBPingSlotPeriodicity">
                <Select disabled={!isMulticastClassB || props.disabled}>
                  <Select.Option value={0}>Every second</Select.Option>
                  <Select.Option value={1}>Every 2 seconds</Select.Option>
                  <Select.Option value={2}>Every 4 seconds</Select.Option>
                  <Select.Option value={3}>Every 8 seconds</Select.Option>
                  <Select.Option value={4}>Every 16 seconds</Select.Option>
                  <Select.Option value={5}>Every 32 seconds</Select.Option>
                  <Select.Option value={6}>Every 64 seconds</Select.Option>
                  <Select.Option value={7}>Every 128 seconds</Select.Option>
                </Select>
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item
                label="Class-C scheduling type"
                name="multicastClassCSchedulingType"
                tooltip="In order to reach all devices, it might be needed to transmit a downlink through multiple gateways. In case of Delay each gateway will transmit one by one, in case of GPS Time all required gateways will transmit at the same GPS time."
              >
                <Select disabled={isMulticastClassB || props.disabled}>
                  <Select.Option value={MulticastGroupSchedulingType.DELAY}>Delay</Select.Option>
                  <Select.Option value={MulticastGroupSchedulingType.GPS_TIME}>GPS Time</Select.Option>
                </Select>
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={8}>
              <Form.Item
                label="Multicast data-rate"
                name="multicastDr"
                rules={[{ required: true, message: "Please enter a multicast data-rate!" }]}
                tooltip="The data-rate to use when transmitting the multicast frames. Please refer to the LoRaWAN Regional Parameters specification for valid values."
              >
                <InputNumber min={0} max={15} disabled={props.disabled} style={{ width: "100%" }} addonBefore="DR" />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item
                label="Multicast frequency (Hz)"
                name="multicastFrequency"
                tooltip="The frequency to use when transmitting the multicast frames. Please refer to the LoRaWAN Regional Parameters specification for valid values."
                rules={[{ required: true, message: "Please enter a frequency!" }]}
              >
                <InputNumber min={0} disabled={props.disabled} style={{ width: "100%" }} addonAfter="Hz" />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item label="Fragmentation redundancy (%)" name="fragmentationRedundancyPercentage">
                <InputNumber min={0} max={100} addonAfter="%" style={{ width: "100%" }} disabled={props.disabled} />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={8}>
              <Form.Item
                label="Fragmentation status request"
                name="requestFragmentationSessionStatus"
                tooltip="After fragment enqueue is recommended for Class-A devices, after session timeout is recommended for Class-B / Class-C devices."
              >
                <Select disabled={props.disabled}>
                  <Select.Option value={RequestFragmentationSessionStatus.NO_REQUEST}>Do not request</Select.Option>
                  <Select.Option value={RequestFragmentationSessionStatus.AFTER_FRAGMENT_ENQUEUE}>
                    After fragment enqueue
                  </Select.Option>
                  <Select.Option value={RequestFragmentationSessionStatus.AFTER_SESSION_TIMEOUT}>
                    After session timeout
                  </Select.Option>
                </Select>
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={6}>
              <Form.Item
                label="Calculate multicast-timeout"
                name="calculateMulticastTimeout"
                tooltip="If checked, ChirpStack will calculate the multicast-timeout."
              >
                <Switch onChange={(v: boolean) => setCalculateMulticastTimeout(v)} disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={6}>
              <Form.Item label="Multicast timeout" name="multicastTimeout">
                {isMulticastClassB && (
                  <Select disabled={props.disabled || calculateMulticastTimeout}>
                    <Select.Option value={0}>1 beacon period</Select.Option>
                    <Select.Option value={1}>2 beacon periods</Select.Option>
                    <Select.Option value={2}>4 beacon periods</Select.Option>
                    <Select.Option value={3}>8 beacon periods</Select.Option>
                    <Select.Option value={4}>16 beacon periods</Select.Option>
                    <Select.Option value={5}>32 beacon periods</Select.Option>
                    <Select.Option value={6}>64 beacon periods</Select.Option>
                    <Select.Option value={7}>128 beacon periods</Select.Option>
                    <Select.Option value={8}>256 beacon periods</Select.Option>
                    <Select.Option value={9}>512 beacon periods</Select.Option>
                    <Select.Option value={10}>1024 beacon periods</Select.Option>
                    <Select.Option value={11}>2048 beacon periods</Select.Option>
                    <Select.Option value={12}>4096 beacon periods</Select.Option>
                    <Select.Option value={13}>8192 beacon periods</Select.Option>
                    <Select.Option value={14}>16384 beacon periods</Select.Option>
                    <Select.Option value={15}>32768 beacon periods</Select.Option>
                  </Select>
                )}
                {!isMulticastClassB && (
                  <Select disabled={props.disabled || calculateMulticastTimeout}>
                    <Select.Option value={0}>1 second</Select.Option>
                    <Select.Option value={1}>2 seconds</Select.Option>
                    <Select.Option value={2}>4 seconds</Select.Option>
                    <Select.Option value={3}>8 seconds</Select.Option>
                    <Select.Option value={4}>16 seconds</Select.Option>
                    <Select.Option value={5}>32 seconds</Select.Option>
                    <Select.Option value={6}>64 seconds</Select.Option>
                    <Select.Option value={7}>128 seconds</Select.Option>
                    <Select.Option value={8}>256 seconds</Select.Option>
                    <Select.Option value={9}>512 seconds</Select.Option>
                    <Select.Option value={10}>1024 seconds</Select.Option>
                    <Select.Option value={11}>2048 seconds</Select.Option>
                    <Select.Option value={12}>4096 seconds</Select.Option>
                    <Select.Option value={13}>8192 seconds</Select.Option>
                    <Select.Option value={14}>16384 seconds</Select.Option>
                    <Select.Option value={15}>32768 seconds</Select.Option>
                  </Select>
                )}
              </Form.Item>
            </Col>
            <Col span={6}>
              <Form.Item
                label="Calculate fragment size"
                name="calculateFragmentationFragmentSize"
                tooltip="If checked, ChirpStack will calculate the fragment size for fragmentation."
              >
                <Switch onChange={(v: boolean) => setCalculateFragmentationFragmentSize(v)} disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={6}>
              <Form.Item label="Fragment size" name="fragmentationFragmentSize">
                <InputNumber
                  min={0}
                  max={255}
                  disabled={props.disabled || calculateFragmentationFragmentSize}
                  style={{ width: "100%" }}
                  addonAfter="Bytes"
                />
              </Form.Item>
            </Col>
          </Row>
          <Form.Item label="Payload" name="payload" required>
            <Upload
              beforeUpload={beforeUpload}
              onRemove={onRemoveUpload}
              maxCount={1}
              fileList={fileList}
              disabled={props.disabled}
            >
              <Button icon={<UploadOutlined />} disabled={props.disabled}>
                Click to upload
              </Button>
            </Upload>
          </Form.Item>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Set device tags (on complete)" key="2">
          <Form.List name="onCompleteSetDeviceTagsMap">
            {(fields, { add, remove }) => (
              <>
                {fields.map(({ key, name, ...restField }) => (
                  <Row gutter={24}>
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 0]}
                        fieldKey={[name, 0]}
                        rules={[{ required: true, message: "Please enter a key!" }]}
                      >
                        <Input placeholder="Key" />
                      </Form.Item>
                    </Col>
                    <Col span={16}>
                      <Form.Item
                        {...restField}
                        name={[name, 1]}
                        fieldKey={[name, 1]}
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
      </Tabs>
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default FuotaDeploymentForm;
