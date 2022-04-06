import * as jspb from 'google-protobuf'

import * as google_protobuf_any_pb from 'google-protobuf/google/protobuf/any_pb';


export class HttpBody extends jspb.Message {
  getContentType(): string;
  setContentType(value: string): HttpBody;

  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): HttpBody;

  getExtensionsList(): Array<google_protobuf_any_pb.Any>;
  setExtensionsList(value: Array<google_protobuf_any_pb.Any>): HttpBody;
  clearExtensionsList(): HttpBody;
  addExtensions(value?: google_protobuf_any_pb.Any, index?: number): google_protobuf_any_pb.Any;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HttpBody.AsObject;
  static toObject(includeInstance: boolean, msg: HttpBody): HttpBody.AsObject;
  static serializeBinaryToWriter(message: HttpBody, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HttpBody;
  static deserializeBinaryFromReader(message: HttpBody, reader: jspb.BinaryReader): HttpBody;
}

export namespace HttpBody {
  export type AsObject = {
    contentType: string,
    data: Uint8Array | string,
    extensionsList: Array<google_protobuf_any_pb.Any.AsObject>,
  }
}

