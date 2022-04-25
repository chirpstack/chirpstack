import { notification } from "antd";
import { Error } from "grpc-web";

import history from "../history";

export function HandleError(e: Error) {
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

export function HandleLoginError(e: Error) {
  console.log("API error: ", e);
  notification.error({
    message: "Error",
    description: e.message,
    duration: 3,
  });
}
