// package: gw
// file: gw/gw.proto

import * as jspb from "google-protobuf";
import * as common_common_pb from "../common/common_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_duration_pb from "google-protobuf/google/protobuf/duration_pb";

export class Modulation extends jspb.Message {
  hasLora(): boolean;
  clearLora(): void;
  getLora(): LoRaModulationInfo | undefined;
  setLora(value?: LoRaModulationInfo): void;

  hasFsk(): boolean;
  clearFsk(): void;
  getFsk(): FSKModulationInfo | undefined;
  setFsk(value?: FSKModulationInfo): void;

  hasLrFhss(): boolean;
  clearLrFhss(): void;
  getLrFhss(): LRFHSSModulationInfo | undefined;
  setLrFhss(value?: LRFHSSModulationInfo): void;

  getParametersCase(): Modulation.ParametersCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Modulation.AsObject;
  static toObject(includeInstance: boolean, msg: Modulation): Modulation.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Modulation, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Modulation;
  static deserializeBinaryFromReader(message: Modulation, reader: jspb.BinaryReader): Modulation;
}

export namespace Modulation {
  export type AsObject = {
    lora?: LoRaModulationInfo.AsObject,
    fsk?: FSKModulationInfo.AsObject,
    lrFhss?: LRFHSSModulationInfo.AsObject,
  }

  export enum ParametersCase {
    PARAMETERS_NOT_SET = 0,
    LORA = 3,
    FSK = 4,
    LR_FHSS = 5,
  }
}

export class UplinkTXInfo extends jspb.Message {
  getFrequency(): number;
  setFrequency(value: number): void;

  getModulation(): common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap];
  setModulation(value: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap]): void;

  hasLoraModulationInfo(): boolean;
  clearLoraModulationInfo(): void;
  getLoraModulationInfo(): LoRaModulationInfo | undefined;
  setLoraModulationInfo(value?: LoRaModulationInfo): void;

  hasFskModulationInfo(): boolean;
  clearFskModulationInfo(): void;
  getFskModulationInfo(): FSKModulationInfo | undefined;
  setFskModulationInfo(value?: FSKModulationInfo): void;

  hasLrFhssModulationInfo(): boolean;
  clearLrFhssModulationInfo(): void;
  getLrFhssModulationInfo(): LRFHSSModulationInfo | undefined;
  setLrFhssModulationInfo(value?: LRFHSSModulationInfo): void;

  getModulationInfoCase(): UplinkTXInfo.ModulationInfoCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkTXInfo.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkTXInfo): UplinkTXInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkTXInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkTXInfo;
  static deserializeBinaryFromReader(message: UplinkTXInfo, reader: jspb.BinaryReader): UplinkTXInfo;
}

export namespace UplinkTXInfo {
  export type AsObject = {
    frequency: number,
    modulation: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap],
    loraModulationInfo?: LoRaModulationInfo.AsObject,
    fskModulationInfo?: FSKModulationInfo.AsObject,
    lrFhssModulationInfo?: LRFHSSModulationInfo.AsObject,
  }

  export enum ModulationInfoCase {
    MODULATION_INFO_NOT_SET = 0,
    LORA_MODULATION_INFO = 3,
    FSK_MODULATION_INFO = 4,
    LR_FHSS_MODULATION_INFO = 5,
  }
}

export class LoRaModulationInfo extends jspb.Message {
  getBandwidth(): number;
  setBandwidth(value: number): void;

  getSpreadingFactor(): number;
  setSpreadingFactor(value: number): void;

  getCodeRate(): string;
  setCodeRate(value: string): void;

  getPolarizationInversion(): boolean;
  setPolarizationInversion(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoRaModulationInfo.AsObject;
  static toObject(includeInstance: boolean, msg: LoRaModulationInfo): LoRaModulationInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LoRaModulationInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoRaModulationInfo;
  static deserializeBinaryFromReader(message: LoRaModulationInfo, reader: jspb.BinaryReader): LoRaModulationInfo;
}

export namespace LoRaModulationInfo {
  export type AsObject = {
    bandwidth: number,
    spreadingFactor: number,
    codeRate: string,
    polarizationInversion: boolean,
  }
}

export class FSKModulationInfo extends jspb.Message {
  getFrequencyDeviation(): number;
  setFrequencyDeviation(value: number): void;

