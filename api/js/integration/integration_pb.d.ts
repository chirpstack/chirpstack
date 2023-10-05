// package: integration
// file: integration/integration.proto

import * as jspb from "google-protobuf";
import * as common_common_pb from "../common/common_pb";
import * as gw_gw_pb from "../gw/gw_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_struct_pb from "google-protobuf/google/protobuf/struct_pb";

export class DeviceInfo extends jspb.Message {
  getTenantId(): string;
  setTenantId(value: string): void;

  getTenantName(): string;
  setTenantName(value: string): void;

  getApplicationId(): string;
  setApplicationId(value: string): void;

  getApplicationName(): string;
  setApplicationName(value: string): void;

  getDeviceProfileId(): string;
  setDeviceProfileId(value: string): void;

  getDeviceProfileName(): string;
  setDeviceProfileName(value: string): void;

  getDeviceName(): string;
  setDeviceName(value: string): void;

  getDevEui(): string;
  setDevEui(value: string): void;

  getDeviceClassEnabled(): common_common_pb.DeviceClassMap[keyof common_common_pb.DeviceClassMap];
  setDeviceClassEnabled(value: common_common_pb.DeviceClassMap[keyof common_common_pb.DeviceClassMap]): void;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeviceInfo.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceInfo): DeviceInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeviceInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeviceInfo;
  static deserializeBinaryFromReader(message: DeviceInfo, reader: jspb.BinaryReader): DeviceInfo;
}

export namespace DeviceInfo {
  export type AsObject = {
    tenantId: string,
    tenantName: string,
    applicationId: string,
    applicationName: string,
    deviceProfileId: string,
    deviceProfileName: string,
    deviceName: string,
    devEui: string,
    deviceClassEnabled: common_common_pb.DeviceClassMap[keyof common_common_pb.DeviceClassMap],
    tagsMap: Array<[string, string]>,
  }
}

export class UplinkRelayRxInfo extends jspb.Message {
  getDevEui(): string;
  setDevEui(value: string): void;

  getFrequency(): number;
  setFrequency(value: number): void;

  getDr(): number;
  setDr(value: number): void;

  getSnr(): number;
  setSnr(value: number): void;

  getRssi(): number;
  setRssi(value: number): void;

  getWorChannel(): number;
  setWorChannel(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkRelayRxInfo.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkRelayRxInfo): UplinkRelayRxInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkRelayRxInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkRelayRxInfo;
  static deserializeBinaryFromReader(message: UplinkRelayRxInfo, reader: jspb.BinaryReader): UplinkRelayRxInfo;
}

export namespace UplinkRelayRxInfo {
  export type AsObject = {
    devEui: string,
    frequency: number,
    dr: number,
    snr: number,
    rssi: number,
    worChannel: number,
  }
}

export class JoinServerContext extends jspb.Message {
  getSessionKeyId(): string;
  setSessionKeyId(value: string): void;

  hasAppSKey(): boolean;
  clearAppSKey(): void;
  getAppSKey(): common_common_pb.KeyEnvelope | undefined;
  setAppSKey(value?: common_common_pb.KeyEnvelope): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JoinServerContext.AsObject;
  static toObject(includeInstance: boolean, msg: JoinServerContext): JoinServerContext.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: JoinServerContext, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JoinServerContext;
  static deserializeBinaryFromReader(message: JoinServerContext, reader: jspb.BinaryReader): JoinServerContext;
}

export namespace JoinServerContext {
  export type AsObject = {
    sessionKeyId: string,
    appSKey?: common_common_pb.KeyEnvelope.AsObject,
  }
}

export class UplinkEvent extends jspb.Message {
  getDeduplicationId(): string;
  setDeduplicationId(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeviceInfo(): boolean;
  clearDeviceInfo(): void;
  getDeviceInfo(): DeviceInfo | undefined;
  setDeviceInfo(value?: DeviceInfo): void;

  getDevAddr(): string;
  setDevAddr(value: string): void;

  getAdr(): boolean;
  setAdr(value: boolean): void;

  getDr(): number;
  setDr(value: number): void;

  getFCnt(): number;
  setFCnt(value: number): void;

  getFPort(): number;
  setFPort(value: number): void;

  getConfirmed(): boolean;
  setConfirmed(value: boolean): void;

  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): void;

