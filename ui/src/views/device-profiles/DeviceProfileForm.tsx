import React, { useState, useEffect } from "react";

import { Form, Input, Select, InputNumber, Switch, Row, Col, Button, Tabs, Modal, Spin, Cascader, Card } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import {
  DeviceProfile,
  CodecRuntime,
  Measurement,
  MeasurementKind,
  CadPeriodicity,
  SecondChAckOffset,
  RelayModeActivation,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { Region, MacVersion, RegParamsRevision } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { ListRegionsResponse, RegionListItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { ListDeviceProfileAdrAlgorithmsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  ListDeviceProfileTemplatesRequest,
  ListDeviceProfileTemplatesResponse,
  GetDeviceProfileTemplateRequest,
  GetDeviceProfileTemplateResponse,
  DeviceProfileTemplateListItem,
  DeviceProfileTemplate,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";

import { getEnumName, onFinishFailed } from "../helpers";
import InternalStore from "../../stores/InternalStore";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import DeviceProfileTemplateStore from "../../stores/DeviceProfileTemplateStore";
import CodeEditor from "../../components/CodeEditor";

interface ModalProps {
  onOk: (dp: DeviceProfileTemplate) => void;
  onCancel: () => void;
  visible: boolean;
}

interface Option {
  value: string;
  label: string;
  children?: Option[];
}

function TemplateModal(props: ModalProps) {
  const [templates, setTemplates] = useState<DeviceProfileTemplateListItem[]>([]);
  const [templatesLoaded, setTemplatesLoaded] = useState<boolean>(false);
  const [templateId, setTemplateId] = useState<string | undefined>(undefined);

  useEffect(() => {
    if (props.visible) {
      setTemplatesLoaded(false);

      let req = new ListDeviceProfileTemplatesRequest();
      req.setLimit(99999);

      DeviceProfileTemplateStore.list(req, (resp: ListDeviceProfileTemplatesResponse) => {
        setTemplatesLoaded(true);
        setTemplates(resp.getResultList());
      });
    }
  }, [props]);

  const onChange = (value: (string | number)[]) => {
    setTemplateId(value.at(-1)! as string);
  };

  const onOk = () => {
    if (templateId) {
      let req = new GetDeviceProfileTemplateRequest();
      req.setId(templateId);

      DeviceProfileTemplateStore.get(req, (resp: GetDeviceProfileTemplateResponse) => {
        const dp = resp.getDeviceProfileTemplate();
        if (dp) {
          props.onOk(dp);
        }
      });
    }
  };

  let options: Option[] = [];
  let vendor = "";
  let device = "";
  let firmware = "";
  let region = "";

  for (const item of templates) {
    if (vendor !== item.getVendor()) {
      options.push({
        value: item.getId(),
        label: item.getVendor(),
        children: [],
      });

      vendor = item.getVendor();
      device = "";
      firmware = "";
      region = "";
    }

    if (device !== item.getName()) {
      options.at(-1)!.children!.push({
        value: item.getId(),
        label: item.getName(),
        children: [],
      });

      device = item.getName();
      firmware = "";
      region = "";
    }

    if (firmware !== item.getFirmware()) {
      options
        .at(-1)!
        .children!.at(-1)!
        .children!.push({
          value: item.getId(),
          label: "FW version: " + item.getFirmware(),
          children: [],
        });

      firmware = item.getFirmware();
      region = "";
    }

    if (region !== getEnumName(Region, item.getRegion())) {
      options
        .at(-1)!
        .children!.at(-1)!
        .children!.at(-1)!
        .children!.push({
          value: item.getId(),
          label: getEnumName(Region, item.getRegion()),
          children: [],
        });

      region = getEnumName(Region, item.getRegion());
    }
  }

  return (
    <Modal
      title="Select device-profile template"
      visible={props.visible}
      width="80%"
      bodyStyle={{ height: 300 }}
      onOk={onOk}
      onCancel={props.onCancel}
      okButtonProps={{ disabled: !!!templateId }}
    >
      {!templatesLoaded && (
        <div className="spinner">
          <Spin />
        </div>
      )}
      {templatesLoaded && (
        <Cascader
          style={{ width: "100%" }}
          placeholder="Select a device-profile template"
          options={options}
          onChange={onChange}
        />
      )}
    </Modal>
  );
}

interface IProps {
  initialValues: DeviceProfile;
  onFinish: (obj: DeviceProfile) => void;
  disabled?: boolean;
}

function DeviceProfileForm(props: IProps) {
  const [form] = Form.useForm();

  const [supportsOtaa, setSupportsOtaa] = useState<boolean>(false);
  const [supportsClassB, setSupportsClassB] = useState<boolean>(false);
  const [supportsClassC, setSupportsClassC] = useState<boolean>(false);
  const [isRelay, setIsRelay] = useState<boolean>(false);
  const [isRelayEd, setIsRelayEd] = useState<boolean>(false);
  const [payloadCodecRuntime, setPayloadCodecRuntime] = useState<CodecRuntime>(CodecRuntime.NONE);
  const [adrAlgorithms, setAdrAlgorithms] = useState<[string, string][]>([]);
  const [regionConfigurations, setRegionConfigurations] = useState<RegionListItem[]>([]);
  const [regionConfigurationsFiltered, setRegionConfigurationsFiltered] = useState<[string, string][]>([]);
  const [templateModalVisible, setTemplateModalVisible] = useState<boolean>(false);
  const [tabActive, setTabActive] = useState<string>("1");

  useEffect(() => {
    const v = props.initialValues;
    setSupportsOtaa(v.getSupportsOtaa());
    setSupportsClassB(v.getSupportsClassB());
    setSupportsClassC(v.getSupportsClassC());
    setPayloadCodecRuntime(v.getPayloadCodecRuntime());
    setIsRelay(v.getIsRelay());
    setIsRelayEd(v.getIsRelayEd());

    InternalStore.listRegions((resp: ListRegionsResponse) => {
      setRegionConfigurations(resp.getRegionsList());

      let regionConfigurationsFiltered: [string, string][] = [];
      for (const r of resp.getRegionsList()) {
        if (v.getRegion() === r.getRegion()) {
          regionConfigurationsFiltered.push([r.getId(), r.getDescription()]);
        }
      }

      setRegionConfigurationsFiltered(regionConfigurationsFiltered);
    });

    DeviceProfileStore.listAdrAlgorithms((resp: ListDeviceProfileAdrAlgorithmsResponse) => {
      let adrAlgorithms: [string, string][] = [];
      for (const a of resp.getResultList()) {
        adrAlgorithms.push([a.getId(), a.getName()]);
      }

      setAdrAlgorithms(adrAlgorithms);
    });
  }, [props]);

  const onTabChange = (activeKey: string) => {
    setTabActive(activeKey);
  };

  const onFinish = (values: DeviceProfile.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);

    let dp = new DeviceProfile();
    dp.setId(v.id);
    dp.setTenantId(v.tenantId);

    dp.setName(v.name);
    dp.setDescription(v.description);
    dp.setRegion(v.region);
    dp.setRegionConfigId(v.regionConfigId);
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

    // relay
    dp.setIsRelay(v.isRelay);
    dp.setIsRelayEd(v.isRelayEd);
    dp.setRelayEdRelayOnly(v.relayEdRelayOnly);
    dp.setRelayEnabled(v.relayEnabled);
    dp.setRelayCadPeriodicity(v.relayCadPeriodicity);
    dp.setRelayDefaultChannelIndex(v.relayDefaultChannelIndex);
    dp.setRelaySecondChannelFreq(v.relaySecondChannelFreq);
    dp.setRelaySecondChannelDr(v.relaySecondChannelDr);
    dp.setRelaySecondChannelAckOffset(v.relaySecondChannelAckOffset);
    dp.setRelayEdActivationMode(v.relayEdActivationMode);
    dp.setRelayEdSmartEnableLevel(v.relayEdSmartEnableLevel);
    dp.setRelayEdBackOff(v.relayEdBackOff);
    dp.setRelayEdUplinkLimitReloadRate(v.relayEdUplinkLimitReloadRate);
    dp.setRelayEdUplinkLimitBucketSize(v.relayEdUplinkLimitBucketSize);
    dp.setRelayJoinReqLimitReloadRate(v.relayJoinReqLimitReloadRate);
    dp.setRelayNotifyLimitReloadRate(v.relayNotifyLimitReloadRate);
    dp.setRelayGlobalUplinkLimitReloadRate(v.relayGlobalUplinkLimitReloadRate);
    dp.setRelayOverallLimitReloadRate(v.relayOverallLimitReloadRate);
    dp.setRelayJoinReqLimitBucketSize(v.relayJoinReqLimitBucketSize);
    dp.setRelayNotifyLimitBucketSize(v.relayNotifyLimitBucketSize);
    dp.setRelayGlobalUplinkLimitBucketSize(v.relayGlobalUplinkLimitBucketSize);
    dp.setRelayOverallLimitBucketSize(v.relayOverallLimitBucketSize);

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

  const onIsRelayChange = (checked: boolean) => {
    setIsRelay(checked);
  };

  const onIsRelayEdChange = (checked: boolean) => {
    setIsRelayEd(checked);
  };

  const showTemplateModal = () => {
    setTemplateModalVisible(true);
  };

  const onTemplateModalOk = (dp: DeviceProfileTemplate) => {
    setTemplateModalVisible(false);

    form.setFieldsValue({
      name: dp.getName(),
      description: dp.getDescription(),
      region: dp.getRegion(),
      macVersion: dp.getMacVersion(),
      regParamsRevision: dp.getRegParamsRevision(),
      adrAlgorithmId: dp.getAdrAlgorithmId(),
      payloadCodecRuntime: dp.getPayloadCodecRuntime(),
      payloadCodecScript: dp.getPayloadCodecScript(),
      flushQueueOnActivate: dp.getFlushQueueOnActivate(),
      uplinkInterval: dp.getUplinkInterval(),
      deviceStatusReqInterval: dp.getDeviceStatusReqInterval(),
      supportsOtaa: dp.getSupportsOtaa(),
      supportsClassB: dp.getSupportsClassB(),
      supportsClassC: dp.getSupportsClassC(),
      classCTimeout: dp.getClassCTimeout(),
      classBTimeout: dp.getClassBTimeout(),
      classBPingSlotNbK: dp.getClassBPingSlotNbK(),
      classBPingSlotDr: dp.getClassBPingSlotDr(),
      classBPingSlotFreq: dp.getClassBPingSlotFreq(),
      abpRx1Delay: dp.getAbpRx1Delay(),
      abpRx2Dr: dp.getAbpRx2Dr(),
      abpRx2Freq: dp.getAbpRx2Freq(),
      abpRx1DrOffset: dp.getAbpRx1DrOffset(),
      tagsMap: dp.toObject().tagsMap,
      measurementsMap: dp.toObject().measurementsMap,
    });

    setSupportsOtaa(dp.getSupportsOtaa());
    setSupportsClassB(dp.getSupportsClassB());
    setSupportsClassC(dp.getSupportsClassC());
    setPayloadCodecRuntime(dp.getPayloadCodecRuntime());
  };

  const onTemplateModalCancel = () => {
    setTemplateModalVisible(false);
  };

  const onRegionChange = (region: Region) => {
    let regionConfigurationsFiltered: [string, string][] = [];
    for (const r of regionConfigurations) {
      if (region === r.getRegion()) {
        regionConfigurationsFiltered.push([r.getId(), r.getDescription()]);
      }
    }

    setRegionConfigurationsFiltered(regionConfigurationsFiltered);
    form.setFieldsValue({
      regionConfigId: "",
    });
  };

  const adrOptions = adrAlgorithms.map(v => <Select.Option value={v[0]}>{v[1]}</Select.Option>);
  const regionConfigOptions = regionConfigurationsFiltered.map(v => <Select.Option value={v[0]}>{v[1]}</Select.Option>);
  const regionOptions = regionConfigurations
    .map(v => v.getRegion())
    .filter((v, i, a) => a.indexOf(v) === i)
    .map(v => <Select.Option value={v}>{getEnumName(Region, v).replace("_", "-")}</Select.Option>);
  const operations = (
    <Button type="primary" onClick={showTemplateModal}>
      Select device-profile template
    </Button>
  );

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed} form={form}>
      <TemplateModal visible={templateModalVisible} onOk={onTemplateModalOk} onCancel={onTemplateModalCancel} />
      <Tabs tabBarExtraContent={operations} activeKey={tabActive} onChange={onTabChange}>
        <Tabs.TabPane tab="General" key="1" forceRender>
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input disabled={props.disabled} />
          </Form.Item>
          <Form.Item label="Description" name="description">
            <Input.TextArea rows={6} disabled={props.disabled} />
          </Form.Item>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item label="Region" name="region" rules={[{ required: true, message: "Please select a region!" }]}>
                <Select disabled={props.disabled} onChange={onRegionChange}>
                  {regionOptions}
                </Select>
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item
                label="Region configuration"
                tooltip="By selecting a region configuration, the device will only work within the selected region configuration. If left blank, the device will work under all region configurations of the selected region."
                name="regionConfigId"
              >
                <Select disabled={props.disabled} allowClear>
                  {regionConfigOptions}
                </Select>
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="MAC version"
                tooltip="The LoRaWAN MAC version supported by the device."
                name="macVersion"
                rules={[{ required: true, message: "Please select a MAC version!" }]}
              >
                <Select disabled={props.disabled}>
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
                <Select disabled={props.disabled}>
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
            <Select disabled={props.disabled}>{adrOptions}</Select>
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
                <InputNumber min={0} disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item
                label="Device-status request frequency (req/day)"
                tooltip="Frequency to initiate an End-Device status request (request/day). Set to 0 to disable."
                name="deviceStatusReqInterval"
              >
                <InputNumber min={0} disabled={props.disabled} />
              </Form.Item>
            </Col>
          </Row>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Join (OTAA / ABP)" key="2" forceRender>
          <Form.Item label="Device supports OTAA" name="supportsOtaa" valuePropName="checked">
            <Switch onChange={onSupportsOtaaChange} disabled={props.disabled} />
          </Form.Item>
          {!supportsOtaa && (
            <Row>
              <Col span={12}>
                <Form.Item
                  label="RX1 delay"
                  name="abpRx1Delay"
                  tooltip="This value must match the RX1 delay of the device. Please refer to the device documentation."
                  rules={[{ required: true, message: "Please enter a RX1 delay!" }]}
                >
                  <InputNumber min={0} max={15} disabled={props.disabled} />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item
                  label="RX1 data-rate offset"
                  tooltip="This value must match the RX1 data-rate offset of the device. Please refer to the device documentation."
                  name="abpRx1DrOffset"
                  rules={[
                    {
                      required: true,
                      message: "Please enter a RX1 data-rate offset!",
                    },
                  ]}
                >
                  <InputNumber min={0} max={15} disabled={props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          )}
          {!supportsOtaa && (
            <Row>
              <Col span={12}>
                <Form.Item
                  label="RX2 data-rate"
                  tooltip="This value must match the RX2 data-rate of the device. Please refer to the device documentation."
                  name="abpRx2Dr"
                  rules={[
                    {
                      required: true,
                      message: "Please enter a RX2 data-rate!",
                    },
                  ]}
                >
                  <InputNumber min={0} max={15} disabled={props.disabled} />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item
                  label="RX2 frequency (Hz)"
                  name="abpRx2Freq"
                  tooltip="This value must match the RX2 frequency of the device. Please refer to the device documentation."
                  rules={[
                    {
                      required: true,
                      message: "Please enter a RX2 frequency!",
                    },
                  ]}
                >
                  <InputNumber min={0} style={{ width: "200px" }} disabled={props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          )}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Class-B" key="3" forceRender>
          <Form.Item label="Device supports Class-B" name="supportsClassB" valuePropName="checked">
            <Switch onChange={onSupportsClassBChnage} disabled={props.disabled} />
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
                    <Select disabled={props.disabled}>
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
                    <InputNumber min={0} disabled={props.disabled} />
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
                    <InputNumber min={0} style={{ width: "200px" }} disabled={props.disabled} />
                  </Form.Item>
                </Col>
              </Row>
            </>
          )}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Class-C" key="4" forceRender>
          <Form.Item label="Device supports Class-C" name="supportsClassC" valuePropName="checked">
            <Switch onChange={onSupportsClassCChange} disabled={props.disabled} />
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
              <InputNumber min={0} disabled={props.disabled} />
            </Form.Item>
          )}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Codec" key="5" forceRender>
          <Form.Item
            label="Payload codec"
            name="payloadCodecRuntime"
            tooltip="By defining a payload codec, ChirpStack can encode and decode the binary device payload for you."
          >
            <Select onChange={onPayloadCodecRuntimeChange} disabled={props.disabled}>
              <Select.Option value={CodecRuntime.NONE}>None</Select.Option>
              <Select.Option value={CodecRuntime.CAYENNE_LPP}>Cayenne LPP</Select.Option>
              <Select.Option value={CodecRuntime.JS}>JavaScript functions</Select.Option>
            </Select>
          </Form.Item>
          {payloadCodecRuntime === CodecRuntime.JS && (
            <CodeEditor label="Codec functions" name="payloadCodecScript" disabled={props.disabled} />
          )}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Relay" key="6" forceRender>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="Device is a Relay"
                name="isRelay"
                valuePropName="checked"
                tooltip="Enable this if the device(s) under this profile implement the Relay specification (please refer to the TS011 specification for more information)"
              >
                <Switch onChange={onIsRelayChange} disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={12}>
              {isRelay && (
                <Form.Item
                  label="Relay enabled"
                  name="relayEnabled"
                  valuePropName="checked"
                  tooltip="This will configure the Relay to be enabled / disabled through mac-commands."
                >
                  <Switch disabled={props.disabled} />
                </Form.Item>
              )}
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="Device is a Relay capable end-device"
                name="isRelayEd"
                valuePropName="checked"
                tooltip="Enable this of the device(s) under this profile are able to operate under a Relay as specified by the TS011 specification."
              >
                <Switch onChange={onIsRelayEdChange} disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={12}>
              {isRelayEd && (
                <Form.Item
                  label="Only use Relay (end-device)"
                  name="relayEdRelayOnly"
                  valuePropName="checked"
                  tooltip="If enabled, device(s) under this profile will only be able to communicate through a Relay device. Uplink messages received directly by ChirpStack will be discarded. Enabling this feature can be helpful for testing the Relay communication."
                >
                  <Switch disabled={props.disabled} />
                </Form.Item>
              )}
            </Col>
          </Row>
          {(isRelay || isRelayEd) && (
            <Row gutter={24}>
              <Col span={8}>
                <Form.Item
                  label="Default channel index"
                  name="relayDefaultChannelIndex"
                  tooltip="Please refer to the RP002 specification for the meaning of index 0 and 1."
                  rules={[
                    {
                      required: true,
                      message: "Please enter a channel number!",
                    },
                  ]}
                >
                  <InputNumber min={0} max={1} disabled={props.disabled} />
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item
                  label="Second channel frequency (Hz)"
                  name="relaySecondChannelFreq"
                  tooltip="To disable the second channel, set this value to 0."
                  rules={[{ required: true, message: "Please enter a frequency!" }]}
                >
                  <InputNumber min={0} style={{ width: "200px" }} disabled={props.disabled} />
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item
                  label="Second channel data-rate"
                  name="relaySecondChannelDr"
                  rules={[{ required: true, message: "Please enter a data-rate!" }]}
                >
                  <InputNumber min={0} max={15} disabled={props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          )}
          {(isRelay || isRelayEd) && (
            <Row gutter={24}>
              <Col span={12}>
                <Form.Item
                  label="Second channel ACK offset"
                  name="relaySecondChannelAckOffset"
                  rules={[{ required: true, message: "Please select an ACK offset!" }]}
                >
                  <Select disabled={props.disabled}>
                    <Select.Option value={SecondChAckOffset.KHZ_0}>0 kHz</Select.Option>
                    <Select.Option value={SecondChAckOffset.KHZ_200}>200 kHz</Select.Option>
                    <Select.Option value={SecondChAckOffset.KHZ_400}>400 kHz</Select.Option>
                    <Select.Option value={SecondChAckOffset.KHZ_800}>800 kHz</Select.Option>
                    <Select.Option value={SecondChAckOffset.KHZ_1600}>1600 kHz</Select.Option>
                    <Select.Option value={SecondChAckOffset.KHZ_3200}>3200 kHz</Select.Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={12}>
                {isRelay && (
                  <Form.Item
                    label="CAD periodicity"
                    name="relayCadPeriodicity"
                    rules={[
                      {
                        required: true,
                        message: "Please select a CAD periodicity!",
                      },
                    ]}
                  >
                    <Select disabled={props.disabled}>
                      <Select.Option value={CadPeriodicity.SEC_1}>1 second</Select.Option>
                      <Select.Option value={CadPeriodicity.MS_500}>500 milliseconds</Select.Option>
                      <Select.Option value={CadPeriodicity.MS_250}>250 milliseconds</Select.Option>
                      <Select.Option value={CadPeriodicity.MS_100}>100 milliseconds</Select.Option>
                      <Select.Option value={CadPeriodicity.MS_50}>50 milliseconds</Select.Option>
                      <Select.Option value={CadPeriodicity.MS_20}>20 milliseconds</Select.Option>
                    </Select>
                  </Form.Item>
                )}
              </Col>
            </Row>
          )}
          {isRelayEd && (
            <Row gutter={24}>
              <Col span={8}>
                <Form.Item
                  label="End-device activation mode"
                  name="relayEdActivationMode"
                  rules={[
                    {
                      required: true,
                      message: "Please select an activation mode!",
                    },
                  ]}
                >
                  <Select disabled={props.disabled}>
                    <Select.Option value={RelayModeActivation.DISABLE_RELAY_MODE}>Disable relay mode</Select.Option>
                    <Select.Option value={RelayModeActivation.ENABLE_RELAY_MODE}>Enable relay mode</Select.Option>
                    <Select.Option value={RelayModeActivation.DYNAMIC}>Dynamic</Select.Option>
                    <Select.Option value={RelayModeActivation.END_DEVICE_CONTROLLED}>
                      End-device controlled
                    </Select.Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item
                  label="Smart enable level"
                  name="relayEdSmartEnableLevel"
                  tooltip="This indicates that the relay mode shall be enabled if the end-device does not receive a valid downlink after X consecutive uplinks."
                  rules={[
                    {
                      required: true,
                      message: "Please select an enable level!",
                    },
                  ]}
                >
                  <Select disabled={props.disabled}>
                    <Select.Option value={0}>8</Select.Option>
                    <Select.Option value={1}>16</Select.Option>
                    <Select.Option value={2}>32</Select.Option>
                    <Select.Option value={3}>64</Select.Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item
                  label="Back-off"
                  name="relayEdBackOff"
                  tooltip="This indicates how the end-device SHALL behave when it does not receive a WOR ACK frame. 0 = Always send a LoRaWAN uplink. 1..63 = Send a LoRaWAN uplink after X WOR frames without a WOR ACK."
                >
                  <InputNumber min={0} max={63} disabled={props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          )}
          {isRelayEd && (
            <Row gutter={24}>
              <Col span={12}>
                <Form.Item
                  label="End-device uplink limit bucket size"
                  name="relayEdUplinkLimitBucketSize"
                  tooltip="Indicates the multiplier to determine the bucket size"
                >
                  <Select disabled={props.disabled}>
                    <Select.Option value={0}>1 x reload rate</Select.Option>
                    <Select.Option value={1}>2 x reload rate</Select.Option>
                    <Select.Option value={2}>4 x reload rate</Select.Option>
                    <Select.Option value={3}>12 x reload rate</Select.Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item
                  label="End-device uplink limit reload rate"
                  name="relayEdUplinkLimitReloadRate"
                  tooltip="0..62 = X tokens every hour, 63 = no limitation (forward all valid uplinks)"
                >
                  <InputNumber min={0} max={63} disabled={props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          )}
          {isRelay && (
            <Row gutter={24}>
              <Col span={6}>
                <Form.Item
                  label="Join-request limit bucket size"
                  name="relayJoinReqLimitBucketSize"
                  tooltip="Indicates the multiplier to determine the bucket size"
                >
                  <Select disabled={props.disabled}>
                    <Select.Option value={0}>1 x reload rate</Select.Option>
                    <Select.Option value={1}>2 x reload rate</Select.Option>
                    <Select.Option value={2}>4 x reload rate</Select.Option>
                    <Select.Option value={3}>12 x reload rate</Select.Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={6}>
                <Form.Item
                  label="Join-request limit reload rate"
                  name="relayJoinReqLimitReloadRate"
                  tooltip="0..126 = X tokens every hour, 127 = no limitation"
                >
                  <InputNumber min={0} max={127} disabled={props.disabled} />
                </Form.Item>
              </Col>
              <Col span={6}>
                <Form.Item
                  label="Notify limit bucket size"
                  name="relayNotifyLimitBucketSize"
                  tooltip="Indicates the multiplier to determine the bucket size"
                >
                  <Select disabled={props.disabled}>
                    <Select.Option value={0}>1 x reload rate</Select.Option>
                    <Select.Option value={1}>2 x reload rate</Select.Option>
                    <Select.Option value={2}>4 x reload rate</Select.Option>
                    <Select.Option value={3}>12 x reload rate</Select.Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={6}>
                <Form.Item
                  label="Notify limit reload rate"
                  name="relayNotifyLimitReloadRate"
                  tooltip="0..126 = X tokens every hour, 127 = no limitation"
                >
                  <InputNumber min={0} max={127} disabled={props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          )}
          {isRelay && (
            <Row gutter={24}>
              <Col span={6}>
                <Form.Item
                  label="Global uplink limit bucket size"
                  name="relayGlobalUplinkLimitBucketSize"
                  tooltip="Indicates the multiplier to determine the bucket size"
                >
                  <Select disabled={props.disabled}>
                    <Select.Option value={0}>1 x reload rate</Select.Option>
                    <Select.Option value={1}>2 x reload rate</Select.Option>
                    <Select.Option value={2}>4 x reload rate</Select.Option>
                    <Select.Option value={3}>12 x reload rate</Select.Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={6}>
                <Form.Item
                  label="Global uplink limit reload rate"
                  name="relayGlobalUplinkLimitReloadRate"
                  tooltip="0..126 = X tokens every hour, 127 = no limitation"
                >
                  <InputNumber min={0} max={127} disabled={props.disabled} />
                </Form.Item>
              </Col>
              <Col span={6}>
                <Form.Item
                  label="Overall limit bucket size"
                  name="relayOverallLimitBucketSize"
                  tooltip="Indicates the multiplier to determine the bucket size"
                >
                  <Select disabled={props.disabled}>
                    <Select.Option value={0}>1 x reload rate</Select.Option>
                    <Select.Option value={1}>2 x reload rate</Select.Option>
                    <Select.Option value={2}>4 x reload rate</Select.Option>
                    <Select.Option value={3}>12 x reload rate</Select.Option>
                  </Select>
                </Form.Item>
              </Col>
              <Col span={6}>
                <Form.Item
                  label="Overall limit reload rate"
                  name="relayOverallLimitReloadRate"
                  tooltip="0..126 = X tokens every hour, 127 = no limitation"
                >
                  <InputNumber min={0} max={127} disabled={props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          )}
        </Tabs.TabPane>
        <Tabs.TabPane tab="Tags" key="7" forceRender>
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
                        <Input placeholder="Key" disabled={props.disabled} />
                      </Form.Item>
                    </Col>
                    <Col span={16}>
                      <Form.Item
                        {...restField}
                        name={[name, 1]}
                        fieldKey={[name, 1]}
                        rules={[{ required: true, message: "Please enter a value!" }]}
                      >
                        <Input placeholder="Value" disabled={props.disabled} />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button disabled={props.disabled} type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    Add tag
                  </Button>
                </Form.Item>
              </>
            )}
          </Form.List>
        </Tabs.TabPane>
        <Tabs.TabPane tab="Measurements" key="8" forceRender>
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
                        <Input placeholder="Measurement key" disabled={props.disabled} />
                      </Form.Item>
                    </Col>
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 1, "kind"]}
                        fieldKey={[name, 1, "kind"]}
                        rules={[{ required: true, message: "Please select a kind!" }]}
                      >
                        <Select disabled={props.disabled} placeholder="Measurement kind">
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
                        rules={[{ required: true, message: "Please enter a name!" }]}
                      >
                        <Input placeholder="Measurement name" disabled={props.disabled} />
                      </Form.Item>
                    </Col>
                    <Col span={2}>
                      <MinusCircleOutlined onClick={() => remove(name)} />
                    </Col>
                  </Row>
                ))}
                <Form.Item>
                  <Button disabled={props.disabled} type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                    Add measurement
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

export default DeviceProfileForm;
