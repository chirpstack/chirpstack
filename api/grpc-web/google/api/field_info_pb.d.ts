import * as jspb from 'google-protobuf'

import * as google_protobuf_descriptor_pb from 'google-protobuf/google/protobuf/descriptor_pb';


export class FieldInfo extends jspb.Message {
  getFormat(): FieldInfo.Format;
  setFormat(value: FieldInfo.Format): FieldInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FieldInfo.AsObject;
  static toObject(includeInstance: boolean, msg: FieldInfo): FieldInfo.AsObject;
  static serializeBinaryToWriter(message: FieldInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FieldInfo;
  static deserializeBinaryFromReader(message: FieldInfo, reader: jspb.BinaryReader): FieldInfo;
}

export namespace FieldInfo {
  export type AsObject = {
    format: FieldInfo.Format,
  }

  export enum Format { 
    FORMAT_UNSPECIFIED = 0,
    UUID4 = 1,
    IPV4 = 2,
    IPV6 = 3,
    IPV4_OR_IPV6 = 4,
  }
}