  hasObject(): boolean;
  clearObject(): void;
  getObject(): google_protobuf_struct_pb.Struct | undefined;
  setObject(value?: google_protobuf_struct_pb.Struct): void;

  clearRxInfoList(): void;
  getRxInfoList(): Array<gw_gw_pb.UplinkRxInfo>;
  setRxInfoList(value: Array<gw_gw_pb.UplinkRxInfo>): void;
  addRxInfo(value?: gw_gw_pb.UplinkRxInfo, index?: number): gw_gw_pb.UplinkRxInfo;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): gw_gw_pb.UplinkTxInfo | undefined;
  setTxInfo(value?: gw_gw_pb.UplinkTxInfo): void;

  hasRelayRxInfo(): boolean;
  clearRelayRxInfo(): void;
  getRelayRxInfo(): UplinkRelayRxInfo | undefined;
  setRelayRxInfo(value?: UplinkRelayRxInfo): void;

  hasJoinServerContext(): boolean;
  clearJoinServerContext(): void;
  getJoinServerContext(): JoinServerContext | undefined;
  setJoinServerContext(value?: JoinServerContext): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkEvent.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkEvent): UplinkEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkEvent;
  static deserializeBinaryFromReader(message: UplinkEvent, reader: jspb.BinaryReader): UplinkEvent;
}

export namespace UplinkEvent {
  export type AsObject = {
    deduplicationId: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceInfo?: DeviceInfo.AsObject,
    devAddr: string,
    adr: boolean,
    dr: number,
    fCnt: number,
    fPort: number,
    confirmed: boolean,
    data: Uint8Array | string,
    object?: google_protobuf_struct_pb.Struct.AsObject,
    rxInfoList: Array<gw_gw_pb.UplinkRxInfo.AsObject>,
    txInfo?: gw_gw_pb.UplinkTxInfo.AsObject,
    relayRxInfo?: UplinkRelayRxInfo.AsObject,
    joinServerContext?: JoinServerContext.AsObject,
  }
}

