import React, { Component } from "react";
import { RouteComponentProps, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, PageHeader } from "antd";

import {
  DeviceProfileTemplate,
  GetDeviceProfileTemplateRequest,
  GetDeviceProfileTemplateResponse,
  UpdateDeviceProfileTemplateRequest,
  DeleteDeviceProfileTemplateRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_profile_template_pb";

import DeviceProfileTemplateForm from "./DeviceProfileTemplateForm";
import DeviceProfileTemplateStore from "../../stores/DeviceProfileTemplateStore";
import DeleteConfirm from "../../components/DeleteConfirm";

interface IState {
  deviceProfileTemplate?: DeviceProfileTemplate;
}

interface MatchParams {
  deviceProfileTemplateId: string;
}

interface IProps extends RouteComponentProps<MatchParams> {}

class EditDeviceProfileTemplate extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.getDeviceProfileTemplate();
  }

  getDeviceProfileTemplate = () => {
    const id = this.props.match.params.deviceProfileTemplateId;
    let req = new GetDeviceProfileTemplateRequest();
    req.setId(id);

    DeviceProfileTemplateStore.get(req, (resp: GetDeviceProfileTemplateResponse) => {
      this.setState({
        deviceProfileTemplate: resp.getDeviceProfileTemplate(),
      });
    });
  };

  onFinish = (obj: DeviceProfileTemplate) => {
    let req = new UpdateDeviceProfileTemplateRequest();
    req.setDeviceProfileTemplate(obj);

    DeviceProfileTemplateStore.update(req, () => {
      this.props.history.push(`/device-profile-templates`);
    });
  };

  deleteDeviceProfileTemplate = () => {
    let req = new DeleteDeviceProfileTemplateRequest();
    req.setId(this.props.match.params.deviceProfileTemplateId);

    DeviceProfileTemplateStore.delete(req, () => {
      this.props.history.push(`/device-profile-templates`);
    });
  };

  render() {
    const dp = this.state.deviceProfileTemplate;

    if (!dp) {
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
                <span>
                  <Link to={`/device-profile-templates`}>Device-profile templates</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>{dp.getName()}</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          title={dp.getName()}
          subTitle={`device-profile template id: ${dp.getId()}`}
          extra={[
            <DeleteConfirm
              typ="device-profile template"
              confirm={dp.getName()}
              onConfirm={this.deleteDeviceProfileTemplate}
            >
              <Button danger type="primary">
                Delete device-profile template
              </Button>
            </DeleteConfirm>,
          ]}
        />
        <Card>
          <DeviceProfileTemplateForm initialValues={dp} update={true} onFinish={this.onFinish} />
        </Card>
      </Space>
    );
  }
}

export default EditDeviceProfileTemplate;
