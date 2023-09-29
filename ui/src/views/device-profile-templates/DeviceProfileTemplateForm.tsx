import React, { useState, useEffect } from "react";

import { Form, Input, Select, InputNumber, Switch, Row, Col, Button, Tabs, Card } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import { DeviceProfileTemplate } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";
import { CodecRuntime, Measurement, MeasurementKind } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { Region, MacVersion, RegParamsRevision } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { ListDeviceProfileAdrAlgorithmsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import { onFinishFailed } from "../helpers";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import CodeEditor from "../../components/CodeEditor";

interface IProps {
  initialValues: DeviceProfileTemplate;
  onFinish: (obj: DeviceProfileTemplate) => void;
  update?: boolean;
}

function DeviceProfileTemplateForm(props: IProps) {
  const [form] = Form.useForm();
  const [supportsOtaa, setSupportsOtaa] = useState<boolean>(false);
  const [supportsClassB, setSupportsClassB] = useState<boolean>(false);
  const [supportsClassC, setSupportsClassC] = useState<boolean>(false);
  const [payloadCodecRuntime, setPayloadCodecRuntime] = useState<CodecRuntime>(CodecRuntime.NONE);
  const [adrAlgorithms, setAdrAlgorithms] = useState<[string, string][]>([]);

  useEffect(() => {
    const v = props.initialValues;
    setSupportsOtaa(v.getSupportsOtaa());
    setSupportsClassB(v.getSupportsClassB());
    setSupportsClassC(v.getSupportsClassC());
    setPayloadCodecRuntime(v.getPayloadCodecRuntime());

    DeviceProfileStore.listAdrAlgorithms((resp: ListDeviceProfileAdrAlgorithmsResponse) => {
      let adrAlgorithms: [string, string][] = [];
      for (const a of resp.getResultList()) {
        adrAlgorithms.push([a.getId(), a.getName()]);
      }

      setAdrAlgorithms(adrAlgorithms);
    });
  }, [props.initialValues]);

  const onFinish = (values: DeviceProfileTemplate.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let dp = new DeviceProfileTemplate();
    dp.setId(v.id);

    dp.setName(v.name);
    dp.setDescription(v.description);
    dp.setVendor(v.vendor);
    dp.setFirmware(v.firmware);
    dp.setRegion(v.region);
    dp.setMacVersion(v.macVersion);
    dp.setRegParamsRevision(v.regParamsRevision);
    dp.setAdrAlgorithmId(v.adrAlgorithmId);
    dp.setFlushQueueOnActivate(v.flushQueueOnActivate);
    dp.setUplinkInterval(v.uplinkInterval);
    dp.setDeviceStatusReqInterval(v.deviceStatusReqInterval);

    // join otaa /abp
    dp.setSupportsOtaa(v.supportsOtaa);
    dp.setAbpRx1Delay(v.abpRx1Delay);
    dp.setAbpRx1DrOffset(v.abpRx1DrOffset);
    dp.setAbpRx2Dr(v.abpRx2Dr);
    dp.setAbpRx2Freq(v.abpRx2Freq);

    // class-b
    dp.setSupportsClassB(v.supportsClassB);
    dp.setClassBTimeout(v.classBTimeout);
    dp.setClassBPingSlotNbK(v.classBPingSlotNbK);
    dp.setClassBPingSlotDr(v.classBPingSlotDr);
    dp.setClassBPingSlotFreq(v.classBPingSlotFreq);

    // class-c
    dp.setSupportsClassC(v.supportsClassC);
    dp.setClassCTimeout(v.classCTimeout);

    // codec
    dp.setPayloadCodecRuntime(v.payloadCodecRuntime);
    dp.setPayloadCodecScript(v.payloadCodecScript);

    // tags
    for (const elm of v.tagsMap) {
      dp.getTagsMap().set(elm[0], elm[1]);
    }

    // measurements
    for (const elm of v.measurementsMap) {
      let m = new Measurement();
      m.setKind(elm[1].kind);
      m.setName(elm[1].name);
      dp.getMeasurementsMap().set(elm[0], m);
    }
    dp.setAutoDetectMeasurements(v.autoDetectMeasurements);

    props.onFinish(dp);
  };

  const onSupportsOtaaChange = (checked: boolean) => {
    setSupportsOtaa(checked);
  };

  const onSupportsClassBChnage = (checked: boolean) => {
    setSupportsClassB(checked);
  };

  const onSupportsClassCChange = (checked: boolean) => {
    setSupportsClassC(checked);
  };

  const onPayloadCodecRuntimeChange = (value: CodecRuntime) => {
    setPayloadCodecRuntime(value);
  };

  const adrOptions = adrAlgorithms.map(v => <Select.Option value={v[0]}>{v[1]}</Select.Option>);

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed} form={form}>
      <Tabs>
        <Tabs.TabPane tab="General" key="1">
          <Form.Item
            label="ID"
            name="id"
            rules={[
              {
                required: true,
                pattern: new RegExp(/^[\w-]*$/g),
                message: "Please enter a valid id!",
              },
            ]}
          >
            <Input disabled={!!props.update} />
          </Form.Item>
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input />
          </Form.Item>
          <Form.Item label="Vendor" name="vendor" rules={[{ required: true, message: "Please enter a vendor!" }]}>
            <Input />
          </Form.Item>
          <Form.Item
            label="Firmware version"
            name="firmware"
            rules={[{ required: true, message: "Please enter a firmware version!" }]}
          >
            <Input />
          </Form.Item>
          <Form.Item label="Description" name="description">
            <Input.TextArea rows={6} />
          </Form.Item>
          <Form.Item label="Region" name="region" rules={[{ required: true, message: "Please select a region!" }]}>
            <Select>
              <Select.Option value={Region.AS923}>AS923</Select.Option>
              <Select.Option value={Region.AS923_2}>AS923-2</Select.Option>
              <Select.Option value={Region.AS923_3}>AS923-3</Select.Option>
              <Select.Option value={Region.AS923_4}>AS923-4</Select.Option>
              <Select.Option value={Region.AU915}>AU915</Select.Option>
              <Select.Option value={Region.CN779}>CN779</Select.Option>
              <Select.Option value={Region.EU433}>EU433</Select.Option>
              <Select.Option value={Region.EU868}>EU868</Select.Option>
              <Select.Option value={Region.IN865}>IN865</Select.Option>
              <Select.Option value={Region.ISM2400}>ISM2400</Select.Option>
              <Select.Option value={Region.KR920}>KR920</Select.Option>
              <Select.Option value={Region.RU864}>RU864</Select.Option>
              <Select.Option value={Region.US915}>US915</Select.Option>
            </Select>
          </Form.Item>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="MAC version"
                tooltip="The LoRaWAN MAC version supported by the device."
                name="macVersion"
                rules={[{ required: true, message: "Please select a MAC version!" }]}
              >
                <Select>
                  <Select.Option value={MacVersion.LORAWAN_1_0_0}>LoRaWAN 1.0.0</Select.Option>
                  <Select.Option value={MacVersion.LORAWAN_1_0_1}>LoRaWAN 1.0.1</Select.Option>
                  <Select.Option value={MacVersion.LORAWAN_1_0_2}>LoRaWAN 1.0.2</Select.Option>
                  <Select.Option value={MacVersion.LORAWAN_1_0_3}>LoRaWAN 1.0.3</Select.Option>
                  <Select.Option value={MacVersion.LORAWAN_1_0_4}>LoRaWAN 1.0.4</Select.Option>
                  <Select.Option value={MacVersion.LORAWAN_1_1_0}>LoRaWAN 1.1.0</Select.Option>
                </Select>
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item
                label="Regional parameters revision"
                tooltip="Revision of the Regional Parameters specification supported by the device."
                name="regParamsRevision"
                rules={[
                  {
                    required: true,
                    message: "Please select a regional parameters revision!",
                  },
                ]}
              >
                <Select>
                  <Select.Option value={RegParamsRevision.A}>A</Select.Option>
                  <Select.Option value={RegParamsRevision.B}>B</Select.Option>
                  <Select.Option value={RegParamsRevision.RP002_1_0_0}>RP002-1.0.0</Select.Option>
                  <Select.Option value={RegParamsRevision.RP002_1_0_1}>RP002-1.0.1</Select.Option>
                  <Select.Option value={RegParamsRevision.RP002_1_0_2}>RP002-1.0.2</Select.Option>
                  <Select.Option value={RegParamsRevision.RP002_1_0_3}>RP002-1.0.3</Select.Option>
                </Select>
              </Form.Item>
            </Col>
          </Row>
          <Form.Item
            label="ADR algorithm"
            tooltip="The ADR algorithm that will be used for controlling the device data-rate."
            name="adrAlgorithmId"
            rules={[{ required: true, message: "Please select an ADR algorithm!" }]}
          >
            <Select>{adrOptions}</Select>
          </Form.Item>
          <Row gutter={24}>
            <Col span={8}>
              <Form.Item
                label="Flush queue on activate"
                name="flushQueueOnActivate"
                valuePropName="checked"
                tooltip="If enabled, the device-queue will be flushed on ABP or OTAA activation."
              >
                <Switch />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item
                label="Expected uplink interval (secs)"
                tooltip="The expected interval in seconds in which the device sends uplink messages. This is used to determine if a device is active or inactive."
                name="uplinkInterval"
                rules={[
                  {
                    required: true,
                    message: "Please enter an uplink interval!",
                  },
                ]}
              >
                <InputNumber min={0} />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item
                label="Device-status request frequency (req/day)"
                tooltip="Frequency to initiate an End-Device status request (request/day). Set to 0 to disable."
                name="deviceStatusReqInterval"
              >
                <InputNumber min={0} />
              </Form.Item>
            </Col>
          </Row>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Join (OTAA / ABP)" key="2">
          <Form.Item label="Device supports OTAA" name="supportsOtaa" valuePropName="checked">
            <Switch onChange={onSupportsOtaaChange} />
          </Form.Item>
          {!supportsOtaa && (
            <Row>
              <Col span={12}>
                <Form.Item
                  label="RX1 delay"
                  name="abpRx1Delay"
                  rules={[{ required: true, message: "Please enter a RX1 delay!" }]}
                >
                  <InputNumber min={0} max={15} />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item
                  label="RX1 data-rate offset"
                  tooltip="Please refer the LoRaWAN Regional Parameters specification for valid values."
                  name="abpRx1DrOffset"
                  rules={[
                    {
                      required: true,
                      message: "Please enter a RX1 data-rate offset!",
                    },
                  ]}
                >
                  <InputNumber min={0} max={15} />
                </Form.Item>
              </Col>
            </Row>
          )}
          {!supportsOtaa && (
            <Row>
              <Col span={12}>
                <Form.Item
                  label="RX2 data-rate"
                  tooltip="Please refer the LoRaWAN Regional Parameters specification for valid values."
                  name="abpRx2Dr"
                  rules={[
                    {
                      required: true,
                      message: "Please enter a RX2 data-rate!",
                    },
                  ]}
                >
                  <InputNumber min={0} max={15} />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item
                  label="RX2 frequency (Hz)"
                  name="abpRx2Freq"
                  rules={[
                    {
                      required: true,
                      message: "Please enter a RX2 frequency!",
                    },
                  ]}
                >
                  <InputNumber min={0} style={{ width: "200px" }} />
                </Form.Item>
              </Col>
            </Row>
          )}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Class-B" key="3">
          <Form.Item label="Device supports Class-B" name="supportsClassB" valuePropName="checked">
            <Switch onChange={onSupportsClassBChnage} />
          </Form.Item>
          {supportsClassB && (
            <>
              <Row gutter={24}>
                <Col span={12}>
                  <Form.Item
                    label="Class-B confirmed downlink timeout (seconds)"
                    tooltip="Class-B timeout (in seconds) for confirmed downlink transmissions."
                    name="classBTimeout"
                    rules={[
                      {
                        required: true,
                        message: "Please enter a Class-B confirmed downlink timeout!",
                      },
                    ]}
                  >
                    <InputNumber min={0} />
                  </Form.Item>
                </Col>
                <Col span={12}>
                  <Form.Item
                    label="Class-B ping-slot periodicity"
                    tooltip="This value must match the ping-slot periodicity of the device. Please refer to the device documentation."
                    name="classBPingSlotNbK"
                    rules={[
                      {
                        required: true,
                        message: "Please select the ping-slot periodicity!",
                      },
                    ]}
                  >
                    <Select>
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
              </Row>
              <Row gutter={24}>
                <Col span={12}>
                  <Form.Item
                    label="Class-B ping-slot data-rate"
                    tooltip="This value must match the ping-slot data-rate of the device. Please refer to the device documentation."
                    name="classBPingSlotDr"
                    rules={[
                      {
                        required: true,
                        message: "Please enter the ping-slot data-rate!",
                      },
                    ]}
                  >
                    <InputNumber min={0} />
                  </Form.Item>
                </Col>
                <Col span={12}>
                  <Form.Item
                    label="Class-B ping-slot frequency (Hz)"
                    tooltip="This value must match the ping-slot frequency of the device. Please refer to the device documentation."
                    name="classBPingSlotFreq"
                    rules={[
                      {
                        required: true,
                        message: "Please enter the ping-slot frequency!",
                      },
                    ]}
                  >
                    <InputNumber min={0} style={{ width: "200px" }} />
                  </Form.Item>
                </Col>
              </Row>
            </>
          )}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Class-C" key="4">
          <Form.Item label="Device supports Class-C" name="supportsClassC" valuePropName="checked">
            <Switch onChange={onSupportsClassCChange} />
          </Form.Item>
          {supportsClassC && (
            <Form.Item
              label="Class-C confirmed downlink timeout (seconds)"
              tooltip="Class-C timeout (in seconds) for confirmed downlink transmissions."
              name="classCTimeout"
              rules={[
                {
                  required: true,
                  message: "Please enter a Class-C confirmed downlink timeout!",
                },
              ]}
            >
              <InputNumber min={0} />
            </Form.Item>
          )}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Codec" key="5">
          <Form.Item
            label="Payload codec"
            name="payloadCodecRuntime"
            tooltip="By defining a payload codec, ChirpStack Application Server can encode and decode the binary device payload for you."
          >
            <Select onChange={onPayloadCodecRuntimeChange}>
              <Select.Option value={CodecRuntime.NONE}>None</Select.Option>
              <Select.Option value={CodecRuntime.CAYENNE_LPP}>Cayenne LPP</Select.Option>
              <Select.Option value={CodecRuntime.JS}>JavaScript functions</Select.Option>
            </Select>
          </Form.Item>
          {payloadCodecRuntime === CodecRuntime.JS && <CodeEditor label="Codec functions" name="payloadCodecScript" />}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Tags" key="6">
          <Form.List name="tagsMap">
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
        <Tabs.TabPane tab="Measurements" key="7">
          <Card bordered={false}>
            <p>
              ChirpStack can aggregate and visualize decoded device measurements in the device dashboard. To setup the
              aggregation of device measurements, you must configure the key, kind of measurement and name
              (user-defined). The following measurement-kinds can be selected:
            </p>
            <ul>
              <li>
                <strong>Unknown / unset</strong>: Default for auto-detected keys. This disables the aggregation of this
                metric.
              </li>
              <li>
                <strong>Counter</strong>: For continuous incrementing counters.
              </li>
              <li>
                <strong>Absolute</strong>: For counters which get reset upon reading / uplink.
              </li>
              <li>
                <strong>Gauge</strong>: For temperature, humidity, pressure etc...
              </li>
              <li>
                <strong>String</strong>: For boolean or string values.
              </li>
            </ul>
          </Card>
          <Form.Item
            label="Automatically detect measurement keys"
            name="autoDetectMeasurements"
            valuePropName="checked"
            tooltip="If enabled, measurement-keys will be automatically added based on the decoded payload keys. If the decoded payload contains random keys, you want to disable auto-detection."
          >
            <Switch />
          </Form.Item>
          <Form.List name="measurementsMap">
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
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 1, "kind"]}
                        fieldKey={[name, 1, "kind"]}
                        rules={[{ required: true, message: "Please select a kind!" }]}
                      >
                        <Select>
                          <Select.Option value={MeasurementKind.UNKNOWN}>Unknown / unset</Select.Option>
                          <Select.Option value={MeasurementKind.COUNTER}>Counter</Select.Option>
                          <Select.Option value={MeasurementKind.ABSOLUTE}>Absolute</Select.Option>
                          <Select.Option value={MeasurementKind.GAUGE}>Gauge</Select.Option>
                          <Select.Option value={MeasurementKind.STRING}>String</Select.Option>
                        </Select>
                      </Form.Item>
                    </Col>
                    <Col span={10}>
                      <Form.Item
                        {...restField}
                        name={[name, 1, "name"]}
                        fieldKey={[name, 1, "name"]}
                        rules={[
                          {
                            required: true,
                            message: "Please enter a description!",
                          },
                        ]}
                      >
                        <Input placeholder="Name" />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    Add measurement
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

export default DeviceProfileTemplateForm;