export class JoinEvent extends jspb.Message {
  getDeduplicationId(): string;
  setDeduplicationId(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeviceInfo(): boolean;
  clearDeviceInfo(): void;
  getDeviceInfo(): DeviceInfo | undefined;
  setDeviceInfo(value?: DeviceInfo): void;

  getDevAddr(): string;
  setDevAddr(value: string): void;

  hasRelayRxInfo(): boolean;
  clearRelayRxInfo(): void;
  getRelayRxInfo(): UplinkRelayRxInfo | undefined;
  setRelayRxInfo(value?: UplinkRelayRxInfo): void;

  hasJoinServerContext(): boolean;
  clearJoinServerContext(): void;
  getJoinServerContext(): JoinServerContext | undefined;
  setJoinServerContext(value?: JoinServerContext): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JoinEvent.AsObject;
  static toObject(includeInstance: boolean, msg: JoinEvent): JoinEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: JoinEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JoinEvent;
  static deserializeBinaryFromReader(message: JoinEvent, reader: jspb.BinaryReader): JoinEvent;
}

export namespace JoinEvent {
  export type AsObject = {
    deduplicationId: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceInfo?: DeviceInfo.AsObject,
    devAddr: string,
    relayRxInfo?: UplinkRelayRxInfo.AsObject,
    joinServerContext?: JoinServerContext.AsObject,
  }
}

export class AckEvent extends jspb.Message {
  getDeduplicationId(): string;
  setDeduplicationId(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeviceInfo(): boolean;
  clearDeviceInfo(): void;
  getDeviceInfo(): DeviceInfo | undefined;
  setDeviceInfo(value?: DeviceInfo): void;

  getQueueItemId(): string;
  setQueueItemId(value: string): void;

  getAcknowledged(): boolean;
  setAcknowledged(value: boolean): void;

  getFCntDown(): number;
  setFCntDown(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AckEvent.AsObject;
  static toObject(includeInstance: boolean, msg: AckEvent): AckEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AckEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AckEvent;
  static deserializeBinaryFromReader(message: AckEvent, reader: jspb.BinaryReader): AckEvent;
}

export namespace AckEvent {
  export type AsObject = {
    deduplicationId: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceInfo?: DeviceInfo.AsObject,
    queueItemId: string,
    acknowledged: boolean,
    fCntDown: number,
  }
}

export class TxAckEvent extends jspb.Message {
  getDownlinkId(): number;
  setDownlinkId(value: number): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeviceInfo(): boolean;
  clearDeviceInfo(): void;
  getDeviceInfo(): DeviceInfo | undefined;
  setDeviceInfo(value?: DeviceInfo): void;

  getQueueItemId(): string;
  setQueueItemId(value: string): void;

  getFCntDown(): number;
  setFCntDown(value: number): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): gw_gw_pb.DownlinkTxInfo | undefined;
  setTxInfo(value?: gw_gw_pb.DownlinkTxInfo): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TxAckEvent.AsObject;
  static toObject(includeInstance: boolean, msg: TxAckEvent): TxAckEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: TxAckEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TxAckEvent;
  static deserializeBinaryFromReader(message: TxAckEvent, reader: jspb.BinaryReader): TxAckEvent;
}

export namespace TxAckEvent {
  export type AsObject = {
    downlinkId: number,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceInfo?: DeviceInfo.AsObject,
    queueItemId: string,
    fCntDown: number,
    gatewayId: string,
    txInfo?: gw_gw_pb.DownlinkTxInfo.AsObject,
  }
}

export class LogEvent extends jspb.Message {
  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeviceInfo(): boolean;
  clearDeviceInfo(): void;
  getDeviceInfo(): DeviceInfo | undefined;
  setDeviceInfo(value?: DeviceInfo): void;

  getLevel(): LogLevelMap[keyof LogLevelMap];
  setLevel(value: LogLevelMap[keyof LogLevelMap]): void;

  getCode(): LogCodeMap[keyof LogCodeMap];
  setCode(value: LogCodeMap[keyof LogCodeMap]): void;

  getDescription(): string;
  setDescription(value: string): void;

