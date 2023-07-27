import React, { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { useParams } from "react-router-dom";

import { Space, Breadcrumb, Card, Row, Col, List, Typography } from "antd";
import { PageHeader } from "@ant-design/pro-layout";
import ReactMarkdown from "react-markdown";

import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import { GetRegionRequest, GetRegionResponse } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import { getEnumName } from "../helpers";
import InternalStore from "../../stores/InternalStore";

function RegionDetails() {
  const [region, setRegion] = useState<GetRegionResponse | undefined>(undefined);
  const { id } = useParams();

  useEffect(() => {
    let req = new GetRegionRequest();
    req.setId(id!);

    InternalStore.getRegion(req, (resp: GetRegionResponse) => {
      setRegion(resp);
    });
  }, [id]);

  if (region === undefined) {
    return null;
  }

  return (
    <Space direction="vertical" style={{ width: "100%" }} size="large">
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb>
            <Breadcrumb.Item>
              <span>Network Server</span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>
                <Link to="/regions">Regions</Link>
              </span>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <span>{region.getDescription()}</span>
            </Breadcrumb.Item>
          </Breadcrumb>
        )}
        title={region.getDescription()}
        subTitle={`id: ${id}, common-name: ${getEnumName(Region, region.getRegion())}`}
      />
      {region.getUserInfo() !== "" && (
        <Card>
          <ReactMarkdown>{region.getUserInfo()}</ReactMarkdown>
        </Card>
      )}
      <Row gutter={24}>
        <Col span={12}>
          <Space direction="vertical" style={{ width: "100%" }} size="large">
            <Card title="Uplink channels">
              <List
                itemLayout="horizontal"
                dataSource={region.getUplinkChannelsList()}
                renderItem={item => (
                  <List.Item>
                    <List.Item.Meta
                      title={`${item.getFrequency()} Hz`}
                      description={`Min DR: ${item.getDrMin()}, max DR: ${item.getDrMax()}`}
                    />
                  </List.Item>
                )}
              />
            </Card>
          </Space>
        </Col>
        <Col span={12}>
          <Space direction="vertical" style={{ width: "100%" }} size="large">
            <Card title="Downlink">
              <List
                dataSource={[
                  ["RX1 delay", `${region.getRx1Delay()} sec`],
                  ["RX1 DR offset", region.getRx1DrOffset()],
                  ["RX2 DR", region.getRx2Dr()],
                  ["RX2 frequency", `${region.getRx2Frequency()} Hz`],
                ]}
                renderItem={item => (
                  <List.Item>
                    <Typography.Text
                      strong
                      style={{
                        width: "175px",
                        display: "block",
                        float: "left",
                      }}
                    >
                      {item[0]}:
                    </Typography.Text>{" "}
                    {item[1]}
                  </List.Item>
                )}
              />
            </Card>
            <Card title="Class-B">
              <List
                dataSource={[
                  ["Ping-slot DR", region.getClassBPingSlotDr()],
                  [
                    "Ping-slot frequency",
                    region.getClassBPingSlotFrequency() === 0
                      ? "default frequency or frequency hopping"
                      : `${region.getClassBPingSlotFrequency()} Hz`,
                  ],
                ]}
                renderItem={item => (
                  <List.Item>
                    <Typography.Text
                      strong
                      style={{
                        width: "175px",
                        display: "block",
                        float: "left",
                      }}
                    >
                      {item[0]}:
                    </Typography.Text>{" "}
                    {item[1]}
                  </List.Item>
                )}
              />
            </Card>
          </Space>
        </Col>
      </Row>
    </Space>
  );
}

export default RegionDetails;