  getDatarate(): number;
  setDatarate(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FSKModulationInfo.AsObject;
  static toObject(includeInstance: boolean, msg: FSKModulationInfo): FSKModulationInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FSKModulationInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FSKModulationInfo;
  static deserializeBinaryFromReader(message: FSKModulationInfo, reader: jspb.BinaryReader): FSKModulationInfo;
}

export namespace FSKModulationInfo {
  export type AsObject = {
    frequencyDeviation: number,
    datarate: number,
  }
}

export class LRFHSSModulationInfo extends jspb.Message {
  getOperatingChannelWidth(): number;
  setOperatingChannelWidth(value: number): void;

  getCodeRate(): string;
  setCodeRate(value: string): void;

  getGridSteps(): number;
  setGridSteps(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LRFHSSModulationInfo.AsObject;
  static toObject(includeInstance: boolean, msg: LRFHSSModulationInfo): LRFHSSModulationInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LRFHSSModulationInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LRFHSSModulationInfo;
  static deserializeBinaryFromReader(message: LRFHSSModulationInfo, reader: jspb.BinaryReader): LRFHSSModulationInfo;
}

export namespace LRFHSSModulationInfo {
  export type AsObject = {
    operatingChannelWidth: number,
    codeRate: string,
    gridSteps: number,
  }
}

export class EncryptedFineTimestamp extends jspb.Message {
  getAesKeyIndex(): number;
  setAesKeyIndex(value: number): void;

  getEncryptedNs(): Uint8Array | string;
  getEncryptedNs_asU8(): Uint8Array;
  getEncryptedNs_asB64(): string;
  setEncryptedNs(value: Uint8Array | string): void;

  getFpgaId(): Uint8Array | string;
  getFpgaId_asU8(): Uint8Array;
  getFpgaId_asB64(): string;
  setFpgaId(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EncryptedFineTimestamp.AsObject;
  static toObject(includeInstance: boolean, msg: EncryptedFineTimestamp): EncryptedFineTimestamp.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: EncryptedFineTimestamp, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EncryptedFineTimestamp;
  static deserializeBinaryFromReader(message: EncryptedFineTimestamp, reader: jspb.BinaryReader): EncryptedFineTimestamp;
}

export namespace EncryptedFineTimestamp {
  export type AsObject = {
    aesKeyIndex: number,
    encryptedNs: Uint8Array | string,
    fpgaId: Uint8Array | string,
  }
}

export class PlainFineTimestamp extends jspb.Message {
  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PlainFineTimestamp.AsObject;
  static toObject(includeInstance: boolean, msg: PlainFineTimestamp): PlainFineTimestamp.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PlainFineTimestamp, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PlainFineTimestamp;
  static deserializeBinaryFromReader(message: PlainFineTimestamp, reader: jspb.BinaryReader): PlainFineTimestamp;
}

export namespace PlainFineTimestamp {
  export type AsObject = {
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class GatewayStats extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getIp(): string;
  setIp(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasLocation(): boolean;
  clearLocation(): void;
  getLocation(): common_common_pb.Location | undefined;
  setLocation(value?: common_common_pb.Location): void;

  getConfigVersion(): string;
  setConfigVersion(value: string): void;

  getRxPacketsReceived(): number;
  setRxPacketsReceived(value: number): void;

  getRxPacketsReceivedOk(): number;
  setRxPacketsReceivedOk(value: number): void;

  getTxPacketsReceived(): number;
  setTxPacketsReceived(value: number): void;

  getTxPacketsEmitted(): number;
  setTxPacketsEmitted(value: number): void;

  getMetaDataMap(): jspb.Map<string, string>;
  clearMetaDataMap(): void;
  getStatsId(): Uint8Array | string;
  getStatsId_asU8(): Uint8Array;
  getStatsId_asB64(): string;
  setStatsId(value: Uint8Array | string): void;

  getTxPacketsPerFrequencyMap(): jspb.Map<number, number>;
  clearTxPacketsPerFrequencyMap(): void;
  getRxPacketsPerFrequencyMap(): jspb.Map<number, number>;
  clearRxPacketsPerFrequencyMap(): void;
  clearTxPacketsPerModulationList(): void;
  getTxPacketsPerModulationList(): Array<PerModulationCount>;
  setTxPacketsPerModulationList(value: Array<PerModulationCount>): void;
  addTxPacketsPerModulation(value?: PerModulationCount, index?: number): PerModulationCount;

  clearRxPacketsPerModulationList(): void;
  getRxPacketsPerModulationList(): Array<PerModulationCount>;
  setRxPacketsPerModulationList(value: Array<PerModulationCount>): void;
  addRxPacketsPerModulation(value?: PerModulationCount, index?: number): PerModulationCount;

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
    gatewayId: Uint8Array | string,
    ip: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    location?: common_common_pb.Location.AsObject,
    configVersion: string,
    rxPacketsReceived: number,
    rxPacketsReceivedOk: number,
    txPacketsReceived: number,
    txPacketsEmitted: number,
    metaDataMap: Array<[string, string]>,
    statsId: Uint8Array | string,
    txPacketsPerFrequencyMap: Array<[number, number]>,
    rxPacketsPerFrequencyMap: Array<[number, number]>,
    txPacketsPerModulationList: Array<PerModulationCount.AsObject>,
    rxPacketsPerModulationList: Array<PerModulationCount.AsObject>,
    txPacketsPerStatusMap: Array<[string, number]>,
  }
}

export class PerModulationCount extends jspb.Message {
  hasModulation(): boolean;
  clearModulation(): void;
  getModulation(): Modulation | undefined;
  setModulation(value?: Modulation): void;

