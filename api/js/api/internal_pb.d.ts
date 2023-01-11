// package: api
// file: api/internal.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as common_common_pb from "../common/common_pb";
import * as api_user_pb from "../api/user_pb";

export class ApiKey extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiKey.AsObject;
  static toObject(includeInstance: boolean, msg: ApiKey): ApiKey.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ApiKey, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ApiKey;
  static deserializeBinaryFromReader(message: ApiKey, reader: jspb.BinaryReader): ApiKey;
}

export namespace ApiKey {
  export type AsObject = {
    id: string,
    name: string,
    isAdmin: boolean,
    tenantId: string,
  }
}

export class CreateApiKeyRequest extends jspb.Message {
  hasApiKey(): boolean;
  clearApiKey(): void;
  getApiKey(): ApiKey | undefined;
  setApiKey(value?: ApiKey): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateApiKeyRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateApiKeyRequest): CreateApiKeyRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateApiKeyRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateApiKeyRequest;
  static deserializeBinaryFromReader(message: CreateApiKeyRequest, reader: jspb.BinaryReader): CreateApiKeyRequest;
}

export namespace CreateApiKeyRequest {
  export type AsObject = {
    apiKey?: ApiKey.AsObject,
  }
}

export class CreateApiKeyResponse extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getToken(): string;
  setToken(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateApiKeyResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateApiKeyResponse): CreateApiKeyResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateApiKeyResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateApiKeyResponse;
  static deserializeBinaryFromReader(message: CreateApiKeyResponse, reader: jspb.BinaryReader): CreateApiKeyResponse;
}

export namespace CreateApiKeyResponse {
  export type AsObject = {
    id: string,
    token: string,
  }
}

export class DeleteApiKeyRequest extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteApiKeyRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteApiKeyRequest): DeleteApiKeyRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteApiKeyRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteApiKeyRequest;
  static deserializeBinaryFromReader(message: DeleteApiKeyRequest, reader: jspb.BinaryReader): DeleteApiKeyRequest;
}

export namespace DeleteApiKeyRequest {
  export type AsObject = {
    id: string,
  }
}

export class ListApiKeysRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListApiKeysRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListApiKeysRequest): ListApiKeysRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListApiKeysRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListApiKeysRequest;
  static deserializeBinaryFromReader(message: ListApiKeysRequest, reader: jspb.BinaryReader): ListApiKeysRequest;
}

export namespace ListApiKeysRequest {
  export type AsObject = {
    limit: number,
    offset: number,
    isAdmin: boolean,
    tenantId: string,
  }
}

export class ListApiKeysResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<ApiKey>;
  setResultList(value: Array<ApiKey>): void;
  addResult(value?: ApiKey, index?: number): ApiKey;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListApiKeysResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListApiKeysResponse): ListApiKeysResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListApiKeysResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListApiKeysResponse;
  static deserializeBinaryFromReader(message: ListApiKeysResponse, reader: jspb.BinaryReader): ListApiKeysResponse;
}

export namespace ListApiKeysResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<ApiKey.AsObject>,
  }
}

export class UserTenantLink extends jspb.Message {
  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasUpdatedAt(): boolean;
  clearUpdatedAt(): void;
  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): void;

  getIsDeviceAdmin(): boolean;
  setIsDeviceAdmin(value: boolean): void;

  getIsGatewayAdmin(): boolean;
  setIsGatewayAdmin(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserTenantLink.AsObject;
  static toObject(includeInstance: boolean, msg: UserTenantLink): UserTenantLink.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UserTenantLink, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserTenantLink;
  static deserializeBinaryFromReader(message: UserTenantLink, reader: jspb.BinaryReader): UserTenantLink;
}

export namespace UserTenantLink {
  export type AsObject = {
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    tenantId: string,
    isAdmin: boolean,
    isDeviceAdmin: boolean,
    isGatewayAdmin: boolean,
  }
}

