import { notification } from "antd";
import type { RpcError } from "grpc-web";
import { useRef, useEffect } from "react";

import history from "../history";

export function HandleError(e: RpcError) {
  console.log("API error: ", e);

  if (e.code === 16 || e.code === 2) {
    history.push("/login");
    return;
  }

  notification.error({
    message: "Error",
    description: e.message,
    duration: 3,
  });
}

export function HandleLoginError(e: RpcError) {
  console.log("API error: ", e);
  notification.error({
    message: "Error",
    description: e.message,
    duration: 3,
  });
}

export function useTitle(...v: unknown[]) {
  const documentDefined = typeof document !== 'undefined';
  const originalTitle = useRef(documentDefined ? document.title : null);

  useEffect(() => {
    if (!documentDefined) return;

    const title = [...v, 'ChirpStack LoRaWAN® Network-Server'].join(' | ');

    if (document.title !== title) { document.title = title; }

    return () => {
      if (originalTitle.current) {
        // eslint-disable-next-line react-hooks/exhaustive-deps
        document.title = originalTitle.current;
      }
    };
  }, [documentDefined, v]);
}