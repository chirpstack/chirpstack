import { notification, Input, Button, Space, Form, Dropdown, Menu } from "antd";
import { ReloadOutlined, CopyOutlined } from "@ant-design/icons";

import type { GetRandomDevAddrResponse } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import { GetRandomDevAddrRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import DeviceStore from "../stores/DeviceStore";

interface IProps {
  label: string;
  name: string;
  devEui: string;
  required?: boolean;
  disabled?: boolean;
}

function DevAddrInput(props: IProps) {
  const form = Form.useFormInstance();

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const v = e.target.value;
    const match = v.match(/[A-Fa-f0-9]/g);

    let value = "";
    if (match) {
      if (match.length > 8) {
        value = match.slice(0, 8).join("");
      } else {
        value = match.join("");
      }
    }

    form.setFieldValue(props.name, value);
  };

  const generateRandom = () => {
    const req = new GetRandomDevAddrRequest();
    req.setDevEui(props.devEui);

    DeviceStore.getRandomDevAddr(req, (resp: GetRandomDevAddrResponse) => {
      form.setFieldValue(props.name, resp.getDevAddr());
    });
  };

  const copyToClipboard = () => {
    const bytes = form.getFieldValue(props.name).match(/[A-Fa-f0-9]{2}/g);

    if (bytes !== null && navigator.clipboard !== undefined) {
      navigator.clipboard
        .writeText(bytes.join("").toUpperCase())
        .then(() => {
          notification.success({
            message: "Copied to clipboard",
            duration: 3,
          });
        })
        .catch(e => {
          notification.error({
            message: "Error",
            description: e,
            duration: 3,
          });
        });
    } else {
      notification.error({
        message: "Error",
        description: "Clipboard functionality is not available.",
        duration: 3,
      });
    }
  };

  const copyToClipboardHexArray = () => {
    const bytes = form.getFieldValue(props.name).match(/[A-Fa-f0-9]{2}/g);

    if (bytes !== null && navigator.clipboard !== undefined) {
      navigator.clipboard
        .writeText(
          bytes
            .join(", ")
            .toUpperCase()
            .replace(/[A-Fa-f0-9]{2}/g, "0x$&"),
        )
        .then(() => {
          notification.success({
            message: "Copied to clipboard",
            duration: 3,
          });
        })
        .catch(e => {
          notification.error({
            message: "Error",
            description: e,
            duration: 3,
          });
        });
    }
  };

  const copyMenu = (
    <Menu
      items={[
        {
          key: "1",
          label: (
            <Button type="text" onClick={copyToClipboard}>
              HEX string
            </Button>
          ),
        },
        {
          key: "2",
          label: (
            <Button type="text" onClick={copyToClipboardHexArray}>
              HEX array
            </Button>
          ),
        },
      ]}
    />
  );

  const addon = (
    <Space size="large">
      <Button type="text" size="small" onClick={generateRandom} disabled={props.disabled}>
        <ReloadOutlined />
      </Button>
      <Dropdown overlay={copyMenu}>
        <Button type="text" size="small">
          <CopyOutlined />
        </Button>
      </Dropdown>
    </Space>
  );

  return (
    <Form.Item
      rules={[
        {
          required: props.required,
          message: `Please enter a valid ${props.label}`,
          pattern: new RegExp(/[A-Fa-f0-9]{8}/g),
        },
      ]}
      label={props.label}
      name={props.name}
    >
      <Input onChange={onChange} addonAfter={addon} className="input-code" disabled={props.disabled} />
    </Form.Item>
  );
}

export default DevAddrInput;
