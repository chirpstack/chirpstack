import React, { Component } from "react";
import { Link } from "react-router-dom";

import { ColumnsType } from "antd/es/table";

import { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import {
  ListRelaysRequest,
  ListRelaysResponse,
  RelayListItem,
} from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";
import RelayStore from "../../stores/RelayStore";
import DataTable, { GetPageCallbackFunc } from "../../components/DataTable";

interface IProps {
  application: Application;
}

interface IState {

}

class ListRelays extends Component<IProps, IState> {
  columns = (): ColumnsType<RelayListItem.AsObject> => {
    return [
      {
        title: "DevEUI",
        dataIndex: "devEui",
        key: "devEui",
        width: 250,
        render: (text, record) => (
          <Link
            to={`/tenants/${this.props.application.getTenantId()}/applications/${this.props.application.getId()}/relays/${
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
    ];
  }

  getPage = (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => {
    let req = new ListRelaysRequest();
    req.setApplicationId(this.props.application.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    RelayStore.list(req, (resp: ListRelaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  }

  render() {
    return (
      <DataTable
        columns={this.columns()}
        getPage={this.getPage}
        rowKey="devEui"
      />
    );
  }
}

export default ListRelays;
