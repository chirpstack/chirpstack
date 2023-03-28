import React, { Component } from "react";

import { Form, Input, InputNumber, Switch, Button, Tabs, Collapse } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import {
  LoraCloudIntegration,
  LoraCloudModemGeolocationServices,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

interface IProps {
  initialValues: LoraCloudIntegration;
  onFinish: (obj: LoraCloudIntegration) => void;
}

interface IState {
  modemEnabled: boolean;
  geolocationTdoa: boolean;
  geolocationRssi: boolean;
  geolocationWifi: boolean;
  geolocationGnss: boolean;
}

class LoRaCloudIntegrationForm extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      modemEnabled: false,
      geolocationTdoa: false,
      geolocationRssi: false,
      geolocationWifi: false,
      geolocationGnss: false,
    };
  }

  componentDidMount() {
    const v = this.props.initialValues;
    const mgs = v.getModemGeolocationServices();

    if (mgs !== undefined) {
      this.setState({
        modemEnabled: mgs.getModemEnabled(),
        geolocationTdoa: mgs.getGeolocationTdoa(),
        geolocationRssi: mgs.getGeolocationRssi(),
        geolocationWifi: mgs.getGeolocationWifi(),
        geolocationGnss: mgs.getGeolocationGnss(),
      });
    }
  }

  onFinish = (values: LoraCloudIntegration.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
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

    this.props.onFinish(i);
  };

  onModemEnabledChange = (v: boolean) => {
    this.setState({
      modemEnabled: v,
    });
  };

  onGeolocationTdoaChange = (v: boolean) => {
    this.setState({
      geolocationTdoa: v,
    });
  };

  onGeolocationRssiChange = (v: boolean) => {
    this.setState({
      geolocationRssi: v,
    });
  };

  onGeolocationWifiChange = (v: boolean) => {
    this.setState({
      geolocationWifi: v,
    });
  };

  onGeolocationGnssChange = (v: boolean) => {
    this.setState({
      geolocationGnss: v,
    });
  };

  render() {
    return (
      <Form layout="vertical" initialValues={this.props.initialValues.toObject()} onFinish={this.onFinish}>
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
              <Switch onChange={this.onModemEnabledChange} />
            </Form.Item>
            {this.state.modemEnabled && (
              <Form.List name={["modemGeolocationServices", "forwardFPortsList"]}>
                {(fields, { add, remove }) => (
                  <Form.Item label="Forward messages on these FPorts to LoRa Cloud">
                    {fields.map((field, index) => (
                      <Form.Item
                        {...field}
                        rules={[{ required: true, message: "Please a FPort value!" }]}
                        style={{ display: "inline-block", width: "100px", marginRight: "24px" }}
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
            {this.state.modemEnabled && (
              <Form.Item
                label="Use receive timestamp for GNSS geolocation"
                name={["modemGeolocationServices", "gnssUseRxTime"]}
                tooltip="If enabled, the receive timestamp of the gateway will be used as reference instead of the timestamp included in the GNSS payload."
                valuePropName="checked"
              >
                <Switch />
              </Form.Item>
            )}
            {this.state.modemEnabled && (
              <Form.Item
                label="Use location of receiving gateways for assistance"
                name={["modemGeolocationServices", "gnssUseGatewayLocation"]}
                tooltip="If enabled, the gateway location will be provided to the geolocation resolver to aid the resolving process."
                valuePropName="checked"
              >
                <Switch />
              </Form.Item>
            )}
            {this.state.modemEnabled && (
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
                  <Switch onChange={this.onGeolocationTdoaChange} />
                </Form.Item>
                <Form.Item
                  label="RSSI based geolocation"
                  name={["modemGeolocationServices", "geolocationRssi"]}
                  tooltip="If enabled, geolocation will be based on RSSI values reported by the receiving gateways."
                  valuePropName="checked"
                >
                  <Switch onChange={this.onGeolocationRssiChange} />
                </Form.Item>
                <Form.Item
                  label="Wi-Fi based geolocation"
                  name={["modemGeolocationServices", "geolocationWifi"]}
                  tooltip="If enabled, geolocation will be based on Wi-Fi access-point data reported by the device."
                  valuePropName="checked"
                >
                  <Switch onChange={this.onGeolocationWifiChange} />
                </Form.Item>
                <Form.Item
                  label="GNSS based geolocation (LR1110)"
                  name={["modemGeolocationServices", "geolocationGnss"]}
                  tooltip="If enabled, geolocation will be based on GNSS data reported by the device."
                  valuePropName="checked"
                >
                  <Switch onChange={this.onGeolocationGnssChange} />
                </Form.Item>
                {(this.state.geolocationTdoa || this.state.geolocationRssi) && (
                  <Form.Item
                    label="Geolocation buffer (TTL in seconds)"
                    name={["modemGeolocationServices", "geolocationBufferTtl"]}
                    tooltip="The time in seconds that historical uplinks will be stored in the geolocation buffer. Used for TDOA and RSSI geolocation."
                  >
                    <InputNumber min={0} max={86400} />
                  </Form.Item>
                )}
                {(this.state.geolocationTdoa || this.state.geolocationRssi) && (
                  <Form.Item
                    label="Geolocation min buffer size"
                    name={["modemGeolocationServices", "geolocationMinBufferSize"]}
                    tooltip="The minimum buffer size required before using geolocation. Using multiple uplinks for geolocation can increase the accuracy of the geolocation results. Used for TDOA and RSSI geolocation."
                  >
                    <InputNumber min={0} />
                  </Form.Item>
                )}
                {this.state.geolocationWifi && (
                  <Form.Item
                    label="Wifi payload field"
                    name={["modemGeolocationServices", "geolocationWifiPayloadField"]}
                    tooltip="This must match the name of the field in the decoded payload which holds array of Wifi access-points. Each element in the array must contain two keys: 1) macAddress: array of 6 bytes, 2) signalStrength: RSSI of the access-point."
                  >
                    <Input />
                  </Form.Item>
                )}
                {this.state.geolocationGnss && (
                  <Form.Item
                    label="GNSS payload field"
                    name={["modemGeolocationServices", "geolocationGnssPayloadField"]}
                    tooltip="This must match the name of the field in the decoded payload which holds the LR1110 GNSS bytes."
                  >
                    <Input />
                  </Form.Item>
                )}
                {this.state.geolocationGnss && (
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
}

export default LoRaCloudIntegrationForm;
