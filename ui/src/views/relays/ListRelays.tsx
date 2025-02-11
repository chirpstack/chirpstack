import { Link } from "react-router-dom";

import type { ColumnsType } from "antd/es/table";

import type { Application } from "@chirpstack/chirpstack-api-grpc-web/api/application_pb";
import type { ListRelaysResponse, RelayListItem } from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";
import { ListRelaysRequest } from "@chirpstack/chirpstack-api-grpc-web/api/relay_pb";
import RelayStore from "../../stores/RelayStore";
import type { GetPageCallbackFunc } from "../../components/DataTable";
import DataTable from "../../components/DataTable";

interface IProps {
  application: Application;
}

function ListRelays(props: IProps) {
  const columns: ColumnsType<RelayListItem.AsObject> = [
    {
      title: "DevEUI",
      dataIndex: "devEui",
      key: "devEui",
      width: 250,
      render: (text, record) => (
        <Link
          to={`/tenants/${props.application.getTenantId()}/applications/${props.application.getId()}/relays/${
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

  const getPage = (
    limit: number,
    offset: number,
    _filters: object,
    orderBy: string | void,
    orderByDesc: boolean | void,
    callbackFunc: GetPageCallbackFunc,
  ) => {
    const req = new ListRelaysRequest();
    req.setApplicationId(props.application.getId());
    req.setLimit(limit);
    req.setOffset(offset);

    RelayStore.list(req, (resp: ListRelaysResponse) => {
      const obj = resp.toObject();
      callbackFunc(obj.totalCount, obj.resultList);
    });
  };

  return <DataTable columns={columns} getPage={getPage} rowKey="devEui" />;
}

export default ListRelays;
