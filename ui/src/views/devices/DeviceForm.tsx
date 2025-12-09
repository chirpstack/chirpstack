import { useState, useEffect } from "react";
import { Form, Input, Row, Col, Button, Tabs, Switch, Cascader } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import type {
  ListDeviceProfilesResponse,
  ListDeviceProfileVendorsResponse,
  ListDeviceProfileDevicesResponse,
  GetDeviceProfileResponse,
  GetDeviceProfileDeviceResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";
import {
  ListDeviceProfilesRequest,
  ListDeviceProfileVendorsRequest,
  ListDeviceProfileDevicesRequest,
  GetDeviceProfileRequest,
  GetDeviceProfileDeviceRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import { onFinishFailed, getEnumName } from "../helpers";
import EuiInput from "../../components/EuiInput";
import DeviceProfileStore from "../../stores/DeviceProfileStore";

interface IProps {
  tenant: Tenant;
  initialValues: Device;
  onFinish: (obj: Device) => void;
  update?: boolean;
}

interface CascaderOption {
  type: string;
  value: string;
  label: React.ReactNode;
  children?: CascaderOption[];
  isLeaf?: boolean;
}

function DeviceForm(props: IProps) {
  const [form] = Form.useForm();
  const [dpOptions, setDpOptions] = useState<CascaderOption[]>([
    { label: "Tenant", value: "tenant", isLeaf: false, type: "init" },
    { label: "Vendor", value: "vendor", isLeaf: false, type: "init" },
  ]);
  const [dpId, setDpId] = useState<string>("");

  useEffect(() => {
    if (props.initialValues.getDeviceProfileId() !== "") {
      // Get device-profile
      const req = new GetDeviceProfileRequest();
      req.setId(props.initialValues.getDeviceProfileId());
      DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
        const dp = resp.getDeviceProfile();
        if (dp === undefined) {
          return;
        }

        if (dp.getDeviceId() !== "") {
          // Get device-profile device
          const req = new GetDeviceProfileDeviceRequest();
          req.setId(dp.getDeviceId());
          DeviceProfileStore.getDevice(req, (resp: GetDeviceProfileDeviceResponse) => {
            const dev = resp.getDevice();
            if (dev === undefined) {
              return;
            }

            // Get all device-profile devices for this tenant
            const req = new ListDeviceProfileDevicesRequest();
            req.setVendorId(dev.getVendorId());
            req.setLimit(9999);
            DeviceProfileStore.listDevices(req, (resp: ListDeviceProfileDevicesResponse) => {
              const devices = resp.getResultList();

              // Get all device-profile vendors
              const req = new ListDeviceProfileVendorsRequest();
              req.setLimit(9999);
              DeviceProfileStore.listVendors(req, (resp: ListDeviceProfileVendorsResponse) => {
                const vendors = resp.getResultList();

                // Get all device-profiles (for the device)
                const req = new ListDeviceProfilesRequest();
                req.setDeviceId(dp.getDeviceId());
                req.setLimit(9999);
                req.setGlobalOnly(true);

                DeviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
                  const profiles = resp.getResultList();

                  dpOptions[1].children = vendors.map(v => {
                    if (v.getId() === dev.getVendorId()) {
                      return {
                        type: "vendor",
                        label: v.getName(),
                        value: v.getId(),
                        isLeaf: false,
                        children: devices.map(d => {
                          if (d.getId() === dp.getDeviceId()) {
                            return {
                              type: "device",
                              label: d.getName(),
                              value: d.getId(),
                              isLeaf: false,
                              children: profiles.map(p => {
                                return {
                                  type: "profile",
                                  label: `${p.getName()} (region: ${getEnumName(Region, p.getRegion())}, firmware: ${p.getFirmwareVersion()})`,
                                  value: p.getId(),
                                  isLeaf: true,
                                };
                              }),
                            };
                          } else {
                            return {
                              type: "device",
                              label: d.getName(),
                              value: d.getId(),
                              isLeaf: false,
                            };
                          }
                        }),
                      };
                    } else {
                      return {
                        type: "vendor",
                        label: v.getName(),
                        value: v.getId(),
                        isLeaf: false,
                      };
                    }
                  });

                  setDpOptions([...dpOptions]);
                  form.setFieldValue("deviceProfileIdCascader", [
                    "vendor",
                    dev.getVendorId(),
                    dp.getDeviceId(),
                    dp.getId(),
                  ]);
                });
              });
            });
          });
        } else if (dp !== undefined) {
          const req = new ListDeviceProfilesRequest();
          req.setTenantId(props.tenant.getId());
          req.setLimit(9999);
          req.setTenantOnly(true);

          DeviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
            dpOptions[0].children = resp.getResultList().map(v => {
              return {
                type: "profile",
                label: v.getName(),
                value: v.getId(),
                isLeaf: true,
              };
            });

            setDpOptions([...dpOptions]);
          });

          form.setFieldValue("deviceProfileIdCascader", ["tenant", dp.getId()]);
        }
      });
    }
  }, [props.initialValues, props.tenant, form]);

  const onFinish = (values: Device.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    const d = new Device();

    d.setApplicationId(v.applicationId);
    d.setName(v.name);
    d.setDescription(v.description);
    d.setDevEui(v.devEui);
    d.setDeviceProfileId(dpId);
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

  const loadDeviceProfileData = (selectedOptions: CascaderOption[]) => {
    if (selectedOptions.length === 0) {
      return;
    }

    const targetOption = selectedOptions[selectedOptions.length - 1];

    if (targetOption.type === "init" && targetOption.value === "vendor") {
      const req = new ListDeviceProfileVendorsRequest();
      req.setLimit(9999);

      DeviceProfileStore.listVendors(req, (resp: ListDeviceProfileVendorsResponse) => {
        targetOption.children = resp.getResultList().map(v => {
          return {
            type: "vendor",
            label: v.getName(),
            value: v.getId(),
            isLeaf: false,
          };
        });

        setDpOptions([...dpOptions]);
      });
    } else if (targetOption.type === "init" && targetOption.value === "tenant") {
      const req = new ListDeviceProfilesRequest();
      req.setTenantId(props.tenant.getId());
      req.setTenantOnly(true);
      req.setLimit(9999);

      DeviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
        targetOption.children = resp.getResultList().map(v => {
          return {
            type: "profile",
            label: v.getName(),
            value: v.getId(),
            isLeaf: true,
          };
        });

        setDpOptions([...dpOptions]);
      });
    } else if (targetOption.type === "vendor") {
      const req = new ListDeviceProfileDevicesRequest();
      req.setLimit(9999);
      if (typeof targetOption.value === "string") {
        req.setVendorId(targetOption.value);
      }

      DeviceProfileStore.listDevices(req, (resp: ListDeviceProfileDevicesResponse) => {
        targetOption.children = resp.getResultList().map(v => {
          return {
            type: "device",
            label: v.getName(),
            value: v.getId(),
            isLeaf: false,
          };
        });

        setDpOptions([...dpOptions]);
      });
    } else if (targetOption.type === "device") {
      const req = new ListDeviceProfilesRequest();
      req.setLimit(9999);
      req.setGlobalOnly(true);

      if (typeof targetOption.value === "string") {
        req.setDeviceId(targetOption.value);
      }

      DeviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
        targetOption.children = resp.getResultList().map(v => {
          return {
            type: "profile",
            label: `${v.getName()} (region: ${getEnumName(Region, v.getRegion())}, firmware: ${v.getFirmwareVersion()})`,
            value: v.getId(),
            isLeaf: true,
          };
        });

        setDpOptions([...dpOptions]);
      });
    }
  };

  const onDeviceProfileChange = (_: any, selectedOptions: CascaderOption[]) => {
    setDpId(selectedOptions[selectedOptions.length - 1].value);
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
        <Tabs.TabPane tab="Device" key="1">
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input />
          </Form.Item>
          <Form.Item label="Description" name="description">
            <Input.TextArea />
          </Form.Item>
          <Row gutter={24}>
            <Col span={12}>
              <EuiInput
                label="Device EUI (EUI64)"
                name="devEui"
                value={props.initialValues.getDevEui()}
                disabled={props.update}
                required
              />
            </Col>
            <Col span={12}>
              <EuiInput
                label="Join EUI (EUI64)"
                name="joinEui"
                value={props.initialValues.getJoinEui()}
                tooltip="The Join EUI will be automatically set / updated on OTAA. However, in some cases this field must be configured before OTAA (e.g. OTAA using a Relay)."
              />
            </Col>
          </Row>
          <Form.Item label="Device profile" name="deviceProfileIdCascader">
            <Cascader options={dpOptions} onChange={onDeviceProfileChange} loadData={loadDeviceProfileData} />
          </Form.Item>
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
