import * as jspb from 'google-protobuf'

import * as google_api_annotations_pb from '../google/api/annotations_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class Tenant extends jspb.Message {
  getId(): string;
  setId(value: string): Tenant;

  getName(): string;
  setName(value: string): Tenant;

  getDescription(): string;
  setDescription(value: string): Tenant;

  getCanHaveGateways(): boolean;
  setCanHaveGateways(value: boolean): Tenant;

  getMaxGatewayCount(): number;
  setMaxGatewayCount(value: number): Tenant;

  getMaxDeviceCount(): number;
  setMaxDeviceCount(value: number): Tenant;

  getPrivateGatewaysUp(): boolean;
  setPrivateGatewaysUp(value: boolean): Tenant;

  getPrivateGatewaysDown(): boolean;
  setPrivateGatewaysDown(value: boolean): Tenant;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Tenant.AsObject;
  static toObject(includeInstance: boolean, msg: Tenant): Tenant.AsObject;
  static serializeBinaryToWriter(message: Tenant, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Tenant;
  static deserializeBinaryFromReader(message: Tenant, reader: jspb.BinaryReader): Tenant;
}

export namespace Tenant {
  export type AsObject = {
    id: string,
    name: string,
    description: string,
    canHaveGateways: boolean,
    maxGatewayCount: number,
    maxDeviceCount: number,
    privateGatewaysUp: boolean,
    privateGatewaysDown: boolean,
  }
}

export class TenantListItem extends jspb.Message {
  getId(): string;
  setId(value: string): TenantListItem;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): TenantListItem;
  hasCreatedAt(): boolean;
  clearCreatedAt(): TenantListItem;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): TenantListItem;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): TenantListItem;

  getName(): string;
  setName(value: string): TenantListItem;

  getCanHaveGateways(): boolean;
  setCanHaveGateways(value: boolean): TenantListItem;

  getPrivateGatewaysUp(): boolean;
  setPrivateGatewaysUp(value: boolean): TenantListItem;

  getPrivateGatewaysDown(): boolean;
  setPrivateGatewaysDown(value: boolean): TenantListItem;

  getMaxGatewayCount(): number;
  setMaxGatewayCount(value: number): TenantListItem;

  getMaxDeviceCount(): number;
  setMaxDeviceCount(value: number): TenantListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TenantListItem.AsObject;
  static toObject(includeInstance: boolean, msg: TenantListItem): TenantListItem.AsObject;
  static serializeBinaryToWriter(message: TenantListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TenantListItem;
  static deserializeBinaryFromReader(message: TenantListItem, reader: jspb.BinaryReader): TenantListItem;
}

export namespace TenantListItem {
  export type AsObject = {
    id: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    name: string,
    canHaveGateways: boolean,
    privateGatewaysUp: boolean,
    privateGatewaysDown: boolean,
    maxGatewayCount: number,
    maxDeviceCount: number,
  }
}

export class CreateTenantRequest extends jspb.Message {
  getTenant(): Tenant | undefined;
  setTenant(value?: Tenant): CreateTenantRequest;
  hasTenant(): boolean;
  clearTenant(): CreateTenantRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateTenantRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateTenantRequest): CreateTenantRequest.AsObject;
  static serializeBinaryToWriter(message: CreateTenantRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateTenantRequest;
  static deserializeBinaryFromReader(message: CreateTenantRequest, reader: jspb.BinaryReader): CreateTenantRequest;
}

export namespace CreateTenantRequest {
  export type AsObject = {
    tenant?: Tenant.AsObject,
  }
}

export class CreateTenantResponse extends jspb.Message {
  getId(): string;
  setId(value: string): CreateTenantResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateTenantResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateTenantResponse): CreateTenantResponse.AsObject;
  static serializeBinaryToWriter(message: CreateTenantResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateTenantResponse;
  static deserializeBinaryFromReader(message: CreateTenantResponse, reader: jspb.BinaryReader): CreateTenantResponse;
}

export namespace CreateTenantResponse {
  export type AsObject = {
    id: string,
  }
}

export class GetTenantRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetTenantRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetTenantRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetTenantRequest): GetTenantRequest.AsObject;
  static serializeBinaryToWriter(message: GetTenantRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetTenantRequest;
  static deserializeBinaryFromReader(message: GetTenantRequest, reader: jspb.BinaryReader): GetTenantRequest;
}

export namespace GetTenantRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetTenantResponse extends jspb.Message {
  getTenant(): Tenant | undefined;
  setTenant(value?: Tenant): GetTenantResponse;
  hasTenant(): boolean;
  clearTenant(): GetTenantResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetTenantResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): GetTenantResponse;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetTenantResponse;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): GetTenantResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetTenantResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetTenantResponse): GetTenantResponse.AsObject;
  static serializeBinaryToWriter(message: GetTenantResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetTenantResponse;
  static deserializeBinaryFromReader(message: GetTenantResponse, reader: jspb.BinaryReader): GetTenantResponse;
}

