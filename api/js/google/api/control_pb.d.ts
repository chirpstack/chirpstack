// package: google.api
// file: google/api/control.proto

import * as jspb from "google-protobuf";
import * as google_api_policy_pb from "../../google/api/policy_pb";

export class Control extends jspb.Message {
  getEnvironment(): string;
  setEnvironment(value: string): void;

  clearMethodPoliciesList(): void;
  getMethodPoliciesList(): Array<google_api_policy_pb.MethodPolicy>;
  setMethodPoliciesList(value: Array<google_api_policy_pb.MethodPolicy>): void;
  addMethodPolicies(value?: google_api_policy_pb.MethodPolicy, index?: number): google_api_policy_pb.MethodPolicy;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Control.AsObject;
  static toObject(includeInstance: boolean, msg: Control): Control.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Control, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Control;
  static deserializeBinaryFromReader(message: Control, reader: jspb.BinaryReader): Control;
}

export namespace Control {
  export type AsObject = {
    environment: string,
    methodPoliciesList: Array<google_api_policy_pb.MethodPolicy.AsObject>,
  }
}

