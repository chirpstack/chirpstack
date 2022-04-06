import * as jspb from 'google-protobuf'

import * as common_common_pb from '../common/common_pb';
import * as gw_gw_pb from '../gw/gw_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';


export class UplinkEvent extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): UplinkEvent;

  getApplicationName(): string;
  setApplicationName(value: string): UplinkEvent;

  getDeviceName(): string;
  setDeviceName(value: string): UplinkEvent;

  getDevEui(): string;
  setDevEui(value: string): UplinkEvent;

  getRxInfoList(): Array<gw_gw_pb.UplinkRXInfo>;
  setRxInfoList(value: Array<gw_gw_pb.UplinkRXInfo>): UplinkEvent;
  clearRxInfoList(): UplinkEvent;
  addRxInfo(value?: gw_gw_pb.UplinkRXInfo, index?: number): gw_gw_pb.UplinkRXInfo;

  getTxInfo(): gw_gw_pb.UplinkTXInfo | undefined;
  setTxInfo(value?: gw_gw_pb.UplinkTXInfo): UplinkEvent;
  hasTxInfo(): boolean;
  clearTxInfo(): UplinkEvent;

  getAdr(): boolean;
  setAdr(value: boolean): UplinkEvent;

  getDr(): number;
  setDr(value: number): UplinkEvent;

  getFCnt(): number;
  setFCnt(value: number): UplinkEvent;

  getFPort(): number;
  setFPort(value: number): UplinkEvent;

  getData(): string;
  setData(value: string): UplinkEvent;

  getObjectJson(): string;
  setObjectJson(value: string): UplinkEvent;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): UplinkEvent;

  getConfirmedUplink(): boolean;
  setConfirmedUplink(value: boolean): UplinkEvent;

  getDevAddr(): string;
  setDevAddr(value: string): UplinkEvent;

  getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): UplinkEvent;
  hasPublishedAt(): boolean;
  clearPublishedAt(): UplinkEvent;

  getDeviceProfileId(): string;
  setDeviceProfileId(value: string): UplinkEvent;

  getDeviceProfileName(): string;
  setDeviceProfileName(value: string): UplinkEvent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkEvent.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkEvent): UplinkEvent.AsObject;
  static serializeBinaryToWriter(message: UplinkEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkEvent;
  static deserializeBinaryFromReader(message: UplinkEvent, reader: jspb.BinaryReader): UplinkEvent;
}

export namespace UplinkEvent {
  export type AsObject = {
    applicationId: string,
    applicationName: string,
    deviceName: string,
    devEui: string,
    rxInfoList: Array<gw_gw_pb.UplinkRXInfo.AsObject>,
    txInfo?: gw_gw_pb.UplinkTXInfo.AsObject,
    adr: boolean,
    dr: number,
    fCnt: number,
    fPort: number,
    data: string,
    objectJson: string,
    tagsMap: Array<[string, string]>,
    confirmedUplink: boolean,
    devAddr: string,
    publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceProfileId: string,
    deviceProfileName: string,
  }
}

export class JoinEvent extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): JoinEvent;

  getApplicationName(): string;
  setApplicationName(value: string): JoinEvent;

  getDeviceName(): string;
  setDeviceName(value: string): JoinEvent;

  getDevEui(): string;
  setDevEui(value: string): JoinEvent;

  getDevAddr(): string;
  setDevAddr(value: string): JoinEvent;

  getRxInfoList(): Array<gw_gw_pb.UplinkRXInfo>;
  setRxInfoList(value: Array<gw_gw_pb.UplinkRXInfo>): JoinEvent;
  clearRxInfoList(): JoinEvent;
  addRxInfo(value?: gw_gw_pb.UplinkRXInfo, index?: number): gw_gw_pb.UplinkRXInfo;

  getTxInfo(): gw_gw_pb.UplinkTXInfo | undefined;
  setTxInfo(value?: gw_gw_pb.UplinkTXInfo): JoinEvent;
  hasTxInfo(): boolean;
  clearTxInfo(): JoinEvent;

  getDr(): number;
  setDr(value: number): JoinEvent;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): JoinEvent;

  getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): JoinEvent;
  hasPublishedAt(): boolean;
  clearPublishedAt(): JoinEvent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JoinEvent.AsObject;
  static toObject(includeInstance: boolean, msg: JoinEvent): JoinEvent.AsObject;
  static serializeBinaryToWriter(message: JoinEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JoinEvent;
  static deserializeBinaryFromReader(message: JoinEvent, reader: jspb.BinaryReader): JoinEvent;
}

