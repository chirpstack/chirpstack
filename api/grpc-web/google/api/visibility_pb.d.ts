import * as jspb from 'google-protobuf'

import * as google_protobuf_descriptor_pb from 'google-protobuf/google/protobuf/descriptor_pb';


export class Visibility extends jspb.Message {
  getRulesList(): Array<VisibilityRule>;
  setRulesList(value: Array<VisibilityRule>): Visibility;
  clearRulesList(): Visibility;
  addRules(value?: VisibilityRule, index?: number): VisibilityRule;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Visibility.AsObject;
  static toObject(includeInstance: boolean, msg: Visibility): Visibility.AsObject;
  static serializeBinaryToWriter(message: Visibility, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Visibility;
  static deserializeBinaryFromReader(message: Visibility, reader: jspb.BinaryReader): Visibility;
}

export namespace Visibility {
  export type AsObject = {
    rulesList: Array<VisibilityRule.AsObject>,
  }
}

export class VisibilityRule extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): VisibilityRule;

  getRestriction(): string;
  setRestriction(value: string): VisibilityRule;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisibilityRule.AsObject;
  static toObject(includeInstance: boolean, msg: VisibilityRule): VisibilityRule.AsObject;
  static serializeBinaryToWriter(message: VisibilityRule, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisibilityRule;
  static deserializeBinaryFromReader(message: VisibilityRule, reader: jspb.BinaryReader): VisibilityRule;
}

export namespace VisibilityRule {
  export type AsObject = {
    selector: string,
    restriction: string,
  }
}

