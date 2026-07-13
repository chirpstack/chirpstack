import type { PropsWithChildren } from "react";
import { useState, useEffect } from "react";

import SessionStore from "../stores/SessionStore";

interface IProps {
  tenantId?: string;
  applicationId?: string;
  deviceProfileId?: string;
  isDeviceAdmin?: boolean;
  isGatewayAdmin?: boolean;
  isTenantAdmin?: boolean;
  isApplicationAdmin?: boolean;
  isApplicationAdminRo?: boolean;
  isDeviceProfileAdmin?: boolean;
}

function Admin(props: PropsWithChildren<IProps>) {
  const [admin, setAdmin] = useState<boolean>(false);

  useEffect(() => {
    const setIsAdmin = () => {
      if (
        !props.isDeviceAdmin &&
        !props.isGatewayAdmin &&
        !props.isTenantAdmin &&
        !props.isApplicationAdmin &&
        !props.isApplicationAdminRo &&
        !props.isDeviceProfileAdmin
      ) {
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

        if (props.applicationId) {
          if (props.isApplicationAdmin) {
            setAdmin(
              SessionStore.isAdmin() ||
                SessionStore.isTenantDeviceAdmin(props.tenantId) ||
                SessionStore.isApplicationAdmin(props.applicationId),
            );
          }

          if (props.isApplicationAdminRo) {
            setAdmin(
              SessionStore.isAdmin() ||
                SessionStore.isTenantDeviceAdmin(props.tenantId) ||
                SessionStore.isApplicationAdmin(props.applicationId) ||
                SessionStore.isApplicationAdminRo(props.applicationId),
            );
          }
        }

        if (props.deviceProfileId) {
          if (props.isDeviceProfileAdmin) {
            setAdmin(
              SessionStore.isAdmin() ||
                SessionStore.isTenantDeviceAdmin(props.tenantId) ||
                SessionStore.isDeviceProfileAdmin(props.deviceProfileId),
            );
          }
        }
      }
    };

    SessionStore.on("change", setIsAdmin);
    setIsAdmin();

    return () => {
      SessionStore.removeListener("change", setIsAdmin);
    };
  }, [props]);

  if (admin) {
    return <>{props.children}</>;
  }

  return null;
}

export default Admin;
