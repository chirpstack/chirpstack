import React, { Component } from "react";
import moment from "moment";
import JSONTreeOriginal from "react-json-tree";

import { Tag, Drawer, Button, Table, Spin } from "antd";
import { ZoomInOutlined } from "@ant-design/icons";

import { LogItem } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

interface IProps {
  logs: LogItem[];
}

interface IState {
  drawerOpen: boolean;
  body: any;
}

class LogTable extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      drawerOpen: false,
      body: null,
    };
  }

  onDrawerClose = () => {
    this.setState({
      drawerOpen: false,
    });
  };

  onDrawerOpen = (body: any) => {
    return () => {
      this.setState({
        body: body,
        drawerOpen: true,
      });
    };
  };

  render() {
    let items = this.props.logs.map((l, i) => l.toObject());
    let body = JSON.parse(this.state.body);

    const theme = {
      scheme: "google",
      author: "seth wright (http://sethawright.com)",
      base00: "#000000",
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
      <div>
        <Drawer
          title="Details"
          placement="right"
          width={650}
          onClose={this.onDrawerClose}
          visible={this.state.drawerOpen}
        >
          <JSONTreeOriginal
            data={body}
            theme={theme}
            hideRoot={true}
            shouldExpandNode={() => {
              return true;
            }}
          />
        </Drawer>
        {items.length !== 0 && (
          <div className="spinner">
            <Spin />
          </div>
        )}
        <Table
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
              render: (text, obj) => {
                let ts = new Date(0);
                ts.setUTCSeconds(obj.time!.seconds);
                return moment(ts).format("YYYY-MM-DD HH:mm:ss");
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
                  onClick={this.onDrawerOpen(obj.body)}
                >
                  {text}
                </Button>
              ),
            },
            {
              title: "Properties",
              dataIndex: "properties",
              key: "properties",
              render: (text, obj) =>
                obj.propertiesMap.map((p, i) => {
                  if (p[1] !== "") {
                    return (
                      <Tag>
                        <pre>
                          {p[0]}: {p[1]}
                        </pre>
                      </Tag>
                    );
                  }

                  return null;
                }),
            },
          ]}
        />
      </div>
    );
  }
}

export default LogTable;