export class LoginRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): void;

  getPassword(): string;
  setPassword(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoginRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LoginRequest): LoginRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LoginRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoginRequest;
  static deserializeBinaryFromReader(message: LoginRequest, reader: jspb.BinaryReader): LoginRequest;
}

export namespace LoginRequest {
  export type AsObject = {
    email: string,
    password: string,
  }
}

export class LoginResponse extends jspb.Message {
  getJwt(): string;
  setJwt(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoginResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LoginResponse): LoginResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LoginResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoginResponse;
  static deserializeBinaryFromReader(message: LoginResponse, reader: jspb.BinaryReader): LoginResponse;
}

export namespace LoginResponse {
  export type AsObject = {
    jwt: string,
  }
}

export class ProfileResponse extends jspb.Message {
  hasUser(): boolean;
  clearUser(): void;
  getUser(): api_user_pb.User | undefined;
  setUser(value?: api_user_pb.User): void;

  clearTenantsList(): void;
  getTenantsList(): Array<UserTenantLink>;
  setTenantsList(value: Array<UserTenantLink>): void;
  addTenants(value?: UserTenantLink, index?: number): UserTenantLink;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ProfileResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ProfileResponse): ProfileResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ProfileResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ProfileResponse;
  static deserializeBinaryFromReader(message: ProfileResponse, reader: jspb.BinaryReader): ProfileResponse;
}

export namespace ProfileResponse {
  export type AsObject = {
    user?: api_user_pb.User.AsObject,
    tenantsList: Array<UserTenantLink.AsObject>,
  }
}

export class GlobalSearchRequest extends jspb.Message {
  getSearch(): string;
  setSearch(value: string): void;

  getLimit(): number;
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GlobalSearchRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GlobalSearchRequest): GlobalSearchRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GlobalSearchRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GlobalSearchRequest;
  static deserializeBinaryFromReader(message: GlobalSearchRequest, reader: jspb.BinaryReader): GlobalSearchRequest;
}

export namespace GlobalSearchRequest {
  export type AsObject = {
    search: string,
    limit: number,
    offset: number,
  }
}

export class GlobalSearchResponse extends jspb.Message {
  clearResultList(): void;
  getResultList(): Array<GlobalSearchResult>;
  setResultList(value: Array<GlobalSearchResult>): void;
  addResult(value?: GlobalSearchResult, index?: number): GlobalSearchResult;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GlobalSearchResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GlobalSearchResponse): GlobalSearchResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GlobalSearchResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GlobalSearchResponse;
  static deserializeBinaryFromReader(message: GlobalSearchResponse, reader: jspb.BinaryReader): GlobalSearchResponse;
}

export namespace GlobalSearchResponse {
  export type AsObject = {
    resultList: Array<GlobalSearchResult.AsObject>,
  }
}

export class GlobalSearchResult extends jspb.Message {
  getKind(): string;
  setKind(value: string): void;

  getScore(): number;
  setScore(value: number): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  getTenantName(): string;
  setTenantName(value: string): void;

  getApplicationId(): string;
  setApplicationId(value: string): void;

  getApplicationName(): string;
  setApplicationName(value: string): void;

  getDeviceDevEui(): string;
  setDeviceDevEui(value: string): void;