  getCount(): number;
  setCount(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PerModulationCount.AsObject;
  static toObject(includeInstance: boolean, msg: PerModulationCount): PerModulationCount.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PerModulationCount, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PerModulationCount;
  static deserializeBinaryFromReader(message: PerModulationCount, reader: jspb.BinaryReader): PerModulationCount;
}

export namespace PerModulationCount {
  export type AsObject = {
    modulation?: Modulation.AsObject,
    count: number,
  }
}

export class UplinkRXInfo extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasTimeSinceGpsEpoch(): boolean;
  clearTimeSinceGpsEpoch(): void;
  getTimeSinceGpsEpoch(): google_protobuf_duration_pb.Duration | undefined;
  setTimeSinceGpsEpoch(value?: google_protobuf_duration_pb.Duration): void;

  getRssi(): number;
  setRssi(value: number): void;

  getLoraSnr(): number;
  setLoraSnr(value: number): void;

  getChannel(): number;
  setChannel(value: number): void;

  getRfChain(): number;
  setRfChain(value: number): void;

  getBoard(): number;
  setBoard(value: number): void;

  getAntenna(): number;
  setAntenna(value: number): void;

  hasLocation(): boolean;
  clearLocation(): void;
  getLocation(): common_common_pb.Location | undefined;
  setLocation(value?: common_common_pb.Location): void;

  getFineTimestampType(): FineTimestampTypeMap[keyof FineTimestampTypeMap];
  setFineTimestampType(value: FineTimestampTypeMap[keyof FineTimestampTypeMap]): void;

  hasEncryptedFineTimestamp(): boolean;
  clearEncryptedFineTimestamp(): void;
  getEncryptedFineTimestamp(): EncryptedFineTimestamp | undefined;
  setEncryptedFineTimestamp(value?: EncryptedFineTimestamp): void;

  hasPlainFineTimestamp(): boolean;
  clearPlainFineTimestamp(): void;
  getPlainFineTimestamp(): PlainFineTimestamp | undefined;
  setPlainFineTimestamp(value?: PlainFineTimestamp): void;

  getContext(): Uint8Array | string;
  getContext_asU8(): Uint8Array;
  getContext_asB64(): string;
  setContext(value: Uint8Array | string): void;

  getUplinkId(): Uint8Array | string;
  getUplinkId_asU8(): Uint8Array;
  getUplinkId_asB64(): string;
  setUplinkId(value: Uint8Array | string): void;

  getCrcStatus(): CRCStatusMap[keyof CRCStatusMap];
  setCrcStatus(value: CRCStatusMap[keyof CRCStatusMap]): void;

