import * as jspb from 'google-protobuf'



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
