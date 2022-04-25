import React, { Component } from "react";

import { Form, Input, InputNumber, Select, Row, Col, Button } from "antd";

import { MulticastGroup, MulticastGroupType } from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import AesKeyInput from "../../components/AesKeyInput";
import DevAddrInput from "../../components/DevAddrInput";

interface IProps {
  initialValues: MulticastGroup;
  onFinish: (obj: MulticastGroup) => void;
  disabled?: boolean;
}

interface IState {
  selectPingSlotPeriod: boolean;
}

class MulticastGroupForm extends Component<IProps, IState> {
  formRef = React.createRef<any>();

  constructor(props: IProps) {
    super(props);
    this.state = {
      selectPingSlotPeriod: false,
    };
  }

  onFinish = (values: MulticastGroup.AsObject) => {
    const v = Object.assign(this.props.initialValues.toObject(), values);
    let mg = new MulticastGroup();
    mg.setId(v.id);
    mg.setApplicationId(v.applicationId);

    mg.setName(v.name);
    mg.setMcAddr(v.mcAddr);
    mg.setMcNwkSKey(v.mcNwkSKey);
    mg.setMcAppSKey(v.mcAppSKey);
    mg.setDr(v.dr);
    mg.setFCnt(v.fCnt);
    mg.setFrequency(v.frequency);
    mg.setGroupType(v.groupType);
    mg.setClassBPingSlotPeriod(v.classBPingSlotPeriod);

    this.props.onFinish(mg);
  };

  onGroupTypeChange = (groupType: MulticastGroupType) => {
    this.setState({
      selectPingSlotPeriod: groupType === MulticastGroupType.CLASS_B,
    });
  };

  render() {
    return (
      <Form
        layout="vertical"
        initialValues={this.props.initialValues.toObject()}
        onFinish={this.onFinish}
        ref={this.formRef}
      >
        <Form.Item
          label="Multicast-group name"
          name="name"
          rules={[{ required: true, message: "Please enter a name!" }]}
        >
          <Input disabled={this.props.disabled} />
        </Form.Item>
        <DevAddrInput
          label="Multicast address"
          name="mcAddr"
          value={this.props.initialValues.getMcAddr()}
          formRef={this.formRef}
          devEui=""
          disabled={this.props.disabled}
          required
        />
        <AesKeyInput
          label="Multicast network session key"
          name="mcNwkSKey"
          value={this.props.initialValues.getMcNwkSKey()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <AesKeyInput
          label="Multicast application session key"
          name="mcAppSKey"
          value={this.props.initialValues.getMcAppSKey()}
          formRef={this.formRef}
          disabled={this.props.disabled}
          required
        />
        <Row gutter={24}>
          <Col span={8}>
            <Form.Item
              label="Data-rate"
              name="dr"
              rules={[{ required: true, message: "Please enter a data-rate!" }]}
              tooltip="The data-rate to use when transmitting the multicast frames. Please refer to the LoRaWAN Regional Parameters specification for valid values."
            >
              <InputNumber min={0} max={15} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="Frame-counter"
              name="fCnt"
              rules={[{ required: true, message: "Please enter a frame-counter!" }]}
            >
              <InputNumber min={0} disabled={this.props.disabled} />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item
              label="Frequency (Hz)"
              name="frequency"
              tooltip="The frequency to use when transmitting the multicast frames. Please refer to the LoRaWAN Regional Parameters specification for valid values."
              rules={[{ required: true, message: "Please enter a frequency!" }]}
            >
              <InputNumber min={0} disabled={this.props.disabled} style={{ width: "200px" }} />
            </Form.Item>
          </Col>
        </Row>
        <Row gutter={24}>
          <Col span={12}>
            <Form.Item
              label="Group type"
              name="groupType"
              tooltip="The multicast-group type defines the way how multicast frames are scheduled by the network-server."
              rules={[{ required: true, message: "Please select a group-type!" }]}
            >
              <Select onChange={this.onGroupTypeChange} disabled={this.props.disabled}>
                <Select.Option value={MulticastGroupType.CLASS_C}>Class-C</Select.Option>
                <Select.Option value={MulticastGroupType.CLASS_B}>Class-B</Select.Option>
              </Select>
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item label="Class-B ping-slot periodicity" name="classBPingSlotPeriod">
              <Select disabled={!this.state.selectPingSlotPeriod || this.props.disabled}>
                <Select.Option value={32 * 1}>Every second</Select.Option>
                <Select.Option value={32 * 2}>Every 2 seconds</Select.Option>
                <Select.Option value={32 * 4}>Every 4 seconds</Select.Option>
                <Select.Option value={32 * 8}>Every 8 seconds</Select.Option>
                <Select.Option value={32 * 16}>Every 16 seconds</Select.Option>
                <Select.Option value={32 * 32}>Every 32 seconds</Select.Option>
                <Select.Option value={32 * 64}>Every 64 seconds</Select.Option>
                <Select.Option value={32 * 128}>Every 128 seconds</Select.Option>
              </Select>
            </Form.Item>
          </Col>
        </Row>
        <Form.Item>
          <Button type="primary" htmlType="submit" disabled={this.props.disabled}>
            Submit
          </Button>
        </Form.Item>
      </Form>
    );
  }
}

export default MulticastGroupForm;
