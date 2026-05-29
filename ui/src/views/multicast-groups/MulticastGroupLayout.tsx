import { useCallback, useState, useEffect } from "react";
import { Route, Routes, useNavigate, useParams, useLocation, Link } from "react-router-dom";

import { Space, Breadcrumb, Card, Button, Menu, Modal, Form, Input, InputNumber } from "antd";
import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";

import type { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";
import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type {
  GetMulticastGroupResponse,
  MulticastGroup,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  GetMulticastGroupRequest,
  DeleteMulticastGroupRequest,
  MulticastGroupSetup,
  SyncMulticastGroupTs005SessionRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";

import MulticastGroupStore from "../../stores/MulticastGroupStore";
import DeleteConfirm from "../../components/DeleteConfirm";
import ListMulticastGroupDevices from "./ListMulticastGroupDevices";
import ListMulticastGroupGateways from "./ListMulticastGroupGateways";
import EditMulticastGroup from "./EditMulticastGroup";
import Admin from "../../components/Admin";
import MulticastGroupQueue from "./MulticastGroupQueue";
import { useTitle } from "../helpers";
import PageHeader from "../../components/PageHeader";

interface IProps {
  tenant: Tenant;
  application: Application;
}

interface SyncTs005SessionFormValues {
  mcSessionStartInput: string;
  mcSessionTimeout: number;
}

function toDateTimeLocal(date: Date): string {
  const localDate = new Date(date.getTime() - date.getTimezoneOffset() * 60000);
  return localDate.toISOString().slice(0, 19);
}

function MulticastGroupLayout(props: IProps) {
  const { multicastGroupId } = useParams();
  const navigate = useNavigate();
  const location = useLocation();
  const [syncForm] = Form.useForm();
  const [multicastGroup, setMulticastGroup] = useState<MulticastGroup | undefined>(undefined);
  const [syncModalVisible, setSyncModalVisible] = useState<boolean>(false);
  useTitle(
    "Tenants",
    props.tenant.getName(),
    "Applications",
    props.application.getName(),
    "Multicast-groups",
    multicastGroup?.getName(),
  );

  const loadMulticastGroup = useCallback(() => {
    const req = new GetMulticastGroupRequest();
    req.setId(multicastGroupId!);

    MulticastGroupStore.get(req, (resp: GetMulticastGroupResponse) => {
      setMulticastGroup(resp.getMulticastGroup());
    });
  }, [multicastGroupId]);

  useEffect(() => {
    loadMulticastGroup();
  }, [loadMulticastGroup]);

  const deleteMulticastGroup = () => {
    const req = new DeleteMulticastGroupRequest();
    req.setId(multicastGroupId!);

    MulticastGroupStore.delete(req, () => {
      navigate(`/tenants/${props.tenant.getId()}/applications/${props.application.getId()}/multicast-groups`);
    });
  };

  const showSyncTs005SessionModal = () => {
    const currentSessionStart = multicastGroup?.getMcSessionStart()?.toDate();
    const fallbackSessionStart = new Date(Date.now() + 15 * 60 * 1000);
    const sessionStart =
      currentSessionStart && currentSessionStart.getTime() > Date.now() ? currentSessionStart : fallbackSessionStart;

    syncForm.setFieldsValue({
      mcSessionStartInput: toDateTimeLocal(sessionStart),
      mcSessionTimeout: multicastGroup?.getMcSessionTimeout() ?? 0,
    });
    setSyncModalVisible(true);
  };

  const syncTs005Session = (values: SyncTs005SessionFormValues) => {
    const req = new SyncMulticastGroupTs005SessionRequest();
    req.setMulticastGroupId(multicastGroupId!);
    req.setMcSessionStart(Timestamp.fromDate(new Date(values.mcSessionStartInput)));
    req.setMcSessionTimeout(values.mcSessionTimeout);

    MulticastGroupStore.syncTs005Session(req, () => {
      setSyncModalVisible(false);
      loadMulticastGroup();
    });
  };

  const handleSyncTs005SessionModalOk = () => {
    syncForm.validateFields().then((values: SyncTs005SessionFormValues) => {
      syncTs005Session(values);
    });
  };

  const tenant = props.tenant;
  const app = props.application;
  const mg = multicastGroup;

  if (!mg) {
    return null;
  }

  let tab = "devices";

  const path = location.pathname;
  if (path.endsWith("gateways")) {
    tab = "gateways";
  }
  if (path.endsWith("edit")) {
    tab = "edit";
  }
  if (path.endsWith("queue")) {
    tab = "queue";
  }

  return (
    <Space orientation="vertical" style={{ width: "100%" }} size="large">
      <Modal
        title="Sync TS005 session"
        open={syncModalVisible}
        onOk={handleSyncTs005SessionModalOk}
        onCancel={() => setSyncModalVisible(false)}
      >
        <Form layout="vertical" form={syncForm}>
          <Form.Item
            label="Session start"
            name="mcSessionStartInput"
            rules={[{ required: true, message: "Please enter a session start!" }]}
          >
            <Input type="datetime-local" />
          </Form.Item>
          <Form.Item
            label="Session timeout"
            name="mcSessionTimeout"
            rules={[{ required: true, message: "Please enter a session timeout!" }]}
          >
            <InputNumber min={0} max={15} style={{ width: "100%" }} />
          </Form.Item>
        </Form>
      </Modal>
      <PageHeader
        breadcrumbRender={() => (
          <Breadcrumb
            items={[
              { title: "Tenants" },
              { title: <Link to={`/tenants/${props.tenant.getId()}`}>{props.tenant.getName()}</Link> },
              { title: <Link to={`/tenants/${props.tenant.getId()}/applications`}>Applications</Link> },
              {
                title: <Link to={`/tenants/${props.tenant.getId()}/applications/${app.getId()}`}>{app.getName()}</Link>,
              },
              {
                title: (
                  <Link to={`/tenants/${props.tenant.getId()}/applications/${app.getId()}/multicast-groups`}>
                    Multicast-groups
                  </Link>
                ),
              },
              { title: mg.getName() },
            ]}
          />
        )}
        title={mg.getName()}
        subTitle={`multicast-group id: ${mg.getId()}`}
        extra={[
          <Admin tenantId={tenant.getId()} isDeviceAdmin key="sync-delete-multicast-group">
            <Space orientation="horizontal" style={{ float: "right" }}>
              {mg.getSetup() === MulticastGroupSetup.TS005 && (
                <Button type="primary" onClick={showSyncTs005SessionModal}>
                  Sync session
                </Button>
              )}
              <DeleteConfirm typ="multicast-group" confirm={mg.getName()} onConfirm={deleteMulticastGroup}>
                <Button danger type="primary">
                  Delete multicast-group
                </Button>
              </DeleteConfirm>
            </Space>
          </Admin>,
        ]}
      />
      <Card>
        <Menu
          mode="horizontal"
          selectedKeys={[tab]}
          style={{ marginBottom: 24 }}
          items={[
            {
              key: "devices",
              label: (
                <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}`}>
                  Devices
                </Link>
              ),
            },
            {
              key: "gateways",
              label: (
                <Link
                  to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}/gateways`}
                >
                  Gateways
                </Link>
              ),
            },
            {
              key: "edit",
              label: (
                <Link to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}/edit`}>
                  Configuration
                </Link>
              ),
            },
            {
              key: "queue",
              label: (
                <Link
                  to={`/tenants/${tenant.getId()}/applications/${app.getId()}/multicast-groups/${mg.getId()}/queue`}
                >
                  Queue
                </Link>
              ),
            },
          ]}
        />
        <Routes>
          <Route path="/" element={<ListMulticastGroupDevices multicastGroup={mg} />} />
          <Route path="/gateways" element={<ListMulticastGroupGateways multicastGroup={mg} application={app} />} />
          <Route path="/edit" element={<EditMulticastGroup application={app} multicastGroup={mg} />} />
          <Route path="/queue" element={<MulticastGroupQueue multicastGroup={mg} />} />
        </Routes>
      </Card>
    </Space>
  );
}

export default MulticastGroupLayout;
