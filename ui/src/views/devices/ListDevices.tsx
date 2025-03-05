import { useState, useEffect } from "react";
import { Link } from "react-router-dom";

import { format } from "date-fns";
import { Space, Button, Dropdown, Menu, Modal, Select, Tag, Popover, Typography } from "antd";
import type { ColumnsType } from "antd/es/table";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faPlug,
  faBatteryFull,
  faBatteryQuarter,
  faBatteryHalf,
  faBatteryThreeQuarters,
} from "@fortawesome/free-solid-svg-icons";

import {
  Application,
  ListApplicationDeviceProfilesResponse,
  ApplicationDeviceProfileListItem,
  ListApplicationDeviceTagsResponse,
  ApplicationDeviceTagListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type { ListDevicesResponse, DeviceListItem } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import {
  ListApplicationDeviceProfilesRequest,
  ListApplicationDeviceTagsRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import { ListDevicesRequest } from "@chirpstack/chirpstack-api-grpc-web/api/device_pb";
import type {
  ListMulticastGroupsResponse,
  MulticastGroupListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import {
  ListMulticastGroupsRequest,
  AddDeviceToMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import type { ListRelaysResponse, RelayListItem } from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";
import { ListRelaysRequest, AddRelayDeviceRequest } from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";
import type {
  ListFuotaDeploymentsResponse,
  FuotaDeploymentListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";
import {
  ListFuotaDeploymentsRequest,
  AddDevicesToFuotaDeploymentRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";
import DeviceStore from "../../stores/DeviceStore";
import ApplicationStore from "../../stores/ApplicationStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import FuotaStore from "../../stores/FuotaStore";
import RelayStore from "../../stores/RelayStore";
import Admin from "../../components/Admin";

interface IProps {
  application: Application;
}

function ListDevices(props: IProps) {
  const [selectedRowIds, setSelectedRowIds] = useState<string[]>([]);
  const [multicastGroups, setMulticastGroups] = useState<MulticastGroupListItem[]>([]);
  const [relays, setRelays] = useState<RelayListItem[]>([]);
  const [fuotaDeployments, setFuotaDeployments] = useState<FuotaDeploymentListItem[]>([]);
  const [mgModalVisible, setMgModalVisible] = useState<boolean>(false);
  const [fuotaModalVisible, setFuotaModalVisible] = useState<boolean>(false);
  const [relayModalVisible, setRelayModalVisible] = useState<boolean>(false);
  const [mgSelected, setMgSelected] = useState<string>("");
  const [fuotaDeploymentSelected, setFuotaDeploymentSelected] = useState<string>("");
  const [relaySelected, setRelaySelected] = useState<string>("");
  const [applicationDeviceProfiles, setApplicationDeviceProfiles] = useState<ApplicationDeviceProfileListItem[]>([]);
  const [applicationDeviceTags, setApplicationDeviceTags] = useState<ApplicationDeviceTagListItem[]>([]);

  useEffect(() => {
    const appDpReq = new ListApplicationDeviceProfilesRequest();
    appDpReq.setApplicationId(props.application.getId());
    ApplicationStore.listDeviceProfiles(appDpReq, (resp: ListApplicationDeviceProfilesResponse) => {
      setApplicationDeviceProfiles(resp.getResultList());
    });

    const appDevTagsReq = new ListApplicationDeviceTagsRequest();
    appDevTagsReq.setApplicationId(props.application.getId());
    ApplicationStore.listDeviceTags(appDevTagsReq, (resp: ListApplicationDeviceTagsResponse) => {
      setApplicationDeviceTags(resp.getResultList());
    });

    const mgReq = new ListMulticastGroupsRequest();
    mgReq.setLimit(999);
    mgReq.setApplicationId(props.application.getId());
    MulticastGroupStore.list(mgReq, (resp: ListMulticastGroupsResponse) => {
      setMulticastGroups(resp.getResultList());
    });

    const relayReq = new ListRelaysRequest();
    relayReq.setLimit(999);
    relayReq.setApplicationId(props.application.getId());
    RelayStore.list(relayReq, (resp: ListRelaysResponse) => {
      setRelays(resp.getResultList());
    });

    const fuotaReq = new ListFuotaDeploymentsRequest();
    fuotaReq.setLimit(999);
    fuotaReq.setApplicationId(props.application.getId());
    FuotaStore.listDeployments(fuotaReq, (resp: ListFuotaDeploymentsResponse) => {
      setFuotaDeployments(resp.getResultList());
    });
  }, [props]);

  const columns: ColumnsType<DeviceListItem.AsObject> = [
    {
      title: "Last seen",
      dataIndex: "lastSeenAt",
      key: "lastSeenAt",
      width: 250,
      render: (_text, record) => {
        if (record.lastSeenAt !== undefined) {
          const ts = new Date(0);
          ts.setUTCSeconds(record.lastSeenAt.seconds);
          return format(ts, "yyyy-MM-dd HH:mm:ss");
        }
        return "Never";
      },
      sorter: true,
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
      sorter: true,
    },
    {
      title: "Name",
      dataIndex: "name",
      key: "name",
      sorter: true,
    },
    {
      title: "Device profile",
      dataIndex: "deviceProfileName",
      key: "deviceProfileId",
      render: (text, record) => (
        <Link to={`/tenants/${props.application.getTenantId()}/device-profiles/${record.deviceProfileId}/edit`}>
          {text}
        </Link>
      ),
      sorter: true,
      filterMultiple: false,
      filters: applicationDeviceProfiles.map(v => {
        return {
          text: v.getName(),
          value: v.getId(),
        };
      }),
    },
    {
      title: "Tags",
      dataIndex: "tagsMap",
      key: "tags",
      render: (text, record) => (
        <>
          {text.map((v: string[]) => (
            <Popover content={v[1]}>
              <Tag>{v[0]}</Tag>
            </Popover>
          ))}
        </>
      ),
      filterMultiple: false,
      filters: applicationDeviceTags.map(v => {
        return {
          text: v.getKey(),
          value: v.getKey(),
          children: v.getValuesList().map(vv => {
            return {
              text: vv,
              value: `${v.getKey()}=${vv}`,
            };
          }),
        };
      }),
    },
    {
      title: "Battery",
      dataIndex: "deviceStatus",
      key: "deviceStatus",
      width: 50,
      render: (_text, record) => {
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

  const getPage = (
    limit: number,
    offset: number,
    filters: object,
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    let f = filters as any;

    function getOrderBy(orderBy: string | void): ListDevicesRequest.OrderBy {
      switch (orderBy) {
        case "lastSeenAt":
          return ListDevicesRequest.OrderBy.LAST_SEEN_AT;
        case "deviceProfileName":
          return ListDevicesRequest.OrderBy.DEVICE_PROFILE_NAME;
        case "devEui":
          return ListDevicesRequest.OrderBy.DEV_EUI;
        default:
          return ListDevicesRequest.OrderBy.NAME;
      }
    }

    function getDeviceProfileId(filters: any): string {
      let dpIdFilter = filters.deviceProfileId;
      if (Array.isArray(dpIdFilter) && dpIdFilter.length > 0) {
        return dpIdFilter[0] as string;
      }

      return "";
    }

    const req = new ListDevicesRequest();
    req.setApplicationId(props.application.getId());
    req.setDeviceProfileId(getDeviceProfileId(f));
    req.setLimit(limit);
    req.setOffset(offset);
    req.setOrderBy(getOrderBy(orderBy));
    req.setOrderByDesc(orderByDesc || false);

    {
      let tagsFilter = f.tags;
      if (Array.isArray(tagsFilter)) {
        tagsFilter.forEach(v => {
          const parts = v.split("=");
          req.getTagsMap().set(parts[0], parts[1]);
        });
      }
    }

    console.log(req.toObject());

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
    for (const devEui of selectedRowIds) {
      const req = new AddDeviceToMulticastGroupRequest();
      req.setMulticastGroupId(mgSelected);
      req.setDevEui(devEui);

      MulticastGroupStore.addDevice(req, () => {});
    }

    setMgModalVisible(false);
  };

  const handleRelayModalOk = () => {
    for (const devEui of selectedRowIds) {
      const req = new AddRelayDeviceRequest();
      req.setRelayDevEui(relaySelected);
      req.setDeviceDevEui(devEui);

      RelayStore.addDevice(req, () => {});
    }

    setRelayModalVisible(false);
  };

  const handleFuotaModalOk = () => {
    const req = new AddDevicesToFuotaDeploymentRequest();
    req.setFuotaDeploymentId(fuotaDeploymentSelected);
    req.setDevEuisList(selectedRowIds);

    FuotaStore.addDevices(req, () => {});
    setFuotaModalVisible(false);
  };

  const menu = (
    <Menu>
      <Menu.Item onClick={showMgModal}>Add to multicast-group</Menu.Item>
      <Menu.Item onClick={() => setFuotaModalVisible(true)}>Add to FUOTA deployment</Menu.Item>
      <Menu.Item onClick={showRelayModal}>Add to relay</Menu.Item>
    </Menu>
  );

  const mgOptions = multicastGroups.map(mg => <Select.Option value={mg.getId()}>{mg.getName()}</Select.Option>);
  const relayOptions = relays.map(r => <Select.Option value={r.getDevEui()}>{r.getName()}</Select.Option>);
  const fuotaOptions = fuotaDeployments.map(r => <Select.Option value={r.getId()}>{r.getName()}</Select.Option>);

  return (
    <Space direction="vertical" size="large" style={{ width: "100%" }}>
      <Modal
        title="Add selected devices to multicast-group"
        open={mgModalVisible}
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
        title="Add selected devices to FUOTA deployment"
        open={fuotaModalVisible}
        onOk={handleFuotaModalOk}
        onCancel={() => setFuotaModalVisible(false)}
        okButtonProps={{ disabled: fuotaDeploymentSelected === "" }}
      >
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <Typography.Text>
            This will add the selected devices to a FUOTA deployment. Devices must have the same device-profile as
            associated with the FUOTA deployment.
          </Typography.Text>
          <Select
            style={{ width: "100%" }}
            onChange={v => setFuotaDeploymentSelected(v)}
            placeholder="Select FUOTA deployment"
          >
            {fuotaOptions}
          </Select>
        </Space>
      </Modal>
      <Modal
        title="Add selected devices to relay"
        open={relayModalVisible}
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
