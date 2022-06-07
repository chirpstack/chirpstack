import React, { Component } from "react";
import { Controlled as CodeMirror } from "react-codemirror2";

import { Form } from "antd";

import "codemirror/mode/javascript/javascript";

interface IProps {
  formRef: React.RefObject<any>;
  label?: string;
  name: string;
  required?: boolean;
  value?: string;
  disabled?: boolean;
  tooltip?: string;
}

interface IState {
  value: string;
}

class CodeEditor extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      value: "",
    };
  }

  componentDidMount() {
    if (this.props.value) {
      this.setState({
        value: this.props.value,
      });
    }
  }

  componentDidUpdate(oldProps: IProps) {
    if (this.props === oldProps) {
      return;
    }

    if (this.props.value) {
      this.setState({
        value: this.props.value,
      });
    }
  }

  updateField = () => {
    let value = this.state.value;

    this.props.formRef.current.setFieldsValue({
      [this.props.name]: value,
    });
  };

  handleChange = (editor: any, data: any, newCode: string) => {
    this.setState(
      {
        value: newCode,
      },
      this.updateField,
    );
  };

  render() {
    const codeMirrorOptions = {
      lineNumbers: true,
      mode: "javascript",
      theme: "base16-light",
      readOnly: this.props.disabled,
    };

    return (
      <Form.Item label={this.props.label} name={this.props.name} tooltip={this.props.tooltip}>
        <div style={{ border: "1px solid #cccccc" }}>
          <CodeMirror value={this.state.value} options={codeMirrorOptions} onBeforeChange={this.handleChange} />
        </div>
      </Form.Item>
    );
  }
}

export default CodeEditor;
