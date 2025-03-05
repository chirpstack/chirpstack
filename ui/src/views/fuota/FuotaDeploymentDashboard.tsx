import { useState, useEffect } from "react";

import { Spin, Button, Space, Timeline, Row, Col, TimelineProps, Card, Tag, Popover, Table } from "antd";
import { LoadingOutlined, ReloadOutlined } from "@ant-design/icons";
import type { ColumnsType } from "antd/es/table";
import { format } from "date-fns";

import { format_dt, format_dt_from_secs } from "../helpers";

import { ListFuotaDeploymentJobsRequest } from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import {
  GetFuotaDeploymentResponse,
  ListFuotaDeploymentJobsResponse,
  FuotaDeploymentJob,
} from "@chirpstack/chirpstack-api-grpc-web/api/fuota_pb";

import FuotaStore from "../../stores/FuotaStore";

interface IProps {
  getFuotaDeploymentResponse: GetFuotaDeploymentResponse;
}

function FuotaDeploymentDashboard(props: IProps) {
  const [fuotaJobs, setFuotaJobs] = useState<FuotaDeploymentJob.AsObject[]>([]);

  useEffect(() => {
    getFuotaJobs();

    const interval = setInterval(() => {
      if (!props.getFuotaDeploymentResponse.getCompletedAt()) {
        getFuotaJobs();
      }
    }, 10000);

    return () => clearInterval(interval);
  }, [props.getFuotaDeploymentResponse]);

  const jobs: Record<string, string> = {
    CREATE_MC_GROUP: "Create multicast group",
    ADD_DEVS_TO_MC_GROUP: "Add devices to multicast group",
    ADD_GWS_TO_MC_GROUP: "Add gateways to multicast group",
    MC_GROUP_SETUP: "Multicast group setup",
    FRAG_SESSION_SETUP: "Fragmentation session setup",
    MC_SESSION: "Multicast session setup",
    ENQUEUE: "Enqueue fragments",
    FRAG_STATUS: "Request fragmentation status",
    COMPLETE: "Complete deployment",
  };

  const columns: ColumnsType<FuotaDeploymentJob.AsObject> = [
    {
      title: "Status",
      key: "status",
      width: 100,
      render: (_text, record) => {
        if (record.errorMsg !== "") {
          return (
            <Popover content={record.errorMsg} placement="right">
              <Tag color="red">error</Tag>
            </Popover>
          );
        } else if (!record.completedAt) {
          return <Spin indicator={<LoadingOutlined spin />} size="small" />;
        } else {
          return <Tag color="green">ok</Tag>;
        }
      },
    },
    {
      title: "Job",
      dataIndex: "job",
      key: "job",
      render: text => jobs[text],
      width: 250,
    },
    {
      title: "Created at",
      dataIndex: "createdAt",
      key: "createdAt",
      render: (_text, record) => format_dt_from_secs(record.createdAt?.seconds),
      width: 250,
    },
    {
      title: "Completed at",
      dataIndex: "completedAt",
      key: "completedAt",
      render: (_text, record) => format_dt_from_secs(record.completedAt?.seconds),
      width: 250,
    },
    {
      title: "Attempt count",
      dataIndex: "attemptCount",
      key: "attemptCount",
      width: 150,
    },
    {
      title: "Max. retry",
      dataIndex: "maxRetryCount",
      key: "maxRetryCount",
      width: 150,
    },
  ];

  const getFuotaJobs = () => {
    const req = new ListFuotaDeploymentJobsRequest();
    req.setFuotaDeploymentId(props.getFuotaDeploymentResponse.getDeployment()!.getId());
    FuotaStore.listJobs(req, (resp: ListFuotaDeploymentJobsResponse) => {
      const obj = resp.toObject();
      setFuotaJobs(obj.jobsList);
    });
  };

  let loadingProps = undefined;
  if (props.getFuotaDeploymentResponse.getStartedAt() && fuotaJobs.length === 0) {
    loadingProps = {
      delay: 300,
    };
  }

  return (
    <Space style={{ width: "100%" }} direction="vertical">
      <Table loading={loadingProps} dataSource={fuotaJobs} columns={columns} pagination={false} />
    </Space>
  );
}

export default FuotaDeploymentDashboard;
