// package: gw
// file: gw/gw.proto

import * as jspb from "google-protobuf";
import * as common_common_pb from "../common/common_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_duration_pb from "google-protobuf/google/protobuf/duration_pb";
import * as google_protobuf_struct_pb from "google-protobuf/google/protobuf/struct_pb";

export class Modulation extends jspb.Message {
  hasLora(): boolean;
  clearLora(): void;
  getLora(): LoraModulationInfo | undefined;
  setLora(value?: LoraModulationInfo): void;

  hasFsk(): boolean;
  clearFsk(): void;
  getFsk(): FskModulationInfo | undefined;
  setFsk(value?: FskModulationInfo): void;

  hasLrFhss(): boolean;
  clearLrFhss(): void;
  getLrFhss(): LrFhssModulationInfo | undefined;
  setLrFhss(value?: LrFhssModulationInfo): void;

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
    lora?: LoraModulationInfo.AsObject,
    fsk?: FskModulationInfo.AsObject,
    lrFhss?: LrFhssModulationInfo.AsObject,
  }

  export enum ParametersCase {
    PARAMETERS_NOT_SET = 0,
    LORA = 3,
    FSK = 4,
    LR_FHSS = 5,
  }
}

export class UplinkTxInfoLegacy extends jspb.Message {
  getFrequency(): number;
  setFrequency(value: number): void;

  getModulation(): common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap];
  setModulation(value: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap]): void;

  hasLoraModulationInfo(): boolean;
  clearLoraModulationInfo(): void;
  getLoraModulationInfo(): LoraModulationInfo | undefined;
  setLoraModulationInfo(value?: LoraModulationInfo): void;

  hasFskModulationInfo(): boolean;
  clearFskModulationInfo(): void;
  getFskModulationInfo(): FskModulationInfo | undefined;
  setFskModulationInfo(value?: FskModulationInfo): void;

  hasLrFhssModulationInfo(): boolean;
  clearLrFhssModulationInfo(): void;
  getLrFhssModulationInfo(): LrFhssModulationInfo | undefined;
  setLrFhssModulationInfo(value?: LrFhssModulationInfo): void;

  getModulationInfoCase(): UplinkTxInfoLegacy.ModulationInfoCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkTxInfoLegacy.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkTxInfoLegacy): UplinkTxInfoLegacy.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkTxInfoLegacy, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkTxInfoLegacy;
  static deserializeBinaryFromReader(message: UplinkTxInfoLegacy, reader: jspb.BinaryReader): UplinkTxInfoLegacy;
}

export namespace UplinkTxInfoLegacy {
  export type AsObject = {
    frequency: number,
    modulation: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap],
    loraModulationInfo?: LoraModulationInfo.AsObject,
    fskModulationInfo?: FskModulationInfo.AsObject,
    lrFhssModulationInfo?: LrFhssModulationInfo.AsObject,
  }

  export enum ModulationInfoCase {
    MODULATION_INFO_NOT_SET = 0,
    LORA_MODULATION_INFO = 3,
    FSK_MODULATION_INFO = 4,
    LR_FHSS_MODULATION_INFO = 5,
  }
}

export class UplinkTxInfo extends jspb.Message {
  getFrequency(): number;
  setFrequency(value: number): void;

  hasModulation(): boolean;
  clearModulation(): void;
  getModulation(): Modulation | undefined;
  setModulation(value?: Modulation): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkTxInfo.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkTxInfo): UplinkTxInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkTxInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkTxInfo;
  static deserializeBinaryFromReader(message: UplinkTxInfo, reader: jspb.BinaryReader): UplinkTxInfo;
}

export namespace UplinkTxInfo {
  export type AsObject = {
    frequency: number,
    modulation?: Modulation.AsObject,
  }
}

export class LoraModulationInfo extends jspb.Message {
  getBandwidth(): number;
  setBandwidth(value: number): void;

  getSpreadingFactor(): number;
  setSpreadingFactor(value: number): void;

