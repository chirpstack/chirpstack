import { useState, useEffect } from "react";

import { Form } from "antd";
import AceEditor from "react-ace";

import "ace-builds/src-noconflict/mode-javascript";
import "ace-builds/src-noconflict/mode-json";
import "ace-builds/src-noconflict/theme-github";

interface IProps {
  label?: string;
  name: string;
  required?: boolean;
  disabled?: boolean;
  tooltip?: string;
  mode?: string;
}

function CodeEditor(props: IProps) {
  const form = Form.useFormInstance();
  const [value, setValue] = useState<string>("");

  useEffect(() => {
    setValue(form.getFieldValue(props.name) || "");
  }, [form, props]);

  const onChange = (newValue: string) => {
    setValue(newValue);
    form.setFieldsValue({
      [props.name]: newValue,
    });
  };

  return (
    <Form.Item label={props.label} name={props.name} tooltip={props.tooltip}>
      <div style={{ border: "1px solid #cccccc" }}>
        <AceEditor
          mode={props.mode || "javascript"}
          theme="github"
          onChange={onChange}
          value={value}
          width="100%"
          height="600px"
          editorProps={{ $blockScrolling: true }}
        />
      </div>
    </Form.Item>
  );
}

export default CodeEditor;
