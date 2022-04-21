// package: meta
// file: meta/meta.proto

import * as jspb from "google-protobuf";
import * as common_common_pb from "../common/common_pb";
import * as gw_gw_pb from "../gw/gw_pb";

export class UplinkMeta extends jspb.Message {
  getDevEui(): string;
  setDevEui(value: string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): gw_gw_pb.UplinkTxInfo | undefined;
  setTxInfo(value?: gw_gw_pb.UplinkTxInfo): void;

  clearRxInfoList(): void;
  getRxInfoList(): Array<gw_gw_pb.UplinkRxInfo>;
  setRxInfoList(value: Array<gw_gw_pb.UplinkRxInfo>): void;
  addRxInfo(value?: gw_gw_pb.UplinkRxInfo, index?: number): gw_gw_pb.UplinkRxInfo;

  getPhyPayloadByteCount(): number;
  setPhyPayloadByteCount(value: number): void;

  getMacCommandByteCount(): number;
  setMacCommandByteCount(value: number): void;

  getApplicationPayloadByteCount(): number;
  setApplicationPayloadByteCount(value: number): void;

  getMessageType(): common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap];
  setMessageType(value: common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkMeta.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkMeta): UplinkMeta.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkMeta, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkMeta;
  static deserializeBinaryFromReader(message: UplinkMeta, reader: jspb.BinaryReader): UplinkMeta;
}

export namespace UplinkMeta {
  export type AsObject = {
    devEui: string,
    txInfo?: gw_gw_pb.UplinkTxInfo.AsObject,
    rxInfoList: Array<gw_gw_pb.UplinkRxInfo.AsObject>,
    phyPayloadByteCount: number,
    macCommandByteCount: number,
    applicationPayloadByteCount: number,
    messageType: common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap],
  }
}

export class DownlinkMeta extends jspb.Message {
  getDevEui(): string;
  setDevEui(value: string): void;

  getMulticastGroupId(): string;
  setMulticastGroupId(value: string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): gw_gw_pb.DownlinkTxInfo | undefined;
  setTxInfo(value?: gw_gw_pb.DownlinkTxInfo): void;

  getPhyPayloadByteCount(): number;
  setPhyPayloadByteCount(value: number): void;

  getMacCommandByteCount(): number;
  setMacCommandByteCount(value: number): void;

  getApplicationPayloadByteCount(): number;
  setApplicationPayloadByteCount(value: number): void;

  getMessageType(): common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap];
  setMessageType(value: common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap]): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkMeta.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkMeta): DownlinkMeta.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkMeta, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkMeta;
  static deserializeBinaryFromReader(message: DownlinkMeta, reader: jspb.BinaryReader): DownlinkMeta;
}

export namespace DownlinkMeta {
  export type AsObject = {
    devEui: string,
    multicastGroupId: string,
    txInfo?: gw_gw_pb.DownlinkTxInfo.AsObject,
    phyPayloadByteCount: number,
    macCommandByteCount: number,
    applicationPayloadByteCount: number,
    messageType: common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap],
    gatewayId: string,
  }
}