  getDeviceName(): string;
  setDeviceName(value: string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

  getGatewayName(): string;
  setGatewayName(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GlobalSearchResult.AsObject;
  static toObject(includeInstance: boolean, msg: GlobalSearchResult): GlobalSearchResult.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GlobalSearchResult, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GlobalSearchResult;
  static deserializeBinaryFromReader(message: GlobalSearchResult, reader: jspb.BinaryReader): GlobalSearchResult;
}

export namespace GlobalSearchResult {
  export type AsObject = {
    kind: string,
    score: number,
    tenantId: string,
    tenantName: string,
    applicationId: string,
    applicationName: string,
    deviceDevEui: string,
    deviceName: string,
    gatewayId: string,
    gatewayName: string,
  }
}

export class SettingsResponse extends jspb.Message {
  hasOpenidConnect(): boolean;
  clearOpenidConnect(): void;
  getOpenidConnect(): OpenIdConnect | undefined;
  setOpenidConnect(value?: OpenIdConnect): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SettingsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SettingsResponse): SettingsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SettingsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SettingsResponse;
  static deserializeBinaryFromReader(message: SettingsResponse, reader: jspb.BinaryReader): SettingsResponse;
}

export namespace SettingsResponse {
  export type AsObject = {
    openidConnect?: OpenIdConnect.AsObject,
  }
}

export class OpenIdConnect extends jspb.Message {
  getEnabled(): boolean;
  setEnabled(value: boolean): void;

  getLoginUrl(): string;
  setLoginUrl(value: string): void;

  getLoginLabel(): string;
  setLoginLabel(value: string): void;

  getLogoutUrl(): string;
  setLogoutUrl(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): OpenIdConnect.AsObject;
  static toObject(includeInstance: boolean, msg: OpenIdConnect): OpenIdConnect.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: OpenIdConnect, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): OpenIdConnect;
  static deserializeBinaryFromReader(message: OpenIdConnect, reader: jspb.BinaryReader): OpenIdConnect;
}

export namespace OpenIdConnect {
  export type AsObject = {
    enabled: boolean,
    loginUrl: string,
    loginLabel: string,
    logoutUrl: string,
  }
}

export class OpenIdConnectLoginRequest extends jspb.Message {
  getCode(): string;
  setCode(value: string): void;

  getState(): string;
  setState(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): OpenIdConnectLoginRequest.AsObject;
  static toObject(includeInstance: boolean, msg: OpenIdConnectLoginRequest): OpenIdConnectLoginRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: OpenIdConnectLoginRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): OpenIdConnectLoginRequest;
  static deserializeBinaryFromReader(message: OpenIdConnectLoginRequest, reader: jspb.BinaryReader): OpenIdConnectLoginRequest;
}

export namespace OpenIdConnectLoginRequest {
  export type AsObject = {
    code: string,
    state: string,
  }
}

export class OpenIdConnectLoginResponse extends jspb.Message {
  getToken(): string;
  setToken(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): OpenIdConnectLoginResponse.AsObject;
  static toObject(includeInstance: boolean, msg: OpenIdConnectLoginResponse): OpenIdConnectLoginResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: OpenIdConnectLoginResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): OpenIdConnectLoginResponse;
  static deserializeBinaryFromReader(message: OpenIdConnectLoginResponse, reader: jspb.BinaryReader): OpenIdConnectLoginResponse;
}

export namespace OpenIdConnectLoginResponse {
  export type AsObject = {
    token: string,
  }
}

export class GetDevicesSummaryRequest extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDevicesSummaryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDevicesSummaryRequest): GetDevicesSummaryRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDevicesSummaryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDevicesSummaryRequest;
  static deserializeBinaryFromReader(message: GetDevicesSummaryRequest, reader: jspb.BinaryReader): GetDevicesSummaryRequest;
}

export namespace GetDevicesSummaryRequest {
  export type AsObject = {
    tenantId: string,
  }
}

export class GetDevicesSummaryResponse extends jspb.Message {
  getActiveCount(): number;
  setActiveCount(value: number): void;

  getInactiveCount(): number;
  setInactiveCount(value: number): void;

  getDrCountMap(): jspb.Map<number, number>;
  clearDrCountMap(): void;
  getNeverSeenCount(): number;
  setNeverSeenCount(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDevicesSummaryResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDevicesSummaryResponse): GetDevicesSummaryResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDevicesSummaryResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDevicesSummaryResponse;
  static deserializeBinaryFromReader(message: GetDevicesSummaryResponse, reader: jspb.BinaryReader): GetDevicesSummaryResponse;
}

