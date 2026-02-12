import { notification, Input, Button, Space, Form, Dropdown, Menu } from "antd";
import { ReloadOutlined, CopyOutlined } from "@ant-design/icons";
import { Buffer } from "buffer";
import { MenuProps } from "antd/lib";

interface IProps {
  label: string;
  name: string;
  required?: boolean;
  disabled?: boolean;
  tooltip?: string;
}

function AesKeyInput(props: IProps) {
  const form = Form.useFormInstance();

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const v = e.target.value;
    const match = v.match(/[A-Fa-f0-9]/g);

    let value = "";
    if (match) {
      if (match.length > 32) {
        value = match.slice(0, 32).join("");
      } else {
        value = match.join("");
      }
    }

    form.setFieldValue(props.name, value);
  };

  const generateRandom = () => {
    const cryptoObj = window.crypto || window.Crypto;
    const b = new Uint8Array(16);
    cryptoObj.getRandomValues(b);

    const key = Buffer.from(b).toString("hex");
    form.setFieldValue(props.name, key);
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

  const copyMenu: MenuProps = {
    items: [
      {
        key: "1",
        label: "HEX string",
        onClick: copyToClipboard,
      },
      {
        key: "2",
        label: "HEX array",
        onClick: copyToClipboardHexArray,
      },
    ],
  };

  const addon = (
    <Space size="large">
      <Button type="text" size="small" onClick={generateRandom} disabled={props.disabled}>
        <ReloadOutlined />
      </Button>
      <Dropdown menu={copyMenu}>
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
          pattern: new RegExp(/[A-Fa-f0-9]{32}/g),
        },
      ]}
      label={props.label}
      name={props.name}
      tooltip={props.tooltip}
    >
      <Input onChange={onChange} addonAfter={addon} className="input-code" disabled={props.disabled} />
    </Form.Item>
  );
}

export default AesKeyInput;
