import * as jspb from 'google-protobuf'



export class LabelDescriptor extends jspb.Message {
  getKey(): string;
  setKey(value: string): LabelDescriptor;

  getValueType(): LabelDescriptor.ValueType;
  setValueType(value: LabelDescriptor.ValueType): LabelDescriptor;

  getDescription(): string;
  setDescription(value: string): LabelDescriptor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LabelDescriptor.AsObject;
  static toObject(includeInstance: boolean, msg: LabelDescriptor): LabelDescriptor.AsObject;
  static serializeBinaryToWriter(message: LabelDescriptor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LabelDescriptor;
  static deserializeBinaryFromReader(message: LabelDescriptor, reader: jspb.BinaryReader): LabelDescriptor;
}

export namespace LabelDescriptor {
  export type AsObject = {
    key: string,
    valueType: LabelDescriptor.ValueType,
    description: string,
  }

  export enum ValueType { 
    STRING = 0,
    BOOL = 1,
    INT64 = 2,
  }
}

