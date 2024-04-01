// package: google.api
// file: google/api/field_info.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_descriptor_pb from "google-protobuf/google/protobuf/descriptor_pb";

export class FieldInfo extends jspb.Message {
  getFormat(): FieldInfo.FormatMap[keyof FieldInfo.FormatMap];
  setFormat(value: FieldInfo.FormatMap[keyof FieldInfo.FormatMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FieldInfo.AsObject;
  static toObject(includeInstance: boolean, msg: FieldInfo): FieldInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FieldInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FieldInfo;
  static deserializeBinaryFromReader(message: FieldInfo, reader: jspb.BinaryReader): FieldInfo;
}

export namespace FieldInfo {
  export type AsObject = {
    format: FieldInfo.FormatMap[keyof FieldInfo.FormatMap],
  }

  export interface FormatMap {
    FORMAT_UNSPECIFIED: 0;
    UUID4: 1;
    IPV4: 2;
    IPV6: 3;
    IPV4_OR_IPV6: 4;
  }

  export const Format: FormatMap;
}

  export const fieldInfo: jspb.ExtensionFieldInfo<FieldInfo>;

