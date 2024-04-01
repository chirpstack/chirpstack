import * as jspb from 'google-protobuf'

import * as google_protobuf_descriptor_pb from 'google-protobuf/google/protobuf/descriptor_pb';


export class FieldPolicy extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): FieldPolicy;

  getResourcePermission(): string;
  setResourcePermission(value: string): FieldPolicy;

  getResourceType(): string;
  setResourceType(value: string): FieldPolicy;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FieldPolicy.AsObject;
  static toObject(includeInstance: boolean, msg: FieldPolicy): FieldPolicy.AsObject;
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
  setSelector(value: string): MethodPolicy;

  getRequestPoliciesList(): Array<FieldPolicy>;
  setRequestPoliciesList(value: Array<FieldPolicy>): MethodPolicy;
  clearRequestPoliciesList(): MethodPolicy;
  addRequestPolicies(value?: FieldPolicy, index?: number): FieldPolicy;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MethodPolicy.AsObject;
  static toObject(includeInstance: boolean, msg: MethodPolicy): MethodPolicy.AsObject;
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

