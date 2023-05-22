// package: api
// file: api/frame_log.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as common_common_pb from "../common/common_pb";
import * as gw_gw_pb from "../gw/gw_pb";

export class UplinkFrameLog extends jspb.Message {
  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): gw_gw_pb.UplinkTxInfo | undefined;
  setTxInfo(value?: gw_gw_pb.UplinkTxInfo): void;

  clearRxInfoList(): void;
  getRxInfoList(): Array<gw_gw_pb.UplinkRxInfo>;
  setRxInfoList(value: Array<gw_gw_pb.UplinkRxInfo>): void;
  addRxInfo(value?: gw_gw_pb.UplinkRxInfo, index?: number): gw_gw_pb.UplinkRxInfo;

  getMType(): common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap];
  setMType(value: common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap]): void;

  getDevAddr(): string;
  setDevAddr(value: string): void;

  getDevEui(): string;
  setDevEui(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getPlaintextFOpts(): boolean;
  setPlaintextFOpts(value: boolean): void;

  getPlaintextFrmPayload(): boolean;
  setPlaintextFrmPayload(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkFrameLog.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkFrameLog): UplinkFrameLog.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkFrameLog, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkFrameLog;
  static deserializeBinaryFromReader(message: UplinkFrameLog, reader: jspb.BinaryReader): UplinkFrameLog;
}

export namespace UplinkFrameLog {
  export type AsObject = {
    phyPayload: Uint8Array | string,
    txInfo?: gw_gw_pb.UplinkTxInfo.AsObject,
    rxInfoList: Array<gw_gw_pb.UplinkRxInfo.AsObject>,
    mType: common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap],
    devAddr: string,
    devEui: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    plaintextFOpts: boolean,
    plaintextFrmPayload: boolean,
  }
}

export class DownlinkFrameLog extends jspb.Message {
  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): gw_gw_pb.DownlinkTxInfo | undefined;
  setTxInfo(value?: gw_gw_pb.DownlinkTxInfo): void;

  getDownlinkId(): number;
  setDownlinkId(value: number): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

  getMType(): common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap];
  setMType(value: common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap]): void;

  getDevAddr(): string;
  setDevAddr(value: string): void;

  getDevEui(): string;
  setDevEui(value: string): void;

  getPlaintextFOpts(): boolean;
  setPlaintextFOpts(value: boolean): void;

  getPlaintextFrmPayload(): boolean;
  setPlaintextFrmPayload(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkFrameLog.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkFrameLog): DownlinkFrameLog.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkFrameLog, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkFrameLog;
  static deserializeBinaryFromReader(message: DownlinkFrameLog, reader: jspb.BinaryReader): DownlinkFrameLog;
}

export namespace DownlinkFrameLog {
  export type AsObject = {
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    phyPayload: Uint8Array | string,
    txInfo?: gw_gw_pb.DownlinkTxInfo.AsObject,
    downlinkId: number,
    gatewayId: string,
    mType: common_common_pb.MTypeMap[keyof common_common_pb.MTypeMap],
    devAddr: string,
    devEui: string,
    plaintextFOpts: boolean,
    plaintextFrmPayload: boolean,
  }
}

