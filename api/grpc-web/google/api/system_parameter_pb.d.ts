import * as jspb from 'google-protobuf'



export class SystemParameters extends jspb.Message {
  getRulesList(): Array<SystemParameterRule>;
  setRulesList(value: Array<SystemParameterRule>): SystemParameters;
  clearRulesList(): SystemParameters;
  addRules(value?: SystemParameterRule, index?: number): SystemParameterRule;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SystemParameters.AsObject;
  static toObject(includeInstance: boolean, msg: SystemParameters): SystemParameters.AsObject;
  static serializeBinaryToWriter(message: SystemParameters, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SystemParameters;
  static deserializeBinaryFromReader(message: SystemParameters, reader: jspb.BinaryReader): SystemParameters;
}

export namespace SystemParameters {
  export type AsObject = {
    rulesList: Array<SystemParameterRule.AsObject>,
  }
}

export class SystemParameterRule extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): SystemParameterRule;

  getParametersList(): Array<SystemParameter>;
  setParametersList(value: Array<SystemParameter>): SystemParameterRule;
  clearParametersList(): SystemParameterRule;
  addParameters(value?: SystemParameter, index?: number): SystemParameter;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SystemParameterRule.AsObject;
  static toObject(includeInstance: boolean, msg: SystemParameterRule): SystemParameterRule.AsObject;
  static serializeBinaryToWriter(message: SystemParameterRule, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SystemParameterRule;
  static deserializeBinaryFromReader(message: SystemParameterRule, reader: jspb.BinaryReader): SystemParameterRule;
}

export namespace SystemParameterRule {
  export type AsObject = {
    selector: string,
    parametersList: Array<SystemParameter.AsObject>,
  }
}

export class SystemParameter extends jspb.Message {
  getName(): string;
  setName(value: string): SystemParameter;

  getHttpHeader(): string;
  setHttpHeader(value: string): SystemParameter;

  getUrlQueryParameter(): string;
  setUrlQueryParameter(value: string): SystemParameter;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SystemParameter.AsObject;
  static toObject(includeInstance: boolean, msg: SystemParameter): SystemParameter.AsObject;
  static serializeBinaryToWriter(message: SystemParameter, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SystemParameter;
  static deserializeBinaryFromReader(message: SystemParameter, reader: jspb.BinaryReader): SystemParameter;
}

export namespace SystemParameter {
  export type AsObject = {
    name: string,
    httpHeader: string,
    urlQueryParameter: string,
  }
}

