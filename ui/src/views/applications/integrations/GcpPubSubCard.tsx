import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteGcpPubSubIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function GcpPubSubCard(props: IProps) {
  const onDelete = () => {
    let req = new DeleteGcpPubSubIntegrationRequest();
    req.setApplicationId(props.application.getId());
    ApplicationStore.deleteGcpPubSubIntegration(req, () => {});
  };

  let actions: any[] = [];

  if (!!props.add) {
    actions = [
      <Link to="gcp-pub-sub/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="gcp-pub-sub/edit">
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
        title="GCP Pub/Sub"
        className="integration-card"
        cover={<img alt="GCP Pub/Sub" src="/integrations/gcp_pubsub.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="The Google Cloud Pub/Sub integration forwards events to a GCP Pub/Sub topic." />
      </Card>
    </Col>
  );
}

export default GcpPubSubCard;
