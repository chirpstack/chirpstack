import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';


export class Location extends jspb.Message {
  getLatitude(): number;
  setLatitude(value: number): Location;

  getLongitude(): number;
  setLongitude(value: number): Location;

  getAltitude(): number;
  setAltitude(value: number): Location;

  getSource(): LocationSource;
  setSource(value: LocationSource): Location;

  getAccuracy(): number;
  setAccuracy(value: number): Location;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Location.AsObject;
  static toObject(includeInstance: boolean, msg: Location): Location.AsObject;
  static serializeBinaryToWriter(message: Location, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Location;
  static deserializeBinaryFromReader(message: Location, reader: jspb.BinaryReader): Location;
}

export namespace Location {
  export type AsObject = {
    latitude: number,
    longitude: number,
    altitude: number,
    source: LocationSource,
    accuracy: number,
  }
}

export class KeyEnvelope extends jspb.Message {
  getKekLabel(): string;
  setKekLabel(value: string): KeyEnvelope;

  getAesKey(): Uint8Array | string;
  getAesKey_asU8(): Uint8Array;
  getAesKey_asB64(): string;
  setAesKey(value: Uint8Array | string): KeyEnvelope;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): KeyEnvelope.AsObject;
  static toObject(includeInstance: boolean, msg: KeyEnvelope): KeyEnvelope.AsObject;
  static serializeBinaryToWriter(message: KeyEnvelope, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): KeyEnvelope;
  static deserializeBinaryFromReader(message: KeyEnvelope, reader: jspb.BinaryReader): KeyEnvelope;
}

export namespace KeyEnvelope {
  export type AsObject = {
    kekLabel: string,
    aesKey: Uint8Array | string,
  }
}

export class Metric extends jspb.Message {
  getName(): string;
  setName(value: string): Metric;

  getTimestampsList(): Array<google_protobuf_timestamp_pb.Timestamp>;
  setTimestampsList(value: Array<google_protobuf_timestamp_pb.Timestamp>): Metric;
  clearTimestampsList(): Metric;
  addTimestamps(value?: google_protobuf_timestamp_pb.Timestamp, index?: number): google_protobuf_timestamp_pb.Timestamp;

  getDatasetsList(): Array<MetricDataset>;
  setDatasetsList(value: Array<MetricDataset>): Metric;
  clearDatasetsList(): Metric;
  addDatasets(value?: MetricDataset, index?: number): MetricDataset;

  getKind(): MetricKind;
  setKind(value: MetricKind): Metric;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Metric.AsObject;
  static toObject(includeInstance: boolean, msg: Metric): Metric.AsObject;
  static serializeBinaryToWriter(message: Metric, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Metric;
  static deserializeBinaryFromReader(message: Metric, reader: jspb.BinaryReader): Metric;
}

export namespace Metric {
  export type AsObject = {
    name: string,
    timestampsList: Array<google_protobuf_timestamp_pb.Timestamp.AsObject>,
    datasetsList: Array<MetricDataset.AsObject>,
    kind: MetricKind,
  }
}

export class MetricDataset extends jspb.Message {
  getLabel(): string;
  setLabel(value: string): MetricDataset;

  getDataList(): Array<number>;
  setDataList(value: Array<number>): MetricDataset;
  clearDataList(): MetricDataset;
  addData(value: number, index?: number): MetricDataset;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MetricDataset.AsObject;
  static toObject(includeInstance: boolean, msg: MetricDataset): MetricDataset.AsObject;
  static serializeBinaryToWriter(message: MetricDataset, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MetricDataset;
  static deserializeBinaryFromReader(message: MetricDataset, reader: jspb.BinaryReader): MetricDataset;
}

export namespace MetricDataset {
  export type AsObject = {
    label: string,
    dataList: Array<number>,
  }
}

export enum Modulation { 
  LORA = 0,
  FSK = 1,
  LR_FHSS = 2,
}
export enum Region { 
  EU868 = 0,
  US915 = 2,
  CN779 = 3,
  EU433 = 4,
  AU915 = 5,
  CN470 = 6,
  AS923 = 7,
  AS923_2 = 12,
  AS923_3 = 13,
  AS923_4 = 14,
  KR920 = 8,
  IN865 = 9,
  RU864 = 10,
  ISM2400 = 11,
}
export enum MType { 
  JOIN_REQUEST = 0,
  JOIN_ACCEPT = 1,
  UNCONFIRMED_DATA_UP = 2,
  UNCONFIRMED_DATA_DOWN = 3,
  CONFIRMED_DATA_UP = 4,
  CONFIRMED_DATA_DOWN = 5,
  REJOIN_REQUEST = 6,
  PROPRIETARY = 7,
}
export enum MacVersion { 
  LORAWAN_1_0_0 = 0,
  LORAWAN_1_0_1 = 1,
  LORAWAN_1_0_2 = 2,
  LORAWAN_1_0_3 = 3,
  LORAWAN_1_0_4 = 4,
  LORAWAN_1_1_0 = 5,
}
export enum RegParamsRevision { 
  A = 0,
  B = 1,
  RP002_1_0_0 = 2,
  RP002_1_0_1 = 3,
  RP002_1_0_2 = 4,
  RP002_1_0_3 = 5,
}
export enum LocationSource { 
  UNKNOWN = 0,
  GPS = 1,
  CONFIG = 2,
  GEO_RESOLVER_TDOA = 3,
  GEO_RESOLVER_RSSI = 4,
  GEO_RESOLVER_GNSS = 5,
  GEO_RESOLVER_WIFI = 6,
}
export enum Aggregation { 
  HOUR = 0,
  DAY = 1,
  MONTH = 2,
}
export enum MetricKind { 
  COUNTER = 0,
  ABSOLUTE = 1,
  GAUGE = 2,
}
export enum DeviceClass { 
  CLASS_A = 0,
  CLASS_B = 1,
  CLASS_C = 2,
}
