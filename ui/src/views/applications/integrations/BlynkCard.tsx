import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { DeleteBlynkIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function BlynkCard(props: IProps) {
  const onDelete = () => {
    const req = new DeleteBlynkIntegrationRequest();
    req.setApplicationId(props.application.getId());
    ApplicationStore.deleteBlynkIntegration(req, () => {});
  };

  let actions: JSX.Element[] = [];

  if (props.add) {
    actions = [
      <Link to="blynk/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="blynk/edit">
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
        title="Blynk"
        className="integration-card"
        cover={<img alt="Blynk" src="/integrations/blynk.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="The Blynk integration forwards events to Blynk’s low-code cloud to apps platform." />
      </Card>
    </Col>
  );
}

export default BlynkCard;
