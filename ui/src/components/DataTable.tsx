import React, { Component } from "react";

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

interface IState {
  totalCount: number;
  pageSize: number;
  currentPage: number;
  rows: object[];
  loading: boolean;
}

class DataTable extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      totalCount: 0,
      pageSize: SessionStore.getRowsPerPage(),
      currentPage: 1,
      rows: [],
      loading: true,
    };
  }

  componentDidMount() {
    this.onChangePage(this.state.currentPage, this.state.pageSize);
  }

  componentDidUpdate(prevProps: IProps) {
    if (this.props === prevProps) {
      return;
    }

    this.onChangePage(this.state.currentPage, this.state.pageSize);
  }

  onChangePage = (page: number, pageSize?: number | void) => {
    this.setState(
      {
        loading: true,
      },
      () => {
        let pz = pageSize;
        if (!pz) {
          pz = this.state.pageSize;
        }

        this.props.getPage(pz, (page - 1) * pz, (totalCount: number, rows: object[]) => {
          this.setState({
            currentPage: page,
            totalCount: totalCount,
            rows: rows,
            pageSize: pz || 0,
            loading: false,
          });
        });
      },
    );
  };

  onShowSizeChange = (page: number, pageSize: number) => {
    this.onChangePage(page, pageSize);
    SessionStore.setRowsPerPage(pageSize);
  };

  onRowsSelectChange = (ids: React.Key[]) => {
    const idss = ids as string[];
    if (this.props.onRowsSelectChange) {
      this.props.onRowsSelectChange(idss);
    }
  };

  render() {
    const { getPage, refreshKey, ...otherProps } = this.props;
    let loadingProps = undefined;
    if (this.state.loading) {
      loadingProps = {
        delay: 300,
      };
    }

    let pagination = undefined;
    if (this.props.noPagination === undefined || this.props.noPagination === false) {
      pagination = {
        current: this.state.currentPage,
        total: this.state.totalCount,
        pageSize: this.state.pageSize,
        onChange: this.onChangePage,
        showSizeChanger: true,
        onShowSizeChange: this.onShowSizeChange,
      };
    }

    let rowSelection = undefined;
    if (this.props.onRowsSelectChange) {
      rowSelection = {
        onChange: this.onRowsSelectChange,
      };
    }

    return (
      <Table
        loading={loadingProps}
        dataSource={this.state.rows}
        pagination={pagination || false}
        rowSelection={rowSelection}
        {...otherProps}
      />
    );
  }
}

export default DataTable;
