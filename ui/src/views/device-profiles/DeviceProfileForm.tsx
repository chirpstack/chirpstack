import { useState, useEffect } from "react";

import { Form, Input, Select, InputNumber, Switch, Row, Col, Button, Tabs, Card } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";
import type { TabsProps, SelectProps } from "antd/lib";

import {
  DeviceProfile,
  AppLayerParams,
  CodecRuntime,
  Measurement,
  MeasurementKind,
  CadPeriodicity,
  SecondChAckOffset,
  RelayModeActivation,
  Ts003Version,
  Ts004Version,
  Ts005Version,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { Region, MacVersion, RegParamsRevision } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import type { ListRegionsResponse, RegionListItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import type { ListDeviceProfileAdrAlgorithmsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import { getEnumName, onFinishFailed } from "../helpers";
import InternalStore from "../../stores/InternalStore";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import CodeEditor from "../../components/CodeEditor";

interface IProps {
  initialValues: DeviceProfile;
  onFinish: (obj: DeviceProfile) => void;
  disabled?: boolean;
}

const dataRates: SelectProps["options"] = [
  { label: "DR0", value: 0 },
  { label: "DR1", value: 1 },
  { label: "DR2", value: 2 },
  { label: "DR3", value: 3 },
  { label: "DR4", value: 4 },
  { label: "DR5", value: 5 },
  { label: "DR6", value: 6 },
  { label: "DR7", value: 7 },
  { label: "DR8", value: 8 },
  { label: "DR9", value: 9 },
  { label: "DR10", value: 10 },
  { label: "DR11", value: 11 },
  { label: "DR12", value: 12 },
  { label: "DR13", value: 13 },
  { label: "DR14", value: 14 },
  { label: "DR15", value: 15 },
];

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

      const regionConfigurationsFiltered: [string, string][] = [];
      for (const r of resp.getRegionsList()) {
        if (v.getRegion() === r.getRegion()) {
          regionConfigurationsFiltered.push([r.getId(), r.getDescription()]);
        }
      }

      setRegionConfigurationsFiltered(regionConfigurationsFiltered);
    });

    DeviceProfileStore.listAdrAlgorithms((resp: ListDeviceProfileAdrAlgorithmsResponse) => {
      const adrAlgorithms: [string, string][] = [];
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

    const dp = new DeviceProfile();
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
    dp.setRx1Delay(v.rx1Delay);
    dp.setSupportedUplinkDataRatesList(v.supportedUplinkDataRatesList);

    // join otaa /abp
    dp.setSupportsOtaa(v.supportsOtaa);
    dp.setAbpRx1Delay(v.abpRx1Delay);
    dp.setAbpRx1DrOffset(v.abpRx1DrOffset);
    dp.setAbpRx2Dr(v.abpRx2Dr);
    dp.setAbpRx2Freq(v.abpRx2Freq);

    // class-b
    dp.setSupportsClassB(v.supportsClassB);
    dp.setClassBTimeout(v.classBTimeout);
    dp.setClassBPingSlotPeriodicity(v.classBPingSlotPeriodicity);
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

    // roaming
    dp.setAllowRoaming(v.allowRoaming);

    // tags
    for (const elm of v.tagsMap) {
      dp.getTagsMap().set(elm[0], elm[1]);
    }

    // measurements
    for (const elm of v.measurementsMap) {
      const m = new Measurement();
      m.setKind(elm[1].kind);
      m.setName(elm[1].name);
      dp.getMeasurementsMap().set(elm[0], m);
    }
    dp.setAutoDetectMeasurements(v.autoDetectMeasurements);

    // App layer
    const appLayer = new AppLayerParams();
    if (v.appLayerParams) {
      appLayer.setTs003Version(v.appLayerParams.ts003Version);
      appLayer.setTs004Version(v.appLayerParams.ts004Version);
      appLayer.setTs005Version(v.appLayerParams.ts005Version);

      appLayer.setTs003FPort(v.appLayerParams.ts003FPort);
      appLayer.setTs004FPort(v.appLayerParams.ts004FPort);
      appLayer.setTs005FPort(v.appLayerParams.ts005FPort);
    }
    dp.setAppLayerParams(appLayer);

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

  const onRegionChange = (region: Region) => {
    const regionConfigurationsFiltered: [string, string][] = [];
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

  const adrOptions: SelectProps["options"] = adrAlgorithms.map(v => {
    return {
      label: v[1],
      value: v[0],
    };
  });
  const regionConfigOptions: SelectProps["options"] = regionConfigurationsFiltered.map(v => {
    return {
      label: v[1],
      value: v[0],
    };
  });
  const regionOptions: SelectProps["options"] = regionConfigurations
    .map(v => v.getRegion())
    .filter((v, i, a) => a.indexOf(v) === i)
    .map(v => {
      return {
        label: getEnumName(Region, v).replace("_", "-"),
        value: v,
      };
    });

  const tabItems: TabsProps["items"] = [
    {
      key: "1",
      label: "General",
      children: (
        <>
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input disabled={props.disabled} />
          </Form.Item>
          <Form.Item label="Description" name="description">
            <Input.TextArea rows={6} disabled={props.disabled} />
          </Form.Item>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item label="Region" name="region" rules={[{ required: true, message: "Please select a region!" }]}>
                <Select disabled={props.disabled} onChange={onRegionChange} options={regionOptions} />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item
                label="Region configuration"
                tooltip="By selecting a region configuration, the device will only work within the selected region configuration. If left blank, the device will work under all region configurations of the selected region."
                name="regionConfigId"
              >
                <Select disabled={props.disabled} allowClear options={regionConfigOptions} />
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
                <Select
                  disabled={props.disabled}
                  options={[
                    { value: MacVersion.LORAWAN_1_0_0, label: "LoRaWAN 1.0.0" },
                    { value: MacVersion.LORAWAN_1_0_1, label: "LoRaWAN 1.0.1" },
                    { value: MacVersion.LORAWAN_1_0_2, label: "LoRaWAN 1.0.2" },
                    { value: MacVersion.LORAWAN_1_0_3, label: "LoRaWAN 1.0.3" },
                    { value: MacVersion.LORAWAN_1_0_4, label: "LoRaWAN 1.0.4" },
                    { value: MacVersion.LORAWAN_1_1_0, label: "LoRaWAN 1.1.0" },
                  ]}
                />
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
                <Select
                  disabled={props.disabled}
                  options={[
                    { value: RegParamsRevision.A, label: "A" },
                    { value: RegParamsRevision.B, label: "B" },
                    { value: RegParamsRevision.RP002_1_0_0, label: "RP002-1.0.0" },
                    { value: RegParamsRevision.RP002_1_0_1, label: "RP002-1.0.1" },
                    { value: RegParamsRevision.RP002_1_0_2, label: "RP002-1.0.2" },
                    { value: RegParamsRevision.RP002_1_0_3, label: "RP002-1.0.3" },
                    { value: RegParamsRevision.RP002_1_0_4, label: "RP002-1.0.4" },
                    { value: RegParamsRevision.RP002_1_0_5, label: "RP002-1.0.5" },
                  ]}
                />
              </Form.Item>
            </Col>
          </Row>
          <Form.Item
            label="ADR algorithm"
            tooltip="The ADR algorithm that will be used for controlling the device data-rate."
            name="adrAlgorithmId"
            rules={[{ required: true, message: "Please select an ADR algorithm!" }]}
          >
            <Select disabled={props.disabled} options={adrOptions} />
          </Form.Item>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="Flush queue on activate"
                name="flushQueueOnActivate"
                valuePropName="checked"
                tooltip="If enabled, the device-queue will be flushed on ABP or OTAA activation."
              >
                <Switch />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item
                label="Allow roaming"
                name="allowRoaming"
                valuePropName="checked"
                tooltip="If enabled (and if roaming is configured on the server), this allows the device to use roaming."
              >
                <Switch />
              </Form.Item>
            </Col>
          </Row>
          <Row>
            <Col span={12}>
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
            <Col span={12}>
              <Form.Item
                label="Device-status request frequency (req/day)"
                tooltip="Frequency to initiate an End-Device status request (request/day). Set to 0 to disable."
                name="deviceStatusReqInterval"
              >
                <InputNumber min={0} disabled={props.disabled} />
              </Form.Item>
            </Col>
          </Row>
          <Row>
            <Col span={12}>
              <Form.Item
                label="RX1 Delay (0 = use system default)"
                tooltip="This option makes it possible to set a higher RX1 Delay for devices using this device-profile. Note that a lower value than the system default will be ignored. If configured and incremented, then ChirpStack will increase the downlink data delay with the same increment."
                name="rx1Delay"
              >
                <InputNumber min={0} max={15} disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item
                label="Supported data-rates"
                tooltip="If not set, the default region data-rates will be used (which is most likely what you want). Only set this value if you know what you are doing."
                name="supportedUplinkDataRatesList"
              >
                <Select mode="multiple" options={dataRates} disabled={props.disabled} allowClear />
              </Form.Item>
            </Col>
          </Row>
        </>
      ),
    },
    {
      key: "2",
      label: "Join (OTAA / ABP)",
      children: (
        <>
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
        </>
      ),
    },
    {
      key: "3",
      label: "Class B",
      children: (
        <>
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
                    name="classBPingSlotPeriodicity"
                    rules={[
                      {
                        required: true,
                        message: "Please select the ping-slot periodicity!",
                      },
                    ]}
                  >
                    <Select
                      disabled={props.disabled}
                      options={[
                        { value: 0, label: "Every second" },
                        { value: 1, label: "Every 2 seconds" },
                        { value: 2, label: "Every 4 seconds" },
                        { value: 3, label: "Every 8 seconds" },
                        { value: 4, label: "Every 16 seconds" },
                        { value: 5, label: "Every 32 seconds" },
                        { value: 6, label: "Every 64 seconds" },
                        { value: 7, label: "Every 128 seconds" },
                      ]}
                    ></Select>
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
        </>
      ),
    },
    {
      key: "4",
      label: "Class-C",
      children: (
        <>
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
        </>
      ),
    },
    {
      key: "5",
      label: "Payload codec",
      children: (
        <>
          <Form.Item
            label="Payload codec"
            name="payloadCodecRuntime"
            tooltip="By defining a payload codec, ChirpStack can encode and decode the binary device payload for you."
          >
            <Select
              onChange={onPayloadCodecRuntimeChange}
              disabled={props.disabled}
              options={[
                { value: CodecRuntime.NONE, label: "None" },
                { value: CodecRuntime.CAYENNE_LPP, label: "Cayenne LPP" },
                { value: CodecRuntime.JS, label: "JavaScript functions" },
              ]}
            />
          </Form.Item>
          {payloadCodecRuntime === CodecRuntime.JS && (
            <CodeEditor label="Codec functions" name="payloadCodecScript" disabled={props.disabled} />
          )}
        </>
      ),
    },
    {
      key: "6",
      label: "Relay",
      children: (
        <>
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
                  <Select
                    disabled={props.disabled}
                    options={[
                      { value: SecondChAckOffset.KHZ_0, label: "0 kHz" },
                      { value: SecondChAckOffset.KHZ_200, label: "200 kHz" },
                      { value: SecondChAckOffset.KHZ_400, label: "400 kHz" },
                      { value: SecondChAckOffset.KHZ_800, label: "800 kHz" },
                      { value: SecondChAckOffset.KHZ_1600, label: "1600 kHz" },
                      { value: SecondChAckOffset.KHZ_3200, label: "3200 kHz" },
                    ]}
                  />
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
                    <Select
                      disabled={props.disabled}
                      options={[
                        { value: CadPeriodicity.SEC_1, label: "1 second" },
                        { value: CadPeriodicity.MS_500, label: "500 milliseconds" },
                        { value: CadPeriodicity.MS_250, label: "250 milliseconds" },
                        { value: CadPeriodicity.MS_100, label: "100 milliseconds" },
                        { value: CadPeriodicity.MS_50, label: "50 milliseconds" },
                        { value: CadPeriodicity.MS_20, label: "20 milliseconds" },
                      ]}
                    />
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
                  <Select
                    disabled={props.disabled}
                    options={[
                      { value: RelayModeActivation.DISABLE_RELAY_MODE, label: "Disable relay mode" },
                      { value: RelayModeActivation.ENABLE_RELAY_MODE, label: "Enable relay mode" },
                      { value: RelayModeActivation.DYNAMIC, label: "Dynamic" },
                      { value: RelayModeActivation.END_DEVICE_CONTROLLED, label: "End-device controlled" },
                    ]}
                  />
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
                  <Select
                    disabled={props.disabled}
                    options={[
                      { value: 0, label: "8" },
                      { value: 1, label: "16" },
                      { value: 2, label: "32" },
                      { value: 3, label: "64" },
                    ]}
                  />
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
                  <Select
                    disabled={props.disabled}
                    options={[
                      { value: 0, label: "1 x reload rate" },
                      { value: 1, label: "2 x reload rate" },
                      { value: 2, label: "4 x reload rate" },
                      { value: 3, label: "12 x reload rate" },
                    ]}
                  />
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
                  <Select
                    disabled={props.disabled}
                    options={[
                      { value: 0, label: "1 x reload rate" },
                      { value: 1, label: "2 x reload rate" },
                      { value: 2, label: "4 x reload rate" },
                      { value: 3, label: "12 x reload rate" },
                    ]}
                  />
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
                  <Select
                    disabled={props.disabled}
                    options={[
                      { value: 0, label: "1 x reload rate" },
                      { value: 1, label: "2 x reload rate" },
                      { value: 2, label: "4 x reload rate" },
                      { value: 3, label: "12 x reload rate" },
                    ]}
                  />
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
                  <Select
                    disabled={props.disabled}
                    options={[
                      { value: 0, label: "1 x reload rate" },
                      { value: 1, label: "2 x reload rate" },
                      { value: 2, label: "4 x reload rate" },
                      { value: 3, label: "12 x reload rate" },
                    ]}
                  />
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
                  <Select
                    disabled={props.disabled}
                    options={[
                      { value: 0, label: "1 x reload rate" },
                      { value: 1, label: "2 x reload rate" },
                      { value: 2, label: "4 x reload rate" },
                      { value: 3, label: "12 x reload rate" },
                    ]}
                  />
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
        </>
      ),
    },
    {
      key: "7",
      label: "Application layer",
      children: (
        <>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="Clock sync version (TS003)"
                name={["appLayerParams", "ts003Version"]}
                tooltip="If an implemented version is selected, ChirpStack will handle payloads received on the matching fPort"
              >
                <Select
                  disabled={props.disabled}
                  options={[
                    { value: Ts003Version.TS003_NOT_IMPLEMENTED, label: "Not implemented" },
                    { value: Ts003Version.TS003_V100, label: "v1.0.0" },
                    { value: Ts003Version.TS003_V200, label: "v2.0.0" },
                  ]}
                />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item label="Clock sync fPort (TS003)" name={["appLayerParams", "ts003FPort"]}>
                <InputNumber min={0} max={255} disabled={props.disabled} />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="Fragmented data block transport (TS004)"
                name={["appLayerParams", "ts004Version"]}
                tooltip="If an implemented version is selected, ChirpStack will handle payloads received on the matching fPort"
              >
                <Select
                  disabled={props.disabled}
                  options={[
                    { value: Ts004Version.TS004_NOT_IMPLEMENTED, label: "Not implemented" },
                    { value: Ts004Version.TS004_V100, label: "v1.0.0" },
                    { value: Ts004Version.TS004_V200, label: "v2.0.0" },
                  ]}
                />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item label="Fragmented data block transport fPort (TS004)" name={["appLayerParams", "ts004FPort"]}>
                <InputNumber min={0} max={255} disabled={props.disabled} />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={24}>
            <Col span={12}>
              <Form.Item
                label="Remote multicast setup version (TS005)"
                name={["appLayerParams", "ts005Version"]}
                tooltip="If an implemented version is selected, ChirpStack will handle payloads received on the matching fPort"
              >
                <Select
                  disabled={props.disabled}
                  options={[
                    { value: Ts005Version.TS005_NOT_IMPLEMENTED, label: "Not implemented" },
                    { value: Ts005Version.TS005_V100, label: "v1.0.0" },
                    { value: Ts005Version.TS005_V200, label: "v2.0.0" },
                  ]}
                />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item label="Remote multicast setup fPort (TS005)" name={["appLayerParams", "ts005FPort"]}>
                <InputNumber min={0} max={255} disabled={props.disabled} />
              </Form.Item>
            </Col>
          </Row>
        </>
      ),
    },
    {
      key: "8",
      label: "Tags",
      children: (
        <Form.List name="tagsMap">
          {(fields, { add, remove }) => (
            <>
              {fields.map(({ key, name, ...restField }) => (
                <Row gutter={24} key={key}>
                  <Col span={6}>
                    <Form.Item
                      {...restField}
                      name={[name, 0]}
                      rules={[{ required: true, message: "Please enter a key!" }]}
                    >
                      <Input placeholder="Key" disabled={props.disabled} />
                    </Form.Item>
                  </Col>
                  <Col span={16}>
                    <Form.Item
                      {...restField}
                      name={[name, 1]}
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
      ),
    },
    {
      key: "9",
      label: "Measurements",
      children: (
        <>
          <Card variant="borderless">
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
                  <Row gutter={24} key={key}>
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 0]}
                        rules={[{ required: true, message: "Please enter a key!" }]}
                      >
                        <Input placeholder="Measurement key" disabled={props.disabled} />
                      </Form.Item>
                    </Col>
                    <Col span={6}>
                      <Form.Item
                        {...restField}
                        name={[name, 1, "kind"]}
                        rules={[{ required: true, message: "Please select a kind!" }]}
                      >
                        <Select
                          disabled={props.disabled}
                          placeholder="Measurement kind"
                          options={[
                            { value: MeasurementKind.UNKNOWN, label: "Unknown / unset" },
                            { value: MeasurementKind.COUNTER, label: "Counter" },
                            { value: MeasurementKind.ABSOLUTE, label: "Absolute" },
                            { value: MeasurementKind.GAUGE, label: "Gauge" },
                            { value: MeasurementKind.STRING, label: "String" },
                          ]}
                        />
                      </Form.Item>
                    </Col>
                    <Col span={10}>
                      <Form.Item {...restField} name={[name, 1, "name"]}>
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
        </>
      ),
    },
  ];

  return (
    <Form
      layout="vertical"
      initialValues={props.initialValues.toObject()}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
      form={form}
    >
      <Tabs activeKey={tabActive} onChange={onTabChange} items={tabItems} />
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default DeviceProfileForm;
