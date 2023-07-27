import { Form } from "antd";

import Autocomplete, { OptionCallbackFunc, OptionsCallbackFunc } from "./Autocomplete";

interface IProps {
  label: string;
  name: string;
  required?: boolean;
  value?: string;
  getOption: (s: string, fn: OptionCallbackFunc) => void;
  getOptions: (s: string, fn: OptionsCallbackFunc) => void;
}

function AutocompleteInput(props: IProps) {
  const form = Form.useFormInstance();

  const onSelect = (value: string) => {
    form.setFieldsValue({
      [props.name]: value,
    });
  };

  return (
    <Form.Item
      rules={[
        {
          required: props.required,
          message: `Please select a ${props.label}`,
        },
      ]}
      label={props.label}
      name={props.name}
    >
      <Autocomplete
        placeholder={`Select a ${props.label}`}
        className=""
        getOption={props.getOption}
        getOptions={props.getOptions}
        onSelect={onSelect}
      />
    </Form.Item>
  );
}

export default AutocompleteInput;
