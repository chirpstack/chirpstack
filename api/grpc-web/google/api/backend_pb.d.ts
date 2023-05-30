import * as jspb from 'google-protobuf'



export class Backend extends jspb.Message {
  getRulesList(): Array<BackendRule>;
  setRulesList(value: Array<BackendRule>): Backend;
  clearRulesList(): Backend;
  addRules(value?: BackendRule, index?: number): BackendRule;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Backend.AsObject;
  static toObject(includeInstance: boolean, msg: Backend): Backend.AsObject;
  static serializeBinaryToWriter(message: Backend, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Backend;
  static deserializeBinaryFromReader(message: Backend, reader: jspb.BinaryReader): Backend;
}

export namespace Backend {
  export type AsObject = {
    rulesList: Array<BackendRule.AsObject>,
  }
}

export class BackendRule extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): BackendRule;

  getAddress(): string;
  setAddress(value: string): BackendRule;

  getDeadline(): number;
  setDeadline(value: number): BackendRule;

  getMinDeadline(): number;
  setMinDeadline(value: number): BackendRule;

  getOperationDeadline(): number;
  setOperationDeadline(value: number): BackendRule;

  getPathTranslation(): BackendRule.PathTranslation;
  setPathTranslation(value: BackendRule.PathTranslation): BackendRule;

  getJwtAudience(): string;
  setJwtAudience(value: string): BackendRule;

  getDisableAuth(): boolean;
  setDisableAuth(value: boolean): BackendRule;

  getProtocol(): string;
  setProtocol(value: string): BackendRule;

  getOverridesByRequestProtocolMap(): jspb.Map<string, BackendRule>;
  clearOverridesByRequestProtocolMap(): BackendRule;

  getAuthenticationCase(): BackendRule.AuthenticationCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BackendRule.AsObject;
  static toObject(includeInstance: boolean, msg: BackendRule): BackendRule.AsObject;
  static serializeBinaryToWriter(message: BackendRule, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BackendRule;
  static deserializeBinaryFromReader(message: BackendRule, reader: jspb.BinaryReader): BackendRule;
}

export namespace BackendRule {
  export type AsObject = {
    selector: string,
    address: string,
    deadline: number,
    minDeadline: number,
    operationDeadline: number,
    pathTranslation: BackendRule.PathTranslation,
    jwtAudience: string,
    disableAuth: boolean,
    protocol: string,
    overridesByRequestProtocolMap: Array<[string, BackendRule.AsObject]>,
  }

  export enum PathTranslation { 
    PATH_TRANSLATION_UNSPECIFIED = 0,
    CONSTANT_ADDRESS = 1,
    APPEND_PATH_TO_ADDRESS = 2,
  }

  export enum AuthenticationCase { 
    AUTHENTICATION_NOT_SET = 0,
    JWT_AUDIENCE = 7,
    DISABLE_AUTH = 8,
  }
}