export namespace JoinEvent {
  export type AsObject = {
    applicationId: string,
    applicationName: string,
    deviceName: string,
    devEui: string,
    devAddr: string,
    rxInfoList: Array<gw_gw_pb.UplinkRXInfo.AsObject>,
    txInfo?: gw_gw_pb.UplinkTXInfo.AsObject,
    dr: number,
    tagsMap: Array<[string, string]>,
    publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class AckEvent extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): AckEvent;

  getApplicationName(): string;
  setApplicationName(value: string): AckEvent;

  getDeviceName(): string;
  setDeviceName(value: string): AckEvent;

  getDevEui(): string;
  setDevEui(value: string): AckEvent;

  getAcknowledged(): boolean;
  setAcknowledged(value: boolean): AckEvent;

  getFCnt(): number;
  setFCnt(value: number): AckEvent;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): AckEvent;

  getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): AckEvent;
  hasPublishedAt(): boolean;
  clearPublishedAt(): AckEvent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AckEvent.AsObject;
  static toObject(includeInstance: boolean, msg: AckEvent): AckEvent.AsObject;
  static serializeBinaryToWriter(message: AckEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AckEvent;
  static deserializeBinaryFromReader(message: AckEvent, reader: jspb.BinaryReader): AckEvent;
}

