import React from "react";
import { Space, Row, Col, Typography } from "antd";

interface IProps {
  title: React.ReactNode;
  subTitle?: React.ReactNode;
  breadcrumbRender?: () => React.ReactNode;
  extra?: React.ReactNode;
  children?: React.ReactNode;
}

function PageHeader(props: IProps) {
  return (
    <Space orientation="vertical" style={{ width: "100%", padding: "16px", paddingTop: "12px" }}>
      {props.breadcrumbRender && (
        <Row>
          <Col span={24}>{props.breadcrumbRender()}</Col>
        </Row>
      )}
      <Row align="middle">
        <Col flex="auto">
          <Space size="medium">
            <Typography.Title level={4} style={{ marginBottom: 0 }}>
              {props.title}
            </Typography.Title>
            <Typography.Text type="secondary">{props.subTitle}</Typography.Text>
          </Space>
        </Col>
        {props.extra && <Col>{props.extra}</Col>}
      </Row>
      {props.children && <Row style={{ paddingTop: "12px" }}>{props.children}</Row>}
    </Space>
  );
}

export default PageHeader;
