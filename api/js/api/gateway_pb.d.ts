// package: api
// file: api/gateway.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as common_common_pb from "../common/common_pb";

export class Gateway extends jspb.Message {
  getGatewayId(): string;
  setGatewayId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

  hasLocation(): boolean;
  clearLocation(): void;
  getLocation(): common_common_pb.Location | undefined;
  setLocation(value?: common_common_pb.Location): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): void;
  getPropertiesMap(): jspb.Map<string, string>;
  clearPropertiesMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Gateway.AsObject;
  static toObject(includeInstance: boolean, msg: Gateway): Gateway.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Gateway, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Gateway;
  static deserializeBinaryFromReader(message: Gateway, reader: jspb.BinaryReader): Gateway;
}

export namespace Gateway {
  export type AsObject = {
    gatewayId: string,
    name: string,
    description: string,
    location?: common_common_pb.Location.AsObject,
    tenantId: string,
    tagsMap: Array<[string, string]>,
    propertiesMap: Array<[string, string]>,
  }
}

export class GatewayListItem extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

  hasLocation(): boolean;
  clearLocation(): void;
  getLocation(): common_common_pb.Location | undefined;
  setLocation(value?: common_common_pb.Location): void;

  getPropertiesMap(): jspb.Map<string, string>;
  clearPropertiesMap(): void;
  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasUpdatedAt(): boolean;
  clearUpdatedAt(): void;
  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasLastSeenAt(): boolean;
  clearLastSeenAt(): void;
  getLastSeenAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setLastSeenAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GatewayListItem.AsObject;
  static toObject(includeInstance: boolean, msg: GatewayListItem): GatewayListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GatewayListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GatewayListItem;
  static deserializeBinaryFromReader(message: GatewayListItem, reader: jspb.BinaryReader): GatewayListItem;
}

export namespace GatewayListItem {
  export type AsObject = {
    tenantId: string,
    gatewayId: string,
    name: string,
    description: string,
    location?: common_common_pb.Location.AsObject,
    propertiesMap: Array<[string, string]>,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    lastSeenAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class CreateGatewayRequest extends jspb.Message {
  hasGateway(): boolean;
  clearGateway(): void;
  getGateway(): Gateway | undefined;
  setGateway(value?: Gateway): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateGatewayRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateGatewayRequest): CreateGatewayRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateGatewayRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateGatewayRequest;
  static deserializeBinaryFromReader(message: CreateGatewayRequest, reader: jspb.BinaryReader): CreateGatewayRequest;
}

export namespace CreateGatewayRequest {
  export type AsObject = {
    gateway?: Gateway.AsObject,
  }
}

export class GetGatewayRequest extends jspb.Message {
  getGatewayId(): string;
  setGatewayId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGatewayRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetGatewayRequest): GetGatewayRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetGatewayRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGatewayRequest;
  static deserializeBinaryFromReader(message: GetGatewayRequest, reader: jspb.BinaryReader): GetGatewayRequest;
}

export namespace GetGatewayRequest {
  export type AsObject = {
    gatewayId: string,
  }
}

export class GetGatewayResponse extends jspb.Message {
  hasGateway(): boolean;
  clearGateway(): void;
  getGateway(): Gateway | undefined;
  setGateway(value?: Gateway): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasUpdatedAt(): boolean;
  clearUpdatedAt(): void;
  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasLastSeenAt(): boolean;
  clearLastSeenAt(): void;
  getLastSeenAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setLastSeenAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGatewayResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetGatewayResponse): GetGatewayResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetGatewayResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGatewayResponse;
  static deserializeBinaryFromReader(message: GetGatewayResponse, reader: jspb.BinaryReader): GetGatewayResponse;
}

export namespace GetGatewayResponse {
  export type AsObject = {
    gateway?: Gateway.AsObject,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    lastSeenAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class UpdateGatewayRequest extends jspb.Message {
  hasGateway(): boolean;
  clearGateway(): void;
  getGateway(): Gateway | undefined;
  setGateway(value?: Gateway): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateGatewayRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateGatewayRequest): UpdateGatewayRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateGatewayRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateGatewayRequest;
  static deserializeBinaryFromReader(message: UpdateGatewayRequest, reader: jspb.BinaryReader): UpdateGatewayRequest;
}

export namespace UpdateGatewayRequest {
  export type AsObject = {
    gateway?: Gateway.AsObject,
  }
}

export class DeleteGatewayRequest extends jspb.Message {
  getGatewayId(): string;
  setGatewayId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteGatewayRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteGatewayRequest): DeleteGatewayRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteGatewayRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteGatewayRequest;
  static deserializeBinaryFromReader(message: DeleteGatewayRequest, reader: jspb.BinaryReader): DeleteGatewayRequest;
}

export namespace DeleteGatewayRequest {
  export type AsObject = {
    gatewayId: string,
  }
}

export class ListGatewaysRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  getSearch(): string;
  setSearch(value: string): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListGatewaysRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListGatewaysRequest): ListGatewaysRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListGatewaysRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListGatewaysRequest;
  static deserializeBinaryFromReader(message: ListGatewaysRequest, reader: jspb.BinaryReader): ListGatewaysRequest;
}

