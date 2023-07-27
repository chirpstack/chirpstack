import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import {
  Application,
  DeleteThingsBoardIntegrationRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function ThingsBoardCard(props: IProps) {
  const onDelete = () => {
    let req = new DeleteThingsBoardIntegrationRequest();
    req.setApplicationId(props.application.getId());
    ApplicationStore.deleteThingsBoardIntegration(req, () => {});
  };

  let actions: any[] = [];

  if (!!props.add) {
    actions = [
      <Link to="thingsboard/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="thingsboard/edit">
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
        title="ThingsBoard"
        className="integration-card"
        cover={<img alt="ThingsBoard" src="/integrations/thingsboard.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="The ThingsBoard integration forwards events to a ThingsBoard instance." />
      </Card>
    </Col>
  );
}

export default ThingsBoardCard;
