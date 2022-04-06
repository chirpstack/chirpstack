import * as jspb from 'google-protobuf'



export class ConfigChange extends jspb.Message {
  getElement(): string;
  setElement(value: string): ConfigChange;

  getOldValue(): string;
  setOldValue(value: string): ConfigChange;

  getNewValue(): string;
  setNewValue(value: string): ConfigChange;

  getChangeType(): ChangeType;
  setChangeType(value: ChangeType): ConfigChange;

  getAdvicesList(): Array<Advice>;
  setAdvicesList(value: Array<Advice>): ConfigChange;
  clearAdvicesList(): ConfigChange;
  addAdvices(value?: Advice, index?: number): Advice;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ConfigChange.AsObject;
  static toObject(includeInstance: boolean, msg: ConfigChange): ConfigChange.AsObject;
  static serializeBinaryToWriter(message: ConfigChange, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ConfigChange;
  static deserializeBinaryFromReader(message: ConfigChange, reader: jspb.BinaryReader): ConfigChange;
}

export namespace ConfigChange {
  export type AsObject = {
    element: string,
    oldValue: string,
    newValue: string,
    changeType: ChangeType,
    advicesList: Array<Advice.AsObject>,
  }
}

export class Advice extends jspb.Message {
  getDescription(): string;
  setDescription(value: string): Advice;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Advice.AsObject;
  static toObject(includeInstance: boolean, msg: Advice): Advice.AsObject;
  static serializeBinaryToWriter(message: Advice, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Advice;
  static deserializeBinaryFromReader(message: Advice, reader: jspb.BinaryReader): Advice;
}

export namespace Advice {
  export type AsObject = {
    description: string,
  }
}

export enum ChangeType { 
  CHANGE_TYPE_UNSPECIFIED = 0,
  ADDED = 1,
  REMOVED = 2,
  MODIFIED = 3,
}
