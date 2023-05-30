import * as jspb from 'google-protobuf'

import * as google_api_launch_stage_pb from '../../google/api/launch_stage_pb';
import * as google_protobuf_descriptor_pb from 'google-protobuf/google/protobuf/descriptor_pb';
import * as google_protobuf_duration_pb from 'google-protobuf/google/protobuf/duration_pb';


export class CommonLanguageSettings extends jspb.Message {
  getReferenceDocsUri(): string;
  setReferenceDocsUri(value: string): CommonLanguageSettings;

  getDestinationsList(): Array<ClientLibraryDestination>;
  setDestinationsList(value: Array<ClientLibraryDestination>): CommonLanguageSettings;
  clearDestinationsList(): CommonLanguageSettings;
  addDestinations(value: ClientLibraryDestination, index?: number): CommonLanguageSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CommonLanguageSettings.AsObject;
  static toObject(includeInstance: boolean, msg: CommonLanguageSettings): CommonLanguageSettings.AsObject;
  static serializeBinaryToWriter(message: CommonLanguageSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CommonLanguageSettings;
  static deserializeBinaryFromReader(message: CommonLanguageSettings, reader: jspb.BinaryReader): CommonLanguageSettings;
}

export namespace CommonLanguageSettings {
  export type AsObject = {
    referenceDocsUri: string,
    destinationsList: Array<ClientLibraryDestination>,
  }
}

export class ClientLibrarySettings extends jspb.Message {
  getVersion(): string;
  setVersion(value: string): ClientLibrarySettings;

  getLaunchStage(): google_api_launch_stage_pb.LaunchStage;
  setLaunchStage(value: google_api_launch_stage_pb.LaunchStage): ClientLibrarySettings;

  getRestNumericEnums(): boolean;
  setRestNumericEnums(value: boolean): ClientLibrarySettings;

  getJavaSettings(): JavaSettings | undefined;
  setJavaSettings(value?: JavaSettings): ClientLibrarySettings;
  hasJavaSettings(): boolean;
  clearJavaSettings(): ClientLibrarySettings;

  getCppSettings(): CppSettings | undefined;
  setCppSettings(value?: CppSettings): ClientLibrarySettings;
  hasCppSettings(): boolean;
  clearCppSettings(): ClientLibrarySettings;

  getPhpSettings(): PhpSettings | undefined;
  setPhpSettings(value?: PhpSettings): ClientLibrarySettings;
  hasPhpSettings(): boolean;
  clearPhpSettings(): ClientLibrarySettings;

  getPythonSettings(): PythonSettings | undefined;
  setPythonSettings(value?: PythonSettings): ClientLibrarySettings;
  hasPythonSettings(): boolean;
  clearPythonSettings(): ClientLibrarySettings;

  getNodeSettings(): NodeSettings | undefined;
  setNodeSettings(value?: NodeSettings): ClientLibrarySettings;
  hasNodeSettings(): boolean;
  clearNodeSettings(): ClientLibrarySettings;

  getDotnetSettings(): DotnetSettings | undefined;
  setDotnetSettings(value?: DotnetSettings): ClientLibrarySettings;
  hasDotnetSettings(): boolean;
  clearDotnetSettings(): ClientLibrarySettings;

  getRubySettings(): RubySettings | undefined;
  setRubySettings(value?: RubySettings): ClientLibrarySettings;
  hasRubySettings(): boolean;
  clearRubySettings(): ClientLibrarySettings;

  getGoSettings(): GoSettings | undefined;
  setGoSettings(value?: GoSettings): ClientLibrarySettings;
  hasGoSettings(): boolean;
  clearGoSettings(): ClientLibrarySettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ClientLibrarySettings.AsObject;
  static toObject(includeInstance: boolean, msg: ClientLibrarySettings): ClientLibrarySettings.AsObject;
  static serializeBinaryToWriter(message: ClientLibrarySettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ClientLibrarySettings;
  static deserializeBinaryFromReader(message: ClientLibrarySettings, reader: jspb.BinaryReader): ClientLibrarySettings;
}

export namespace ClientLibrarySettings {
  export type AsObject = {
    version: string,
    launchStage: google_api_launch_stage_pb.LaunchStage,
    restNumericEnums: boolean,
    javaSettings?: JavaSettings.AsObject,
    cppSettings?: CppSettings.AsObject,
    phpSettings?: PhpSettings.AsObject,
    pythonSettings?: PythonSettings.AsObject,
    nodeSettings?: NodeSettings.AsObject,
    dotnetSettings?: DotnetSettings.AsObject,
    rubySettings?: RubySettings.AsObject,
    goSettings?: GoSettings.AsObject,
  }
}

export class Publishing extends jspb.Message {
  getMethodSettingsList(): Array<MethodSettings>;
  setMethodSettingsList(value: Array<MethodSettings>): Publishing;
  clearMethodSettingsList(): Publishing;
  addMethodSettings(value?: MethodSettings, index?: number): MethodSettings;