  getMetadataMap(): jspb.Map<string, string>;
  clearMetadataMap(): void;
  getFineTimestampCase(): UplinkRXInfo.FineTimestampCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkRXInfo.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkRXInfo): UplinkRXInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkRXInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkRXInfo;
  static deserializeBinaryFromReader(message: UplinkRXInfo, reader: jspb.BinaryReader): UplinkRXInfo;
}

export namespace UplinkRXInfo {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    timeSinceGpsEpoch?: google_protobuf_duration_pb.Duration.AsObject,
    rssi: number,
    loraSnr: number,
    channel: number,
    rfChain: number,
    board: number,
    antenna: number,
    location?: common_common_pb.Location.AsObject,
    fineTimestampType: FineTimestampTypeMap[keyof FineTimestampTypeMap],
    encryptedFineTimestamp?: EncryptedFineTimestamp.AsObject,
    plainFineTimestamp?: PlainFineTimestamp.AsObject,
    context: Uint8Array | string,
    uplinkId: Uint8Array | string,
    crcStatus: CRCStatusMap[keyof CRCStatusMap],
    metadataMap: Array<[string, string]>,
  }

  export enum FineTimestampCase {
    FINE_TIMESTAMP_NOT_SET = 0,
    ENCRYPTED_FINE_TIMESTAMP = 13,
    PLAIN_FINE_TIMESTAMP = 14,
  }
}

export class DownlinkTXInfo extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getFrequency(): number;
  setFrequency(value: number): void;

  getPower(): number;
  setPower(value: number): void;

  getModulation(): common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap];
  setModulation(value: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap]): void;

  hasLoraModulationInfo(): boolean;
  clearLoraModulationInfo(): void;
  getLoraModulationInfo(): LoRaModulationInfo | undefined;
  setLoraModulationInfo(value?: LoRaModulationInfo): void;

  hasFskModulationInfo(): boolean;
  clearFskModulationInfo(): void;
  getFskModulationInfo(): FSKModulationInfo | undefined;
  setFskModulationInfo(value?: FSKModulationInfo): void;

  getBoard(): number;
  setBoard(value: number): void;

  getAntenna(): number;
  setAntenna(value: number): void;

  getTiming(): DownlinkTimingMap[keyof DownlinkTimingMap];
  setTiming(value: DownlinkTimingMap[keyof DownlinkTimingMap]): void;

  hasImmediatelyTimingInfo(): boolean;
  clearImmediatelyTimingInfo(): void;
  getImmediatelyTimingInfo(): ImmediatelyTimingInfo | undefined;
  setImmediatelyTimingInfo(value?: ImmediatelyTimingInfo): void;

  hasDelayTimingInfo(): boolean;
  clearDelayTimingInfo(): void;
  getDelayTimingInfo(): DelayTimingInfo | undefined;
  setDelayTimingInfo(value?: DelayTimingInfo): void;

  hasGpsEpochTimingInfo(): boolean;
  clearGpsEpochTimingInfo(): void;
  getGpsEpochTimingInfo(): GPSEpochTimingInfo | undefined;
  setGpsEpochTimingInfo(value?: GPSEpochTimingInfo): void;

  getContext(): Uint8Array | string;
  getContext_asU8(): Uint8Array;
  getContext_asB64(): string;
  setContext(value: Uint8Array | string): void;

  getModulationInfoCase(): DownlinkTXInfo.ModulationInfoCase;
  getTimingInfoCase(): DownlinkTXInfo.TimingInfoCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkTXInfo.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkTXInfo): DownlinkTXInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkTXInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkTXInfo;
  static deserializeBinaryFromReader(message: DownlinkTXInfo, reader: jspb.BinaryReader): DownlinkTXInfo;
}

export namespace DownlinkTXInfo {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    frequency: number,
    power: number,
    modulation: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap],
    loraModulationInfo?: LoRaModulationInfo.AsObject,
    fskModulationInfo?: FSKModulationInfo.AsObject,
    board: number,
    antenna: number,
    timing: DownlinkTimingMap[keyof DownlinkTimingMap],
    immediatelyTimingInfo?: ImmediatelyTimingInfo.AsObject,
    delayTimingInfo?: DelayTimingInfo.AsObject,
    gpsEpochTimingInfo?: GPSEpochTimingInfo.AsObject,
    context: Uint8Array | string,
  }

  export enum ModulationInfoCase {
    MODULATION_INFO_NOT_SET = 0,
    LORA_MODULATION_INFO = 8,
    FSK_MODULATION_INFO = 9,
  }

  export enum TimingInfoCase {
    TIMING_INFO_NOT_SET = 0,
    IMMEDIATELY_TIMING_INFO = 13,
    DELAY_TIMING_INFO = 14,
    GPS_EPOCH_TIMING_INFO = 15,
  }
}

