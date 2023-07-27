import { useState, PropsWithChildren } from "react";
import { Popover, Button, Typography, Space, Input } from "antd";

interface IProps {
  typ: string;
  confirm: string;
  onConfirm: () => void;
}

function DeleteConfirmContent(props: IProps) {
  const [confirm, setConfirm] = useState<string>("");

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setConfirm(e.target.value);
  };

  return (
    <Space direction="vertical">
      <Typography.Text>
        Enter '{props.confirm}' to confirm you want to delete this {props.typ}:
      </Typography.Text>
      <Input placeholder={props.confirm} onChange={onChange} />
      <Button onClick={props.onConfirm} disabled={confirm !== props.confirm} style={{ float: "right" }}>
        Delete
      </Button>
    </Space>
  );
}

function DeleteConfirm(props: PropsWithChildren<IProps>) {
  return (
    <Popover content={<DeleteConfirmContent {...props} />} trigger="click" placement="left">
      {props.children}
    </Popover>
  );
}

export default DeleteConfirm;
