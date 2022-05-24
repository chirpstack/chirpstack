import React, { Component } from "react";

import { Struct } from "google-protobuf/google/protobuf/struct_pb";

import { notification } from "antd";
import { Button, Tabs, Space, Card, Row, Form, Input, InputNumber, Checkbox, Popconfirm } from "antd";
import { ColumnsType } from "antd/es/table";
import { RedoOutlined, DeleteOutlined } from "@ant-design/icons";
import {Buffer} from "buffer";

import {
  Device,
  EnqueueDeviceQueueItemRequest,
  GetDeviceQueueItemsRequest,
  GetDeviceQueueItemsResponse,
  FlushDeviceQueueRequest,
  DeviceQueueItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import DeviceStore from "../../stores/DeviceStore";
import CodeEditor from "../../components/CodeEditor";

interface IProps {
  device: Device;
}

interface IState {
  refreshCounter: number;
}

class DeviceQueue extends Component<IProps, IState> {
  formRef = React.createRef<any>();

  constructor(props: IProps) {
    super(props);
    this.state = {
      refreshCounter: 0,
    };
  }

  columns = (): ColumnsType<DeviceQueueItem.AsObject> => {
    return [
      {
        title: "ID",
        dataIndex: "id",
        key: "id",
        width: 350,
      },
      {
        title: "Is pending",
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
        title: "Frame-counter",
        dataIndex: "fCntDown",
        key: "fCntDown",
        width: 200,
        render: (text, record) => {
          if (record.isPending === true) {
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
  };

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new GetDeviceQueueItemsRequest();
    req.setDevEui(this.props.device.getDevEui());

    DeviceStore.getQueue(req, (resp: GetDeviceQueueItemsResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  refreshQueue = () => {
    this.setState({
      refreshCounter: this.state.refreshCounter + 1,
    });
  };

  flushQueue = () => {
    let req = new FlushDeviceQueueRequest();
    req.setDevEui(this.props.device.getDevEui());
    DeviceStore.flushQueue(req, () => {
      this.refreshQueue();
    });
  };

  onEnqueue = (values: any) => {
    let req = new EnqueueDeviceQueueItemRequest();
    let item = new DeviceQueueItem();

    item.setDevEui(this.props.device.getDevEui());
    item.setFPort(values.fPort);
    item.setConfirmed(values.confirmed);

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

    req.setItem(item);

    DeviceStore.enqueue(req, _ => {
      this.formRef.current.resetFields();
      this.refreshQueue();
    });
  };

  render() {
    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <Card title="Enqueue">
          <Form layout="horizontal" onFinish={this.onEnqueue} ref={this.formRef} initialValues={{ fPort: 1 }}>
            <Row>
              <Space direction="horizontal" style={{ width: "100%" }} size="large">
                <Form.Item name="confirmed" label="Confirmed" valuePropName="checked">
                  <Checkbox />
                </Form.Item>
                <Form.Item name="fPort" label="FPort">
                  <InputNumber min={1} max={254} />
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
              <Tabs.TabPane tab="JSON" key="3">
                <CodeEditor name="json" value="{}" formRef={this.formRef} />
              </Tabs.TabPane>
            </Tabs>
            <Button type="primary" htmlType="submit">
              Enqueue
            </Button>
          </Form>
        </Card>
        <Row justify="end">
          <Space direction="horizontal" size="large">
            <Button icon={<RedoOutlined />} onClick={this.refreshQueue}>
              Reload
            </Button>
            <Popconfirm title="Are you sure you want to flush the queue?" placement="left" onConfirm={this.flushQueue}>
              <Button icon={<DeleteOutlined />}>Flush queue</Button>
            </Popconfirm>
          </Space>
        </Row>
        <DataTable
          columns={this.columns()}
          getPage={this.getPage}
          refreshKey={this.state.refreshCounter}
          rowKey="id"
          noPagination
        />
      </Space>
    );
  }
}

export default DeviceQueue;
