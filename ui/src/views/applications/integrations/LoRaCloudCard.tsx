import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteLoraCloudIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function LoRaCloudCard(props: IProps) {
  const onDelete = () => {
    let req = new DeleteLoraCloudIntegrationRequest();
    req.setApplicationId(props.application.getId());
    ApplicationStore.deleteLoraCloudIntegration(req, () => {});
  };

  let actions: any[] = [];

  if (!!props.add) {
    actions = [
      <Link to="loracloud/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="loracloud/edit">
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
        title="Semtech LoRa Cloud&trade;"
        className="integration-card"
        cover={<img alt="Semtech LoRa Cloud" src="/integrations/loracloud.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="The Semtech LoRa Cloud integration provides Modem & Geolocation Services." />
      </Card>
    </Col>
  );
}

export default LoRaCloudCard;