  getNewIssueUri(): string;
  setNewIssueUri(value: string): Publishing;

  getDocumentationUri(): string;
  setDocumentationUri(value: string): Publishing;

  getApiShortName(): string;
  setApiShortName(value: string): Publishing;

  getGithubLabel(): string;
  setGithubLabel(value: string): Publishing;

  getCodeownerGithubTeamsList(): Array<string>;
  setCodeownerGithubTeamsList(value: Array<string>): Publishing;
  clearCodeownerGithubTeamsList(): Publishing;
  addCodeownerGithubTeams(value: string, index?: number): Publishing;

  getDocTagPrefix(): string;
  setDocTagPrefix(value: string): Publishing;

  getOrganization(): ClientLibraryOrganization;
  setOrganization(value: ClientLibraryOrganization): Publishing;

  getLibrarySettingsList(): Array<ClientLibrarySettings>;
  setLibrarySettingsList(value: Array<ClientLibrarySettings>): Publishing;
  clearLibrarySettingsList(): Publishing;
  addLibrarySettings(value?: ClientLibrarySettings, index?: number): ClientLibrarySettings;

  getProtoReferenceDocumentationUri(): string;
  setProtoReferenceDocumentationUri(value: string): Publishing;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Publishing.AsObject;
  static toObject(includeInstance: boolean, msg: Publishing): Publishing.AsObject;
  static serializeBinaryToWriter(message: Publishing, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Publishing;
  static deserializeBinaryFromReader(message: Publishing, reader: jspb.BinaryReader): Publishing;
}

export namespace Publishing {
  export type AsObject = {
    methodSettingsList: Array<MethodSettings.AsObject>,
    newIssueUri: string,
    documentationUri: string,
    apiShortName: string,
    githubLabel: string,
    codeownerGithubTeamsList: Array<string>,
    docTagPrefix: string,
    organization: ClientLibraryOrganization,
    librarySettingsList: Array<ClientLibrarySettings.AsObject>,
    protoReferenceDocumentationUri: string,
  }
}

export class JavaSettings extends jspb.Message {
  getLibraryPackage(): string;
  setLibraryPackage(value: string): JavaSettings;

  getServiceClassNamesMap(): jspb.Map<string, string>;
  clearServiceClassNamesMap(): JavaSettings;

