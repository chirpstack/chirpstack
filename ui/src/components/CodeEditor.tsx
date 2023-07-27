import React, { useState, useEffect } from "react";
import { Controlled as CodeMirror } from "react-codemirror2";

import { Form } from "antd";

import "codemirror/mode/javascript/javascript";

interface IProps {
  label?: string;
  name: string;
  required?: boolean;
  disabled?: boolean;
  tooltip?: string;
}

function CodeEditor(props: IProps) {
  const form = Form.useFormInstance();
  const [value, setValue] = useState<string>("");
  const [reloadKey, setReloadKey] = useState<number>(1);

  useEffect(() => {
    setValue(form.getFieldValue(props.name));
    setReloadKey(k => k + 1);
  }, [form, props]);

  const handleChange = (editor: any, data: any, newCode: string) => {
    setValue(newCode);
    form.setFieldsValue({
      [props.name]: newCode,
    });
  };

  const codeMirrorOptions = {
    lineNumbers: true,
    mode: "javascript",
    theme: "base16-light",
    readOnly: props.disabled,
  };

  return (
    <Form.Item label={props.label} name={props.name} tooltip={props.tooltip}>
      <div style={{ border: "1px solid #cccccc" }}>
        <CodeMirror
          key={`code-editor-refresh-${reloadKey}`}
          value={value}
          options={codeMirrorOptions}
          onBeforeChange={handleChange}
        />
      </div>
    </Form.Item>
  );
}

export default CodeEditor;