  getCodeRateLegacy(): string;
  setCodeRateLegacy(value: string): void;

  getCodeRate(): CodeRateMap[keyof CodeRateMap];
  setCodeRate(value: CodeRateMap[keyof CodeRateMap]): void;

  getPolarizationInversion(): boolean;
  setPolarizationInversion(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoraModulationInfo.AsObject;
  static toObject(includeInstance: boolean, msg: LoraModulationInfo): LoraModulationInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LoraModulationInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoraModulationInfo;
  static deserializeBinaryFromReader(message: LoraModulationInfo, reader: jspb.BinaryReader): LoraModulationInfo;
}

export namespace LoraModulationInfo {
  export type AsObject = {
    bandwidth: number,
    spreadingFactor: number,
    codeRateLegacy: string,
    codeRate: CodeRateMap[keyof CodeRateMap],
    polarizationInversion: boolean,
  }
}

export class FskModulationInfo extends jspb.Message {
  getFrequencyDeviation(): number;
  setFrequencyDeviation(value: number): void;

  getDatarate(): number;
  setDatarate(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FskModulationInfo.AsObject;
  static toObject(includeInstance: boolean, msg: FskModulationInfo): FskModulationInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FskModulationInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FskModulationInfo;
  static deserializeBinaryFromReader(message: FskModulationInfo, reader: jspb.BinaryReader): FskModulationInfo;
}

export namespace FskModulationInfo {
  export type AsObject = {
    frequencyDeviation: number,
    datarate: number,
  }
}

export class LrFhssModulationInfo extends jspb.Message {
  getOperatingChannelWidth(): number;
  setOperatingChannelWidth(value: number): void;

  getCodeRateLegacy(): string;
  setCodeRateLegacy(value: string): void;

  getCodeRate(): CodeRateMap[keyof CodeRateMap];
  setCodeRate(value: CodeRateMap[keyof CodeRateMap]): void;

  getGridSteps(): number;
  setGridSteps(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LrFhssModulationInfo.AsObject;
  static toObject(includeInstance: boolean, msg: LrFhssModulationInfo): LrFhssModulationInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LrFhssModulationInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LrFhssModulationInfo;
  static deserializeBinaryFromReader(message: LrFhssModulationInfo, reader: jspb.BinaryReader): LrFhssModulationInfo;
}

export namespace LrFhssModulationInfo {
  export type AsObject = {
    operatingChannelWidth: number,
    codeRateLegacy: string,
    codeRate: CodeRateMap[keyof CodeRateMap],
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
  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

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

  getMetadataMap(): jspb.Map<string, string>;
  clearMetadataMap(): void;
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
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    location?: common_common_pb.Location.AsObject,
    configVersion: string,
    rxPacketsReceived: number,
    rxPacketsReceivedOk: number,
    txPacketsReceived: number,
    txPacketsEmitted: number,
    metadataMap: Array<[string, string]>,
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

export class UplinkRxInfoLegacy extends jspb.Message {
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
  getFineTimestampCase(): UplinkRxInfoLegacy.FineTimestampCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkRxInfoLegacy.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkRxInfoLegacy): UplinkRxInfoLegacy.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkRxInfoLegacy, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkRxInfoLegacy;
  static deserializeBinaryFromReader(message: UplinkRxInfoLegacy, reader: jspb.BinaryReader): UplinkRxInfoLegacy;
}

export namespace UplinkRxInfoLegacy {
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

export class UplinkRxInfo extends jspb.Message {
  getGatewayId(): string;
  setGatewayId(value: string): void;

  getUplinkId(): number;
  setUplinkId(value: number): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasTimeSinceGpsEpoch(): boolean;
  clearTimeSinceGpsEpoch(): void;
  getTimeSinceGpsEpoch(): google_protobuf_duration_pb.Duration | undefined;
  setTimeSinceGpsEpoch(value?: google_protobuf_duration_pb.Duration): void;