export namespace GetTenantResponse {
  export type AsObject = {
    tenant?: Tenant.AsObject,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class UpdateTenantRequest extends jspb.Message {
  getTenant(): Tenant | undefined;
  setTenant(value?: Tenant): UpdateTenantRequest;
  hasTenant(): boolean;
  clearTenant(): UpdateTenantRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateTenantRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateTenantRequest): UpdateTenantRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateTenantRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateTenantRequest;
  static deserializeBinaryFromReader(message: UpdateTenantRequest, reader: jspb.BinaryReader): UpdateTenantRequest;
}

export namespace UpdateTenantRequest {
  export type AsObject = {
    tenant?: Tenant.AsObject,
  }
}

export class DeleteTenantRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteTenantRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteTenantRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteTenantRequest): DeleteTenantRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteTenantRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteTenantRequest;
  static deserializeBinaryFromReader(message: DeleteTenantRequest, reader: jspb.BinaryReader): DeleteTenantRequest;
}

export namespace DeleteTenantRequest {
  export type AsObject = {
    id: string,
  }
}

export class ListTenantsRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): ListTenantsRequest;

  getOffset(): number;
  setOffset(value: number): ListTenantsRequest;

  getSearch(): string;
  setSearch(value: string): ListTenantsRequest;

  getUserId(): string;
  setUserId(value: string): ListTenantsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListTenantsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListTenantsRequest): ListTenantsRequest.AsObject;
  static serializeBinaryToWriter(message: ListTenantsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListTenantsRequest;
  static deserializeBinaryFromReader(message: ListTenantsRequest, reader: jspb.BinaryReader): ListTenantsRequest;
}

export namespace ListTenantsRequest {
  export type AsObject = {
    limit: number,
    offset: number,
    search: string,
    userId: string,
  }
}

export class ListTenantsResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): ListTenantsResponse;

  getResultList(): Array<TenantListItem>;
  setResultList(value: Array<TenantListItem>): ListTenantsResponse;
  clearResultList(): ListTenantsResponse;
  addResult(value?: TenantListItem, index?: number): TenantListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListTenantsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListTenantsResponse): ListTenantsResponse.AsObject;
  static serializeBinaryToWriter(message: ListTenantsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListTenantsResponse;
  static deserializeBinaryFromReader(message: ListTenantsResponse, reader: jspb.BinaryReader): ListTenantsResponse;
}

export namespace ListTenantsResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<TenantListItem.AsObject>,
  }
}

export class TenantUser extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): TenantUser;

  getUserId(): string;
  setUserId(value: string): TenantUser;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): TenantUser;

  getIsDeviceAdmin(): boolean;
  setIsDeviceAdmin(value: boolean): TenantUser;

  getIsGatewayAdmin(): boolean;
  setIsGatewayAdmin(value: boolean): TenantUser;

  getEmail(): string;
  setEmail(value: string): TenantUser;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TenantUser.AsObject;
  static toObject(includeInstance: boolean, msg: TenantUser): TenantUser.AsObject;
  static serializeBinaryToWriter(message: TenantUser, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TenantUser;
  static deserializeBinaryFromReader(message: TenantUser, reader: jspb.BinaryReader): TenantUser;
}

export namespace TenantUser {
  export type AsObject = {
    tenantId: string,
    userId: string,
    isAdmin: boolean,
    isDeviceAdmin: boolean,
    isGatewayAdmin: boolean,
    email: string,
  }
}

export class TenantUserListItem extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): TenantUserListItem;

  getUserId(): string;
  setUserId(value: string): TenantUserListItem;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): TenantUserListItem;
  hasCreatedAt(): boolean;
  clearCreatedAt(): TenantUserListItem;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): TenantUserListItem;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): TenantUserListItem;

  getEmail(): string;
  setEmail(value: string): TenantUserListItem;

  getIsAdmin(): boolean;
  setIsAdmin(value: boolean): TenantUserListItem;

  getIsDeviceAdmin(): boolean;
  setIsDeviceAdmin(value: boolean): TenantUserListItem;

  getIsGatewayAdmin(): boolean;
  setIsGatewayAdmin(value: boolean): TenantUserListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TenantUserListItem.AsObject;
  static toObject(includeInstance: boolean, msg: TenantUserListItem): TenantUserListItem.AsObject;
  static serializeBinaryToWriter(message: TenantUserListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TenantUserListItem;
  static deserializeBinaryFromReader(message: TenantUserListItem, reader: jspb.BinaryReader): TenantUserListItem;
}

export namespace TenantUserListItem {
  export type AsObject = {
    tenantId: string,
    userId: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    email: string,
    isAdmin: boolean,
    isDeviceAdmin: boolean,
    isGatewayAdmin: boolean,
  }
}

