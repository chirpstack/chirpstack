// package: stream
// file: stream/backend_interfaces.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";

export class BackendInterfacesRequest extends jspb.Message {
  getSenderId(): string;
  setSenderId(value: string): void;

  getReceiverId(): string;
  setReceiverId(value: string): void;

  hasTime(): boolean;
  clearTime(): void;
  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getTransactionId(): number;
  setTransactionId(value: number): void;

  getMessageType(): string;
  setMessageType(value: string): void;

  getResultCode(): string;
  setResultCode(value: string): void;

  getRequestBody(): string;
  setRequestBody(value: string): void;

  getRequestError(): string;
  setRequestError(value: string): void;

  getResponseBody(): string;
  setResponseBody(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BackendInterfacesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: BackendInterfacesRequest): BackendInterfacesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: BackendInterfacesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BackendInterfacesRequest;
  static deserializeBinaryFromReader(message: BackendInterfacesRequest, reader: jspb.BinaryReader): BackendInterfacesRequest;
}

export namespace BackendInterfacesRequest {
  export type AsObject = {
    senderId: string,
    receiverId: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    transactionId: number,
    messageType: string,
    resultCode: string,
    requestBody: string,
    requestError: string,
    responseBody: string,
  }
}