  hasFineTimeSinceGpsEpoch(): boolean;
  clearFineTimeSinceGpsEpoch(): void;
  getFineTimeSinceGpsEpoch(): google_protobuf_duration_pb.Duration | undefined;
  setFineTimeSinceGpsEpoch(value?: google_protobuf_duration_pb.Duration): void;

  getRssi(): number;
  setRssi(value: number): void;

  getSnr(): number;
  setSnr(value: number): void;

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

  getContext(): Uint8Array | string;
  getContext_asU8(): Uint8Array;
  getContext_asB64(): string;
  setContext(value: Uint8Array | string): void;

  getMetadataMap(): jspb.Map<string, string>;
  clearMetadataMap(): void;
  getCrcStatus(): CRCStatusMap[keyof CRCStatusMap];
  setCrcStatus(value: CRCStatusMap[keyof CRCStatusMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UplinkRxInfo.AsObject;
  static toObject(includeInstance: boolean, msg: UplinkRxInfo): UplinkRxInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UplinkRxInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UplinkRxInfo;
  static deserializeBinaryFromReader(message: UplinkRxInfo, reader: jspb.BinaryReader): UplinkRxInfo;
}

export namespace UplinkRxInfo {
  export type AsObject = {
    gatewayId: string,
    uplinkId: number,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    timeSinceGpsEpoch?: google_protobuf_duration_pb.Duration.AsObject,
    fineTimeSinceGpsEpoch?: google_protobuf_duration_pb.Duration.AsObject,
    rssi: number,
    snr: number,
    channel: number,
    rfChain: number,
    board: number,
    antenna: number,
    location?: common_common_pb.Location.AsObject,
    context: Uint8Array | string,
    metadataMap: Array<[string, string]>,
    crcStatus: CRCStatusMap[keyof CRCStatusMap],
  }
}

export class DownlinkTxInfoLegacy extends jspb.Message {
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
  getLoraModulationInfo(): LoraModulationInfo | undefined;
  setLoraModulationInfo(value?: LoraModulationInfo): void;

  hasFskModulationInfo(): boolean;
  clearFskModulationInfo(): void;
  getFskModulationInfo(): FskModulationInfo | undefined;
  setFskModulationInfo(value?: FskModulationInfo): void;

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

  getModulationInfoCase(): DownlinkTxInfoLegacy.ModulationInfoCase;
  getTimingInfoCase(): DownlinkTxInfoLegacy.TimingInfoCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkTxInfoLegacy.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkTxInfoLegacy): DownlinkTxInfoLegacy.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkTxInfoLegacy, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkTxInfoLegacy;
  static deserializeBinaryFromReader(message: DownlinkTxInfoLegacy, reader: jspb.BinaryReader): DownlinkTxInfoLegacy;
}

export namespace DownlinkTxInfoLegacy {
  export type AsObject = {
    gatewayId: Uint8Array | string,
    frequency: number,
    power: number,
    modulation: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap],
    loraModulationInfo?: LoraModulationInfo.AsObject,
    fskModulationInfo?: FskModulationInfo.AsObject,
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

export class DownlinkTxInfo extends jspb.Message {
  getFrequency(): number;
  setFrequency(value: number): void;

  getPower(): number;
  setPower(value: number): void;

  hasModulation(): boolean;
  clearModulation(): void;
  getModulation(): Modulation | undefined;
  setModulation(value?: Modulation): void;

  getBoard(): number;
  setBoard(value: number): void;

  getAntenna(): number;
  setAntenna(value: number): void;

  hasTiming(): boolean;
  clearTiming(): void;
  getTiming(): Timing | undefined;
  setTiming(value?: Timing): void;

