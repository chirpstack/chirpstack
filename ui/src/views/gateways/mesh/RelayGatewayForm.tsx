import { Form, Input, InputNumber, Row, Col, Button } from "antd";

import { RelayGateway } from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";

import { onFinishFailed } from "../../helpers";
import RelayIdInput from "../../../components/RelayIdInput";

interface IProps {
  initialValues: RelayGateway;
  onFinish: (obj: RelayGateway) => void;
  update?: boolean;
  disabled?: boolean;
}

function RelayGatewayForm(props: IProps) {
  const onFinish = (values: RelayGateway.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    const relay = new RelayGateway();

    relay.setTenantId(v.tenantId);
    relay.setRelayId(v.relayId);
    relay.setName(v.name);
    relay.setDescription(v.description);
    relay.setStatsInterval(v.statsInterval);
    relay.setRegionConfigId(v.regionConfigId);

    props.onFinish(relay);
  };

  return (
    <Form
      layout="vertical"
      initialValues={props.initialValues.toObject()}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
    >
      <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
        <Input disabled={props.disabled} />
      </Form.Item>
      <Form.Item label="Description" name="description">
        <Input.TextArea disabled={props.disabled} />
      </Form.Item>
      <Row gutter={24}>
        <Col span={12}>
          <RelayIdInput
            label="Relay ID (4 bytes)"
            name="relayId"
            value={props.initialValues.getRelayId()}
            disabled={props.update || props.disabled}
            required
          />
        </Col>
        <Col span={12}>
          <Form.Item
            label="Stats interval (secs)"
            tooltip="The expected interval in seconds in which the relay gateway sends its statistics"
            name="statsInterval"
            rules={[{ required: true, message: "Please enter a stats interval!" }]}
          >
            <InputNumber min={0} disabled={props.disabled} />
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

export default RelayGatewayForm;