export namespace ListGatewaysRequest {
  export type AsObject = {
    limit: number,
    offset: number,
    search: string,
    tenantId: string,
  }
}

export class ListGatewaysResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<GatewayListItem>;
  setResultList(value: Array<GatewayListItem>): void;
  addResult(value?: GatewayListItem, index?: number): GatewayListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListGatewaysResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListGatewaysResponse): ListGatewaysResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListGatewaysResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListGatewaysResponse;
  static deserializeBinaryFromReader(message: ListGatewaysResponse, reader: jspb.BinaryReader): ListGatewaysResponse;
}

export namespace ListGatewaysResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<GatewayListItem.AsObject>,
  }
}

export class GenerateGatewayClientCertificateRequest extends jspb.Message {
  getGatewayId(): string;
  setGatewayId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GenerateGatewayClientCertificateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GenerateGatewayClientCertificateRequest): GenerateGatewayClientCertificateRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GenerateGatewayClientCertificateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GenerateGatewayClientCertificateRequest;
  static deserializeBinaryFromReader(message: GenerateGatewayClientCertificateRequest, reader: jspb.BinaryReader): GenerateGatewayClientCertificateRequest;
}

export namespace GenerateGatewayClientCertificateRequest {
  export type AsObject = {
    gatewayId: string,
  }
}

export class GenerateGatewayClientCertificateResponse extends jspb.Message {
  getTlsCert(): string;
  setTlsCert(value: string): void;

  getTlsKey(): string;
  setTlsKey(value: string): void;

  getCaCert(): string;
  setCaCert(value: string): void;

  hasExpiresAt(): boolean;
  clearExpiresAt(): void;
  getExpiresAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setExpiresAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GenerateGatewayClientCertificateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GenerateGatewayClientCertificateResponse): GenerateGatewayClientCertificateResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GenerateGatewayClientCertificateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GenerateGatewayClientCertificateResponse;
  static deserializeBinaryFromReader(message: GenerateGatewayClientCertificateResponse, reader: jspb.BinaryReader): GenerateGatewayClientCertificateResponse;
}

export namespace GenerateGatewayClientCertificateResponse {
  export type AsObject = {
    tlsCert: string,
    tlsKey: string,
    caCert: string,
    expiresAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class GetGatewayStatsRequest extends jspb.Message {
  getGatewayId(): string;
  setGatewayId(value: string): void;

  hasStart(): boolean;
  clearStart(): void;
  getStart(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setStart(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasEnd(): boolean;
  clearEnd(): void;
  getEnd(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setEnd(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGatewayStatsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetGatewayStatsRequest): GetGatewayStatsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetGatewayStatsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGatewayStatsRequest;
  static deserializeBinaryFromReader(message: GetGatewayStatsRequest, reader: jspb.BinaryReader): GetGatewayStatsRequest;
}

export namespace GetGatewayStatsRequest {
  export type AsObject = {
    gatewayId: string,
    start?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    end?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class GetGatewayStatsResponse extends jspb.Message {
  clearResultList(): void;
  getResultList(): Array<GatewayStats>;
  setResultList(value: Array<GatewayStats>): void;
  addResult(value?: GatewayStats, index?: number): GatewayStats;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGatewayStatsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetGatewayStatsResponse): GetGatewayStatsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetGatewayStatsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGatewayStatsResponse;
  static deserializeBinaryFromReader(message: GetGatewayStatsResponse, reader: jspb.BinaryReader): GetGatewayStatsResponse;
}

export namespace GetGatewayStatsResponse {
  export type AsObject = {
    resultList: Array<GatewayStats.AsObject>,
  }
}

export class GatewayStats extends jspb.Message {
  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getRxPackets(): number;
  setRxPackets(value: number): void;

  getTxPackets(): number;
  setTxPackets(value: number): void;

  getTxPacketsPerFrequencyMap(): jspb.Map<number, number>;
  clearTxPacketsPerFrequencyMap(): void;
  getRxPacketsPerFrequencyMap(): jspb.Map<number, number>;
  clearRxPacketsPerFrequencyMap(): void;
  getTxPacketsPerDrMap(): jspb.Map<number, number>;
  clearTxPacketsPerDrMap(): void;
  getRxPacketsPerDrMap(): jspb.Map<number, number>;
  clearRxPacketsPerDrMap(): void;
  getTxPacketsPerStatusMap(): jspb.Map<string, number>;
  clearTxPacketsPerStatusMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GatewayStats.AsObject;
  static toObject(includeInstance: boolean, msg: GatewayStats): GatewayStats.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GatewayStats, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GatewayStats;
  static deserializeBinaryFromReader(message: GatewayStats, reader: jspb.BinaryReader): GatewayStats;
}

export namespace GatewayStats {
  export type AsObject = {
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    rxPackets: number,
    txPackets: number,
    txPacketsPerFrequencyMap: Array<[number, number]>,
    rxPacketsPerFrequencyMap: Array<[number, number]>,
    txPacketsPerDrMap: Array<[number, number]>,
    rxPacketsPerDrMap: Array<[number, number]>,
    txPacketsPerStatusMap: Array<[string, number]>,
  }
}

