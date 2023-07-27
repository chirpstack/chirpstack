import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteMyDevicesIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function MyDevicesCard(props: IProps) {
  const onDelete = () => {
    let req = new DeleteMyDevicesIntegrationRequest();
    req.setApplicationId(props.application.getId());
    ApplicationStore.deleteMyDevicesIntegration(req, () => {});
  };

  let actions: any[] = [];

  if (!!props.add) {
    actions = [
      <Link to="mydevices/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="mydevices/edit">
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
        title="myDevices"
        className="integration-card"
        cover={<img alt="myDevices" src="/integrations/my_devices.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="The myDevices integration forwards events to the myDevices platform." />
      </Card>
    </Col>
  );
}

export default MyDevicesCard;
