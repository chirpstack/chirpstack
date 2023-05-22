import React, { Component } from "react";
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

interface IState {
  selectedRowIds: string[];
  multicastGroups: MulticastGroupListItem[];
  relays: RelayListItem[];
  mgModalVisible: boolean;
  relayModalVisible: boolean;
  mgSelected: string;
  relaySelected: string;
}

class ListDevices extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      selectedRowIds: [],
      multicastGroups: [],
      relays: [],
      mgModalVisible: false,
      relayModalVisible: false,
      mgSelected: "",
      relaySelected: "",
    };
  }

  componentDidMount() {
    let mgReq = new ListMulticastGroupsRequest();
    mgReq.setLimit(999);
    mgReq.setApplicationId(this.props.application.getId());

    MulticastGroupStore.list(mgReq, (resp: ListMulticastGroupsResponse) => {
      this.setState({
        multicastGroups: resp.getResultList(),
      });
    });

    let relayReq = new ListRelaysRequest();
    relayReq.setLimit(999);
    relayReq.setApplicationId(this.props.application.getId());

    RelayStore.list(relayReq, (resp: ListRelaysResponse) => {
      this.setState({
        relays: resp.getResultList(),
      });
    });
  }

  columns = (): ColumnsType<DeviceListItem.AsObject> => {
    return [
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
            to={`/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/devices/${
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
          <Link to={`/tenants/${this.props.application.getTenantId()}/device-profiles/${record.deviceProfileId}/edit`}>
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
  };

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListDevicesRequest();
    req.setApplicationId(this.props.application.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    DeviceStore.list(req, (resp: ListDevicesResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  onRowsSelectChange = (ids: string[]) => {
    this.setState({
      selectedRowIds: ids,
    });
  };

  showMgModal = () => {
    this.setState({
      mgModalVisible: true,
    });
  };

  showRelayModal = () => {
    this.setState({
      relayModalVisible: true,
    });
  }

  hideMgModal = () => {
    this.setState({
      mgModalVisible: false,
    });
  };

  hideRelayModal = () => {
    this.setState({
      relayModalVisible: false,
    });
  }

  onMgSelected = (value: string) => {
    this.setState({
      mgSelected: value,
    });
  };

  onRelaySelected = (value: string) => {
    this.setState({
      relaySelected: value,
    });
  };

  handleMgModalOk = () => {
    for (let devEui of this.state.selectedRowIds) {
      let req = new AddDeviceToMulticastGroupRequest();
      req.setMulticastGroupId(this.state.mgSelected);
      req.setDevEui(devEui);

      MulticastGroupStore.addDevice(req, () => {});
    }

    this.setState({
      mgModalVisible: false,
    });
  };

  handleRelayModalOk = () => {
    for (let devEui of this.state.selectedRowIds) {
      let req = new AddRelayDeviceRequest();
      req.setRelayDevEui(this.state.relaySelected);
      req.setDeviceDevEui(devEui);

      RelayStore.addDevice(req, () => {});
    }

    this.setState({
      relayModalVisible: false,
    });
  };

  render() {
    const menu = (
      <Menu>
        <Menu.Item onClick={this.showMgModal}>Add to multicast-group</Menu.Item>
        <Menu.Item onClick={this.showRelayModal}>Add to relay</Menu.Item>
      </Menu>
    );

    const mgOptions = this.state.multicastGroups.map((mg, i) => (
      <Select.Option value={mg.getId()}>{mg.getName()}</Select.Option>
    ));

    const relayOptions = this.state.relays.map((r, i) => (
      <Select.Option value={r.getDevEui()}>{r.getName()}</Select.Option>
    ));

    return (
      <Space direction="vertical" size="large" style={{ width: "100%" }}>
        <Modal
          title="Add selected devices to multicast-group"
          visible={this.state.mgModalVisible}
          onOk={this.handleMgModalOk}
          onCancel={this.hideMgModal}
          okButtonProps={{ disabled: this.state.mgSelected === "" }}
        >
          <Space direction="vertical" size="large" style={{ width: "100%" }}>
            <Select style={{ width: "100%" }} onChange={this.onMgSelected} placeholder="Select Multicast-group">
              {mgOptions}
            </Select>
          </Space>
        </Modal>
        <Modal
          title="Add selected devices to relay"
          visible={this.state.relayModalVisible}
          onOk={this.handleRelayModalOk}
          onCancel={this.hideRelayModal}
          okButtonProps={{ disabled: this.state.relaySelected === "" }}
        >
          <Space direction="vertical" size="large" style={{ width: "100%" }}>
            <Select style={{ width: "100%" }} onChange={this.onRelaySelected} placeholder="Select Relay">
              {relayOptions}
            </Select>
          </Space>
        </Modal>
        <Admin tenantId={this.props.application.getTenantId()} isDeviceAdmin>
          <Space direction="horizontal" style={{ float: "right" }}>
            <Button type="primary">
              <Link
                to={`/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/devices/create`}
              >
                Add device
              </Link>
            </Button>
            <Dropdown
              placement="bottomRight"
              overlay={menu}
              trigger={["click"]}
              disabled={this.state.selectedRowIds.length === 0}
            >
              <Button>Selected devices</Button>
            </Dropdown>
          </Space>
        </Admin>
        <DataTable
          columns={this.columns()}
          getPage={this.getPage}
          onRowsSelectChange={this.onRowsSelectChange}
          rowKey="devEui"
        />
      </Space>
    );
  }
}

export default ListDevices;
