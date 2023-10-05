import React, { useState } from "react";

import { Struct } from "google-protobuf/google/protobuf/struct_pb";

import { Switch, notification } from "antd";
import { Button, Tabs, Space, Card, Row, Form, Input, InputNumber, Checkbox, Popconfirm } from "antd";
import { ColumnsType } from "antd/es/table";
import { RedoOutlined, DeleteOutlined } from "@ant-design/icons";
import { Buffer } from "buffer";

import {
  Device,
  EnqueueDeviceQueueItemRequest,
  GetDeviceQueueItemsRequest,
  GetDeviceQueueItemsResponse,
  FlushDeviceQueueRequest,
  DeviceQueueItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import { onFinishFailed } from "../helpers";
import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import DeviceStore from "../../stores/DeviceStore";
import CodeEditor from "../../components/CodeEditor";

interface IProps {
  device: Device;
}

function DeviceQueue(props: IProps) {
  const [refreshCounter, setRefreshCounter] = useState<number>(0);
  const [isEncrypted, setIsEncrypted] = useState<boolean>(false);
  const [form] = Form.useForm();

  const columns: ColumnsType<DeviceQueueItem.AsObject> = [
    {
      title: "ID",
      dataIndex: "id",
      key: "id",
      width: 350,
    },
    {
      title: "Pending",
      dataIndex: "isPending",
      key: "isPending",
      width: 100,
      render: (text, record) => {
        if (record.isPending) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Encrypted",
      dataIndex: "isEncrypted",
      key: "isEncrypted",
      width: 100,
      render: (text, record) => {
        if (record.isEncrypted) {
          return "yes";
        } else {
          return "no";
        }
      },
    },
    {
      title: "Frame-counter",
      dataIndex: "fCntDown",
      key: "fCntDown",
      width: 200,
      render: (text, record) => {
        if (record.isPending === true || record.isEncrypted === true) {
          return record.fCntDown;
        } else {
          return "";
        }
      },
    },
    {
      title: "Confirmed",
      dataIndex: "confirmed",
      key: "confirmed",
      width: 100,
      render: (text, record) => {
        if (record.confirmed) {
          return "yes";
        } else {
          return "no";
        }
      },
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
  ];

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new GetDeviceQueueItemsRequest();
    req.setDevEui(props.device.getDevEui());

    DeviceStore.getQueue(req, (resp: GetDeviceQueueItemsResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const refreshQueue = () => {
    setRefreshCounter(refreshCounter + 1);
  };

  const flushQueue = () => {
    let req = new FlushDeviceQueueRequest();
    req.setDevEui(props.device.getDevEui());
    DeviceStore.flushQueue(req, () => {
      refreshQueue();
    });
  };

  const onEnqueue = (values: any) => {
    let req = new EnqueueDeviceQueueItemRequest();
    let item = new DeviceQueueItem();

    item.setDevEui(props.device.getDevEui());
    item.setFPort(values.fPort);
    item.setConfirmed(values.confirmed);
    item.setIsEncrypted(values.isEncrypted);
    item.setFCntDown(values.fCntDown);

    if (values.hex !== undefined) {
      item.setData(new Uint8Array(Buffer.from(values.hex, "hex")));
    }

    if (values.base64 !== undefined) {
      item.setData(new Uint8Array(Buffer.from(values.base64, "base64")));
    }

    if (values.json !== undefined) {
      try {
        const obj = JSON.parse(values.json);
        let struct = Struct.fromJavaScript(obj);

        item.setObject(struct);
      } catch (err) {
        if (err instanceof Error) {
          notification.error({
            message: "Error",
            description: err.message,
            duration: 3,
          });
        }
      }
    }

    req.setQueueItem(item);

    DeviceStore.enqueue(req, _ => {
      form.resetFields();
      setIsEncrypted(false);
      refreshQueue();
    });
  };

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <Card title="Enqueue">
        <Form layout="horizontal" onFinish={onEnqueue} onFinishFailed={onFinishFailed} form={form} initialValues={{ fPort: 1 }}>
          <Row>
            <Space direction="horizontal" style={{ width: "100%" }} size="large">
              <Form.Item name="confirmed" label="Confirmed" valuePropName="checked">
                <Switch />
              </Form.Item>
              <Form.Item name="fPort" label="FPort">
                <InputNumber min={1} max={254} />
              </Form.Item>
              <Form.Item
                name="isEncrypted"
                label="Is encrypted"
                valuePropName="checked"
                tooltip="Only enable this in case the payload that you would like to enqueue has already been encrypted. In this case you also must enter the downlink frame-counter which has been used for the encryption."
              >
                <Switch onChange={setIsEncrypted} />
              </Form.Item>
              {isEncrypted && (<Form.Item
                name="fCntDown"
                label="Downlink frame-counter used for encryption"
                rules={[{ required: true, message: "Please enter a downlink frame-counter!" }]}
              >
                <InputNumber min={0} />
              </Form.Item>)}
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
            <Tabs.TabPane tab="JSON" key="3">
              <CodeEditor name="json" />
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

export default DeviceQueue;
