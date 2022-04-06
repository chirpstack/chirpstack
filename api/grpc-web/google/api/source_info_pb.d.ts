import * as jspb from 'google-protobuf'

import * as google_protobuf_any_pb from 'google-protobuf/google/protobuf/any_pb';


export class SourceInfo extends jspb.Message {
  getSourceFilesList(): Array<google_protobuf_any_pb.Any>;
  setSourceFilesList(value: Array<google_protobuf_any_pb.Any>): SourceInfo;
  clearSourceFilesList(): SourceInfo;
  addSourceFiles(value?: google_protobuf_any_pb.Any, index?: number): google_protobuf_any_pb.Any;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SourceInfo.AsObject;
  static toObject(includeInstance: boolean, msg: SourceInfo): SourceInfo.AsObject;
  static serializeBinaryToWriter(message: SourceInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SourceInfo;
  static deserializeBinaryFromReader(message: SourceInfo, reader: jspb.BinaryReader): SourceInfo;
}

export namespace SourceInfo {
  export type AsObject = {
    sourceFilesList: Array<google_protobuf_any_pb.Any.AsObject>,
  }
}

