import { useState } from "react";

import { format } from "date-fns";
import { JSONTree as JSONTreeOriginal } from "react-json-tree";
import fileDownload from "js-file-download";

import { Tag, Drawer, Button, Table, Spin, Space } from "antd";
import { ZoomInOutlined } from "@ant-design/icons";

import type { LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

interface IProps {
  logs: LogItem[];
}

function LogTable(props: IProps) {
  const [drawerOpen, setDrawerOpen] = useState<boolean>(false);
  const [body, setBody] = useState<string | null>(null);
  const [drawerTitle, setDrawerTitle] = useState<string | null>(null);

  const onDrawerClose = () => {
    setDrawerOpen(false);
  };

  const onDrawerOpen = (time: { seconds: number } | undefined, body: string) => {
    const ts = new Date(0);
    ts.setUTCSeconds(time!.seconds);
    const drawerTitle = format(ts, "yyyy-MM-dd HH:mm:ss");

    return () => {
      setBody(body);
      setDrawerTitle(drawerTitle);
      setDrawerOpen(true);
    };
  };

  const downloadSingleFrame = () => {
    fileDownload(JSON.stringify(JSON.parse(body!), null, 4), "single-log.json", "application/json");
  };

  const downloadFrames = () => {
    const items = props.logs.map((l, _i) => JSON.parse(l.getBody()));
    fileDownload(JSON.stringify(items, null, 4), "log.json");
  };

  const items = props.logs.map((l, _i) => l.toObject());
  const bodyJson = JSON.parse(body!);

  const theme = {
    scheme: "google",
    author: "seth wright (http://sethawright.com)",
    base00: "#ffffff",
    base01: "#282a2e",
    base02: "#373b41",
    base03: "#969896",
    base04: "#b4b7b4",
    base05: "#c5c8c6",
    base06: "#e0e0e0",
    base07: "#ffffff",
    base08: "#CC342B",
    base09: "#F96A38",
    base0A: "#FBA922",
    base0B: "#198844",
    base0C: "#3971ED",
    base0D: "#3971ED",
    base0E: "#A36AC7",
    base0F: "#3971ED",
  };

  return (
    <Space orientation="vertical" size="large" style={{ width: "100%" }}>
      <Drawer
        title={`Details: ${drawerTitle}`}
        placement="right"
        size={650}
        onClose={onDrawerClose}
        open={drawerOpen}
        extra={<Button onClick={downloadSingleFrame}>Download</Button>}
      >
        <JSONTreeOriginal
          data={bodyJson}
          theme={theme}
          hideRoot={true}
          shouldExpandNodeInitially={() => {
            return true;
          }}
        />
      </Drawer>
      {items.length !== 0 && (
        <Space orientation="horizontal" style={{ float: "right" }} size="large">
          <Spin size="small" />
          <Button onClick={downloadFrames}>Download</Button>
        </Space>
      )}
      <Table
        rowKey="id"
        showHeader={false}
        loading={items.length === 0}
        dataSource={items}
        pagination={false}
        columns={[
          {
            title: "Time",
            dataIndex: "time",
            key: "time",
            width: 200,
            render: (_text, obj) => {
              const ts = new Date(0);
              ts.setUTCSeconds(obj.time!.seconds);
              return format(ts, "yyyy-MM-dd HH:mm:ss");
            },
          },
          {
            title: "Type",
            dataIndex: "description",
            key: "description",
            width: 200,
            render: (text, obj) => (
              <Button
                icon={<ZoomInOutlined />}
                type="primary"
                shape="round"
                size="small"
                onClick={onDrawerOpen(obj.time, obj.body)}
              >
                {text}
              </Button>
            ),
          },
          {
            title: "Properties",
            dataIndex: "properties",
            key: "properties",
            render: (_text, obj) => (
              <Space orientation="horizontal" size="small">
                {obj.propertiesMap.map((p, _i) => {
                  if (p[1] !== "") {
                    return (
                      <Tag key={p[0]} variant="outlined">
                        <pre>
                          {p[0]}: {p[1]}
                        </pre>
                      </Tag>
                    );
                  }

                  return null;
                })}
              </Space>
            ),
          },
        ]}
      />
    </Space>
  );
}

export default LogTable;