export class ImmediatelyTimingInfo extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ImmediatelyTimingInfo.AsObject;
  static toObject(includeInstance: boolean, msg: ImmediatelyTimingInfo): ImmediatelyTimingInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ImmediatelyTimingInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ImmediatelyTimingInfo;
  static deserializeBinaryFromReader(message: ImmediatelyTimingInfo, reader: jspb.BinaryReader): ImmediatelyTimingInfo;
}

export namespace ImmediatelyTimingInfo {
  export type AsObject = {
  }
}

export class DelayTimingInfo extends jspb.Message {
  hasDelay(): boolean;
  clearDelay(): void;
  getDelay(): google_protobuf_duration_pb.Duration | undefined;
  setDelay(value?: google_protobuf_duration_pb.Duration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DelayTimingInfo.AsObject;
  static toObject(includeInstance: boolean, msg: DelayTimingInfo): DelayTimingInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DelayTimingInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DelayTimingInfo;
  static deserializeBinaryFromReader(message: DelayTimingInfo, reader: jspb.BinaryReader): DelayTimingInfo;
}

export namespace DelayTimingInfo {
  export type AsObject = {
    delay?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class GPSEpochTimingInfo extends jspb.Message {
  hasTimeSinceGpsEpoch(): boolean;
  clearTimeSinceGpsEpoch(): void;
  getTimeSinceGpsEpoch(): google_protobuf_duration_pb.Duration | undefined;
  setTimeSinceGpsEpoch(value?: google_protobuf_duration_pb.Duration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GPSEpochTimingInfo.AsObject;
  static toObject(includeInstance: boolean, msg: GPSEpochTimingInfo): GPSEpochTimingInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GPSEpochTimingInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GPSEpochTimingInfo;
  static deserializeBinaryFromReader(message: GPSEpochTimingInfo, reader: jspb.BinaryReader): GPSEpochTimingInfo;
}

export namespace GPSEpochTimingInfo {
  export type AsObject = {
    timeSinceGpsEpoch?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class UplinkFrame extends jspb.Message {
  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): UplinkTXInfo | undefined;
  setTxInfo(value?: UplinkTXInfo): void;

  hasRxInfo(): boolean;
  clearRxInfo(): void;
  getRxInfo(): UplinkRXInfo | undefined;
  setRxInfo(value?: UplinkRXInfo): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkFrame.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkFrame): UplinkFrame.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkFrame, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkFrame;
  static deserializeBinaryFromReader(message: UplinkFrame, reader: jspb.BinaryReader): UplinkFrame;
}

export namespace UplinkFrame {
  export type AsObject = {
    phyPayload: Uint8Array | string,
    txInfo?: UplinkTXInfo.AsObject,
    rxInfo?: UplinkRXInfo.AsObject,
  }
}

export class UplinkFrameSet extends jspb.Message {
  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): UplinkTXInfo | undefined;
  setTxInfo(value?: UplinkTXInfo): void;

  clearRxInfoList(): void;
  getRxInfoList(): Array<UplinkRXInfo>;
  setRxInfoList(value: Array<UplinkRXInfo>): void;
  addRxInfo(value?: UplinkRXInfo, index?: number): UplinkRXInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkFrameSet.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkFrameSet): UplinkFrameSet.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkFrameSet, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkFrameSet;
  static deserializeBinaryFromReader(message: UplinkFrameSet, reader: jspb.BinaryReader): UplinkFrameSet;
}

export namespace UplinkFrameSet {
  export type AsObject = {
    phyPayload: Uint8Array | string,
    txInfo?: UplinkTXInfo.AsObject,
    rxInfoList: Array<UplinkRXInfo.AsObject>,
  }
}

export class DownlinkFrame extends jspb.Message {
  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): DownlinkTXInfo | undefined;
  setTxInfo(value?: DownlinkTXInfo): void;

  getToken(): number;
  setToken(value: number): void;

  getDownlinkId(): Uint8Array | string;
  getDownlinkId_asU8(): Uint8Array;
  getDownlinkId_asB64(): string;
  setDownlinkId(value: Uint8Array | string): void;

