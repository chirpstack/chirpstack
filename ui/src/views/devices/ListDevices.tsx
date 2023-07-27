import React, { useState, useEffect } from "react";
import { Link } from "react-router-dom";

import moment from "moment";
import { Space, Button, Dropdown, Menu, Modal, Select } from "antd";
import { ColumnsType } from "antd/es/table";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faPlug,
  faBatteryFull,
  faBatteryQuarter,
  faBatteryHalf,
  faBatteryThreeQuarters,
} from "@fortawesome/free-solid-svg-icons";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  ListDevicesRequest,
  ListDevicesResponse,
  DeviceListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import {
  ListMulticastGroupsRequest,
  ListMulticastGroupsResponse,
  MulticastGroupListItem,
  AddDeviceToMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  ListRelaysRequest,
  ListRelaysResponse,
  RelayListItem,
  AddRelayDeviceRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import DeviceStore from "../../stores/DeviceStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import RelayStore from "../../stores/RelayStore";
import Admin from "../../components/Admin";

interface IProps {
  application: Application;
}

function ListDevices(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [multicastGroups, setMulticastGroups] = useState<MulticastGroupListItem[]>([]);
  const [relays, setRelays] = useState<RelayListItem[]>([]);
  const [mgModalVisible, setMgModalVisible] = useState<boolean>(false);
  const [relayModalVisible, setRelayModalVisible] = useState<boolean>(false);
  const [mgSelected, setMgSelected] = useState<string>("");
  const [relaySelected, setRelaySelected] = useState<string>("");

  useEffect(() => {
    let mgReq = new ListMulticastGroupsRequest();
    mgReq.setLimit(999);
    mgReq.setApplicationId(props.application.getId());

    MulticastGroupStore.list(mgReq, (resp: ListMulticastGroupsResponse) => {
      setMulticastGroups(resp.getResultList());
    });

    let relayReq = new ListRelaysRequest();
    relayReq.setLimit(999);
    relayReq.setApplicationId(props.application.getId());

    RelayStore.list(relayReq, (resp: ListRelaysResponse) => {
      setRelays(resp.getResultList());
    });
  }, [props]);

  const columns: ColumnsType<DeviceListItem.AsObject> = [
    {
      title: "Last seen",
      dataIndex: "lastSeenAt",
      key: "lastSeenAt",
      width: 250,
      render: (text, record) => {
        if (record.lastSeenAt !== undefined) {
          let ts = new Date(0);
          ts.setUTCSeconds(record.lastSeenAt.seconds);
          return moment(ts).format("YYYY-MM-DD HH:mm:ss");
        }
        return "Never";
      },
    },
    {
      title: "DevEUI",
      dataIndex: "devEui",
      key: "devEui",
      width: 250,
      render: (text, record) => (
        <Link
          to={`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/devices/${
            record.devEui
          }`}
        >
          {text}
        </Link>
      ),
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
    },
    {
      title: "Device profile",
      dataIndex: "deviceProfileName",
      key: "deviceProfileName",
      render: (text, record) => (
        <Link to={`/tenants/${props.application.getTenantId()}/device-profiles/${record.deviceProfileId}/edit`}>
          {text}
        </Link>
      ),
    },
    {
      title: "Battery",
      dataIndex: "deviceStatus",
      key: "deviceStatus",
      render: (text, record) => {
        if (record.deviceStatus === undefined) {
          return;
        }

        if (record.deviceStatus.externalPowerSource === true) {
          return <FontAwesomeIcon icon={faPlug} />;
        } else if (record.deviceStatus.batteryLevel > 75) {
          return <FontAwesomeIcon icon={faBatteryFull} />;
        } else if (record.deviceStatus.batteryLevel > 50) {
          return <FontAwesomeIcon icon={faBatteryThreeQuarters} />;
        } else if (record.deviceStatus.batteryLevel > 25) {
          return <FontAwesomeIcon icon={faBatteryHalf} />;
        } else if (record.deviceStatus.batteryLevel > 0) {
          return <FontAwesomeIcon icon={faBatteryQuarter} />;
        }
      },
    },
  ];

  const getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListDevicesRequest();
    req.setApplicationId(props.application.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    DeviceStore.list(req, (resp: ListDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  const onRowsSelectChange = (ids: string[]) => {
    setSelectedRowIds(ids);
  };

  const showMgModal = () => {
    setMgModalVisible(true);
  };

  const showRelayModal = () => {
    setRelayModalVisible(true);
  };

  const hideMgModal = () => {
    setMgModalVisible(false);
  };

  const hideRelayModal = () => {
    setRelayModalVisible(false);
  };

  const onMgSelected = (value: string) => {
    setMgSelected(value);
  };

  const onRelaySelected = (value: string) => {
    setRelaySelected(value);
  };

  const handleMgModalOk = () => {
    for (let devEui of selectedRowIds) {
      let req = new AddDeviceToMulticastGroupRequest();
      req.setMulticastGroupId(mgSelected);
      req.setDevEui(devEui);

      MulticastGroupStore.addDevice(req, () => {});
    }

    setMgModalVisible(false);
  };

  const handleRelayModalOk = () => {
    for (let devEui of selectedRowIds) {
      let req = new AddRelayDeviceRequest();
      req.setRelayDevEui(relaySelected);
      req.setDeviceDevEui(devEui);

      RelayStore.addDevice(req, () => {});
    }

    setRelayModalVisible(false);
  };

  const menu = (
    <Menu>
      <Menu.Item onClick={showMgModal}>Add to multicast-group</Menu.Item>
      <Menu.Item onClick={showRelayModal}>Add to relay</Menu.Item>
    </Menu>
  );

  const mgOptions = multicastGroups.map((mg, i) => <Select.Option value={mg.getId()}>{mg.getName()}</Select.Option>);

  const relayOptions = relays.map((r, i) => <Select.Option value={r.getDevEui()}>{r.getName()}</Select.Option>);

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <Modal
        title="Add selected devices to multicast-group"
        visible={mgModalVisible}
        onOk={handleMgModalOk}
        onCancel={hideMgModal}
        okButtonProps={{ disabled: mgSelected === "" }}
      >
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <Select style={{ width: "100%" }} onChange={onMgSelected} placeholder="Select Multicast-group">
            {mgOptions}
          </Select>
        </Space>
      </Modal>
      <Modal
        title="Add selected devices to relay"
        visible={relayModalVisible}
        onOk={handleRelayModalOk}
        onCancel={hideRelayModal}
        okButtonProps={{ disabled: relaySelected === "" }}
      >
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <Select style={{ width: "100%" }} onChange={onRelaySelected} placeholder="Select Relay">
            {relayOptions}
          </Select>
        </Space>
      </Modal>
      <Admin tenantId={props.application.getTenantId()} isDeviceAdmin>
        <Space direction="horizontal" style={{ float: "right" }}>
          <Button type="primary">
            <Link
              to={`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/devices/create`}
            >
              Add device
            </Link>
          </Button>
          <Dropdown placement="bottomRight" overlay={menu} trigger={["click"]} disabled={selectedRowIds.length === 0}>
            <Button>Selected devices</Button>
          </Dropdown>
        </Space>
      </Admin>
      <DataTable columns={columns} getPage={getPage} onRowsSelectChange={onRowsSelectChange} rowKey="devEui" />
    </Space>
  );
}

export default ListDevices;
