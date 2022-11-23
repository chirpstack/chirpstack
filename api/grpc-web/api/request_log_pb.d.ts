import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as common_common_pb from '../common/common_pb';
import * as gw_gw_pb from '../gw/gw_pb';


export class RequestLog extends jspb.Message {
  getService(): string;
  setService(value: string): RequestLog;

  getMethod(): string;
  setMethod(value: string): RequestLog;

  getMetadataMap(): jspb.Map<string, string>;
  clearMetadataMap(): RequestLog;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RequestLog.AsObject;
  static toObject(includeInstance: boolean, msg: RequestLog): RequestLog.AsObject;
  static serializeBinaryToWriter(message: RequestLog, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RequestLog;
  static deserializeBinaryFromReader(message: RequestLog, reader: jspb.BinaryReader): RequestLog;
}

export namespace RequestLog {
  export type AsObject = {
    service: string,
    method: string,
    metadataMap: Array<[string, string]>,
  }
}