  clearItemsList(): void;
  getItemsList(): Array<DownlinkFrameItem>;
  setItemsList(value: Array<DownlinkFrameItem>): void;
  addItems(value?: DownlinkFrameItem, index?: number): DownlinkFrameItem;

  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkFrame.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkFrame): DownlinkFrame.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkFrame, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkFrame;
  static deserializeBinaryFromReader(message: DownlinkFrame, reader: jspb.BinaryReader): DownlinkFrame;
}

export namespace DownlinkFrame {
  export type AsObject = {
    phyPayload: Uint8Array | string,
    txInfo?: DownlinkTXInfo.AsObject,
    token: number,
    downlinkId: Uint8Array | string,
    itemsList: Array<DownlinkFrameItem.AsObject>,
    gatewayId: Uint8Array | string,
  }
}

export class DownlinkFrameItem extends jspb.Message {
  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): DownlinkTXInfo | undefined;
  setTxInfo(value?: DownlinkTXInfo): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkFrameItem.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkFrameItem): DownlinkFrameItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkFrameItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkFrameItem;
  static deserializeBinaryFromReader(message: DownlinkFrameItem, reader: jspb.BinaryReader): DownlinkFrameItem;
}

export namespace DownlinkFrameItem {
  export type AsObject = {
    phyPayload: Uint8Array | string,
    txInfo?: DownlinkTXInfo.AsObject,
  }
}

export class DownlinkTXAck extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getToken(): number;
  setToken(value: number): void;

  getError(): string;
  setError(value: string): void;

  getDownlinkId(): Uint8Array | string;
  getDownlinkId_asU8(): Uint8Array;
  getDownlinkId_asB64(): string;
  setDownlinkId(value: Uint8Array | string): void;

  clearItemsList(): void;
  getItemsList(): Array<DownlinkTXAckItem>;
  setItemsList(value: Array<DownlinkTXAckItem>): void;
  addItems(value?: DownlinkTXAckItem, index?: number): DownlinkTXAckItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkTXAck.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkTXAck): DownlinkTXAck.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkTXAck, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkTXAck;
  static deserializeBinaryFromReader(message: DownlinkTXAck, reader: jspb.BinaryReader): DownlinkTXAck;
}

export namespace DownlinkTXAck {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    token: number,
    error: string,
    downlinkId: Uint8Array | string,
    itemsList: Array<DownlinkTXAckItem.AsObject>,
  }
}

export class DownlinkTXAckItem extends jspb.Message {
  getStatus(): TxAckStatusMap[keyof TxAckStatusMap];
  setStatus(value: TxAckStatusMap[keyof TxAckStatusMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkTXAckItem.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkTXAckItem): DownlinkTXAckItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkTXAckItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkTXAckItem;
  static deserializeBinaryFromReader(message: DownlinkTXAckItem, reader: jspb.BinaryReader): DownlinkTXAckItem;
}

export namespace DownlinkTXAckItem {
  export type AsObject = {
    status: TxAckStatusMap[keyof TxAckStatusMap],
  }
}

export class GatewayConfiguration extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getVersion(): string;
  setVersion(value: string): void;

  clearChannelsList(): void;
  getChannelsList(): Array<ChannelConfiguration>;
  setChannelsList(value: Array<ChannelConfiguration>): void;
  addChannels(value?: ChannelConfiguration, index?: number): ChannelConfiguration;

  hasStatsInterval(): boolean;
  clearStatsInterval(): void;
  getStatsInterval(): google_protobuf_duration_pb.Duration | undefined;
  setStatsInterval(value?: google_protobuf_duration_pb.Duration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GatewayConfiguration.AsObject;
  static toObject(includeInstance: boolean, msg: GatewayConfiguration): GatewayConfiguration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GatewayConfiguration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GatewayConfiguration;
  static deserializeBinaryFromReader(message: GatewayConfiguration, reader: jspb.BinaryReader): GatewayConfiguration;
}

export namespace GatewayConfiguration {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    version: string,
    channelsList: Array<ChannelConfiguration.AsObject>,
    statsInterval?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class ChannelConfiguration extends jspb.Message {
  getFrequency(): number;
  setFrequency(value: number): void;

  getModulation(): common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap];
  setModulation(value: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap]): void;

