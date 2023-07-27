import React, { useState, useEffect } from "react";

import { Select } from "antd";

export type OptionsCallbackFunc = (o: { label: string; value: string }[]) => void;
export type OptionCallbackFunc = (o: { label: string; value: string }) => void;

interface Option {
  label: string;
  value: string;
}

interface IProps {
  placeholder: string;
  className: string;
  value?: string;
  getOption: (s: string, fn: OptionCallbackFunc) => void;
  getOptions: (s: string, fn: OptionsCallbackFunc) => void;
  onSelect?: (s: string) => void;
}

function AutoComplete({ placeholder, className, value, getOption, getOptions, onSelect }: IProps) {
  const [option, setOption] = useState<Option | undefined>(undefined);
  const [options, setOptions] = useState<Option[]>([]);

  useEffect(() => {
    if (value && value !== "") {
      getOption(value, (o: Option) => {
        setOptions([o]);
      });
    }
  }, [value, getOption]);

  const onFocus = () => {
    getOptions("", options => {
      if (option !== undefined) {
        const selected = option.value;

        if (options.find(e => e.value === selected) === undefined) {
          options.unshift(option);
        }
      }

      setOptions(options);
    });
  };

  const onSearch = (value: string) => {
    getOptions(value, options => {
      setOptions(options);
    });
  };

  const onSelectFn = (value: string, option: any) => {
    setOption({ label: option.label, value: option.value });

    if (onSelect !== undefined) {
      onSelect(value);
    }
  };

  return (
    <Select
      showSearch
      options={options}
      onFocus={onFocus}
      onSearch={onSearch}
      onSelect={onSelectFn}
      filterOption={false}
      placeholder={placeholder}
      className={className}
      value={value}
    />
  );
}

export default AutoComplete;
