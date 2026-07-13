import google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import type { Metadata } from "grpc-web";

import { EventEmitter } from "events";
import { InternalServiceClient } from "@chirpstack/chirpstack-api-grpc-web/api/internal_grpc_web_pb";
import type {
  UserApplicationLink,
  UserDeviceProfileLink,
  UserTenantLink,
  OpenIdConnectLoginRequest,
  OAuth2LoginRequest,
} from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import { LoginRequest } from "@chirpstack/chirpstack-api-grpc-web/api/internal_pb";
import type { User } from "@chirpstack/chirpstack-api-grpc-web/api/user_pb";

import { HandleError, HandleLoginError } from "./helpers";

class SessionStore extends EventEmitter {
  client: InternalServiceClient;
  user?: User;
  tenants: UserTenantLink[];
  applications: UserApplicationLink[];
  deviceProfiles: UserDeviceProfileLink[];

  constructor() {
    super();

    this.client = new InternalServiceClient("");
    this.tenants = [];
    this.applications = [];
    this.deviceProfiles = [];

    this.fetchProfile(() => {});
  }

  login = (email: string, password: string, callbackFunc: () => void) => {
    const req = new LoginRequest();
    req.setEmail(email);
    req.setPassword(password);
    this.client.login(req, {}, (err, resp) => {
      if (err !== null) {
        HandleLoginError(err);
        return;
      }

      this.setToken(resp.getJwt());
      this.fetchProfile(callbackFunc);
    });
  };

  openIdConnectLogin = (req: OpenIdConnectLoginRequest, callbackFunc: () => void) => {
    this.client.openIdConnectLogin(req, {}, (err, resp) => {
      if (err !== null) {
        HandleLoginError(err);
        return;
      }

      this.setToken(resp.getToken());
      this.fetchProfile(callbackFunc);
    });
  };

  oAuth2Login = (req: OAuth2LoginRequest, callbackFunc: () => void) => {
    this.client.oAuth2Login(req, {}, (err, resp) => {
      if (err !== null) {
        HandleLoginError(err);
        return;
      }

      this.setToken(resp.getToken());
      this.fetchProfile(callbackFunc);
    });
  };

  logout = (emit: boolean, callbackFunc: () => void) => {
    localStorage.clear();
    this.user = undefined;
    this.tenants = [];

    if (emit === true) {
      this.emit("change");
    }

    callbackFunc();
  };

  setToken = (s: string) => {
    localStorage.setItem("token", s);
  };

  getToken = (): string => {
    const token = localStorage.getItem("token");
    if (token == null) {
      return "";
    }
    return token;
  };

  getTenantId = (): string => {
    return localStorage.getItem("tenantId") || "";
  };

  setTenantId = (id: string) => {
    console.log("tenantId set to", id);
    localStorage.setItem("tenantId", id);
    this.emit("tenant.change");
  };

  getRowsPerPage = (): number => {
    return parseInt(localStorage.getItem("rowsPerPage") || "10", 10);
  };

  setRowsPerPage = (count: number) => {
    localStorage.setItem("rowsPerPage", count.toString());
  };

  getMetadata = (): Metadata => {
    if (this.getToken() === "") {
      return {};
    }

    return {
      authorization: "Bearer " + this.getToken(),
    };
  };

  fetchProfile = (callbackFunc: () => void) => {
    if (this.getToken() === "") {
      return;
    }

    this.client.profile(new google_protobuf_empty_pb.Empty(), this.getMetadata(), (err, resp) => {
      if (err !== null) {
        HandleError(err);
        return;
      }

      this.user = resp.getUser();
      this.tenants = resp.getTenantsList();
      this.applications = resp.getApplicationsList();
      this.deviceProfiles = resp.getDeviceProfilesList();
      this.emit("change");

      callbackFunc();
    });
  };

  getUser = (): User | undefined => {
    return this.user;
  };

  isAdmin = (): boolean => {
    if (!this.user) {
      return false;
    }

    return this.user.getIsAdmin();
  };

  isTenantAdmin = (tenantId: string): boolean => {
    for (const t of this.tenants) {
      if (t.getTenantId() === tenantId) {
        return t.getIsAdmin();
      }
    }

    return false;
  };

  isTenantDeviceAdmin = (tenantId: string): boolean => {
    for (const t of this.tenants) {
      if (t.getTenantId() === tenantId) {
        return t.getIsAdmin() || t.getIsDeviceAdmin();
      }
    }

    return false;
  };

  isTenantGatewayAdmin = (tenantId: string): boolean => {
    for (const t of this.tenants) {
      if (t.getTenantId() === tenantId) {
        return t.getIsAdmin() || t.getIsGatewayAdmin();
      }
    }

    return false;
  };

  isApplicationAdmin = (applicationId: string): boolean => {
    for (const a of this.applications) {
      if (a.getApplicationId() === applicationId && a.getIsReadOnly() === false) {
        return true;
      }
    }

    return false;
  };

  isApplicationAdminRo = (applicationId: string): boolean => {
    for (const a of this.applications) {
      if (a.getApplicationId() === applicationId && a.getIsReadOnly() === true) {
        return true;
      }
    }

    return false;
  };

  isDeviceProfileAdmin = (deviceProfileId: string): boolean => {
    for (const dp of this.deviceProfiles) {
      if (dp.getDeviceProfileId() === deviceProfileId) {
        return true;
      }
    }

    return false;
  };
}

const sessionStore = new SessionStore();
export default sessionStore;