  hasLoraModulationConfig(): boolean;
  clearLoraModulationConfig(): void;
  getLoraModulationConfig(): LoRaModulationConfig | undefined;
  setLoraModulationConfig(value?: LoRaModulationConfig): void;

  hasFskModulationConfig(): boolean;
  clearFskModulationConfig(): void;
  getFskModulationConfig(): FSKModulationConfig | undefined;
  setFskModulationConfig(value?: FSKModulationConfig): void;

  getBoard(): number;
  setBoard(value: number): void;

  getDemodulator(): number;
  setDemodulator(value: number): void;

  getModulationConfigCase(): ChannelConfiguration.ModulationConfigCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ChannelConfiguration.AsObject;
  static toObject(includeInstance: boolean, msg: ChannelConfiguration): ChannelConfiguration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ChannelConfiguration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ChannelConfiguration;
  static deserializeBinaryFromReader(message: ChannelConfiguration, reader: jspb.BinaryReader): ChannelConfiguration;
}

export namespace ChannelConfiguration {
  export type AsObject = {
    frequency: number,
    modulation: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap],
    loraModulationConfig?: LoRaModulationConfig.AsObject,
    fskModulationConfig?: FSKModulationConfig.AsObject,
    board: number,
    demodulator: number,
  }

  export enum ModulationConfigCase {
    MODULATION_CONFIG_NOT_SET = 0,
    LORA_MODULATION_CONFIG = 3,
    FSK_MODULATION_CONFIG = 4,
  }
}

export class LoRaModulationConfig extends jspb.Message {
  getBandwidth(): number;
  setBandwidth(value: number): void;

  clearSpreadingFactorsList(): void;
  getSpreadingFactorsList(): Array<number>;
  setSpreadingFactorsList(value: Array<number>): void;
  addSpreadingFactors(value: number, index?: number): number;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoRaModulationConfig.AsObject;
  static toObject(includeInstance: boolean, msg: LoRaModulationConfig): LoRaModulationConfig.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LoRaModulationConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoRaModulationConfig;
  static deserializeBinaryFromReader(message: LoRaModulationConfig, reader: jspb.BinaryReader): LoRaModulationConfig;
}

export namespace LoRaModulationConfig {
  export type AsObject = {
    bandwidth: number,
    spreadingFactorsList: Array<number>,
  }
}

export class FSKModulationConfig extends jspb.Message {
  getBandwidth(): number;
  setBandwidth(value: number): void;

  getBitrate(): number;
  setBitrate(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FSKModulationConfig.AsObject;
  static toObject(includeInstance: boolean, msg: FSKModulationConfig): FSKModulationConfig.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FSKModulationConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FSKModulationConfig;
  static deserializeBinaryFromReader(message: FSKModulationConfig, reader: jspb.BinaryReader): FSKModulationConfig;
}

export namespace FSKModulationConfig {
  export type AsObject = {
    bandwidth: number,
    bitrate: number,
  }
}

export class GatewayCommandExecRequest extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getCommand(): string;
  setCommand(value: string): void;

  getExecid(): Uint8Array | string;
  getExecid_asU8(): Uint8Array;
  getExecid_asB64(): string;
  setExecid(value: Uint8Array | string): void;

  getStdin(): Uint8Array | string;
  getStdin_asU8(): Uint8Array;
  getStdin_asB64(): string;
  setStdin(value: Uint8Array | string): void;

  getEnvironmentMap(): jspb.Map<string, string>;
  clearEnvironmentMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GatewayCommandExecRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GatewayCommandExecRequest): GatewayCommandExecRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GatewayCommandExecRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GatewayCommandExecRequest;
  static deserializeBinaryFromReader(message: GatewayCommandExecRequest, reader: jspb.BinaryReader): GatewayCommandExecRequest;
}

export namespace GatewayCommandExecRequest {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    command: string,
    execid: Uint8Array | string,
    stdin: Uint8Array | string,
    environmentMap: Array<[string, string]>,
  }
}

