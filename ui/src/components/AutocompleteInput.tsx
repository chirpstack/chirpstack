import React, { Component } from "react";

import { Form } from "antd";

import Autocomplete, { OptionCallbackFunc, OptionsCallbackFunc } from "./Autocomplete";

interface IProps {
  formRef: React.RefObject<any>;
  label: string;
  name: string;
  required?: boolean;
  value?: string;
  getOption: (s: string, fn: OptionCallbackFunc) => void;
  getOptions: (s: string, fn: OptionsCallbackFunc) => void;
}

class AutocompleteInput extends Component<IProps> {
  render() {
    return (
      <Form.Item
        rules={[
          {
            required: this.props.required,
            message: `Please select a ${this.props.label}`,
          },
        ]}
        label={this.props.label}
        name={this.props.name}
      >
        <Autocomplete
          placeholder={`Select a ${this.props.label}`}
          className=""
          getOption={this.props.getOption}
          getOptions={this.props.getOptions}
        />
      </Form.Item>
    );
  }
}

export default AutocompleteInput;
