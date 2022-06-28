import React, { Component } from "react";

import { Form, Input, Select, InputNumber, Switch, Row, Col, Button, Tabs, Modal, Spin, Cascader, Card } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import {
  DeviceProfile,
  CodecRuntime,
  Measurement,
  MeasurementKind,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import { Region, MacVersion, RegParamsRevision } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { ListDeviceProfileAdrAlgorithmsResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  ListDeviceProfileTemplatesRequest,
  ListDeviceProfileTemplatesResponse,
  GetDeviceProfileTemplateRequest,
  GetDeviceProfileTemplateResponse,
  DeviceProfileTemplateListItem,
  DeviceProfileTemplate,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";

import { getEnumName } from "../helpers";
import DeviceProfileStore from "../../stores/DeviceProfileStore";
import DeviceProfileTemplateStore from "../../stores/DeviceProfileTemplateStore";
import CodeEditor from "../../components/CodeEditor";

interface ModalProps {
  onOk: (dp: DeviceProfileTemplate) => void;
  onCancel: () => void;
  visible: boolean;
}

interface ModalState {
  templates: DeviceProfileTemplateListItem[];
  templatesLoaded: boolean;
  templateId?: string;
}

interface Option {
  value: string;
  label: string;
  children?: Option[];
}

class TemplateModal extends Component<ModalProps, ModalState> {
  constructor(props: ModalProps) {
    super(props);
    this.state = {
      templates: [],
      templatesLoaded: false,
    };
  }

  componentDidUpdate(prevProps: ModalProps) {
    if (prevProps === this.props) {
      return;
    }

    if (this.props.visible) {
      this.setState({
        templatesLoaded: false,
      });

      let req = new ListDeviceProfileTemplatesRequest();
      req.setLimit(99999);

      DeviceProfileTemplateStore.list(req, (resp: ListDeviceProfileTemplatesResponse) => {
        this.setState({
          templatesLoaded: true,
          templates: resp.getResultList(),
        });
      });
    }
  }

  onChange = (value: (string | number)[]) => {
    this.setState({
      templateId: value.at(-1)! as string,
    });
  };

  onOk = () => {
    if (this.state.templateId) {
      let req = new GetDeviceProfileTemplateRequest();
      req.setId(this.state.templateId);

      DeviceProfileTemplateStore.get(req, (resp: GetDeviceProfileTemplateResponse) => {
        const dp = resp.getDeviceProfileTemplate();
        if (dp) {
          this.props.onOk(dp);
        }
      });
    }
  };

  render() {
    let options: Option[] = [];
    let vendor = "";
    let device = "";
    let firmware = "";
    let region = "";

    for (const item of this.state.templates) {
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
        visible={this.props.visible}
        width="80%"
        bodyStyle={{ height: 300 }}
        onOk={this.onOk}
        onCancel={this.props.onCancel}
        okButtonProps={{ disabled: !!!this.state.templateId }}
      >
        {!this.state.templatesLoaded && (
          <div className="spinner">
            <Spin />
          </div>
        )}
        {this.state.templatesLoaded && (
          <Cascader
            style={{ width: "100%" }}
            placeholder="Select a device-profile template"
            options={options}
            onChange={this.onChange}
          />
        )}
      </Modal>
    );
  }
}

interface IProps {
  initialValues: DeviceProfile;
  onFinish: (obj: DeviceProfile) => void;
  disabled?: boolean;
}

interface IState {
  supportsOtaa: boolean;
  supportsClassB: boolean;
  supportsClassC: boolean;
  payloadCodecRuntime: CodecRuntime;
  adrAlgorithms: [string, string][];
  templateModalVisible: boolean;
  tabActive: string;
}

class DeviceProfileForm extends Component<IProps, IState> {
  formRef = React.createRef<any>();

  constructor(props: IProps) {
    super(props);
    this.state = {
      supportsOtaa: false,
      supportsClassB: false,
      supportsClassC: false,
      payloadCodecRuntime: CodecRuntime.NONE,
      adrAlgorithms: [],
      templateModalVisible: false,
      tabActive: "1",
    };
  }

  componentDidMount() {
    const v = this.props.initialValues;

    this.setState({
      supportsOtaa: v.getSupportsOtaa(),
      supportsClassB: v.getSupportsClassB(),
      supportsClassC: v.getSupportsClassC(),
      payloadCodecRuntime: v.getPayloadCodecRuntime(),
    });

    DeviceProfileStore.listAdrAlgorithms((resp: ListDeviceProfileAdrAlgorithmsResponse) => {
      let adrAlgorithms: [string, string][] = [];
      for (const a of resp.getResultList()) {
        adrAlgorithms.push([a.getId(), a.getName()]);
      }

      this.setState({
        adrAlgorithms: adrAlgorithms,
      });
    });
  }

  onTabChange = (activeKey: string) => {
    this.setState({
      tabActive: activeKey,
    });
  };

  onFinish = (values: DeviceProfile.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);

    let dp = new DeviceProfile();
    dp.setId(v.id);
    dp.setTenantId(v.tenantId);

    dp.setName(v.name);
    dp.setDescription(v.description);
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

    this.props.onFinish(dp);
  };

  onSupportsOtaaChange = (checked: boolean) => {
    this.setState({
      supportsOtaa: checked,
    });
  };

  onSupportsClassBChnage = (checked: boolean) => {
    this.setState({
      supportsClassB: checked,
    });
  };

  onSupportsClassCChange = (checked: boolean) => {
    this.setState({
      supportsClassC: checked,
    });
  };

  onPayloadCodecRuntimeChange = (value: CodecRuntime) => {
    this.setState({
      payloadCodecRuntime: value,
    });
  };

  showTemplateModal = () => {
    this.setState({
      templateModalVisible: true,
    });
  };

  onTemplateModalOk = (dp: DeviceProfileTemplate) => {
    this.setState({
      templateModalVisible: false,
    });

    console.log(dp.toObject().tagsMap);

    this.formRef.current.setFieldsValue({
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
      abpRx1Delay: dp.getAbpRx1Delay(),
      abpRx2Dr: dp.getAbpRx2Dr(),
      abpRx2Freq: dp.getAbpRx2Freq(),
      abpRx1DrOffset: dp.getAbpRx1DrOffset(),
      tagsMap: dp.toObject().tagsMap,
      measurementsMap: dp.toObject().measurementsMap,
    });

    const tabActive = this.state.tabActive;

    this.setState(
      {
        supportsOtaa: dp.getSupportsOtaa(),
        supportsClassB: dp.getSupportsClassB(),
        supportsClassC: dp.getSupportsClassC(),
        payloadCodecRuntime: dp.getPayloadCodecRuntime(),
      },
      () => {
        // This is a workaround as without rendering the TabPane (e.g. the user
        // does not click through the different tabs), setFieldsValue does not
        // actually update the fields. For example if selecting a template with
        // a codec script and immediately click the save button, no codec script
        // is passed to the onFinish function. This seems to be with every field
        // that is not actually rendered before clicking the Save button.
        this.setState(
          {
            tabActive: "1",
          },
          () => {
            this.setState(
              {
                tabActive: "2",
              },
              () => {
                this.setState(
                  {
                    tabActive: "3",
                  },
                  () => {
                    this.setState(
                      {
                        tabActive: "4",
                      },
                      () => {
                        this.setState(
                          {
                            tabActive: "5",
                          },
                          () => {
                            this.setState(
                              {
                                tabActive: "6",
                              },
                              () => {
                                this.setState(
                                  {
                                    tabActive: "7",
                                  },
                                  () => {
                                    this.setState({
                                      tabActive: tabActive,
                                    });
                                  },
                                );
                              },
                            );
                          },
                        );
                      },
                    );
                  },
                );
              },
            );
          },
        );
      },
    );
  };

  onTemplateModalCancel = () => {
    this.setState({
      templateModalVisible: false,
    });
  };

  render() {
    const adrOptions = this.state.adrAlgorithms.map(v => <Select.Option value={v[0]}>{v[1]}</Select.Option>);
    const operations = (
      <Button type="primary" onClick={this.showTemplateModal}>
        Select device-profile template
      </Button>
    );

    return (
      <Form
        layout="vertical"
        initialValues={this.props.initialValues.toObject()}
        onFinish={this.onFinish}
        ref={this.formRef}
      >
        <TemplateModal
          visible={this.state.templateModalVisible}
          onOk={this.onTemplateModalOk}
          onCancel={this.onTemplateModalCancel}
        />
        <Tabs tabBarExtraContent={operations} activeKey={this.state.tabActive} onChange={this.onTabChange}>
          <Tabs.TabPane tab="General" key="1">
            <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
              <Input disabled={this.props.disabled} />
            </Form.Item>
            <Form.Item label="Description" name="description">
              <Input.TextArea rows={6} disabled={this.props.disabled} />
            </Form.Item>
            <Form.Item label="Region" name="region" rules={[{ required: true, message: "Please select a region!" }]}>
              <Select disabled={this.props.disabled}>
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
                  <Select disabled={this.props.disabled}>
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
                  rules={[{ required: true, message: "Please select a regional parameters revision!" }]}
                >
                  <Select disabled={this.props.disabled}>
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
              <Select disabled={this.props.disabled}>{adrOptions}</Select>
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
                  rules={[{ required: true, message: "Please enter an uplink interval!" }]}
                >
                  <InputNumber min={0} disabled={this.props.disabled} />
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item
                  label="Device-status request frequency (req/day)"
                  tooltip="Frequency to initiate an End-Device status request (request/day). Set to 0 to disable."
                  name="deviceStatusReqInterval"
                >
                  <InputNumber min={0} disabled={this.props.disabled} />
                </Form.Item>
              </Col>
            </Row>
          </Tabs.TabPane>
          <Tabs.TabPane tab="Join (OTAA / ABP)" key="2">
            <Form.Item label="Device supports OTAA" name="supportsOtaa" valuePropName="checked">
              <Switch onChange={this.onSupportsOtaaChange} disabled={this.props.disabled} />
            </Form.Item>
            {!this.state.supportsOtaa && (
              <Row>
                <Col span={12}>
                  <Form.Item
                    label="RX1 delay"
                    name="abpRx1Delay"
                    rules={[{ required: true, message: "Please enter a RX1 delay!" }]}
                  >
                    <InputNumber min={0} max={15} disabled={this.props.disabled} />
                  </Form.Item>
                </Col>
                <Col span={12}>
                  <Form.Item
                    label="RX1 data-rate offset"
                    tooltip="Please refer the LoRaWAN Regional Parameters specification for valid values."
                    name="abpRx1DrOffset"
                    rules={[{ required: true, message: "Please enter a RX1 data-rate offset!" }]}
                  >
                    <InputNumber min={0} max={15} disabled={this.props.disabled} />
                  </Form.Item>
                </Col>
              </Row>
            )}
            {!this.state.supportsOtaa && (
              <Row>
                <Col span={12}>
                  <Form.Item
                    label="RX2 data-rate"
                    tooltip="Please refer the LoRaWAN Regional Parameters specification for valid values."
                    name="abpRx2Dr"
                    rules={[{ required: true, message: "Please enter a RX2 data-rate!" }]}
                  >
                    <InputNumber min={0} max={15} disabled={this.props.disabled} />
                  </Form.Item>
                </Col>
                <Col span={12}>
                  <Form.Item
                    label="RX2 frequency (Hz)"
                    name="abpRx2Freq"
                    rules={[{ required: true, message: "Please enter a RX2 frequency!" }]}
                  >
                    <InputNumber min={0} style={{ width: "200px" }} disabled={this.props.disabled} />
                  </Form.Item>
                </Col>
              </Row>
            )}
          </Tabs.TabPane>
          <Tabs.TabPane tab="Class-B" key="3">
            <Form.Item label="Device supports Class-B" name="supportsClassB" valuePropName="checked">
              <Switch onChange={this.onSupportsClassBChnage} disabled={this.props.disabled} />
            </Form.Item>
            {this.state.supportsClassB && (
              <Form.Item
                label="Class-B confirmed downlink timeout (seconds)"
                tooltip="Class-B timeout (in seconds) for confirmed downlink transmissions."
                name="classBTimeout"
                rules={[{ required: true, message: "Please enter a Class-B confirmed downlink timeout!" }]}
              >
                <InputNumber min={0} />
              </Form.Item>
            )}
          </Tabs.TabPane>
          <Tabs.TabPane tab="Class-C" key="4">
            <Form.Item label="Device supports Class-C" name="supportsClassC" valuePropName="checked">
              <Switch onChange={this.onSupportsClassCChange} disabled={this.props.disabled} />
            </Form.Item>
            {this.state.supportsClassC && (
              <Form.Item
                label="Class-C confirmed downlink timeout (seconds)"
                tooltip="Class-C timeout (in seconds) for confirmed downlink transmissions."
                name="classCTimeout"
                rules={[{ required: true, message: "Please enter a Class-C confirmed downlink timeout!" }]}
              >
                <InputNumber min={0} disabled={this.props.disabled} />
              </Form.Item>
            )}
          </Tabs.TabPane>
          <Tabs.TabPane tab="Codec" key="5">
            <Form.Item
              label="Payload codec"
              name="payloadCodecRuntime"
              tooltip="By defining a payload codec, ChirpStack Application Server can encode and decode the binary device payload for you."
            >
              <Select onChange={this.onPayloadCodecRuntimeChange} disabled={this.props.disabled}>
                <Select.Option value={CodecRuntime.NONE}>None</Select.Option>
                <Select.Option value={CodecRuntime.CAYENNE_LPP}>Cayenne LPP</Select.Option>
                <Select.Option value={CodecRuntime.JS}>JavaScript functions</Select.Option>
              </Select>
            </Form.Item>
            {this.state.payloadCodecRuntime === CodecRuntime.JS && (
              <CodeEditor
                label="Codec functions"
                name="payloadCodecScript"
                value={this.formRef.current.getFieldValue("payloadCodecScript")}
                formRef={this.formRef}
                disabled={this.props.disabled}
              />
            )}
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
                          <Input placeholder="Key" disabled={this.props.disabled} />
                        </Form.Item>
                      </Col>
                      <Col span={16}>
                        <Form.Item
                          {...restField}
                          name={[name, 1]}
                          fieldKey={[name, 1]}
                          rules={[{ required: true, message: "Please enter a value!" }]}
                        >
                          <Input placeholder="Value" disabled={this.props.disabled} />
                        </Form.Item>
                      </Col>
                      <Col span={2}>
                        <MinusCircleOutlined onClick={() => remove(name)} />
                      </Col>
                    </Row>
                  ))}
                  <Form.Item>
                    <Button
                      disabled={this.props.disabled}
                      type="dashed"
                      onClick={() => add()}
                      block
                      icon={<PlusOutlined />}
                    >
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
                (user-defined). Please note that ChirpStack will automatically configure the keys once it has received
                the first uplink(s). The following measurement-kinds can be selected:
              </p>
              <ul>
                <li>
                  <strong>Unknown / unset</strong>: Default for auto-detected keys. This disables the aggregation of
                  this metric.
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
                          <Input placeholder="Measurement key" disabled={this.props.disabled} />
                        </Form.Item>
                      </Col>
                      <Col span={6}>
                        <Form.Item
                          {...restField}
                          name={[name, 1, "kind"]}
                          fieldKey={[name, 1, "kind"]}
                          rules={[{ required: true, message: "Please select a kind!" }]}
                        >
                          <Select disabled={this.props.disabled} placeholder="Measurement kind">
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
                          <Input placeholder="Measurement name" disabled={this.props.disabled} />
                        </Form.Item>
                      </Col>
                      <Col span={2}>
                        <MinusCircleOutlined onClick={() => remove(name)} />
                      </Col>
                    </Row>
                  ))}
                  <Form.Item>
                    <Button
                      disabled={this.props.disabled}
                      type="dashed"
                      onClick={() => add()}
                      block
                      icon={<PlusOutlined />}
                    >
                      Add measurement
                    </Button>
                  </Form.Item>
                </>
              )}
            </Form.List>
          </Tabs.TabPane>
        </Tabs>
        <Form.Item>
          <Button type="primary" htmlType="submit" disabled={this.props.disabled}>
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default DeviceProfileForm;
