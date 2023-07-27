import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteInfluxDbIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function InfluxdbCard(props: IProps) {
  const onDelete = () => {
    let req = new DeleteInfluxDbIntegrationRequest();
    req.setApplicationId(props.application.getId());
    ApplicationStore.deleteInfluxDbIntegration(req, () => {});
  };

  let actions: any[] = [];

  if (!!props.add) {
    actions = [
      <Link to="influxdb/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="influxdb/edit">
        <EditOutlined />
      </Link>,
      <Popconfirm title="Are you sure you want to delete this integration?" onConfirm={onDelete}>
        <DeleteOutlined />
      </Popconfirm>,
    ];
  }

  return (
    <Col span={8}>
      <Card
        title="InfluxDB"
        className="integration-card"
        cover={<img alt="InfluxDB" src="/integrations/influxdb.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="The InfluxDB integration writes events into an InfluxDB time-series database." />
      </Card>
    </Col>
  );
}

export default InfluxdbCard;
