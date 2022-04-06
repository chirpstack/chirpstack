import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as common_common_pb from '../common/common_pb';
import * as gw_gw_pb from '../gw/gw_pb';


export class UplinkFrameLog extends jspb.Message {
  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): UplinkFrameLog;

  getTxInfo(): gw_gw_pb.UplinkTXInfo | undefined;
  setTxInfo(value?: gw_gw_pb.UplinkTXInfo): UplinkFrameLog;
  hasTxInfo(): boolean;
  clearTxInfo(): UplinkFrameLog;

  getRxInfoList(): Array<gw_gw_pb.UplinkRXInfo>;
  setRxInfoList(value: Array<gw_gw_pb.UplinkRXInfo>): UplinkFrameLog;
  clearRxInfoList(): UplinkFrameLog;
  addRxInfo(value?: gw_gw_pb.UplinkRXInfo, index?: number): gw_gw_pb.UplinkRXInfo;

  getMType(): common_common_pb.MType;
  setMType(value: common_common_pb.MType): UplinkFrameLog;

  getDevAddr(): string;
  setDevAddr(value: string): UplinkFrameLog;

  getDevEui(): string;
  setDevEui(value: string): UplinkFrameLog;

  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): UplinkFrameLog;
  hasTime(): boolean;
  clearTime(): UplinkFrameLog;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkFrameLog.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkFrameLog): UplinkFrameLog.AsObject;
  static serializeBinaryToWriter(message: UplinkFrameLog, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkFrameLog;
  static deserializeBinaryFromReader(message: UplinkFrameLog, reader: jspb.BinaryReader): UplinkFrameLog;
}

export namespace UplinkFrameLog {
  export type AsObject = {
    phyPayload: Uint8Array | string,
    txInfo?: gw_gw_pb.UplinkTXInfo.AsObject,
    rxInfoList: Array<gw_gw_pb.UplinkRXInfo.AsObject>,
    mType: common_common_pb.MType,
    devAddr: string,
    devEui: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class DownlinkFrameLog extends jspb.Message {
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): DownlinkFrameLog;
  hasTime(): boolean;
  clearTime(): DownlinkFrameLog;

  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): DownlinkFrameLog;

  getTxInfo(): gw_gw_pb.DownlinkTXInfo | undefined;
  setTxInfo(value?: gw_gw_pb.DownlinkTXInfo): DownlinkFrameLog;
  hasTxInfo(): boolean;
  clearTxInfo(): DownlinkFrameLog;

  getDownlinkId(): string;
  setDownlinkId(value: string): DownlinkFrameLog;

  getGatewayId(): string;
  setGatewayId(value: string): DownlinkFrameLog;

  getMType(): common_common_pb.MType;
  setMType(value: common_common_pb.MType): DownlinkFrameLog;

  getDevAddr(): string;
  setDevAddr(value: string): DownlinkFrameLog;

  getDevEui(): string;
  setDevEui(value: string): DownlinkFrameLog;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkFrameLog.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkFrameLog): DownlinkFrameLog.AsObject;
  static serializeBinaryToWriter(message: DownlinkFrameLog, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkFrameLog;
  static deserializeBinaryFromReader(message: DownlinkFrameLog, reader: jspb.BinaryReader): DownlinkFrameLog;
}

export namespace DownlinkFrameLog {
  export type AsObject = {
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    phyPayload: Uint8Array | string,
    txInfo?: gw_gw_pb.DownlinkTXInfo.AsObject,
    downlinkId: string,
    gatewayId: string,
    mType: common_common_pb.MType,
    devAddr: string,
    devEui: string,
  }
}

