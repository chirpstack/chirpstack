// package: api
// file: api/relay.proto

import * as jspb from "google-protobuf";
import * as google_api_annotations_pb from "../google/api/annotations_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

export class RelayListItem extends jspb.Message {
  getDevEui(): string;
  setDevEui(value: string): void;

  getName(): string;
  setName(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RelayListItem.AsObject;
  static toObject(includeInstance: boolean, msg: RelayListItem): RelayListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RelayListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RelayListItem;
  static deserializeBinaryFromReader(message: RelayListItem, reader: jspb.BinaryReader): RelayListItem;
}

export namespace RelayListItem {
  export type AsObject = {
    devEui: string,
    name: string,
  }
}

export class ListRelaysRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListRelaysRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListRelaysRequest): ListRelaysRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListRelaysRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListRelaysRequest;
  static deserializeBinaryFromReader(message: ListRelaysRequest, reader: jspb.BinaryReader): ListRelaysRequest;
}

export namespace ListRelaysRequest {
  export type AsObject = {
    limit: number,
    offset: number,
    applicationId: string,
  }
}

export class ListRelaysResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<RelayListItem>;
  setResultList(value: Array<RelayListItem>): void;
  addResult(value?: RelayListItem, index?: number): RelayListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListRelaysResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListRelaysResponse): ListRelaysResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListRelaysResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListRelaysResponse;
  static deserializeBinaryFromReader(message: ListRelaysResponse, reader: jspb.BinaryReader): ListRelaysResponse;
}

export namespace ListRelaysResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<RelayListItem.AsObject>,
  }
}

export class AddRelayDeviceRequest extends jspb.Message {
  getRelayDevEui(): string;
  setRelayDevEui(value: string): void;

  getDeviceDevEui(): string;
  setDeviceDevEui(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddRelayDeviceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddRelayDeviceRequest): AddRelayDeviceRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AddRelayDeviceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddRelayDeviceRequest;
  static deserializeBinaryFromReader(message: AddRelayDeviceRequest, reader: jspb.BinaryReader): AddRelayDeviceRequest;
}

export namespace AddRelayDeviceRequest {
  export type AsObject = {
    relayDevEui: string,
    deviceDevEui: string,
  }
}

export class RemoveRelayDeviceRequest extends jspb.Message {
  getRelayDevEui(): string;
  setRelayDevEui(value: string): void;

  getDeviceDevEui(): string;
  setDeviceDevEui(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RemoveRelayDeviceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RemoveRelayDeviceRequest): RemoveRelayDeviceRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RemoveRelayDeviceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RemoveRelayDeviceRequest;
  static deserializeBinaryFromReader(message: RemoveRelayDeviceRequest, reader: jspb.BinaryReader): RemoveRelayDeviceRequest;
}

export namespace RemoveRelayDeviceRequest {
  export type AsObject = {
    relayDevEui: string,
    deviceDevEui: string,
  }
}

export class ListRelayDevicesRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  getRelayDevEui(): string;
  setRelayDevEui(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListRelayDevicesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListRelayDevicesRequest): ListRelayDevicesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListRelayDevicesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListRelayDevicesRequest;
  static deserializeBinaryFromReader(message: ListRelayDevicesRequest, reader: jspb.BinaryReader): ListRelayDevicesRequest;
}

export namespace ListRelayDevicesRequest {
  export type AsObject = {
    limit: number,
    offset: number,
    relayDevEui: string,
  }
}

export class RelayDeviceListItem extends jspb.Message {
  getDevEui(): string;
  setDevEui(value: string): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getName(): string;
  setName(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RelayDeviceListItem.AsObject;
  static toObject(includeInstance: boolean, msg: RelayDeviceListItem): RelayDeviceListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RelayDeviceListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RelayDeviceListItem;
  static deserializeBinaryFromReader(message: RelayDeviceListItem, reader: jspb.BinaryReader): RelayDeviceListItem;
}

export namespace RelayDeviceListItem {
  export type AsObject = {
    devEui: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    name: string,
  }
}

export class ListRelayDevicesResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<RelayDeviceListItem>;
  setResultList(value: Array<RelayDeviceListItem>): void;
  addResult(value?: RelayDeviceListItem, index?: number): RelayDeviceListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListRelayDevicesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListRelayDevicesResponse): ListRelayDevicesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListRelayDevicesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListRelayDevicesResponse;
  static deserializeBinaryFromReader(message: ListRelayDevicesResponse, reader: jspb.BinaryReader): ListRelayDevicesResponse;
}

export namespace ListRelayDevicesResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<RelayDeviceListItem.AsObject>,
  }
}

