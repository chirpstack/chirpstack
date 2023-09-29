import React, { useState, useEffect } from "react";

import { Form, Input, InputNumber, Switch, Button, Tabs, Collapse } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import {
  LoraCloudIntegration,
  LoraCloudModemGeolocationServices,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import { onFinishFailed } from "../../helpers";

interface IProps {
  initialValues: LoraCloudIntegration;
  onFinish: (obj: LoraCloudIntegration) => void;
}

function LoRaCloudIntegrationForm(props: IProps) {
  const [modemEnabled, setModemEnabled] = useState<boolean>(false);
  const [geolocationTdoa, setGeolocationTdoa] = useState<boolean>(false);
  const [geolocationRssi, setGeolocationRssi] = useState<boolean>(false);
  const [geolocationWifi, setGeolocationWifi] = useState<boolean>(false);
  const [geolocationGnss, setGeolocationGnss] = useState<boolean>(false);

  useEffect(() => {
    const v = props.initialValues;
    const mgs = v.getModemGeolocationServices();

    if (mgs !== undefined) {
      setModemEnabled(mgs.getModemEnabled());
      setGeolocationTdoa(mgs.getGeolocationTdoa());
      setGeolocationRssi(mgs.getGeolocationRssi());
      setGeolocationWifi(mgs.getGeolocationWifi());
      setGeolocationGnss(mgs.getGeolocationGnss());
    }
  }, [props]);

  const onFinish = (values: LoraCloudIntegration.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    const mgsv = v.modemGeolocationServices;

    let mgs = new LoraCloudModemGeolocationServices();

    if (mgsv !== undefined) {
      mgs.setToken(mgsv.token);
      mgs.setModemEnabled(mgsv.modemEnabled);
      mgs.setForwardFPortsList(mgsv.forwardFPortsList);
      mgs.setGnssUseRxTime(mgsv.gnssUseRxTime);
      mgs.setGnssUseGatewayLocation(mgsv.gnssUseGatewayLocation);
      mgs.setParseTlv(mgsv.parseTlv);
      mgs.setGeolocationBufferTtl(mgsv.geolocationBufferTtl);
      mgs.setGeolocationMinBufferSize(mgsv.geolocationMinBufferSize);
      mgs.setGeolocationTdoa(mgsv.geolocationTdoa);
      mgs.setGeolocationRssi(mgsv.geolocationRssi);
      mgs.setGeolocationGnss(mgsv.geolocationGnss);
      mgs.setGeolocationGnssPayloadField(mgsv.geolocationGnssPayloadField);
      mgs.setGeolocationWifi(mgsv.geolocationWifi);
      mgs.setGeolocationWifiPayloadField(mgsv.geolocationWifiPayloadField);
      mgs.setGeolocationGnssUseRxTime(mgsv.geolocationGnssUseRxTime);
    }

    let i = new LoraCloudIntegration();
    i.setApplicationId(v.applicationId);
    i.setModemGeolocationServices(mgs);

    props.onFinish(i);
  };

  const onModemEnabledChange = (v: boolean) => {
    setModemEnabled(v);
  };

  const onGeolocationTdoaChange = (v: boolean) => {
    setGeolocationTdoa(v);
  };

  const onGeolocationRssiChange = (v: boolean) => {
    setGeolocationRssi(v);
  };

  const onGeolocationWifiChange = (v: boolean) => {
    setGeolocationWifi(v);
  };

  const onGeolocationGnssChange = (v: boolean) => {
    setGeolocationGnss(v);
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed}>
      <Tabs>
        <Tabs.TabPane tab="Modem & Geolocation Services" key="1">
          <Form.Item
            label="Token"
            name={["modemGeolocationServices", "token"]}
            tooltip="This token can be obtained from loracloud.com"
            rules={[{ required: true, message: "Please enter a token!" }]}
          >
            <Input type="password" />
          </Form.Item>
          <Form.Item
            name={["modemGeolocationServices", "modemEnabled"]}
            label="I am using LoRa Edge&trade; LR1110 or my device uses LoRa Basics™ Modem-E"
            valuePropName="checked"
          >
            <Switch onChange={onModemEnabledChange} />
          </Form.Item>
          {modemEnabled && (
            <Form.List name={["modemGeolocationServices", "forwardFPortsList"]}>
              {(fields, { add, remove }) => (
                <Form.Item label="Forward messages on these FPorts to LoRa Cloud">
                  {fields.map((field, index) => (
                    <Form.Item
                      {...field}
                      rules={[{ required: true, message: "Please a FPort value!" }]}
                      style={{
                        display: "inline-block",
                        width: "100px",
                        marginRight: "24px",
                      }}
                    >
                      <InputNumber
                        min={1}
                        max={255}
                        addonAfter={<MinusCircleOutlined onClick={() => remove(index)} />}
                      />
                    </Form.Item>
                  ))}
                  <Button type="dashed" onClick={() => add()} icon={<PlusOutlined />} />
                </Form.Item>
              )}
            </Form.List>
          )}
          {modemEnabled && (
            <Form.Item
              label="Use receive timestamp for GNSS geolocation"
              name={["modemGeolocationServices", "gnssUseRxTime"]}
              tooltip="If enabled, the receive timestamp of the gateway will be used as reference instead of the timestamp included in the GNSS payload."
              valuePropName="checked"
            >
              <Switch />
            </Form.Item>
          )}
          {modemEnabled && (
            <Form.Item
              label="Use location of receiving gateways for assistance"
              name={["modemGeolocationServices", "gnssUseGatewayLocation"]}
              tooltip="If enabled, the gateway location will be provided to the geolocation resolver to aid the resolving process."
              valuePropName="checked"
            >
              <Switch />
            </Form.Item>
          )}
          {modemEnabled && (
            <Form.Item
              label="My device adheres to the LoRa Edge&trade; Tracker Modem-E Version Reference Design protocol"
              name={["modemGeolocationServices", "parseTlv"]}
              tooltip="If enabled, ChirpStack will try to resolve the location of the device if a geolocation payload is detected."
              valuePropName="checked"
            >
              <Switch />
            </Form.Item>
          )}
          <Collapse style={{ marginBottom: 24 }}>
            <Collapse.Panel header="Advanced geolocation options" key={1}>
              <Form.Item
                label="TDOA based geolocation"
                name={["modemGeolocationServices", "geolocationTdoa"]}
                tooltip="If enabled, geolocation will be based on time-difference of arrival (TDOA). Please note that this requires gateways that support the fine-timestamp feature."
                valuePropName="checked"
              >
                <Switch onChange={onGeolocationTdoaChange} />
              </Form.Item>
              <Form.Item
                label="RSSI based geolocation"
                name={["modemGeolocationServices", "geolocationRssi"]}
                tooltip="If enabled, geolocation will be based on RSSI values reported by the receiving gateways."
                valuePropName="checked"
              >
                <Switch onChange={onGeolocationRssiChange} />
              </Form.Item>
              <Form.Item
                label="Wi-Fi based geolocation"
                name={["modemGeolocationServices", "geolocationWifi"]}
                tooltip="If enabled, geolocation will be based on Wi-Fi access-point data reported by the device."
                valuePropName="checked"
              >
                <Switch onChange={onGeolocationWifiChange} />
              </Form.Item>
              <Form.Item
                label="GNSS based geolocation (LR1110)"
                name={["modemGeolocationServices", "geolocationGnss"]}
                tooltip="If enabled, geolocation will be based on GNSS data reported by the device."
                valuePropName="checked"
              >
                <Switch onChange={onGeolocationGnssChange} />
              </Form.Item>
              {(geolocationTdoa || geolocationRssi) && (
                <Form.Item
                  label="Geolocation buffer (TTL in seconds)"
                  name={["modemGeolocationServices", "geolocationBufferTtl"]}
                  tooltip="The time in seconds that historical uplinks will be stored in the geolocation buffer. Used for TDOA and RSSI geolocation."
                >
                  <InputNumber min={0} max={86400} />
                </Form.Item>
              )}
              {(geolocationTdoa || geolocationRssi) && (
                <Form.Item
                  label="Geolocation min buffer size"
                  name={["modemGeolocationServices", "geolocationMinBufferSize"]}
                  tooltip="The minimum buffer size required before using geolocation. Using multiple uplinks for geolocation can increase the accuracy of the geolocation results. Used for TDOA and RSSI geolocation."
                >
                  <InputNumber min={0} />
                </Form.Item>
              )}
              {geolocationWifi && (
                <Form.Item
                  label="Wifi payload field"
                  name={["modemGeolocationServices", "geolocationWifiPayloadField"]}
                  tooltip="This must match the name of the field in the decoded payload which holds array of Wifi access-points. Each element in the array must contain two keys: 1) macAddress: array of 6 bytes, 2) signalStrength: RSSI of the access-point."
                >
                  <Input />
                </Form.Item>
              )}
              {geolocationGnss && (
                <Form.Item
                  label="GNSS payload field"
                  name={["modemGeolocationServices", "geolocationGnssPayloadField"]}
                  tooltip="This must match the name of the field in the decoded payload which holds the LR1110 GNSS bytes."
                >
                  <Input />
                </Form.Item>
              )}
              {geolocationGnss && (
                <Form.Item
                  label="Use receive timestamp for GNSS geolocation"
                  name={["modemGeolocationServices", "geolocationGnssUseRxTime"]}
                  valuePropName="checked"
                >
                  <Switch />
                </Form.Item>
              )}
            </Collapse.Panel>
          </Collapse>
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

export default LoRaCloudIntegrationForm;
