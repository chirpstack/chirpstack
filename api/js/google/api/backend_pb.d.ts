// package: google.api
// file: google/api/backend.proto

import * as jspb from "google-protobuf";

export class Backend extends jspb.Message {
  clearRulesList(): void;
  getRulesList(): Array<BackendRule>;
  setRulesList(value: Array<BackendRule>): void;
  addRules(value?: BackendRule, index?: number): BackendRule;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Backend.AsObject;
  static toObject(includeInstance: boolean, msg: Backend): Backend.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
  setSelector(value: string): void;

  getAddress(): string;
  setAddress(value: string): void;

  getDeadline(): number;
  setDeadline(value: number): void;

  getMinDeadline(): number;
  setMinDeadline(value: number): void;

  getOperationDeadline(): number;
  setOperationDeadline(value: number): void;

  getPathTranslation(): BackendRule.PathTranslationMap[keyof BackendRule.PathTranslationMap];
  setPathTranslation(value: BackendRule.PathTranslationMap[keyof BackendRule.PathTranslationMap]): void;

  hasJwtAudience(): boolean;
  clearJwtAudience(): void;
  getJwtAudience(): string;
  setJwtAudience(value: string): void;

  hasDisableAuth(): boolean;
  clearDisableAuth(): void;
  getDisableAuth(): boolean;
  setDisableAuth(value: boolean): void;

  getProtocol(): string;
  setProtocol(value: string): void;

  getOverridesByRequestProtocolMap(): jspb.Map<string, BackendRule>;
  clearOverridesByRequestProtocolMap(): void;
  getAuthenticationCase(): BackendRule.AuthenticationCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BackendRule.AsObject;
  static toObject(includeInstance: boolean, msg: BackendRule): BackendRule.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
    pathTranslation: BackendRule.PathTranslationMap[keyof BackendRule.PathTranslationMap],
    jwtAudience: string,
    disableAuth: boolean,
    protocol: string,
    overridesByRequestProtocolMap: Array<[string, BackendRule.AsObject]>,
  }

  export interface PathTranslationMap {
    PATH_TRANSLATION_UNSPECIFIED: 0;
    CONSTANT_ADDRESS: 1;
    APPEND_PATH_TO_ADDRESS: 2;
  }

  export const PathTranslation: PathTranslationMap;

  export enum AuthenticationCase {
    AUTHENTICATION_NOT_SET = 0,
    JWT_AUDIENCE = 7,
    DISABLE_AUTH = 8,
  }
}

