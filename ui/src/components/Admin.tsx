import { Component } from "react";

import SessionStore from "../stores/SessionStore";

interface IProps {
  tenantId?: string;
  isDeviceAdmin?: boolean;
  isGatewayAdmin?: boolean;
  isTenantAdmin?: boolean;
}

interface IState {
  admin: boolean;
}

class Admin extends Component<IProps, IState> {
  constructor(props: IProps) {
    super(props);

    this.state = {
      admin: false,
    };
  }

  componentDidMount() {
    SessionStore.on("change", this.setIsAdmin);
    this.setIsAdmin();
  }

  componentWillUnmount() {
    SessionStore.removeListener("change", this.setIsAdmin);
  }

  componentDidUpdate(prevProps: IProps) {
    if (prevProps === this.props) {
      return;
    }

    this.setIsAdmin();
  }

  setIsAdmin = () => {
    if (!this.props.isDeviceAdmin && !this.props.isGatewayAdmin && !this.props.isTenantAdmin) {
      this.setState({
        admin: SessionStore.isAdmin(),
      });
    } else {
      if (this.props.tenantId === undefined) {
        throw new Error("No tenantId is given");
      }

      if (this.props.isTenantAdmin) {
        this.setState({
          admin: SessionStore.isAdmin() || SessionStore.isTenantAdmin(this.props.tenantId),
        });
      }

      if (this.props.isDeviceAdmin) {
        this.setState({
          admin: SessionStore.isAdmin() || SessionStore.isTenantDeviceAdmin(this.props.tenantId),
        });
      }

      if (this.props.isGatewayAdmin) {
        this.setState({
          admin: SessionStore.isAdmin() || SessionStore.isTenantGatewayAdmin(this.props.tenantId),
        });
      }
    }
  };

  render() {
    if (this.state.admin) {
      return this.props.children;
    }

    return null;
  }
}

export default Admin;
