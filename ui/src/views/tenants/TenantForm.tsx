import { Form, Input, InputNumber, Switch, Row, Col, Button, Tabs } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import { onFinishFailed } from "../helpers";

interface IProps {
  initialValues: Tenant;
  onFinish: (obj: Tenant) => void;
  disabled?: boolean;
}

function TenantForm(props: IProps) {
  const onFinish = (values: Tenant.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);

    let tenant = new Tenant();
    tenant.setId(v.id);
    tenant.setName(v.name);
    tenant.setDescription(v.description);
    tenant.setCanHaveGateways(v.canHaveGateways);
    tenant.setMaxGatewayCount(v.maxGatewayCount);
    tenant.setMaxDeviceCount(v.maxDeviceCount);
    tenant.setPrivateGatewaysUp(v.privateGatewaysUp);
    tenant.setPrivateGatewaysDown(v.privateGatewaysDown);

    // tags
    for (const elm of v.tagsMap) {
      tenant.getTagsMap().set(elm[0], elm[1]);
    }

    props.onFinish(tenant);
  };

  return (
    <Form
      layout="vertical"
      initialValues={props.initialValues.toObject()}
      onFinish={onFinish}
      onFinishFailed={onFinishFailed}
    >
      <Tabs>
        <Tabs.TabPane tab="General" key="1">
          <Form.Item label="Name" name="name" rules={[{ required: true, message: "Please enter a name!" }]}>
            <Input disabled={props.disabled} />
          </Form.Item>
          <Form.Item label="Description" name="description">
            <Input.TextArea disabled={props.disabled} />
          </Form.Item>
          <Row>
            <Col span={8}>
              <Form.Item
                label="Tenant can have gateways"
                name="canHaveGateways"
                tooltip="When checked, the tenant can add gateways. Note that the usage of the gateways is not limited to this tenant only unless these are marked private."
                valuePropName="checked"
              >
                <Switch disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item
                label="Gateways are private (uplink)"
                name="privateGatewaysUp"
                tooltip="Uplink received by gateways of this tenant can only be used by the devices of this tenant."
                valuePropName="checked"
              >
                <Switch disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item
                label="Gateways are private (downlink)"
                name="privateGatewaysDown"
                tooltip="Other tenants can not use the gateways of this tenant for downlinks. This can be useful in case uplinks are shared with other tenants, but you want to avoid other tenants using downlink airtime of your gateways."
                valuePropName="checked"
              >
                <Switch disabled={props.disabled} />
              </Form.Item>
            </Col>
          </Row>
          <Row>
            <Col span={12}>
              <Form.Item
                label="Max. gateway count"
                name="maxGatewayCount"
                tooltip="The maximum number of gateways that can be added by this tenant (0 = unlimited)."
              >
                <InputNumber min={0} disabled={props.disabled} />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item
                label="Max. device count"
                name="maxDeviceCount"
                required={true}
                tooltip="The maximum number of devices that can be added by this tenant (0 = unlimited)."
              >
                <InputNumber min={0} disabled={props.disabled} />
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
                    {!props.disabled && (
                      <Col span={2}>
                        <MinusCircleOutlined onClick={() => remove(name)} />
                      </Col>
                    )}
                  </Row>
                ))}
                {!props.disabled && (
                  <Form.Item>
                    <Button type="dashed" onClick={() => add()} block icon={<PlusOutlined />}>
                      Add tag
                    </Button>
                  </Form.Item>
                )}
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

export default TenantForm;