  getContext(): Uint8Array | string;
  getContext_asU8(): Uint8Array;
  getContext_asB64(): string;
  setContext(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkTxInfo.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkTxInfo): DownlinkTxInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkTxInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkTxInfo;
  static deserializeBinaryFromReader(message: DownlinkTxInfo, reader: jspb.BinaryReader): DownlinkTxInfo;
}

export namespace DownlinkTxInfo {
  export type AsObject = {
    frequency: number,
    power: number,
    modulation?: Modulation.AsObject,
    board: number,
    antenna: number,
    timing?: Timing.AsObject,
    context: Uint8Array | string,
  }
}

export class Timing extends jspb.Message {
  hasImmediately(): boolean;
  clearImmediately(): void;
  getImmediately(): ImmediatelyTimingInfo | undefined;
  setImmediately(value?: ImmediatelyTimingInfo): void;

  hasDelay(): boolean;
  clearDelay(): void;
  getDelay(): DelayTimingInfo | undefined;
  setDelay(value?: DelayTimingInfo): void;

  hasGpsEpoch(): boolean;
  clearGpsEpoch(): void;
  getGpsEpoch(): GPSEpochTimingInfo | undefined;
  setGpsEpoch(value?: GPSEpochTimingInfo): void;

  getParametersCase(): Timing.ParametersCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Timing.AsObject;
  static toObject(includeInstance: boolean, msg: Timing): Timing.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Timing, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Timing;
  static deserializeBinaryFromReader(message: Timing, reader: jspb.BinaryReader): Timing;
}

export namespace Timing {
  export type AsObject = {
    immediately?: ImmediatelyTimingInfo.AsObject,
    delay?: DelayTimingInfo.AsObject,
    gpsEpoch?: GPSEpochTimingInfo.AsObject,
  }

  export enum ParametersCase {
    PARAMETERS_NOT_SET = 0,
    IMMEDIATELY = 1,
    DELAY = 2,
    GPS_EPOCH = 3,
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

  hasTxInfoLegacy(): boolean;
  clearTxInfoLegacy(): void;
  getTxInfoLegacy(): UplinkTxInfoLegacy | undefined;
  setTxInfoLegacy(value?: UplinkTxInfoLegacy): void;

  hasRxInfoLegacy(): boolean;
  clearRxInfoLegacy(): void;
  getRxInfoLegacy(): UplinkRxInfoLegacy | undefined;
  setRxInfoLegacy(value?: UplinkRxInfoLegacy): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): UplinkTxInfo | undefined;
  setTxInfo(value?: UplinkTxInfo): void;

  hasRxInfo(): boolean;
  clearRxInfo(): void;
  getRxInfo(): UplinkRxInfo | undefined;
  setRxInfo(value?: UplinkRxInfo): void;

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
    txInfoLegacy?: UplinkTxInfoLegacy.AsObject,
    rxInfoLegacy?: UplinkRxInfoLegacy.AsObject,
    txInfo?: UplinkTxInfo.AsObject,
    rxInfo?: UplinkRxInfo.AsObject,
  }
}

export class UplinkFrameSet extends jspb.Message {
  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): UplinkTxInfo | undefined;
  setTxInfo(value?: UplinkTxInfo): void;

  clearRxInfoList(): void;
  getRxInfoList(): Array<UplinkRxInfo>;
  setRxInfoList(value: Array<UplinkRxInfo>): void;
  addRxInfo(value?: UplinkRxInfo, index?: number): UplinkRxInfo;

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
    txInfo?: UplinkTxInfo.AsObject,
    rxInfoList: Array<UplinkRxInfo.AsObject>,
  }
}

export class DownlinkFrame extends jspb.Message {
  getDownlinkId(): number;
  setDownlinkId(value: number): void;

  getDownlinkIdLegacy(): Uint8Array | string;
  getDownlinkIdLegacy_asU8(): Uint8Array;
  getDownlinkIdLegacy_asB64(): string;
  setDownlinkIdLegacy(value: Uint8Array | string): void;