export namespace GetDevicesSummaryResponse {
  export type AsObject = {
    activeCount: number,
    inactiveCount: number,
    drCountMap: Array<[number, number]>,
    neverSeenCount: number,
  }
}

export class GetGatewaysSummaryRequest extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGatewaysSummaryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetGatewaysSummaryRequest): GetGatewaysSummaryRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetGatewaysSummaryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGatewaysSummaryRequest;
  static deserializeBinaryFromReader(message: GetGatewaysSummaryRequest, reader: jspb.BinaryReader): GetGatewaysSummaryRequest;
}

export namespace GetGatewaysSummaryRequest {
  export type AsObject = {
    tenantId: string,
  }
}

export class GetGatewaysSummaryResponse extends jspb.Message {
  getOnlineCount(): number;
  setOnlineCount(value: number): void;

  getOfflineCount(): number;
  setOfflineCount(value: number): void;

  getNeverSeenCount(): number;
  setNeverSeenCount(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGatewaysSummaryResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetGatewaysSummaryResponse): GetGatewaysSummaryResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetGatewaysSummaryResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGatewaysSummaryResponse;
  static deserializeBinaryFromReader(message: GetGatewaysSummaryResponse, reader: jspb.BinaryReader): GetGatewaysSummaryResponse;
}

export namespace GetGatewaysSummaryResponse {
  export type AsObject = {
    onlineCount: number,
    offlineCount: number,
    neverSeenCount: number,
  }
}

export class LogItem extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getDescription(): string;
  setDescription(value: string): void;

  getBody(): string;
  setBody(value: string): void;

  getPropertiesMap(): jspb.Map<string, string>;
  clearPropertiesMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LogItem.AsObject;
  static toObject(includeInstance: boolean, msg: LogItem): LogItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LogItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LogItem;
  static deserializeBinaryFromReader(message: LogItem, reader: jspb.BinaryReader): LogItem;
}

export namespace LogItem {
  export type AsObject = {
    id: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    description: string,
    body: string,
    propertiesMap: Array<[string, string]>,
  }
}

export class StreamGatewayFramesRequest extends jspb.Message {
  getGatewayId(): string;
  setGatewayId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StreamGatewayFramesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StreamGatewayFramesRequest): StreamGatewayFramesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StreamGatewayFramesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StreamGatewayFramesRequest;
  static deserializeBinaryFromReader(message: StreamGatewayFramesRequest, reader: jspb.BinaryReader): StreamGatewayFramesRequest;
}

export namespace StreamGatewayFramesRequest {
  export type AsObject = {
    gatewayId: string,
  }
}

export class StreamDeviceFramesRequest extends jspb.Message {
  getDevEui(): string;
  setDevEui(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StreamDeviceFramesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StreamDeviceFramesRequest): StreamDeviceFramesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StreamDeviceFramesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StreamDeviceFramesRequest;
  static deserializeBinaryFromReader(message: StreamDeviceFramesRequest, reader: jspb.BinaryReader): StreamDeviceFramesRequest;
}

export namespace StreamDeviceFramesRequest {
  export type AsObject = {
    devEui: string,
  }
}

export class StreamDeviceEventsRequest extends jspb.Message {
  getDevEui(): string;
  setDevEui(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StreamDeviceEventsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StreamDeviceEventsRequest): StreamDeviceEventsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StreamDeviceEventsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StreamDeviceEventsRequest;
  static deserializeBinaryFromReader(message: StreamDeviceEventsRequest, reader: jspb.BinaryReader): StreamDeviceEventsRequest;
}

export namespace StreamDeviceEventsRequest {
  export type AsObject = {
    devEui: string,
  }
}