  getCommon(): CommonLanguageSettings | undefined;
  setCommon(value?: CommonLanguageSettings): JavaSettings;
  hasCommon(): boolean;
  clearCommon(): JavaSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JavaSettings.AsObject;
  static toObject(includeInstance: boolean, msg: JavaSettings): JavaSettings.AsObject;
  static serializeBinaryToWriter(message: JavaSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JavaSettings;
  static deserializeBinaryFromReader(message: JavaSettings, reader: jspb.BinaryReader): JavaSettings;
}

export namespace JavaSettings {
  export type AsObject = {
    libraryPackage: string,
    serviceClassNamesMap: Array<[string, string]>,
    common?: CommonLanguageSettings.AsObject,
  }
}

export class CppSettings extends jspb.Message {
  getCommon(): CommonLanguageSettings | undefined;
  setCommon(value?: CommonLanguageSettings): CppSettings;
  hasCommon(): boolean;
  clearCommon(): CppSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CppSettings.AsObject;
  static toObject(includeInstance: boolean, msg: CppSettings): CppSettings.AsObject;
  static serializeBinaryToWriter(message: CppSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CppSettings;
  static deserializeBinaryFromReader(message: CppSettings, reader: jspb.BinaryReader): CppSettings;
}

export namespace CppSettings {
  export type AsObject = {
    common?: CommonLanguageSettings.AsObject,
  }
}

export class PhpSettings extends jspb.Message {
  getCommon(): CommonLanguageSettings | undefined;
  setCommon(value?: CommonLanguageSettings): PhpSettings;
  hasCommon(): boolean;
  clearCommon(): PhpSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PhpSettings.AsObject;
  static toObject(includeInstance: boolean, msg: PhpSettings): PhpSettings.AsObject;
  static serializeBinaryToWriter(message: PhpSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PhpSettings;
  static deserializeBinaryFromReader(message: PhpSettings, reader: jspb.BinaryReader): PhpSettings;
}

export namespace PhpSettings {
  export type AsObject = {
    common?: CommonLanguageSettings.AsObject,
  }
}

export class PythonSettings extends jspb.Message {
  getCommon(): CommonLanguageSettings | undefined;
  setCommon(value?: CommonLanguageSettings): PythonSettings;
  hasCommon(): boolean;
  clearCommon(): PythonSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PythonSettings.AsObject;
  static toObject(includeInstance: boolean, msg: PythonSettings): PythonSettings.AsObject;
  static serializeBinaryToWriter(message: PythonSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PythonSettings;
  static deserializeBinaryFromReader(message: PythonSettings, reader: jspb.BinaryReader): PythonSettings;
}

export namespace PythonSettings {
  export type AsObject = {
    common?: CommonLanguageSettings.AsObject,
  }
}

export class NodeSettings extends jspb.Message {
  getCommon(): CommonLanguageSettings | undefined;
  setCommon(value?: CommonLanguageSettings): NodeSettings;
  hasCommon(): boolean;
  clearCommon(): NodeSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): NodeSettings.AsObject;
  static toObject(includeInstance: boolean, msg: NodeSettings): NodeSettings.AsObject;
  static serializeBinaryToWriter(message: NodeSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): NodeSettings;
  static deserializeBinaryFromReader(message: NodeSettings, reader: jspb.BinaryReader): NodeSettings;
}

export namespace NodeSettings {
  export type AsObject = {
    common?: CommonLanguageSettings.AsObject,
  }
}

export class DotnetSettings extends jspb.Message {
  getCommon(): CommonLanguageSettings | undefined;
  setCommon(value?: CommonLanguageSettings): DotnetSettings;
  hasCommon(): boolean;
  clearCommon(): DotnetSettings;

  getRenamedServicesMap(): jspb.Map<string, string>;
  clearRenamedServicesMap(): DotnetSettings;

  getRenamedResourcesMap(): jspb.Map<string, string>;
  clearRenamedResourcesMap(): DotnetSettings;

  getIgnoredResourcesList(): Array<string>;
  setIgnoredResourcesList(value: Array<string>): DotnetSettings;
  clearIgnoredResourcesList(): DotnetSettings;
  addIgnoredResources(value: string, index?: number): DotnetSettings;

  getForcedNamespaceAliasesList(): Array<string>;
  setForcedNamespaceAliasesList(value: Array<string>): DotnetSettings;
  clearForcedNamespaceAliasesList(): DotnetSettings;
  addForcedNamespaceAliases(value: string, index?: number): DotnetSettings;