  clearItemsList(): void;
  getItemsList(): Array<DownlinkFrameItem>;
  setItemsList(value: Array<DownlinkFrameItem>): void;
  addItems(value?: DownlinkFrameItem, index?: number): DownlinkFrameItem;

  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

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
    downlinkId: number,
    downlinkIdLegacy: Uint8Array | string,
    itemsList: Array<DownlinkFrameItem.AsObject>,
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
  }
}

export class DownlinkFrameItem extends jspb.Message {
  getPhyPayload(): Uint8Array | string;
  getPhyPayload_asU8(): Uint8Array;
  getPhyPayload_asB64(): string;
  setPhyPayload(value: Uint8Array | string): void;

  hasTxInfoLegacy(): boolean;
  clearTxInfoLegacy(): void;
  getTxInfoLegacy(): DownlinkTxInfoLegacy | undefined;
  setTxInfoLegacy(value?: DownlinkTxInfoLegacy): void;

  hasTxInfo(): boolean;
  clearTxInfo(): void;
  getTxInfo(): DownlinkTxInfo | undefined;
  setTxInfo(value?: DownlinkTxInfo): void;

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
    txInfoLegacy?: DownlinkTxInfoLegacy.AsObject,
    txInfo?: DownlinkTxInfo.AsObject,
  }
}

export class DownlinkTxAck extends jspb.Message {
  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

  getDownlinkId(): number;
  setDownlinkId(value: number): void;

  getDownlinkIdLegacy(): Uint8Array | string;
  getDownlinkIdLegacy_asU8(): Uint8Array;
  getDownlinkIdLegacy_asB64(): string;
  setDownlinkIdLegacy(value: Uint8Array | string): void;

  clearItemsList(): void;
  getItemsList(): Array<DownlinkTxAckItem>;
  setItemsList(value: Array<DownlinkTxAckItem>): void;
  addItems(value?: DownlinkTxAckItem, index?: number): DownlinkTxAckItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkTxAck.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkTxAck): DownlinkTxAck.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkTxAck, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkTxAck;
  static deserializeBinaryFromReader(message: DownlinkTxAck, reader: jspb.BinaryReader): DownlinkTxAck;
}

export namespace DownlinkTxAck {
  export type AsObject = {
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
    downlinkId: number,
    downlinkIdLegacy: Uint8Array | string,
    itemsList: Array<DownlinkTxAckItem.AsObject>,
  }
}

export class DownlinkTxAckItem extends jspb.Message {
  getStatus(): TxAckStatusMap[keyof TxAckStatusMap];
  setStatus(value: TxAckStatusMap[keyof TxAckStatusMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DownlinkTxAckItem.AsObject;
  static toObject(includeInstance: boolean, msg: DownlinkTxAckItem): DownlinkTxAckItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DownlinkTxAckItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DownlinkTxAckItem;
  static deserializeBinaryFromReader(message: DownlinkTxAckItem, reader: jspb.BinaryReader): DownlinkTxAckItem;
}

export namespace DownlinkTxAckItem {
  export type AsObject = {
    status: TxAckStatusMap[keyof TxAckStatusMap],
  }
}

export class GatewayConfiguration extends jspb.Message {
  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

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
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
    version: string,
    channelsList: Array<ChannelConfiguration.AsObject>,
    statsInterval?: google_protobuf_duration_pb.Duration.AsObject,
  }
}

export class ChannelConfiguration extends jspb.Message {
  getFrequency(): number;
  setFrequency(value: number): void;

  getModulationLegacy(): common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap];
  setModulationLegacy(value: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap]): void;

  hasLoraModulationConfig(): boolean;
  clearLoraModulationConfig(): void;
  getLoraModulationConfig(): LoraModulationConfig | undefined;
  setLoraModulationConfig(value?: LoraModulationConfig): void;

  hasFskModulationConfig(): boolean;
  clearFskModulationConfig(): void;
  getFskModulationConfig(): FskModulationConfig | undefined;
  setFskModulationConfig(value?: FskModulationConfig): void;

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
    modulationLegacy: common_common_pb.ModulationMap[keyof common_common_pb.ModulationMap],
    loraModulationConfig?: LoraModulationConfig.AsObject,
    fskModulationConfig?: FskModulationConfig.AsObject,
    board: number,
    demodulator: number,
  }

  export enum ModulationConfigCase {
    MODULATION_CONFIG_NOT_SET = 0,
    LORA_MODULATION_CONFIG = 3,
    FSK_MODULATION_CONFIG = 4,
  }
}