export namespace AckEvent {
  export type AsObject = {
    applicationId: string,
    applicationName: string,
    deviceName: string,
    devEui: string,
    acknowledged: boolean,
    fCnt: number,
    tagsMap: Array<[string, string]>,
    publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class TxAckEvent extends jspb.Message {
  getApplicationId(): number;
  setApplicationId(value: number): TxAckEvent;

  getApplicationName(): string;
  setApplicationName(value: string): TxAckEvent;

  getDeviceName(): string;
  setDeviceName(value: string): TxAckEvent;

  getDevEui(): string;
  setDevEui(value: string): TxAckEvent;

  getFCnt(): number;
  setFCnt(value: number): TxAckEvent;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): TxAckEvent;

  getGatewayId(): string;
  setGatewayId(value: string): TxAckEvent;

  getTxInfo(): gw_gw_pb.DownlinkTXInfo | undefined;
  setTxInfo(value?: gw_gw_pb.DownlinkTXInfo): TxAckEvent;
  hasTxInfo(): boolean;
  clearTxInfo(): TxAckEvent;

  getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): TxAckEvent;
  hasPublishedAt(): boolean;
  clearPublishedAt(): TxAckEvent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TxAckEvent.AsObject;
  static toObject(includeInstance: boolean, msg: TxAckEvent): TxAckEvent.AsObject;
  static serializeBinaryToWriter(message: TxAckEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TxAckEvent;
  static deserializeBinaryFromReader(message: TxAckEvent, reader: jspb.BinaryReader): TxAckEvent;
}

export namespace TxAckEvent {
  export type AsObject = {
    applicationId: number,
    applicationName: string,
    deviceName: string,
    devEui: string,
    fCnt: number,
    tagsMap: Array<[string, string]>,
    gatewayId: string,
    txInfo?: gw_gw_pb.DownlinkTXInfo.AsObject,
    publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class ErrorEvent extends jspb.Message {
  getApplicationId(): number;
  setApplicationId(value: number): ErrorEvent;

  getApplicationName(): string;
  setApplicationName(value: string): ErrorEvent;

  getDeviceName(): string;
  setDeviceName(value: string): ErrorEvent;

  getDevEui(): string;
  setDevEui(value: string): ErrorEvent;

  getType(): ErrorType;
  setType(value: ErrorType): ErrorEvent;

  getError(): string;
  setError(value: string): ErrorEvent;

  getFCnt(): number;
  setFCnt(value: number): ErrorEvent;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): ErrorEvent;

  getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): ErrorEvent;
  hasPublishedAt(): boolean;
  clearPublishedAt(): ErrorEvent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ErrorEvent.AsObject;
  static toObject(includeInstance: boolean, msg: ErrorEvent): ErrorEvent.AsObject;
  static serializeBinaryToWriter(message: ErrorEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ErrorEvent;
  static deserializeBinaryFromReader(message: ErrorEvent, reader: jspb.BinaryReader): ErrorEvent;
}

export namespace ErrorEvent {
  export type AsObject = {
    applicationId: number,
    applicationName: string,
    deviceName: string,
    devEui: string,
    type: ErrorType,
    error: string,
    fCnt: number,
    tagsMap: Array<[string, string]>,
    publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class StatusEvent extends jspb.Message {
  getApplicationId(): number;
  setApplicationId(value: number): StatusEvent;

  getApplicationName(): string;
  setApplicationName(value: string): StatusEvent;

  getDeviceName(): string;
  setDeviceName(value: string): StatusEvent;

  getDevEui(): string;
  setDevEui(value: string): StatusEvent;

  getMargin(): number;
  setMargin(value: number): StatusEvent;

  getExternalPowerSource(): boolean;
  setExternalPowerSource(value: boolean): StatusEvent;

  getBatteryLevelUnavailable(): boolean;
  setBatteryLevelUnavailable(value: boolean): StatusEvent;

  getBatteryLevel(): number;
  setBatteryLevel(value: number): StatusEvent;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): StatusEvent;

  getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): StatusEvent;
  hasPublishedAt(): boolean;
  clearPublishedAt(): StatusEvent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StatusEvent.AsObject;
  static toObject(includeInstance: boolean, msg: StatusEvent): StatusEvent.AsObject;
  static serializeBinaryToWriter(message: StatusEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StatusEvent;
  static deserializeBinaryFromReader(message: StatusEvent, reader: jspb.BinaryReader): StatusEvent;
}

export namespace StatusEvent {
  export type AsObject = {
    applicationId: number,
    applicationName: string,
    deviceName: string,
    devEui: string,
    margin: number,
    externalPowerSource: boolean,
    batteryLevelUnavailable: boolean,
    batteryLevel: number,
    tagsMap: Array<[string, string]>,
    publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class LocationEvent extends jspb.Message {
  getApplicationId(): number;
  setApplicationId(value: number): LocationEvent;

  getApplicationName(): string;
  setApplicationName(value: string): LocationEvent;

  getDeviceName(): string;
  setDeviceName(value: string): LocationEvent;

  getDevEui(): string;
  setDevEui(value: string): LocationEvent;

  getLocation(): common_common_pb.Location | undefined;
  setLocation(value?: common_common_pb.Location): LocationEvent;
  hasLocation(): boolean;
  clearLocation(): LocationEvent;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): LocationEvent;

  getUplinkIdsList(): Array<string>;
  setUplinkIdsList(value: Array<string>): LocationEvent;
  clearUplinkIdsList(): LocationEvent;
  addUplinkIds(value: string, index?: number): LocationEvent;

  getFCnt(): number;
  setFCnt(value: number): LocationEvent;

  getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): LocationEvent;
  hasPublishedAt(): boolean;
  clearPublishedAt(): LocationEvent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocationEvent.AsObject;
  static toObject(includeInstance: boolean, msg: LocationEvent): LocationEvent.AsObject;
  static serializeBinaryToWriter(message: LocationEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocationEvent;
  static deserializeBinaryFromReader(message: LocationEvent, reader: jspb.BinaryReader): LocationEvent;
}

export namespace LocationEvent {
  export type AsObject = {
    applicationId: number,
    applicationName: string,
    deviceName: string,
    devEui: string,
    location?: common_common_pb.Location.AsObject,
    tagsMap: Array<[string, string]>,
    uplinkIdsList: Array<string>,
    fCnt: number,
    publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class IntegrationEvent extends jspb.Message {
  getApplicationId(): number;
  setApplicationId(value: number): IntegrationEvent;

  getApplicationName(): string;
  setApplicationName(value: string): IntegrationEvent;

  getDeviceName(): string;
  setDeviceName(value: string): IntegrationEvent;

  getDevEui(): string;
  setDevEui(value: string): IntegrationEvent;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): IntegrationEvent;

  getIntegrationName(): string;
  setIntegrationName(value: string): IntegrationEvent;

  getEventType(): string;
  setEventType(value: string): IntegrationEvent;

  getObjectJson(): string;
  setObjectJson(value: string): IntegrationEvent;

  getPublishedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setPublishedAt(value?: google_protobuf_timestamp_pb.Timestamp): IntegrationEvent;
  hasPublishedAt(): boolean;
  clearPublishedAt(): IntegrationEvent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IntegrationEvent.AsObject;
  static toObject(includeInstance: boolean, msg: IntegrationEvent): IntegrationEvent.AsObject;
  static serializeBinaryToWriter(message: IntegrationEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IntegrationEvent;
  static deserializeBinaryFromReader(message: IntegrationEvent, reader: jspb.BinaryReader): IntegrationEvent;
}

export namespace IntegrationEvent {
  export type AsObject = {
    applicationId: number,
    applicationName: string,
    deviceName: string,
    devEui: string,
    tagsMap: Array<[string, string]>,
    integrationName: string,
    eventType: string,
    objectJson: string,
    publishedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export enum ErrorType { 
  UNKNOWN = 0,
  DOWNLINK_PAYLOAD_SIZE = 1,
  DOWNLINK_FCNT = 2,
  UPLINK_CODEC = 3,
  DOWNLINK_CODEC = 4,
  OTAA = 5,
  UPLINK_FCNT_RESET = 6,
  UPLINK_MIC = 7,
  UPLINK_FCNT_RETRANSMISSION = 8,
  DOWNLINK_GATEWAY = 9,
}
