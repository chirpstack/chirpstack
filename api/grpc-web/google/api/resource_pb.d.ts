import * as jspb from 'google-protobuf'

import * as google_protobuf_descriptor_pb from 'google-protobuf/google/protobuf/descriptor_pb';


export class ResourceDescriptor extends jspb.Message {
  getType(): string;
  setType(value: string): ResourceDescriptor;

  getPatternList(): Array<string>;
  setPatternList(value: Array<string>): ResourceDescriptor;
  clearPatternList(): ResourceDescriptor;
  addPattern(value: string, index?: number): ResourceDescriptor;

  getNameField(): string;
  setNameField(value: string): ResourceDescriptor;

  getHistory(): ResourceDescriptor.History;
  setHistory(value: ResourceDescriptor.History): ResourceDescriptor;

  getPlural(): string;
  setPlural(value: string): ResourceDescriptor;

  getSingular(): string;
  setSingular(value: string): ResourceDescriptor;

  getStyleList(): Array<ResourceDescriptor.Style>;
  setStyleList(value: Array<ResourceDescriptor.Style>): ResourceDescriptor;
  clearStyleList(): ResourceDescriptor;
  addStyle(value: ResourceDescriptor.Style, index?: number): ResourceDescriptor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ResourceDescriptor.AsObject;
  static toObject(includeInstance: boolean, msg: ResourceDescriptor): ResourceDescriptor.AsObject;
  static serializeBinaryToWriter(message: ResourceDescriptor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ResourceDescriptor;
  static deserializeBinaryFromReader(message: ResourceDescriptor, reader: jspb.BinaryReader): ResourceDescriptor;
}

export namespace ResourceDescriptor {
  export type AsObject = {
    type: string,
    patternList: Array<string>,
    nameField: string,
    history: ResourceDescriptor.History,
    plural: string,
    singular: string,
    styleList: Array<ResourceDescriptor.Style>,
  }

  export enum History { 
    HISTORY_UNSPECIFIED = 0,
    ORIGINALLY_SINGLE_PATTERN = 1,
    FUTURE_MULTI_PATTERN = 2,
  }

  export enum Style { 
    STYLE_UNSPECIFIED = 0,
    DECLARATIVE_FRIENDLY = 1,
  }
}

export class ResourceReference extends jspb.Message {
  getType(): string;
  setType(value: string): ResourceReference;

  getChildType(): string;
  setChildType(value: string): ResourceReference;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ResourceReference.AsObject;
  static toObject(includeInstance: boolean, msg: ResourceReference): ResourceReference.AsObject;
  static serializeBinaryToWriter(message: ResourceReference, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ResourceReference;
  static deserializeBinaryFromReader(message: ResourceReference, reader: jspb.BinaryReader): ResourceReference;
}

export namespace ResourceReference {
  export type AsObject = {
    type: string,
    childType: string,
  }
}

