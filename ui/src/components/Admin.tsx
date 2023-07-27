import React, { PropsWithChildren, useState, useEffect } from "react";

import SessionStore from "../stores/SessionStore";

interface IProps {
  tenantId?: string;
  isDeviceAdmin?: boolean;
  isGatewayAdmin?: boolean;
  isTenantAdmin?: boolean;
}

function Admin(props: PropsWithChildren<IProps>) {
  const [admin, setAdmin] = useState<boolean>(false);

  const setIsAdmin = () => {
    if (!props.isDeviceAdmin && !props.isGatewayAdmin && !props.isTenantAdmin) {
      setAdmin(SessionStore.isAdmin());
    } else {
      if (props.tenantId === undefined) {
        throw new Error("No tenantId is given");
      }

      if (props.isTenantAdmin) {
        setAdmin(SessionStore.isAdmin() || SessionStore.isTenantAdmin(props.tenantId));
      }

      if (props.isDeviceAdmin) {
        setAdmin(SessionStore.isAdmin() || SessionStore.isTenantDeviceAdmin(props.tenantId));
      }

      if (props.isGatewayAdmin) {
        setAdmin(SessionStore.isAdmin() || SessionStore.isTenantGatewayAdmin(props.tenantId));
      }
    }
  };

  useEffect(() => {
    SessionStore.on("change", setIsAdmin);
    setIsAdmin();

    return () => {
      SessionStore.removeListener("change", setIsAdmin);
    };
  }, [props]);

  if (admin) {
    return <div>{props.children}</div>;
  }

  return null;
}

export default Admin;
