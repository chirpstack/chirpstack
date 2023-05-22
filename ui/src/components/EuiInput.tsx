import React, { Component } from "react";

import { notification, Input, Select, Button, Space, Form, Dropdown, Menu } from "antd";
import { ReloadOutlined, CopyOutlined } from "@ant-design/icons";
import { Buffer } from "buffer";

interface IProps {
  formRef: React.RefObject<any>;
  label: string;
  name: string;
  required?: boolean;
  value?: string;
  disabled?: boolean;
  tooltip?: string;
}

interface IState {
  byteOrder: string;
  value: string;
}

class EuiInput extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      byteOrder: "msb",
      value: "",
    };
  }

  updateField = () => {
    let value = this.state.value;

    if (this.state.byteOrder === "lsb") {
      const bytes = value.match(/[A-Fa-f0-9]{2}/g) || [];
      value = bytes.reverse().join("");
    }

    this.props.formRef.current.setFieldsValue({
      [this.props.name]: value,
    });
  };

  componentDidMount() {
    if (this.props.value) {
      this.setState({
        value: this.props.value,
      });
    }
  }

  onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let v = e.target.value;
    const match = v.match(/[A-Fa-f0-9]/g);

    let value = "";
    if (match) {
      if (match.length > 16) {
        value = match.slice(0, 16).join("");
      } else {
        value = match.join("");
      }
    }

    this.setState(
      {
        value: value,
      },
      this.updateField,
    );
  };

  onByteOrderSelect = (v: string) => {
    if (v === this.state.byteOrder) {
      return;
    }

    this.setState({
      byteOrder: v,
    });

    const current = this.state.value;
    const bytes = current.match(/[A-Fa-f0-9]{2}/g) || [];

    this.setState(
      {
        value: bytes.reverse().join(""),
      },
      this.updateField,
    );
  };

  generateRandom = () => {
    let cryptoObj = window.crypto || window.Crypto;
    let b = new Uint8Array(8);
    cryptoObj.getRandomValues(b);

    let key = Buffer.from(b).toString("hex");
    this.setState(
      {
        value: key,
      },
      this.updateField,
    );
  };

  copyToClipboard = () => {
    const bytes = this.state.value.match(/[A-Fa-f0-9]{2}/g);

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
    }
  };

  copyToClipboardHexArray = () => {
    const bytes = this.state.value.match(/[A-Fa-f0-9]{2}/g);

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

  render() {
    const copyMenu = (
      <Menu
        items={[
          {
            key: "1",
            label: (
              <Button type="text" onClick={this.copyToClipboard}>
                HEX string
              </Button>
            ),
          },
          {
            key: "2",
            label: (
              <Button type="text" onClick={this.copyToClipboardHexArray}>
                HEX array
              </Button>
            ),
          },
        ]}
      />
    );

    const addon = (
      <Space size="large">
        <Select value={this.state.byteOrder} onChange={this.onByteOrderSelect}>
          <Select.Option value="msb">MSB</Select.Option>
          <Select.Option value="lsb">LSB</Select.Option>
        </Select>
        <Button type="text" size="small" onClick={this.generateRandom}>
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
            required: this.props.required,
            message: `Please enter a valid ${this.props.label}`,
            pattern: new RegExp(/[A-Fa-f0-9]{16}/g),
          },
        ]}
        label={this.props.label}
        name={this.props.name}
        tooltip={this.props.tooltip}
      >
        <Input hidden />
        <Input
          id={`${this.props.name}Render`}
          onChange={this.onChange}
          addonAfter={!this.props.disabled && addon}
          style={{ fontFamily: "monospace" }}
          value={this.state.value}
          disabled={this.props.disabled}
        />
      </Form.Item>
    );
  }
}

export default EuiInput;
