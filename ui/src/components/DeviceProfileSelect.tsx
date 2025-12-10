import React, { useState, useEffect } from "react";

import { Cascader, Form } from "antd";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
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
import DeviceProfileStore from "../stores/DeviceProfileStore";
import { getEnumName } from "../views/helpers";

interface IProps {
  label: string;
  name: string;
  required?: boolean;
  tenant: Tenant;
  value?: string;
  disabled?: boolean;
  tooltip?: string;
}

interface CascaderOption {
  type: string;
  value: string;
  label: React.ReactNode;
  children?: CascaderOption[];
  isLeaf?: boolean;
}

function DeviceProfileSelect(props: IProps) {
  const form = Form.useFormInstance();
  const [dpOptions, setDpOptions] = useState<CascaderOption[]>([
    { label: "Tenant", value: "tenant", isLeaf: false, type: "init" },
    { label: "Vendor", value: "vendor", isLeaf: false, type: "init" },
  ]);

  useEffect(() => {
    if (props.value && props.value !== "") {
      // Get device-profile
      const req = new GetDeviceProfileRequest();
      req.setId(props.value);
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
                  form.setFieldValue(props.name, ["vendor", dev.getVendorId(), dp.getDeviceId(), dp.getId()]);
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

          form.setFieldValue(props.name, ["tenant", dp.getId()]);
        }
      });
    }
  }, [props.name, props.tenant, form]);

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

  return (
    <Form.Item
      rules={[
        {
          required: props.required,
          message: `Please select a ${props.label}`,
        },
      ]}
      label={props.label}
      name={props.name}
      tooltip={props.tooltip}
    >
      <Cascader options={dpOptions} loadData={loadDeviceProfileData} disabled={props.disabled} />
    </Form.Item>
  );
}

export default DeviceProfileSelect;