  getHandwrittenSignaturesList(): Array<string>;
  setHandwrittenSignaturesList(value: Array<string>): DotnetSettings;
  clearHandwrittenSignaturesList(): DotnetSettings;
  addHandwrittenSignatures(value: string, index?: number): DotnetSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DotnetSettings.AsObject;
  static toObject(includeInstance: boolean, msg: DotnetSettings): DotnetSettings.AsObject;
  static serializeBinaryToWriter(message: DotnetSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DotnetSettings;
  static deserializeBinaryFromReader(message: DotnetSettings, reader: jspb.BinaryReader): DotnetSettings;
}

export namespace DotnetSettings {
  export type AsObject = {
    common?: CommonLanguageSettings.AsObject,
    renamedServicesMap: Array<[string, string]>,
    renamedResourcesMap: Array<[string, string]>,
    ignoredResourcesList: Array<string>,
    forcedNamespaceAliasesList: Array<string>,
    handwrittenSignaturesList: Array<string>,
  }
}

export class RubySettings extends jspb.Message {
  getCommon(): CommonLanguageSettings | undefined;
  setCommon(value?: CommonLanguageSettings): RubySettings;
  hasCommon(): boolean;
  clearCommon(): RubySettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RubySettings.AsObject;
  static toObject(includeInstance: boolean, msg: RubySettings): RubySettings.AsObject;
  static serializeBinaryToWriter(message: RubySettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RubySettings;
  static deserializeBinaryFromReader(message: RubySettings, reader: jspb.BinaryReader): RubySettings;
}

export namespace RubySettings {
  export type AsObject = {
    common?: CommonLanguageSettings.AsObject,
  }
}

export class GoSettings extends jspb.Message {
  getCommon(): CommonLanguageSettings | undefined;
  setCommon(value?: CommonLanguageSettings): GoSettings;
  hasCommon(): boolean;
  clearCommon(): GoSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GoSettings.AsObject;
  static toObject(includeInstance: boolean, msg: GoSettings): GoSettings.AsObject;
  static serializeBinaryToWriter(message: GoSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GoSettings;
  static deserializeBinaryFromReader(message: GoSettings, reader: jspb.BinaryReader): GoSettings;
}

export namespace GoSettings {
  export type AsObject = {
    common?: CommonLanguageSettings.AsObject,
  }
}

export class MethodSettings extends jspb.Message {
  getSelector(): string;
  setSelector(value: string): MethodSettings;

  getLongRunning(): MethodSettings.LongRunning | undefined;
  setLongRunning(value?: MethodSettings.LongRunning): MethodSettings;
  hasLongRunning(): boolean;
  clearLongRunning(): MethodSettings;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MethodSettings.AsObject;
  static toObject(includeInstance: boolean, msg: MethodSettings): MethodSettings.AsObject;
  static serializeBinaryToWriter(message: MethodSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MethodSettings;
  static deserializeBinaryFromReader(message: MethodSettings, reader: jspb.BinaryReader): MethodSettings;
}

export namespace MethodSettings {
  export type AsObject = {
    selector: string,
    longRunning?: MethodSettings.LongRunning.AsObject,
  }

  export class LongRunning extends jspb.Message {
    getInitialPollDelay(): google_protobuf_duration_pb.Duration | undefined;
    setInitialPollDelay(value?: google_protobuf_duration_pb.Duration): LongRunning;
    hasInitialPollDelay(): boolean;
    clearInitialPollDelay(): LongRunning;

    getPollDelayMultiplier(): number;
    setPollDelayMultiplier(value: number): LongRunning;

    getMaxPollDelay(): google_protobuf_duration_pb.Duration | undefined;
    setMaxPollDelay(value?: google_protobuf_duration_pb.Duration): LongRunning;
    hasMaxPollDelay(): boolean;
    clearMaxPollDelay(): LongRunning;

    getTotalPollTimeout(): google_protobuf_duration_pb.Duration | undefined;
    setTotalPollTimeout(value?: google_protobuf_duration_pb.Duration): LongRunning;
    hasTotalPollTimeout(): boolean;
    clearTotalPollTimeout(): LongRunning;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): LongRunning.AsObject;
    static toObject(includeInstance: boolean, msg: LongRunning): LongRunning.AsObject;
    static serializeBinaryToWriter(message: LongRunning, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): LongRunning;
    static deserializeBinaryFromReader(message: LongRunning, reader: jspb.BinaryReader): LongRunning;
  }

  export namespace LongRunning {
    export type AsObject = {
      initialPollDelay?: google_protobuf_duration_pb.Duration.AsObject,
      pollDelayMultiplier: number,
      maxPollDelay?: google_protobuf_duration_pb.Duration.AsObject,
      totalPollTimeout?: google_protobuf_duration_pb.Duration.AsObject,
    }
  }

}

export enum ClientLibraryOrganization { 
  CLIENT_LIBRARY_ORGANIZATION_UNSPECIFIED = 0,
  CLOUD = 1,
  ADS = 2,
  PHOTOS = 3,
  STREET_VIEW = 4,
  SHOPPING = 5,
  GEO = 6,
  GENERATIVE_AI = 7,
}
export enum ClientLibraryDestination { 
  CLIENT_LIBRARY_DESTINATION_UNSPECIFIED = 0,
  GITHUB = 10,
  PACKAGE_MANAGER = 20,
}