export class GatewayCommandExecResponse extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getExecId(): Uint8Array | string;
  getExecId_asU8(): Uint8Array;
  getExecId_asB64(): string;
  setExecId(value: Uint8Array | string): void;

  getStdout(): Uint8Array | string;
  getStdout_asU8(): Uint8Array;
  getStdout_asB64(): string;
  setStdout(value: Uint8Array | string): void;

  getStderr(): Uint8Array | string;
  getStderr_asU8(): Uint8Array;
  getStderr_asB64(): string;
  setStderr(value: Uint8Array | string): void;

  getError(): string;
  setError(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GatewayCommandExecResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GatewayCommandExecResponse): GatewayCommandExecResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GatewayCommandExecResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GatewayCommandExecResponse;
  static deserializeBinaryFromReader(message: GatewayCommandExecResponse, reader: jspb.BinaryReader): GatewayCommandExecResponse;
}

export namespace GatewayCommandExecResponse {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    execId: Uint8Array | string,
    stdout: Uint8Array | string,
    stderr: Uint8Array | string,
    error: string,
  }
}

export class RawPacketForwarderEvent extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getRawId(): Uint8Array | string;
  getRawId_asU8(): Uint8Array;
  getRawId_asB64(): string;
  setRawId(value: Uint8Array | string): void;

  getPayload(): Uint8Array | string;
  getPayload_asU8(): Uint8Array;
  getPayload_asB64(): string;
  setPayload(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RawPacketForwarderEvent.AsObject;
  static toObject(includeInstance: boolean, msg: RawPacketForwarderEvent): RawPacketForwarderEvent.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RawPacketForwarderEvent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RawPacketForwarderEvent;
  static deserializeBinaryFromReader(message: RawPacketForwarderEvent, reader: jspb.BinaryReader): RawPacketForwarderEvent;
}

export namespace RawPacketForwarderEvent {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    rawId: Uint8Array | string,
    payload: Uint8Array | string,
  }
}

export class RawPacketForwarderCommand extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getRawId(): Uint8Array | string;
  getRawId_asU8(): Uint8Array;
  getRawId_asB64(): string;
  setRawId(value: Uint8Array | string): void;

  getPayload(): Uint8Array | string;
  getPayload_asU8(): Uint8Array;
  getPayload_asB64(): string;
  setPayload(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RawPacketForwarderCommand.AsObject;
  static toObject(includeInstance: boolean, msg: RawPacketForwarderCommand): RawPacketForwarderCommand.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RawPacketForwarderCommand, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RawPacketForwarderCommand;
  static deserializeBinaryFromReader(message: RawPacketForwarderCommand, reader: jspb.BinaryReader): RawPacketForwarderCommand;
}

export namespace RawPacketForwarderCommand {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    rawId: Uint8Array | string,
    payload: Uint8Array | string,
  }
}

export class ConnState extends jspb.Message {
  getGatewayId(): Uint8Array | string;
  getGatewayId_asU8(): Uint8Array;
  getGatewayId_asB64(): string;
  setGatewayId(value: Uint8Array | string): void;

  getState(): ConnState.StateMap[keyof ConnState.StateMap];
  setState(value: ConnState.StateMap[keyof ConnState.StateMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ConnState.AsObject;
  static toObject(includeInstance: boolean, msg: ConnState): ConnState.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ConnState, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ConnState;
  static deserializeBinaryFromReader(message: ConnState, reader: jspb.BinaryReader): ConnState;
}

export namespace ConnState {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    state: ConnState.StateMap[keyof ConnState.StateMap],
  }

  export interface StateMap {
    OFFLINE: 0;
    ONLINE: 1;
  }

  export const State: StateMap;
}

export interface DownlinkTimingMap {
  IMMEDIATELY: 0;
  DELAY: 1;
  GPS_EPOCH: 2;
}

export const DownlinkTiming: DownlinkTimingMap;

export interface FineTimestampTypeMap {
  NONE: 0;
  ENCRYPTED: 1;
  PLAIN: 2;
}

export const FineTimestampType: FineTimestampTypeMap;

export interface CRCStatusMap {
  NO_CRC: 0;
  BAD_CRC: 1;
  CRC_OK: 2;
}

export const CRCStatus: CRCStatusMap;

export interface TxAckStatusMap {
  IGNORED: 0;
  OK: 1;
  TOO_LATE: 2;
  TOO_EARLY: 3;
  COLLISION_PACKET: 4;
  COLLISION_BEACON: 5;
  TX_FREQ: 6;
  TX_POWER: 7;
  GPS_UNLOCKED: 8;
  QUEUE_FULL: 9;
  INTERNAL_ERROR: 10;
}

export const TxAckStatus: TxAckStatusMap;

