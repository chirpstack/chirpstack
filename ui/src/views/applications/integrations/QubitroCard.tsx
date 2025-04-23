import { Card, Col, Popconfirm } from "antd";
import { DeleteOutlined, EditOutlined, PlusOutlined } from "@ant-design/icons";

import { Link } from "react-router-dom";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { DeleteQubitroIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function QubitroCard(props: IProps) {
  const onDelete = () => {
    const req = new DeleteQubitroIntegrationRequest();
    req.setApplicationId(props.application.getId());
    ApplicationStore.deleteQubitroIntegration(req, () => {});
  };

  let actions: JSX.Element[] = [];

  if (props.add) {
    actions = [
      <Link to="qubitro/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="qubitro/edit">
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
        title="Qubitro"
        className="integration-card"
        cover={<img alt="Qubitro" src="https://static.qubitro.com/landing/qubitro_nav_logo.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="Sync all devices and data from ChirpStack to Qubitro—no code required." />
      </Card>
    </Col>
  );
}

export default QubitroCard; 