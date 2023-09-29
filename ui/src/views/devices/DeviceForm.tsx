import { Form, Input, Row, Col, Button, Tabs, Switch } from "antd";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";

import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import { Device } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import {
  ListDeviceProfilesRequest,
  ListDeviceProfilesResponse,
  GetDeviceProfileRequest,
  GetDeviceProfileResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_pb";

import { onFinishFailed } from "../helpers";
import EuiInput from "../../components/EuiInput";
import { OptionsCallbackFunc, OptionCallbackFunc } from "../../components/Autocomplete";
import AutocompleteInput from "../../components/AutocompleteInput";
import DeviceProfileStore from "../../stores/DeviceProfileStore";

interface IProps {
  tenant: Tenant;
  initialValues: Device;
  onFinish: (obj: Device) => void;
  update?: boolean;
}

function DeviceForm(props: IProps) {
  const [form] = Form.useForm();

  const onFinish = (values: Device.AsObject) => {
    const v = Object.assign(props.initialValues.toObject(), values);
    let d = new Device();

    d.setApplicationId(v.applicationId);
    d.setName(v.name);
    d.setDescription(v.description);
    d.setDevEui(v.devEui);
    d.setDeviceProfileId(v.deviceProfileId);
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

  const getDeviceProfileOptions = (search: string, fn: OptionsCallbackFunc) => {
    let req = new ListDeviceProfilesRequest();
    req.setTenantId(props.tenant.getId());
    req.setSearch(search);
    req.setLimit(10);

    DeviceProfileStore.list(req, (resp: ListDeviceProfilesResponse) => {
      const options = resp.getResultList().map((o, i) => {
        return { label: o.getName(), value: o.getId() };
      });

      fn(options);
    });
  };

  const getDeviceProfileOption = (id: string, fn: OptionCallbackFunc) => {
    let req = new GetDeviceProfileRequest();
    req.setId(id);

    DeviceProfileStore.get(req, (resp: GetDeviceProfileResponse) => {
      const dp = resp.getDeviceProfile();
      if (dp) {
        fn({ label: dp.getName(), value: dp.getId() });
      }
    });
  };

  return (
    <Form layout="vertical" initialValues={props.initialValues.toObject()} onFinish={onFinish} onFinishFailed={onFinishFailed} form={form}>
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
          <AutocompleteInput
            label="Device profile"
            name="deviceProfileId"
            getOption={getDeviceProfileOption}
            getOptions={getDeviceProfileOptions}
            required
          />
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
                        fieldKey={[name, 0]}
                        rules={[{ required: true, message: "Please enter a key!" }]}
                      >
                        <Input placeholder="Key" />
                      </Form.Item>
                    </Col>
                    <Col span={16}>
                      <Form.Item
                        {...restField}
                        name={[name, 1]}
                        fieldKey={[name, 1]}
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
                        fieldKey={[name, 0]}
                        rules={[{ required: true, message: "Please enter a key!" }]}
                      >
                        <Input placeholder="Key" />
                      </Form.Item>
                    </Col>
                    <Col span={16}>
                      <Form.Item
                        {...restField}
                        name={[name, 1]}
                        fieldKey={[name, 1]}
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
