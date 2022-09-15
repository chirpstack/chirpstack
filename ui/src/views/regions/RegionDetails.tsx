import React, { Component } from "react";
import { Link } from "react-router-dom";
import { RouteComponentProps } from "react-router-dom";

import { Space, Breadcrumb, PageHeader, Card, Row, Col, List, Typography } from "antd";
import ReactMarkdown from "react-markdown";

import { Region } from "@chirpstack/chirpstack-api-grpc-web/common/common_pb";
import {
  GetRegionRequest,
  GetRegionResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";

import { getEnumName } from "../helpers";
import InternalStore from "../../stores/InternalStore";

interface MatchParams {
  name: string;
}

interface IState {
  region?: GetRegionResponse,
}

interface IProps extends RouteComponentProps<MatchParams> {}

class RegionDetails extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {};
  }

  componentDidMount() {
    let req = new GetRegionRequest();
    req.setName(this.props.match.params.name);

    InternalStore.getRegion(req, (resp: GetRegionResponse) => {
      this.setState({
        region: resp,
      });
    });
  }

  render() {
    if (this.state.region === undefined) {
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
                <span><Link to="/regions">Regions</Link></span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>{this.props.match.params.name}</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={this.props.match.params.name}
          subTitle={`common-name: ${getEnumName(Region, this.state.region.getRegion())}`}
        />
        {this.state.region.getUserInfo() !== "" && <Card>
          <ReactMarkdown>
            {this.state.region.getUserInfo()}
          </ReactMarkdown>
        </Card>}
        <Row gutter={24}>
          <Col span={12}>
            <Space direction="vertical" style={{width: "100%"}} size="large">
              <Card title="Uplink channels">
                <List
                  itemLayout="horizontal"
                  dataSource={this.state.region.getUplinkChannelsList()}
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
            <Space direction="vertical" style={{width: "100%"}} size="large">
              <Card title="Downlink">
                <List
                  dataSource={[
                    ["RX1 delay", `${this.state.region.getRx1Delay()} sec`],
                    ["RX1 DR offset", this.state.region.getRx1DrOffset()],
                    ["RX2 DR", this.state.region.getRx2Dr()],
                    ["RX2 frequency", `${this.state.region.getRx2Frequency()} Hz`],
                  ]}
                  renderItem={item => <List.Item><Typography.Text strong style={{width: "175px", display: "block", float: "left"}}>{item[0]}:</Typography.Text> {item[1]}</List.Item>}
                />
              </Card>
              <Card title="Class-B">
                <List
                  dataSource={[
                    ["Ping-slot DR", this.state.region.getClassBPingSlotDr()],
                    ["Ping-slot frequency", this.state.region.getClassBPingSlotFrequency() === 0 ? "default frequency or frequency hopping" : `${this.state.region.getClassBPingSlotFrequency()} Hz`],
                  ]}
                  renderItem={item => <List.Item><Typography.Text strong style={{width: "175px", display: "block", float: "left"}}>{item[0]}:</Typography.Text> {item[1]}</List.Item>}
                />
              </Card>
            </Space>
          </Col>
        </Row>
      </Space>
    );
  }
}

export default RegionDetails;
