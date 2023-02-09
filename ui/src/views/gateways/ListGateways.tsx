import React, { Component } from "react";
import { Link } from "react-router-dom";

import moment from "moment";
import { Space, Breadcrumb, Button, PageHeader, Badge, Menu, Modal, TreeSelect, Dropdown } from "antd";
import { ColumnsType } from "antd/es/table";

import {
  ListGatewaysRequest,
  ListGatewaysResponse,
  GatewayListItem,
  GatewayState,
} from "@chirpstack/chirpstack-api-grpc-web/api/gateway_pb";
import {
  ListApplicationsRequest,
  ListApplicationsResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  ListMulticastGroupsRequest,
  ListMulticastGroupsResponse,
  AddGatewayToMulticastGroupRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/multicast_group_pb";
import { Tenant } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";
import GatewayStore from "../../stores/GatewayStore";
import ApplicationStore from "../../stores/ApplicationStore";
import MulticastGroupStore from "../../stores/MulticastGroupStore";
import Admin from "../../components/Admin";

interface IProps {
  tenant: Tenant;
}

interface IState {
  selectedRowIds: string[];
  multicastGroups: any[];
  mgModalVisible: boolean;
  mgSelected: string;
}

class ListGateways extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);
    this.state = {
      selectedRowIds: [],
      multicastGroups: [],
      mgModalVisible: false,
      mgSelected: "",
    };
  }

  componentDidMount() {
    let req = new ListApplicationsRequest();
    req.setLimit(999);
    req.setTenantId(this.props.tenant.getId());

    ApplicationStore.list(req, (resp: ListApplicationsResponse) => {
      for (const app of resp.getResultList()) {
        let req = new ListMulticastGroupsRequest();
        req.setLimit(999);
        req.setApplicationId(app.getId());

        MulticastGroupStore.list(req, (resp: ListMulticastGroupsResponse) => {
          let multicastGroups = this.state.multicastGroups;
          multicastGroups.push({
            title: app.getName(),
            value: "",
            disabled: true,
            children: resp.getResultList().map((mg, i) => ({
              title: mg.getName(),
              value: mg.getId(),
            })),
          });

          this.setState({
            multicastGroups: multicastGroups,
          });
        });
      }
    });
  }

  columns = (): ColumnsType<GatewayListItem.AsObject> => {
    return [
      {
        title: "",
        dataIndex: "state",
        key: "state",
        width: 150,
        render: (text, record) => {
          if (record.state === GatewayState.NEVER_SEEN) {
            return <Badge status="warning" text="Never seen" />;
          } else if (record.state === GatewayState.OFFLINE) {
            return <Badge status="error" text="Offline" />;
          } else if (record.state === GatewayState.ONLINE) {
            return <Badge status="success" text="Online" />;
          }
        },
      },
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
        },
      },
      {
        title: "Gateway ID",
        dataIndex: "gatewayId",
        key: "gatewayId",
        width: 250,
        render: (text, record) => (
          <Link to={`/tenants/${this.props.tenant.getId()}/gateways/${record.gatewayId}`}>{text}</Link>
        ),
      },
      {
        title: "Name",
        dataIndex: "name",
        key: "name",
      },
      {
        title: "Region ID",
        dataIndex: "propertiesMap",
        key: "regionConfigId",
        width: 150,
        render: (text, record) => {
          for (const [k, v] of record.propertiesMap) {
            if (k === "region_config_id") {
              return <Link to={`/regions/${v}`}>{v}</Link>;
            }
          }

          return "";
        },
      },
      {
        title: "Region common-name",
        dataIndex: "propertiesMap",
        key: "regionCommonName",
        width: 250,
        render: (text, record) => {
          for (const [k, v] of record.propertiesMap) {
            if (k === "region_common_name") {
              return v;
            }
          }

          return "";
        },
      },
    ];
  };

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListGatewaysRequest();
    req.setTenantId(this.props.tenant.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    GatewayStore.list(req, (resp: ListGatewaysResponse) => {
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

  hideMgModal = () => {
    this.setState({
      mgModalVisible: false,
    });
  };

  onMgSelected = (value: string) => {
    this.setState({
      mgSelected: value,
    });
  };

  handleMgModalOk = () => {
    for (let gatewayId of this.state.selectedRowIds) {
      let req = new AddGatewayToMulticastGroupRequest();
      req.setMulticastGroupId(this.state.mgSelected);
      req.setGatewayId(gatewayId);

      MulticastGroupStore.addGateway(req, () => {});
    }

    this.setState({
      mgModalVisible: false,
    });
  };

  render() {
    const menu = (
      <Menu>
        <Menu.Item onClick={this.showMgModal}>Add to multicast-group</Menu.Item>
      </Menu>
    );

    return (
      <Space direction="vertical" style={{ width: "100%" }} size="large">
        <Modal
          title="Add selected gateways to multicast-group"
          visible={this.state.mgModalVisible}
          onOk={this.handleMgModalOk}
          onCancel={this.hideMgModal}
          okButtonProps={{ disabled: this.state.mgSelected === "" }}
          bodyStyle={{ height: 300 }}
        >
          <Space direction="vertical" size="large" style={{ width: "100%" }}>
            <TreeSelect
              style={{ width: "100%" }}
              placeholder="Select multicast-group"
              treeData={this.state.multicastGroups}
              onChange={this.onMgSelected}
              treeDefaultExpandAll
            />
          </Space>
        </Modal>
        <PageHeader
          title="Gateways"
          breadcrumbRender={() => (
            <Breadcrumb>
              <Breadcrumb.Item>
                <span>Tenants</span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>
                  <Link to={`/tenants/${this.props.tenant.getId()}`}>{this.props.tenant.getName()}</Link>
                </span>
              </Breadcrumb.Item>
              <Breadcrumb.Item>
                <span>Gateways</span>
              </Breadcrumb.Item>
            </Breadcrumb>
          )}
          extra={[
            <Admin tenantId={this.props.tenant.getId()} isGatewayAdmin>
              <Space direction="horizontal" style={{ float: "right" }}>
                <Button type="primary">
                  <Link to={`/tenants/${this.props.tenant.getId()}/gateways/create`}>Add gateway</Link>
                </Button>
                <Dropdown
                  placement="bottomRight"
                  overlay={menu}
                  trigger={["click"]}
                  disabled={this.state.selectedRowIds.length === 0}
                >
                  <Button>Selected gateways</Button>
                </Dropdown>
              </Space>
            </Admin>,
          ]}
        />
        <DataTable
          columns={this.columns()}
          getPage={this.getPage}
          onRowsSelectChange={this.onRowsSelectChange}
          rowKey="gatewayId"
        />
      </Space>
    );
  }
}

export default ListGateways;