export class ListRegionsResponse extends jspb.Message {
  clearRegionsList(): void;
  getRegionsList(): Array<RegionListItem>;
  setRegionsList(value: Array<RegionListItem>): void;
  addRegions(value?: RegionListItem, index?: number): RegionListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListRegionsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListRegionsResponse): ListRegionsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListRegionsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListRegionsResponse;
  static deserializeBinaryFromReader(message: ListRegionsResponse, reader: jspb.BinaryReader): ListRegionsResponse;
}

export namespace ListRegionsResponse {
  export type AsObject = {
    regionsList: Array<RegionListItem.AsObject>,
  }
}

export class RegionListItem extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getRegion(): common_common_pb.RegionMap[keyof common_common_pb.RegionMap];
  setRegion(value: common_common_pb.RegionMap[keyof common_common_pb.RegionMap]): void;

  getDescription(): string;
  setDescription(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RegionListItem.AsObject;
  static toObject(includeInstance: boolean, msg: RegionListItem): RegionListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RegionListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RegionListItem;
  static deserializeBinaryFromReader(message: RegionListItem, reader: jspb.BinaryReader): RegionListItem;
}

export namespace RegionListItem {
  export type AsObject = {
    id: string,
    region: common_common_pb.RegionMap[keyof common_common_pb.RegionMap],
    description: string,
  }
}

export class GetRegionRequest extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetRegionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetRegionRequest): GetRegionRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetRegionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetRegionRequest;
  static deserializeBinaryFromReader(message: GetRegionRequest, reader: jspb.BinaryReader): GetRegionRequest;
}

export namespace GetRegionRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetRegionResponse extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getRegion(): common_common_pb.RegionMap[keyof common_common_pb.RegionMap];
  setRegion(value: common_common_pb.RegionMap[keyof common_common_pb.RegionMap]): void;

  getUserInfo(): string;
  setUserInfo(value: string): void;

  clearUplinkChannelsList(): void;
  getUplinkChannelsList(): Array<RegionChannel>;
  setUplinkChannelsList(value: Array<RegionChannel>): void;
  addUplinkChannels(value?: RegionChannel, index?: number): RegionChannel;

  getRx1Delay(): number;
  setRx1Delay(value: number): void;

  getRx1DrOffset(): number;
  setRx1DrOffset(value: number): void;

  getRx2Dr(): number;
  setRx2Dr(value: number): void;

  getRx2Frequency(): number;
  setRx2Frequency(value: number): void;

  getClassBPingSlotDr(): number;
  setClassBPingSlotDr(value: number): void;

  getClassBPingSlotFrequency(): number;
  setClassBPingSlotFrequency(value: number): void;

  getDescription(): string;
  setDescription(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetRegionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetRegionResponse): GetRegionResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetRegionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetRegionResponse;
  static deserializeBinaryFromReader(message: GetRegionResponse, reader: jspb.BinaryReader): GetRegionResponse;
}

export namespace GetRegionResponse {
  export type AsObject = {
    id: string,
    region: common_common_pb.RegionMap[keyof common_common_pb.RegionMap],
    userInfo: string,
    uplinkChannelsList: Array<RegionChannel.AsObject>,
    rx1Delay: number,
    rx1DrOffset: number,
    rx2Dr: number,
    rx2Frequency: number,
    classBPingSlotDr: number,
    classBPingSlotFrequency: number,
    description: string,
  }
}

export class RegionChannel extends jspb.Message {
  getFrequency(): number;
  setFrequency(value: number): void;

  getDrMin(): number;
  setDrMin(value: number): void;

  getDrMax(): number;
  setDrMax(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RegionChannel.AsObject;
  static toObject(includeInstance: boolean, msg: RegionChannel): RegionChannel.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RegionChannel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RegionChannel;
  static deserializeBinaryFromReader(message: RegionChannel, reader: jspb.BinaryReader): RegionChannel;
}

export namespace RegionChannel {
  export type AsObject = {
    frequency: number,
    drMin: number,
    drMax: number,
  }
}