export class AddTenantUserRequest extends jspb.Message {
  getTenantUser(): TenantUser | undefined;
  setTenantUser(value?: TenantUser): AddTenantUserRequest;
  hasTenantUser(): boolean;
  clearTenantUser(): AddTenantUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddTenantUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddTenantUserRequest): AddTenantUserRequest.AsObject;
  static serializeBinaryToWriter(message: AddTenantUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddTenantUserRequest;
  static deserializeBinaryFromReader(message: AddTenantUserRequest, reader: jspb.BinaryReader): AddTenantUserRequest;
}

export namespace AddTenantUserRequest {
  export type AsObject = {
    tenantUser?: TenantUser.AsObject,
  }
}

export class GetTenantUserRequest extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): GetTenantUserRequest;

  getUserId(): string;
  setUserId(value: string): GetTenantUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetTenantUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetTenantUserRequest): GetTenantUserRequest.AsObject;
  static serializeBinaryToWriter(message: GetTenantUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetTenantUserRequest;
  static deserializeBinaryFromReader(message: GetTenantUserRequest, reader: jspb.BinaryReader): GetTenantUserRequest;
}

export namespace GetTenantUserRequest {
  export type AsObject = {
    tenantId: string,
    userId: string,
  }
}

export class GetTenantUserResponse extends jspb.Message {
  getTenantUser(): TenantUser | undefined;
  setTenantUser(value?: TenantUser): GetTenantUserResponse;
  hasTenantUser(): boolean;
  clearTenantUser(): GetTenantUserResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetTenantUserResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): GetTenantUserResponse;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetTenantUserResponse;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): GetTenantUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetTenantUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetTenantUserResponse): GetTenantUserResponse.AsObject;
  static serializeBinaryToWriter(message: GetTenantUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetTenantUserResponse;
  static deserializeBinaryFromReader(message: GetTenantUserResponse, reader: jspb.BinaryReader): GetTenantUserResponse;
}

export namespace GetTenantUserResponse {
  export type AsObject = {
    tenantUser?: TenantUser.AsObject,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class UpdateTenantUserRequest extends jspb.Message {
  getTenantUser(): TenantUser | undefined;
  setTenantUser(value?: TenantUser): UpdateTenantUserRequest;
  hasTenantUser(): boolean;
  clearTenantUser(): UpdateTenantUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateTenantUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateTenantUserRequest): UpdateTenantUserRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateTenantUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateTenantUserRequest;
  static deserializeBinaryFromReader(message: UpdateTenantUserRequest, reader: jspb.BinaryReader): UpdateTenantUserRequest;
}

export namespace UpdateTenantUserRequest {
  export type AsObject = {
    tenantUser?: TenantUser.AsObject,
  }
}

export class DeleteTenantUserRequest extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): DeleteTenantUserRequest;

  getUserId(): string;
  setUserId(value: string): DeleteTenantUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteTenantUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteTenantUserRequest): DeleteTenantUserRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteTenantUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteTenantUserRequest;
  static deserializeBinaryFromReader(message: DeleteTenantUserRequest, reader: jspb.BinaryReader): DeleteTenantUserRequest;
}

export namespace DeleteTenantUserRequest {
  export type AsObject = {
    tenantId: string,
    userId: string,
  }
}

export class ListTenantUsersRequest extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): ListTenantUsersRequest;

  getLimit(): number;
  setLimit(value: number): ListTenantUsersRequest;

  getOffset(): number;
  setOffset(value: number): ListTenantUsersRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListTenantUsersRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListTenantUsersRequest): ListTenantUsersRequest.AsObject;
  static serializeBinaryToWriter(message: ListTenantUsersRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListTenantUsersRequest;
  static deserializeBinaryFromReader(message: ListTenantUsersRequest, reader: jspb.BinaryReader): ListTenantUsersRequest;
}

export namespace ListTenantUsersRequest {
  export type AsObject = {
    tenantId: string,
    limit: number,
    offset: number,
  }
}

export class ListTenantUsersResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): ListTenantUsersResponse;

  getResultList(): Array<TenantUserListItem>;
  setResultList(value: Array<TenantUserListItem>): ListTenantUsersResponse;
  clearResultList(): ListTenantUsersResponse;
  addResult(value?: TenantUserListItem, index?: number): TenantUserListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListTenantUsersResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListTenantUsersResponse): ListTenantUsersResponse.AsObject;
  static serializeBinaryToWriter(message: ListTenantUsersResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListTenantUsersResponse;
  static deserializeBinaryFromReader(message: ListTenantUsersResponse, reader: jspb.BinaryReader): ListTenantUsersResponse;
}

export namespace ListTenantUsersResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<TenantUserListItem.AsObject>,
  }
}