  getContextMap(): jspb.Map<string, string>;
  clearContextMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LogEvent.AsObject;
  static toObject(includeInstance: boolean, msg: LogEvent): LogEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LogEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LogEvent;
  static deserializeBinaryFromReader(message: LogEvent, reader: jspb.BinaryReader): LogEvent;
}

export namespace LogEvent {
  export type AsObject = {
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceInfo?: DeviceInfo.AsObject,
    level: LogLevelMap[keyof LogLevelMap],
    code: LogCodeMap[keyof LogCodeMap],
    description: string,
    contextMap: Array<[string, string]>,
  }
}

export class StatusEvent extends jspb.Message {
  getDeduplicationId(): string;
  setDeduplicationId(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeviceInfo(): boolean;
  clearDeviceInfo(): void;
  getDeviceInfo(): DeviceInfo | undefined;
  setDeviceInfo(value?: DeviceInfo): void;

  getMargin(): number;
  setMargin(value: number): void;

  getExternalPowerSource(): boolean;
  setExternalPowerSource(value: boolean): void;

  getBatteryLevelUnavailable(): boolean;
  setBatteryLevelUnavailable(value: boolean): void;

  getBatteryLevel(): number;
  setBatteryLevel(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StatusEvent.AsObject;
  static toObject(includeInstance: boolean, msg: StatusEvent): StatusEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: StatusEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StatusEvent;
  static deserializeBinaryFromReader(message: StatusEvent, reader: jspb.BinaryReader): StatusEvent;
}

export namespace StatusEvent {
  export type AsObject = {
    deduplicationId: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceInfo?: DeviceInfo.AsObject,
    margin: number,
    externalPowerSource: boolean,
    batteryLevelUnavailable: boolean,
    batteryLevel: number,
  }
}

export class LocationEvent extends jspb.Message {
  getDeduplicationId(): string;
  setDeduplicationId(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeviceInfo(): boolean;
  clearDeviceInfo(): void;
  getDeviceInfo(): DeviceInfo | undefined;
  setDeviceInfo(value?: DeviceInfo): void;

  hasLocation(): boolean;
  clearLocation(): void;
  getLocation(): common_common_pb.Location | undefined;
  setLocation(value?: common_common_pb.Location): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LocationEvent.AsObject;
  static toObject(includeInstance: boolean, msg: LocationEvent): LocationEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LocationEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LocationEvent;
  static deserializeBinaryFromReader(message: LocationEvent, reader: jspb.BinaryReader): LocationEvent;
}

export namespace LocationEvent {
  export type AsObject = {
    deduplicationId: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceInfo?: DeviceInfo.AsObject,
    location?: common_common_pb.Location.AsObject,
  }
}

export class IntegrationEvent extends jspb.Message {
  getDeduplicationId(): string;
  setDeduplicationId(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasDeviceInfo(): boolean;
  clearDeviceInfo(): void;
  getDeviceInfo(): DeviceInfo | undefined;
  setDeviceInfo(value?: DeviceInfo): void;

  getIntegrationName(): string;
  setIntegrationName(value: string): void;

  getEventType(): string;
  setEventType(value: string): void;

  hasObject(): boolean;
  clearObject(): void;
  getObject(): google_protobuf_struct_pb.Struct | undefined;
  setObject(value?: google_protobuf_struct_pb.Struct): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IntegrationEvent.AsObject;
  static toObject(includeInstance: boolean, msg: IntegrationEvent): IntegrationEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IntegrationEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IntegrationEvent;
  static deserializeBinaryFromReader(message: IntegrationEvent, reader: jspb.BinaryReader): IntegrationEvent;
}

export namespace IntegrationEvent {
  export type AsObject = {
    deduplicationId: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    deviceInfo?: DeviceInfo.AsObject,
    integrationName: string,
    eventType: string,
    object?: google_protobuf_struct_pb.Struct.AsObject,
  }
}

export class DownlinkCommand extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getDevEui(): string;
  setDevEui(value: string): void;

  getConfirmed(): boolean;
  setConfirmed(value: boolean): void;

  getFPort(): number;
  setFPort(value: number): void;

  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): void;

  hasObject(): boolean;
  clearObject(): void;
  getObject(): google_protobuf_struct_pb.Struct | undefined;
  setObject(value?: google_protobuf_struct_pb.Struct): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkCommand.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkCommand): DownlinkCommand.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkCommand, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkCommand;
  static deserializeBinaryFromReader(message: DownlinkCommand, reader: jspb.BinaryReader): DownlinkCommand;
}

export namespace DownlinkCommand {
  export type AsObject = {
    id: string,
    devEui: string,
    confirmed: boolean,
    fPort: number,
    data: Uint8Array | string,
    object?: google_protobuf_struct_pb.Struct.AsObject,
  }
}

export interface LogLevelMap {
  INFO: 0;
  WARNING: 1;
  ERROR: 2;
}

export const LogLevel: LogLevelMap;

export interface LogCodeMap {
  UNKNOWN: 0;
  DOWNLINK_PAYLOAD_SIZE: 1;
  UPLINK_CODEC: 2;
  DOWNLINK_CODEC: 3;
  OTAA: 4;
  UPLINK_F_CNT_RESET: 5;
  UPLINK_MIC: 6;
  UPLINK_F_CNT_RETRANSMISSION: 7;
  DOWNLINK_GATEWAY: 8;
  RELAY_NEW_END_DEVICE: 9;
  F_CNT_DOWN: 10;
}

export const LogCode: LogCodeMap;

