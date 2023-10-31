// package: stream
// file: stream/api_request.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as common_common_pb from "../common/common_pb";
import * as gw_gw_pb from "../gw/gw_pb";

export class ApiRequestLog extends jspb.Message {
  getService(): string;
  setService(value: string): void;

  getMethod(): string;
  setMethod(value: string): void;

  getMetadataMap(): jspb.Map<string, string>;
  clearMetadataMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApiRequestLog.AsObject;
  static toObject(includeInstance: boolean, msg: ApiRequestLog): ApiRequestLog.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ApiRequestLog, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ApiRequestLog;
  static deserializeBinaryFromReader(message: ApiRequestLog, reader: jspb.BinaryReader): ApiRequestLog;
}

export namespace ApiRequestLog {
  export type AsObject = {
    service: string,
    method: string,
    metadataMap: Array<[string, string]>,
  }
}

