import React, { Component } from "react";

import { Select } from "antd";

export type OptionsCallbackFunc = (o: { label: string; value: string }[]) => void;
export type OptionCallbackFunc = (o: { label: string; value: string }) => void;

interface IProps {
  placeholder: string;
  className: string;
  value?: string;
  getOption: (s: string, fn: OptionCallbackFunc) => void;
  getOptions: (s: string, fn: OptionsCallbackFunc) => void;
  onSelect?: (s: string) => void;
}

interface IState {
  option?: { label: string; value: string };
  options: { label: string; value: string }[];
}

class Autocomplete extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      options: [],
    };
  }

  componentDidMount() {
    if (this.props.value && this.props.value !== "") {
      this.props.getOption(this.props.value, (o: { label: string; value: string }) => {
        this.setState({
          options: [o],
        });
      });
    }
  }

  componentDidUpdate(prevProps: IProps) {
    if (this.props.value === prevProps.value) {
      return;
    }

    if (this.props.value && this.props.value !== "") {
      this.props.getOption(this.props.value, (o: { label: string; value: string }) => {
        this.setState({
          options: [o],
        });
      });
    }
  }

  onFocus = () => {
    this.props.getOptions("", options => {
      if (this.state.option !== undefined) {
        const selected = this.state.option.value;

        if (options.find(e => e.value === selected) === undefined) {
          options.unshift(this.state.option);
        }
      }

      this.setState({
        options: options,
      });
    });
  };

  onSearch = (value: string) => {
    this.props.getOptions(value, options => {
      this.setState({
        options: options,
      });
    });
  };

  onSelect = (value: string, option: any) => {
    this.setState({
      option: { label: option.label, value: option.value },
    });

    if (this.props.onSelect !== undefined) {
      this.props.onSelect(value);
    }
  };

  render() {
    const { getOption, getOptions, ...otherProps } = this.props;

    return (
      <Select
        showSearch
        options={this.state.options}
        onFocus={this.onFocus}
        onSearch={this.onSearch}
        onSelect={this.onSelect}
        filterOption={false}
        {...otherProps}
      />
    );
  }
}

export default Autocomplete;
