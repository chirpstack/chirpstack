import { useEffect, useState } from "react";

import { Form, Input, InputNumber, Select, Row, Col, Button } from "antd";

import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import {
  MulticastGroup,
  MulticastGroupType,
  MulticastGroupSchedulingType,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import type { ListRegionsResponse, RegionListItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import { getEnumName, onFinishFailed } from "../helpers";
import InternalStore from "../../stores/InternalStore";
import AesKeyInput from "../../components/AesKeyInput";
import DevAddrInput from "../../components/DevAddrInput";

interface IProps {
  initialValues: MulticastGroup;
  onFinish: (obj: MulticastGroup) => void;
  disabled?: boolean;
}

function MulticastGroupForm(props: IProps) {
  const [form] = Form.useForm();
  const [selectPingSlotPeriod, setSelectPingSlotPeriod] = useState<boolean>(false);
  const [regionConfigurations, setRegionConfigurations] = useState<RegionListItem[]>([]);

  useEffect(() => {
    InternalStore.listRegions((resp: ListRegionsResponse) => {
      setRegionConfigurations(resp.getRegionsList());
    });
  }, []);

  const onFinish = (values: MulticastGroup.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    const mg = new MulticastGroup();
    mg.setId(v.id);
    mg.setApplicationId(v.applicationId);

    mg.setName(v.name);
    mg.setMcAddr(v.mcAddr);
    mg.setMcNwkSKey(v.mcNwkSKey);
    mg.setMcAppSKey(v.mcAppSKey);
    mg.setDr(v.dr);
    mg.setFCnt(v.fCnt);
    mg.setFrequency(v.frequency);
    mg.setRegion(v.region);
    mg.setGroupType(v.groupType);
    mg.setClassBPingSlotPeriodicity(v.classBPingSlotPeriodicity);
    mg.setClassCSchedulingType(v.classCSchedulingType);

    props.onFinish(mg);
  };

  const onGroupTypeChange = (groupType: MulticastGroupType) => {
    setSelectPingSlotPeriod(groupType === MulticastGroupType.CLASS_B);
  };

  const regConfs = regionConfigurations
    .map(v => v.getRegion())
    .filter((v, i, a) => a.indexOf(v) === i)
    .map(v => {
      return {
        value: v,
        label: getEnumName(Region, v).replace("_", "-"),
      };
    });

  return (
    <Form
      layout="vertical"
      initialValues={props.initialValues.toObject()}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
      form={form}
    >
      <Form.Item label="Multicast-group name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
        <Input disabled={props.disabled} />
      </Form.Item>
      <DevAddrInput label="Multicast address" name="mcAddr" devEui="" disabled={props.disabled} required />
      <AesKeyInput label="Multicast network session key" name="mcNwkSKey" disabled={props.disabled} required />
      <AesKeyInput label="Multicast application session key" name="mcAppSKey" disabled={props.disabled} required />
      <Row gutter={24}>
        <Col span={8}>
          <Form.Item label="Region" name="region" rules={[{ required: true, message: "Please select a region!" }]}>
            <Select disabled={props.disabled} options={regConfs} />
          </Form.Item>
        </Col>
        <Col span={4}>
          <Form.Item
            label="Data-rate"
            name="dr"
            rules={[{ required: true, message: "Please enter a data-rate!" }]}
            tooltip="The data-rate to use when transmitting the multicast frames. Please refer to the LoRaWAN Regional Parameters specification for valid values."
          >
            <InputNumber min={0} max={15} disabled={props.disabled} style={{ width: "100%" }} />
          </Form.Item>
        </Col>
        <Col span={4}>
          <Form.Item
            label="Frame-counter"
            name="fCnt"
            rules={[{ required: true, message: "Please enter a frame-counter!" }]}
          >
            <InputNumber min={0} disabled={props.disabled} style={{ width: "100%" }} />
          </Form.Item>
        </Col>
        <Col span={8}>
          <Form.Item
            label="Frequency (Hz)"
            name="frequency"
            tooltip="The frequency to use when transmitting the multicast frames. Please refer to the LoRaWAN Regional Parameters specification for valid values."
            rules={[{ required: true, message: "Please enter a frequency!" }]}
          >
            <InputNumber min={0} disabled={props.disabled} style={{ width: "100%" }} />
          </Form.Item>
        </Col>
      </Row>
      <Row gutter={24}>
        <Col span={8}>
          <Form.Item
            label="Group type"
            name="groupType"
            tooltip="The multicast-group type defines the way how multicast frames are scheduled by the network-server."
            rules={[{ required: true, message: "Please select a group-type!" }]}
          >
            <Select
              onChange={onGroupTypeChange}
              disabled={props.disabled}
              options={[
                { value: MulticastGroupType.CLASS_C, label: "Class-C" },
                { value: MulticastGroupType.CLASS_B, label: "Class-B" },
              ]}
            />
          </Form.Item>
        </Col>
        <Col span={8}>
          <Form.Item label="Class-B ping-slot periodicity" name="classBPingSlotPeriodicity">
            <Select
              disabled={!selectPingSlotPeriod || props.disabled}
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
            />
          </Form.Item>
        </Col>
        <Col span={8}>
          <Form.Item
            label="Class-C scheduling type"
            name="classCSchedulingType"
            tooltip="In order to reach all devices, it might be needed to transmit a downlink through multiple gateways. In case of Delay each gateway will transmit one by one, in case of GPS Time all required gateways will transmit at the same GPS time."
          >
            <Select
              disabled={selectPingSlotPeriod || props.disabled}
              options={[
                { value: MulticastGroupSchedulingType.DELAY, label: "Delay" },
                { value: MulticastGroupSchedulingType.GPS_TIME, label: "GPS Time" },
              ]}
            />
          </Form.Item>
        </Col>
      </Row>
      <Form.Item>
        <Button type="primary" htmlType="submit" disabled={props.disabled}>
          Submit
        </Button>
      </Form.Item>
    </Form>
  );
}

export default MulticastGroupForm;
