// package: google.api
// file: google/api/policy.proto

import * as jspb from "google-protobuf";
import * as google_protobuf_descriptor_pb from "google-protobuf/google/protobuf/descriptor_pb";

export class FieldPolicy extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): void;

  getResourcePermission(): string;
  setResourcePermission(value: string): void;

  getResourceType(): string;
  setResourceType(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FieldPolicy.AsObject;
  static toObject(includeInstance: boolean, msg: FieldPolicy): FieldPolicy.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FieldPolicy, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FieldPolicy;
  static deserializeBinaryFromReader(message: FieldPolicy, reader: jspb.BinaryReader): FieldPolicy;
}

export namespace FieldPolicy {
  export type AsObject = {
    selector: string,
    resourcePermission: string,
    resourceType: string,
  }
}

export class MethodPolicy extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): void;

  clearRequestPoliciesList(): void;
  getRequestPoliciesList(): Array<FieldPolicy>;
  setRequestPoliciesList(value: Array<FieldPolicy>): void;
  addRequestPolicies(value?: FieldPolicy, index?: number): FieldPolicy;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MethodPolicy.AsObject;
  static toObject(includeInstance: boolean, msg: MethodPolicy): MethodPolicy.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: MethodPolicy, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MethodPolicy;
  static deserializeBinaryFromReader(message: MethodPolicy, reader: jspb.BinaryReader): MethodPolicy;
}

export namespace MethodPolicy {
  export type AsObject = {
    selector: string,
    requestPoliciesList: Array<FieldPolicy.AsObject>,
  }
}

  export const fieldPolicy: jspb.ExtensionFieldInfo<FieldPolicy>;

  export const methodPolicy: jspb.ExtensionFieldInfo<MethodPolicy>;

