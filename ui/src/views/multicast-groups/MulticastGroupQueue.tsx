import { useState } from "react";

import { format } from "date-fns";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";

import type { DatePickerProps } from "antd";
import { Button, Tabs, Space, Card, Row, Form, Input, InputNumber, Popconfirm, DatePicker } from "antd";
import type { ColumnsType } from "antd/es/table";
import { RedoOutlined, DeleteOutlined } from "@ant-design/icons";
import { Buffer } from "buffer";

import type {
  MulticastGroup,
  ListMulticastGroupQueueResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  EnqueueMulticastGroupQueueItemRequest,
  ListMulticastGroupQueueRequest,
  FlushMulticastGroupQueueRequest,
  MulticastGroupQueueItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import { onFinishFailed } from "../helpers";
import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import MulticastGroupStore from "../../stores/MulticastGroupStore";

interface IProps {
  multicastGroup: MulticastGroup;
}

interface FormRules {
  fPort: number;
  hex: string;
  base64: string;
  expiresAt?: DatePickerProps["value"];
}

function MulticastGroupQueue(props: IProps) {
  const [refreshCounter, setRefreshCounter] = useState<number>(0);
  const [form] = Form.useForm<FormRules>();

  const columns: ColumnsType<MulticastGroupQueueItem.AsObject> = [
    {
      title: "Frame-counter",
      dataIndex: "fCnt",
      key: "fCnt",
      width: 100,
    },
    {
      title: "FPort",
      dataIndex: "fPort",
      key: "fPort",
      width: 100,
    },
    {
      title: "Data (HEX)",
      dataIndex: "data",
      key: "data",
      render: (text, record) => {
        return Buffer.from(record.data as string, "base64").toString("hex");
      },
    },
    {
      title: "Expires at",
      dataIndex: "expiresAt",
      key: "expiresAt",
      width: 250,
      render: (_text, record) => {
        if (record.expiresAt !== undefined) {
          const ts = new Date(0);
          ts.setUTCSeconds(record.expiresAt.seconds);
          return format(ts, "yyyy-MM-dd HH:mm:ss");
        } else {
          return "Never";
        }
      },
    },
  ];

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListMulticastGroupQueueRequest();
    req.setMulticastGroupId(props.multicastGroup.getId());

    MulticastGroupStore.listQueue(req, (resp: ListMulticastGroupQueueResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.itemsList.length, obj.itemsList);
    });
  };

  const refreshQueue = () => {
    setRefreshCounter(refreshCounter + 1);
  };

  const flushQueue = () => {
    const req = new FlushMulticastGroupQueueRequest();
    req.setMulticastGroupId(props.multicastGroup.getId());
    MulticastGroupStore.flushQueue(req, () => {
      refreshQueue();
    });
  };

  const onEnqueue = (values: FormRules) => {
    const req = new EnqueueMulticastGroupQueueItemRequest();
    const item = new MulticastGroupQueueItem();

    item.setMulticastGroupId(props.multicastGroup.getId());
    item.setFPort(values.fPort);

    if (values.expiresAt !== null && values.expiresAt !== undefined) {
      item.setExpiresAt(Timestamp.fromDate(values.expiresAt.toDate()));
    }

    if (values.base64 !== undefined) {
      item.setData(new Uint8Array(Buffer.from(values.base64, "base64")));
    }

    if (values.hex !== undefined) {
      item.setData(new Uint8Array(Buffer.from(values.hex, "hex")));
    }

    req.setQueueItem(item);

    MulticastGroupStore.enqueue(req, _ => {
      form.resetFields();
      refreshQueue();
    });
  };

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <Card title="Enqueue">
        <Form
          layout="horizontal"
          onFinish={onEnqueue}
          onFinishFailed={onFinishFailed}
          form={form}
          initialValues={{ fPort: 1 }}
        >
          <Row>
            <Space direction="horizontal" style={{ width: "100%" }} size="large">
              <Form.Item name="fPort" label="FPort">
                <InputNumber min={1} max={254} />
              </Form.Item>
              <Form.Item
                name="expiresAt"
                label="Expires at"
                tooltip="If set, the queue-item will automatically expire at the given timestamp if it wasn't sent yet."
              >
                <DatePicker showTime />
              </Form.Item>
            </Space>
          </Row>
          <Tabs defaultActiveKey="1">
            <Tabs.TabPane tab="HEX" key="1">
              <Form.Item name="hex">
                <Input />
              </Form.Item>
            </Tabs.TabPane>
            <Tabs.TabPane tab="BASE64" key="2">
              <Form.Item name="base64">
                <Input />
              </Form.Item>
            </Tabs.TabPane>
          </Tabs>
          <Button type="primary" htmlType="submit">
            Enqueue
          </Button>
        </Form>
      </Card>
      <Row justify="end">
        <Space direction="horizontal" size="large">
          <Button icon={<RedoOutlined />} onClick={refreshQueue}>
            Reload
          </Button>
          <Popconfirm title="Are you sure you want to flush the queue?" placement="left" onConfirm={flushQueue}>
            <Button icon={<DeleteOutlined />}>Flush queue</Button>
          </Popconfirm>
        </Space>
      </Row>
      <DataTable columns={columns} getPage={getPage} refreshKey={refreshCounter} rowKey="id" noPagination />
    </Space>
  );
}

export default MulticastGroupQueue;
