import React, { useState, useEffect } from "react";

import { Table } from "antd";
import { ColumnsType } from "antd/es/table";

import SessionStore from "../stores/SessionStore";

export type GetPageCallbackFunc = (totalCount: number, rows: object[]) => void;

interface IProps {
  columns: ColumnsType<any>;
  getPage: (limit: number, offset: number, callbackFunc: GetPageCallbackFunc) => void;
  onRowsSelectChange?: (ids: string[]) => void;
  rowKey: string;
  refreshKey?: any;
  noPagination?: boolean;
}

function DataTable(props: IProps) {
  const [totalCount, setTotalCount] = useState<number>(0);
  const [pageSize, setPageSize] = useState<number>(SessionStore.getRowsPerPage());
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [rows, setRows] = useState<object[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  const onChangePage = (page: number, pz?: number | void) => {
    setLoading(true);

    if (!pz) {
      pz = pageSize;
    }

    props.getPage(pz, (page - 1) * pz, (totalCount: number, rows: object[]) => {
      setCurrentPage(page);
      setTotalCount(totalCount);
      setRows(rows);
      setPageSize(pz || 0);
      setLoading(false);
    });
  };

  const onShowSizeChange = (page: number, pageSize: number) => {
    onChangePage(page, pageSize);
    SessionStore.setRowsPerPage(pageSize);
  };

  const onRowsSelectChange = (ids: React.Key[]) => {
    const idss = ids as string[];
    if (props.onRowsSelectChange) {
      props.onRowsSelectChange(idss);
    }
  };

  useEffect(() => {
    onChangePage(currentPage, pageSize);
  }, [props, currentPage, pageSize]);

  const { getPage, refreshKey, ...otherProps } = props;
  let loadingProps = undefined;
  if (loading) {
    loadingProps = {
      delay: 300,
    };
  }

  let pagination = undefined;
  if (props.noPagination === undefined || props.noPagination === false) {
    pagination = {
      current: currentPage,
      total: totalCount,
      pageSize: pageSize,
      onChange: onChangePage,
      showSizeChanger: true,
      onShowSizeChange: onShowSizeChange,
    };
  }

  let rowSelection = undefined;
  if (props.onRowsSelectChange) {
    rowSelection = {
      onChange: onRowsSelectChange,
    };
  }

  return (
    <Table
      loading={loadingProps}
      dataSource={rows}
      pagination={pagination || false}
      rowSelection={rowSelection}
      {...otherProps}
    />
  );
}

export default DataTable;
