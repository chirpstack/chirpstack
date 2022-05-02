import { notification } from "antd";
import { EventEmitter } from "events";
import { UserServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/user_grpc_web_pb";
import {
  CreateUserRequest,
  CreateUserResponse,
  GetUserRequest,
  GetUserResponse,
  UpdateUserRequest,
  DeleteUserRequest,
  ListUsersRequest,
  ListUsersResponse,
  UpdateUserPasswordRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class UserStore extends EventEmitter {
  client: UserServiceClient;

  constructor() {
    super();
    this.client = new UserServiceClient("");
  }

  create = (req: CreateUserRequest, callbackFunc: (resp: CreateUserResponse) => void) => {
    this.client.create(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "User created",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };

  get = (req: GetUserRequest, callbackFunc: (resp: GetUserResponse) => void) => {
    this.client.get(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  update = (req: UpdateUserRequest, callbackFunc: () => void) => {
    this.client.update(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "User updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  delete = (req: DeleteUserRequest, callbackFunc: () => void) => {
    this.client.delete(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "User deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  list = (req: ListUsersRequest, callbackFunc: (resp: ListUsersResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err != null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updatePassword = (req: UpdateUserPasswordRequest, callbackFunc: () => void) => {
    this.client.updatePassword(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "User password updated",
        duration: 3,
      });

      callbackFunc();
    });
  };
}

const userStore = new UserStore();
export default userStore;
