import { notification } from "antd";
import { EventEmitter } from "events";
import { TenantServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/tenant_grpc_web_pb";
import {
  CreateTenantRequest,
  CreateTenantResponse,
  GetTenantRequest,
  GetTenantResponse,
  UpdateTenantRequest,
  ListTenantsRequest,
  ListTenantsResponse,
  DeleteTenantRequest,
  AddTenantUserRequest,
  GetTenantUserRequest,
  GetTenantUserResponse,
  UpdateTenantUserRequest,
  DeleteTenantUserRequest,
  ListTenantUsersRequest,
  ListTenantUsersResponse,
} from "@chirpstack/chirpstack-api-grpc-web/api/tenant_pb";

import SessionStore from "./SessionStore";
import { HandleError } from "./helpers";

class TenantStore extends EventEmitter {
  client: TenantServiceClient;

  constructor() {
    super();
    this.client = new TenantServiceClient("");
  }

  create = (req: CreateTenantRequest, callbackFunc: (resp: CreateTenantResponse) => void) => {
    this.client.create(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Tenant created",
        duration: 3,
      });

      callbackFunc(resp);
    });
  };

  get = (id: string, callbackFunc: (resp: GetTenantResponse) => void) => {
    let req = new GetTenantRequest();
    req.setId(id);

    this.client.get(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  update = (req: UpdateTenantRequest, callbackFunc: () => void) => {
    this.client.update(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Tenant updated",
        duration: 3,
      });

      this.emit("change");

      callbackFunc();
    });
  };

  delete = (req: DeleteTenantRequest, callbackFunc: () => void) => {
    this.client.delete(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Tenant deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  list = (req: ListTenantsRequest, callbackFunc: (resp: ListTenantsResponse) => void) => {
    this.client.list(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  addUser = (req: AddTenantUserRequest, callbackFunc: () => void) => {
    this.client.addUser(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Tenant user added",
        duration: 3,
      });

      callbackFunc();
    });
  };

  getUser = (req: GetTenantUserRequest, callbackFunc: (resp: GetTenantUserResponse) => void) => {
    this.client.getUser(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };

  updateUser = (req: UpdateTenantUserRequest, callbackFunc: () => void) => {
    this.client.updateUser(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Tenant user updated",
        duration: 3,
      });

      callbackFunc();
    });
  };

  deleteUser = (req: DeleteTenantUserRequest, callbackFunc: () => void) => {
    this.client.deleteUser(req, SessionStore.getMetadata(), err => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      notification.success({
        message: "Tenant user deleted",
        duration: 3,
      });

      callbackFunc();
    });
  };

  listUsers = (req: ListTenantUsersRequest, callbackFunc: (resp: ListTenantUsersResponse) => void) => {
    this.client.listUsers(req, SessionStore.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      callbackFunc(resp);
    });
  };
}

const tenantStore = new TenantStore();
export default tenantStore;
