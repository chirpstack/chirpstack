import React, { Component } from "react";

import { Popover, Button, Typography, Space, Input } from "antd";

interface IProps {
  typ: string;
  confirm: string;
  onConfirm: () => void;
}

interface ConfirmState {
  confirm: string;
}

class DeleteConfirmContent extends Component<IProps, ConfirmState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      confirm: "",
    };
  }

  onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({
      confirm: e.target.value,
    });
  };

  render() {
    return (
      <Space direction="vertical">
        <Typography.Text>
          Enter '{this.props.confirm}' to confirm you want to delete this {this.props.typ}:
        </Typography.Text>
        <Input placeholder={this.props.confirm} onChange={this.onChange} />
        <Button
          onClick={this.props.onConfirm}
          disabled={this.state.confirm !== this.props.confirm}
          style={{ float: "right" }}
        >
          Delete
        </Button>
      </Space>
    );
  }
}

class DeleteConfirm extends Component<IProps> {
  render() {
    return (
      <Popover content={<DeleteConfirmContent {...this.props} />} trigger="click" placement="left">
        {this.props.children}
      </Popover>
    );
  }
}

export default DeleteConfirm;
