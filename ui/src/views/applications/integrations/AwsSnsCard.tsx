import { Link } from "react-router-dom";

import { Col, Card, Popconfirm } from "antd";
import { PlusOutlined, EditOutlined, DeleteOutlined } from "@ant-design/icons";

import { Application, DeleteAwsSnsIntegrationRequest } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";

import ApplicationStore from "../../../stores/ApplicationStore";

interface IProps {
  application: Application;
  add?: boolean;
}

function AwsSns(props: IProps) {
  const onDelete = () => {
    let req = new DeleteAwsSnsIntegrationRequest();
    req.setApplicationId(props.application.getId());

    ApplicationStore.deleteAwsSnsIntegration(req, () => {});
  };

  let actions: any[] = [];

  if (!!props.add) {
    actions = [
      <Link to="aws-sns/create">
        <PlusOutlined />
      </Link>,
    ];
  } else {
    actions = [
      <Link to="aws-sns/edit">
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
        title="AWS SNS"
        className="integration-card"
        cover={<img alt="AWS SNS" src="/integrations/aws_sns.png" style={{ padding: 1 }} />}
        actions={actions}
      >
        <Card.Meta description="The AWS SNS integration forwards events to an AWS SNS topic." />
      </Card>
    </Col>
  );
}

export default AwsSns;
