import React, { useState, useEffect, useCallback } from "react";

import { Table } from "antd";
import type { ColumnsType } from "antd/es/table";

import SessionStore from "../stores/SessionStore";
import {FilterValue, SorterResult, SortOrder, TableCurrentDataSource} from "antd/es/table/interface";
import {TablePaginationConfig} from "antd/lib";

export type GetPageCallbackFunc = (totalCount: number, rows: object[]) => void;

interface IProps {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  columns: ColumnsType<any>;
  getPage: (limit: number, offset: number, orderBy: string | void, callbackFunc: GetPageCallbackFunc) => void;
  onRowsSelectChange?: (ids: string[]) => void;
  rowKey: string;
  refreshKey?: unknown;
  noPagination?: boolean;
}

function DataTable(props: IProps) {
  const [totalCount, setTotalCount] = useState<number>(0);
  const [pageSize, setPageSize] = useState<number>(SessionStore.getRowsPerPage());
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [orderBy, setOrderBy] = useState<string>("name");
  const [rows, setRows] = useState<object[]>([]);
  const [loading, setLoading] = useState<boolean>(true);

  const loadPage = useCallback(
    (page: number, pz: number, orderBy?: string | void) => {
      setLoading(true);

      props.getPage(pz, (page - 1) * pz, orderBy, (totalCount: number, rows: object[]) => {
        setTotalCount(totalCount);
        setRows(rows);
        setLoading(false);
      });
    },
    [props, pageSize],
  );

  const onRowsSelectChange = (ids: React.Key[]) => {
    const idss = ids as string[];
    if (props.onRowsSelectChange) {
      props.onRowsSelectChange(idss);
    }
  };

  const onChange = (
      pagination: TablePaginationConfig,
      filters: Record<string, FilterValue | null>,
      sorter: SorterResult<object> | SorterResult<object>[],
      extra: TableCurrentDataSource<object>
  ) => {
    let page = pagination.current;
    if (!page) {
      page = currentPage;
    }

    let pz = pagination.pageSize;
    if (!pz) {
      pz = pageSize;
    }
    SessionStore.setRowsPerPage(pz);

    let firstSorter: SorterResult<object> | void = undefined;
    if (Array.isArray(sorter)) {
      if (sorter.length) {
        firstSorter = sorter[0];
      }
    } else {
      firstSorter = sorter;
    }
    let sort: string | void = undefined;
    if (firstSorter) {
      if (firstSorter.columnKey) {
        sort = firstSorter.columnKey.toString();
        if (firstSorter.order === "descend") {
          sort += ",desc";
        }
      }
    }
    if (!sort) {
      sort = orderBy;
    }

    setCurrentPage(page);
    setPageSize(pz || 0);
    setOrderBy(sort);
  };

  useEffect(() => {
    loadPage(currentPage, pageSize, orderBy);
  }, [props, currentPage, pageSize, orderBy, loadPage]);

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
      showSizeChanger: true,
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
      onChange={onChange}
      {...otherProps}
    />
  );
}

export default DataTable;