export class LoraModulationConfig extends jspb.Message {
  getBandwidthLegacy(): number;
  setBandwidthLegacy(value: number): void;

  getBandwidth(): number;
  setBandwidth(value: number): void;

  clearSpreadingFactorsList(): void;
  getSpreadingFactorsList(): Array<number>;
  setSpreadingFactorsList(value: Array<number>): void;
  addSpreadingFactors(value: number, index?: number): number;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoraModulationConfig.AsObject;
  static toObject(includeInstance: boolean, msg: LoraModulationConfig): LoraModulationConfig.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LoraModulationConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoraModulationConfig;
  static deserializeBinaryFromReader(message: LoraModulationConfig, reader: jspb.BinaryReader): LoraModulationConfig;
}

export namespace LoraModulationConfig {
  export type AsObject = {
    bandwidthLegacy: number,
    bandwidth: number,
    spreadingFactorsList: Array<number>,
  }
}

export class FskModulationConfig extends jspb.Message {
  getBandwidthLegacy(): number;
  setBandwidthLegacy(value: number): void;

  getBandwidth(): number;
  setBandwidth(value: number): void;

  getBitrate(): number;
  setBitrate(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FskModulationConfig.AsObject;
  static toObject(includeInstance: boolean, msg: FskModulationConfig): FskModulationConfig.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FskModulationConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FskModulationConfig;
  static deserializeBinaryFromReader(message: FskModulationConfig, reader: jspb.BinaryReader): FskModulationConfig;
}

export namespace FskModulationConfig {
  export type AsObject = {
    bandwidthLegacy: number,
    bandwidth: number,
    bitrate: number,
  }
}

export class GatewayCommandExecRequest extends jspb.Message {
  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

  getCommand(): string;
  setCommand(value: string): void;

  getExecId(): number;
  setExecId(value: number): void;

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
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
    command: string,
    execId: number,
    stdin: Uint8Array | string,
    environmentMap: Array<[string, string]>,
  }
}

export class GatewayCommandExecResponse extends jspb.Message {
  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

  getExecId(): number;
  setExecId(value: number): void;

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
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
    execId: number,
    stdout: Uint8Array | string,
    stderr: Uint8Array | string,
    error: string,
  }
}

export class RawPacketForwarderEvent extends jspb.Message {
  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

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
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
    payload: Uint8Array | string,
  }
}

export class RawPacketForwarderCommand extends jspb.Message {
  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

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
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
    payload: Uint8Array | string,
  }
}

export class ConnState extends jspb.Message {
  getGatewayIdLegacy(): Uint8Array | string;
  getGatewayIdLegacy_asU8(): Uint8Array;
  getGatewayIdLegacy_asB64(): string;
  setGatewayIdLegacy(value: Uint8Array | string): void;

  getGatewayId(): string;
  setGatewayId(value: string): void;

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
    gatewayIdLegacy: Uint8Array | string,
    gatewayId: string,
    state: ConnState.StateMap[keyof ConnState.StateMap],
  }

  export interface StateMap {
    OFFLINE: 0;
    ONLINE: 1;
  }

  export const State: StateMap;
}

export interface CodeRateMap {
  CR_UNDEFINED: 0;
  CR_4_5: 1;
  CR_4_6: 2;
  CR_4_7: 3;
  CR_4_8: 4;
  CR_3_8: 5;
  CR_2_6: 6;
  CR_1_4: 7;
  CR_1_6: 8;
  CR_5_6: 9;
  CR_LI_4_5: 10;
  CR_LI_4_6: 11;
  CR_LI_4_8: 12;
}

export const CodeRate: CodeRateMap;

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

