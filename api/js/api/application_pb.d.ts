// package: api
// file: api/application.proto

import * as jspb from "google-protobuf";
import * as google_api_annotations_pb from "../google/api/annotations_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";

export class Application extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Application.AsObject;
  static toObject(includeInstance: boolean, msg: Application): Application.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Application, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Application;
  static deserializeBinaryFromReader(message: Application, reader: jspb.BinaryReader): Application;
}

export namespace Application {
  export type AsObject = {
    id: string,
    name: string,
    description: string,
    tenantId: string,
  }
}

export class ApplicationListItem extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasUpdatedAt(): boolean;
  clearUpdatedAt(): void;
  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  getName(): string;
  setName(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApplicationListItem.AsObject;
  static toObject(includeInstance: boolean, msg: ApplicationListItem): ApplicationListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ApplicationListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ApplicationListItem;
  static deserializeBinaryFromReader(message: ApplicationListItem, reader: jspb.BinaryReader): ApplicationListItem;
}

export namespace ApplicationListItem {
  export type AsObject = {
    id: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    name: string,
    description: string,
  }
}

export class CreateApplicationRequest extends jspb.Message {
  hasApplication(): boolean;
  clearApplication(): void;
  getApplication(): Application | undefined;
  setApplication(value?: Application): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateApplicationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateApplicationRequest): CreateApplicationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateApplicationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateApplicationRequest;
  static deserializeBinaryFromReader(message: CreateApplicationRequest, reader: jspb.BinaryReader): CreateApplicationRequest;
}

export namespace CreateApplicationRequest {
  export type AsObject = {
    application?: Application.AsObject,
  }
}

export class CreateApplicationResponse extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateApplicationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateApplicationResponse): CreateApplicationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateApplicationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateApplicationResponse;
  static deserializeBinaryFromReader(message: CreateApplicationResponse, reader: jspb.BinaryReader): CreateApplicationResponse;
}

export namespace CreateApplicationResponse {
  export type AsObject = {
    id: string,
  }
}

export class GetApplicationRequest extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetApplicationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetApplicationRequest): GetApplicationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetApplicationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetApplicationRequest;
  static deserializeBinaryFromReader(message: GetApplicationRequest, reader: jspb.BinaryReader): GetApplicationRequest;
}

export namespace GetApplicationRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetApplicationResponse extends jspb.Message {
  hasApplication(): boolean;
  clearApplication(): void;
  getApplication(): Application | undefined;
  setApplication(value?: Application): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasUpdatedAt(): boolean;
  clearUpdatedAt(): void;
  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  clearMeasurementKeysList(): void;
  getMeasurementKeysList(): Array<string>;
  setMeasurementKeysList(value: Array<string>): void;
  addMeasurementKeys(value: string, index?: number): string;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetApplicationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetApplicationResponse): GetApplicationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetApplicationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetApplicationResponse;
  static deserializeBinaryFromReader(message: GetApplicationResponse, reader: jspb.BinaryReader): GetApplicationResponse;
}

export namespace GetApplicationResponse {
  export type AsObject = {
    application?: Application.AsObject,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    measurementKeysList: Array<string>,
  }
}

export class UpdateApplicationRequest extends jspb.Message {
  hasApplication(): boolean;
  clearApplication(): void;
  getApplication(): Application | undefined;
  setApplication(value?: Application): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateApplicationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateApplicationRequest): UpdateApplicationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateApplicationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateApplicationRequest;
  static deserializeBinaryFromReader(message: UpdateApplicationRequest, reader: jspb.BinaryReader): UpdateApplicationRequest;
}

export namespace UpdateApplicationRequest {
  export type AsObject = {
    application?: Application.AsObject,
  }
}

export class DeleteApplicationRequest extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteApplicationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteApplicationRequest): DeleteApplicationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteApplicationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteApplicationRequest;
  static deserializeBinaryFromReader(message: DeleteApplicationRequest, reader: jspb.BinaryReader): DeleteApplicationRequest;
}

export namespace DeleteApplicationRequest {
  export type AsObject = {
    id: string,
  }
}

export class ListApplicationsRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  getSearch(): string;
  setSearch(value: string): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListApplicationsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListApplicationsRequest): ListApplicationsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListApplicationsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListApplicationsRequest;
  static deserializeBinaryFromReader(message: ListApplicationsRequest, reader: jspb.BinaryReader): ListApplicationsRequest;
}

export namespace ListApplicationsRequest {
  export type AsObject = {
    limit: number,
    offset: number,
    search: string,
    tenantId: string,
  }
}

export class ListApplicationsResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<ApplicationListItem>;
  setResultList(value: Array<ApplicationListItem>): void;
  addResult(value?: ApplicationListItem, index?: number): ApplicationListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListApplicationsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListApplicationsResponse): ListApplicationsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListApplicationsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListApplicationsResponse;
  static deserializeBinaryFromReader(message: ListApplicationsResponse, reader: jspb.BinaryReader): ListApplicationsResponse;
}

export namespace ListApplicationsResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<ApplicationListItem.AsObject>,
  }
}

export class ListIntegrationsRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListIntegrationsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListIntegrationsRequest): ListIntegrationsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListIntegrationsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListIntegrationsRequest;
  static deserializeBinaryFromReader(message: ListIntegrationsRequest, reader: jspb.BinaryReader): ListIntegrationsRequest;
}

export namespace ListIntegrationsRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class IntegrationListItem extends jspb.Message {
  getKind(): IntegrationKindMap[keyof IntegrationKindMap];
  setKind(value: IntegrationKindMap[keyof IntegrationKindMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IntegrationListItem.AsObject;
  static toObject(includeInstance: boolean, msg: IntegrationListItem): IntegrationListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IntegrationListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IntegrationListItem;
  static deserializeBinaryFromReader(message: IntegrationListItem, reader: jspb.BinaryReader): IntegrationListItem;
}

export namespace IntegrationListItem {
  export type AsObject = {
    kind: IntegrationKindMap[keyof IntegrationKindMap],
  }
}

export class ListIntegrationsResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<IntegrationListItem>;
  setResultList(value: Array<IntegrationListItem>): void;
  addResult(value?: IntegrationListItem, index?: number): IntegrationListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListIntegrationsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListIntegrationsResponse): ListIntegrationsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListIntegrationsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListIntegrationsResponse;
  static deserializeBinaryFromReader(message: ListIntegrationsResponse, reader: jspb.BinaryReader): ListIntegrationsResponse;
}

export namespace ListIntegrationsResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<IntegrationListItem.AsObject>,
  }
}

export class HttpIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getHeadersMap(): jspb.Map<string, string>;
  clearHeadersMap(): void;
  getEncoding(): EncodingMap[keyof EncodingMap];
  setEncoding(value: EncodingMap[keyof EncodingMap]): void;

  getEventEndpointUrl(): string;
  setEventEndpointUrl(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HttpIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: HttpIntegration): HttpIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: HttpIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HttpIntegration;
  static deserializeBinaryFromReader(message: HttpIntegration, reader: jspb.BinaryReader): HttpIntegration;
}

export namespace HttpIntegration {
  export type AsObject = {
    applicationId: string,
    headersMap: Array<[string, string]>,
    encoding: EncodingMap[keyof EncodingMap],
    eventEndpointUrl: string,
  }
}

export class CreateHttpIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): HttpIntegration | undefined;
  setIntegration(value?: HttpIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateHttpIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateHttpIntegrationRequest): CreateHttpIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateHttpIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateHttpIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateHttpIntegrationRequest, reader: jspb.BinaryReader): CreateHttpIntegrationRequest;
}

export namespace CreateHttpIntegrationRequest {
  export type AsObject = {
    integration?: HttpIntegration.AsObject,
  }
}

export class GetHttpIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetHttpIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetHttpIntegrationRequest): GetHttpIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetHttpIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetHttpIntegrationRequest;
  static deserializeBinaryFromReader(message: GetHttpIntegrationRequest, reader: jspb.BinaryReader): GetHttpIntegrationRequest;
}

export namespace GetHttpIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetHttpIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): HttpIntegration | undefined;
  setIntegration(value?: HttpIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetHttpIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetHttpIntegrationResponse): GetHttpIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetHttpIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetHttpIntegrationResponse;
  static deserializeBinaryFromReader(message: GetHttpIntegrationResponse, reader: jspb.BinaryReader): GetHttpIntegrationResponse;
}

export namespace GetHttpIntegrationResponse {
  export type AsObject = {
    integration?: HttpIntegration.AsObject,
  }
}

export class UpdateHttpIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): HttpIntegration | undefined;
  setIntegration(value?: HttpIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateHttpIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateHttpIntegrationRequest): UpdateHttpIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateHttpIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateHttpIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateHttpIntegrationRequest, reader: jspb.BinaryReader): UpdateHttpIntegrationRequest;
}

export namespace UpdateHttpIntegrationRequest {
  export type AsObject = {
    integration?: HttpIntegration.AsObject,
  }
}

export class DeleteHttpIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteHttpIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteHttpIntegrationRequest): DeleteHttpIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteHttpIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteHttpIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteHttpIntegrationRequest, reader: jspb.BinaryReader): DeleteHttpIntegrationRequest;
}

export namespace DeleteHttpIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class InfluxDbIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getEndpoint(): string;
  setEndpoint(value: string): void;

  getDb(): string;
  setDb(value: string): void;

  getUsername(): string;
  setUsername(value: string): void;

  getPassword(): string;
  setPassword(value: string): void;

  getRetentionPolicyName(): string;
  setRetentionPolicyName(value: string): void;

  getPrecision(): InfluxDbPrecisionMap[keyof InfluxDbPrecisionMap];
  setPrecision(value: InfluxDbPrecisionMap[keyof InfluxDbPrecisionMap]): void;

  getVersion(): InfluxDbVersionMap[keyof InfluxDbVersionMap];
  setVersion(value: InfluxDbVersionMap[keyof InfluxDbVersionMap]): void;

  getToken(): string;
  setToken(value: string): void;

  getOrganization(): string;
  setOrganization(value: string): void;

  getBucket(): string;
  setBucket(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InfluxDbIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: InfluxDbIntegration): InfluxDbIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: InfluxDbIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InfluxDbIntegration;
  static deserializeBinaryFromReader(message: InfluxDbIntegration, reader: jspb.BinaryReader): InfluxDbIntegration;
}

export namespace InfluxDbIntegration {
  export type AsObject = {
    applicationId: string,
    endpoint: string,
    db: string,
    username: string,
    password: string,
    retentionPolicyName: string,
    precision: InfluxDbPrecisionMap[keyof InfluxDbPrecisionMap],
    version: InfluxDbVersionMap[keyof InfluxDbVersionMap],
    token: string,
    organization: string,
    bucket: string,
  }
}

export class CreateInfluxDbIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): InfluxDbIntegration | undefined;
  setIntegration(value?: InfluxDbIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateInfluxDbIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateInfluxDbIntegrationRequest): CreateInfluxDbIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateInfluxDbIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateInfluxDbIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateInfluxDbIntegrationRequest, reader: jspb.BinaryReader): CreateInfluxDbIntegrationRequest;
}

export namespace CreateInfluxDbIntegrationRequest {
  export type AsObject = {
    integration?: InfluxDbIntegration.AsObject,
  }
}

export class GetInfluxDbIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetInfluxDbIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetInfluxDbIntegrationRequest): GetInfluxDbIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetInfluxDbIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetInfluxDbIntegrationRequest;
  static deserializeBinaryFromReader(message: GetInfluxDbIntegrationRequest, reader: jspb.BinaryReader): GetInfluxDbIntegrationRequest;
}

export namespace GetInfluxDbIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetInfluxDbIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): InfluxDbIntegration | undefined;
  setIntegration(value?: InfluxDbIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetInfluxDbIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetInfluxDbIntegrationResponse): GetInfluxDbIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetInfluxDbIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetInfluxDbIntegrationResponse;
  static deserializeBinaryFromReader(message: GetInfluxDbIntegrationResponse, reader: jspb.BinaryReader): GetInfluxDbIntegrationResponse;
}

export namespace GetInfluxDbIntegrationResponse {
  export type AsObject = {
    integration?: InfluxDbIntegration.AsObject,
  }
}

export class UpdateInfluxDbIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): InfluxDbIntegration | undefined;
  setIntegration(value?: InfluxDbIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateInfluxDbIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateInfluxDbIntegrationRequest): UpdateInfluxDbIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateInfluxDbIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateInfluxDbIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateInfluxDbIntegrationRequest, reader: jspb.BinaryReader): UpdateInfluxDbIntegrationRequest;
}

export namespace UpdateInfluxDbIntegrationRequest {
  export type AsObject = {
    integration?: InfluxDbIntegration.AsObject,
  }
}

export class DeleteInfluxDbIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteInfluxDbIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteInfluxDbIntegrationRequest): DeleteInfluxDbIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteInfluxDbIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteInfluxDbIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteInfluxDbIntegrationRequest, reader: jspb.BinaryReader): DeleteInfluxDbIntegrationRequest;
}

export namespace DeleteInfluxDbIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class ThingsBoardIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getServer(): string;
  setServer(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ThingsBoardIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: ThingsBoardIntegration): ThingsBoardIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ThingsBoardIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ThingsBoardIntegration;
  static deserializeBinaryFromReader(message: ThingsBoardIntegration, reader: jspb.BinaryReader): ThingsBoardIntegration;
}

export namespace ThingsBoardIntegration {
  export type AsObject = {
    applicationId: string,
    server: string,
  }
}

export class CreateThingsBoardIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): ThingsBoardIntegration | undefined;
  setIntegration(value?: ThingsBoardIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateThingsBoardIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateThingsBoardIntegrationRequest): CreateThingsBoardIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateThingsBoardIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateThingsBoardIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateThingsBoardIntegrationRequest, reader: jspb.BinaryReader): CreateThingsBoardIntegrationRequest;
}

export namespace CreateThingsBoardIntegrationRequest {
  export type AsObject = {
    integration?: ThingsBoardIntegration.AsObject,
  }
}

export class GetThingsBoardIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetThingsBoardIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetThingsBoardIntegrationRequest): GetThingsBoardIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetThingsBoardIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetThingsBoardIntegrationRequest;
  static deserializeBinaryFromReader(message: GetThingsBoardIntegrationRequest, reader: jspb.BinaryReader): GetThingsBoardIntegrationRequest;
}

export namespace GetThingsBoardIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetThingsBoardIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): ThingsBoardIntegration | undefined;
  setIntegration(value?: ThingsBoardIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetThingsBoardIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetThingsBoardIntegrationResponse): GetThingsBoardIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetThingsBoardIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetThingsBoardIntegrationResponse;
  static deserializeBinaryFromReader(message: GetThingsBoardIntegrationResponse, reader: jspb.BinaryReader): GetThingsBoardIntegrationResponse;
}

export namespace GetThingsBoardIntegrationResponse {
  export type AsObject = {
    integration?: ThingsBoardIntegration.AsObject,
  }
}

export class UpdateThingsBoardIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): ThingsBoardIntegration | undefined;
  setIntegration(value?: ThingsBoardIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateThingsBoardIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateThingsBoardIntegrationRequest): UpdateThingsBoardIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateThingsBoardIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateThingsBoardIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateThingsBoardIntegrationRequest, reader: jspb.BinaryReader): UpdateThingsBoardIntegrationRequest;
}

export namespace UpdateThingsBoardIntegrationRequest {
  export type AsObject = {
    integration?: ThingsBoardIntegration.AsObject,
  }
}

export class DeleteThingsBoardIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteThingsBoardIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteThingsBoardIntegrationRequest): DeleteThingsBoardIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteThingsBoardIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteThingsBoardIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteThingsBoardIntegrationRequest, reader: jspb.BinaryReader): DeleteThingsBoardIntegrationRequest;
}

export namespace DeleteThingsBoardIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class MyDevicesIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getEndpoint(): string;
  setEndpoint(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MyDevicesIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: MyDevicesIntegration): MyDevicesIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: MyDevicesIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MyDevicesIntegration;
  static deserializeBinaryFromReader(message: MyDevicesIntegration, reader: jspb.BinaryReader): MyDevicesIntegration;
}

export namespace MyDevicesIntegration {
  export type AsObject = {
    applicationId: string,
    endpoint: string,
  }
}

export class CreateMyDevicesIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): MyDevicesIntegration | undefined;
  setIntegration(value?: MyDevicesIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateMyDevicesIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateMyDevicesIntegrationRequest): CreateMyDevicesIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateMyDevicesIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateMyDevicesIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateMyDevicesIntegrationRequest, reader: jspb.BinaryReader): CreateMyDevicesIntegrationRequest;
}

export namespace CreateMyDevicesIntegrationRequest {
  export type AsObject = {
    integration?: MyDevicesIntegration.AsObject,
  }
}

export class GetMyDevicesIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMyDevicesIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetMyDevicesIntegrationRequest): GetMyDevicesIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetMyDevicesIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMyDevicesIntegrationRequest;
  static deserializeBinaryFromReader(message: GetMyDevicesIntegrationRequest, reader: jspb.BinaryReader): GetMyDevicesIntegrationRequest;
}

export namespace GetMyDevicesIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetMyDevicesIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): MyDevicesIntegration | undefined;
  setIntegration(value?: MyDevicesIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMyDevicesIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetMyDevicesIntegrationResponse): GetMyDevicesIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetMyDevicesIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetMyDevicesIntegrationResponse;
  static deserializeBinaryFromReader(message: GetMyDevicesIntegrationResponse, reader: jspb.BinaryReader): GetMyDevicesIntegrationResponse;
}

export namespace GetMyDevicesIntegrationResponse {
  export type AsObject = {
    integration?: MyDevicesIntegration.AsObject,
  }
}

export class UpdateMyDevicesIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): MyDevicesIntegration | undefined;
  setIntegration(value?: MyDevicesIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateMyDevicesIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateMyDevicesIntegrationRequest): UpdateMyDevicesIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateMyDevicesIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateMyDevicesIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateMyDevicesIntegrationRequest, reader: jspb.BinaryReader): UpdateMyDevicesIntegrationRequest;
}

export namespace UpdateMyDevicesIntegrationRequest {
  export type AsObject = {
    integration?: MyDevicesIntegration.AsObject,
  }
}

export class DeleteMyDevicesIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteMyDevicesIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteMyDevicesIntegrationRequest): DeleteMyDevicesIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteMyDevicesIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteMyDevicesIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteMyDevicesIntegrationRequest, reader: jspb.BinaryReader): DeleteMyDevicesIntegrationRequest;
}

export namespace DeleteMyDevicesIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class LoraCloudIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  hasModemGeolocationServices(): boolean;
  clearModemGeolocationServices(): void;
  getModemGeolocationServices(): LoraCloudModemGeolocationServices | undefined;
  setModemGeolocationServices(value?: LoraCloudModemGeolocationServices): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoraCloudIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: LoraCloudIntegration): LoraCloudIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LoraCloudIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoraCloudIntegration;
  static deserializeBinaryFromReader(message: LoraCloudIntegration, reader: jspb.BinaryReader): LoraCloudIntegration;
}

export namespace LoraCloudIntegration {
  export type AsObject = {
    applicationId: string,
    modemGeolocationServices?: LoraCloudModemGeolocationServices.AsObject,
  }
}

export class LoraCloudModemGeolocationServices extends jspb.Message {
  getToken(): string;
  setToken(value: string): void;

  getModemEnabled(): boolean;
  setModemEnabled(value: boolean): void;

  clearForwardFPortsList(): void;
  getForwardFPortsList(): Array<number>;
  setForwardFPortsList(value: Array<number>): void;
  addForwardFPorts(value: number, index?: number): number;

  getGnssUseRxTime(): boolean;
  setGnssUseRxTime(value: boolean): void;

  getGnssUseGatewayLocation(): boolean;
  setGnssUseGatewayLocation(value: boolean): void;

  getParseTlv(): boolean;
  setParseTlv(value: boolean): void;

  getGeolocationBufferTtl(): number;
  setGeolocationBufferTtl(value: number): void;

  getGeolocationMinBufferSize(): number;
  setGeolocationMinBufferSize(value: number): void;

  getGeolocationTdoa(): boolean;
  setGeolocationTdoa(value: boolean): void;

  getGeolocationRssi(): boolean;
  setGeolocationRssi(value: boolean): void;

  getGeolocationGnss(): boolean;
  setGeolocationGnss(value: boolean): void;

  getGeolocationGnssPayloadField(): string;
  setGeolocationGnssPayloadField(value: string): void;

  getGeolocationGnssUseRxTime(): boolean;
  setGeolocationGnssUseRxTime(value: boolean): void;

  getGeolocationWifi(): boolean;
  setGeolocationWifi(value: boolean): void;

  getGeolocationWifiPayloadField(): string;
  setGeolocationWifiPayloadField(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoraCloudModemGeolocationServices.AsObject;
  static toObject(includeInstance: boolean, msg: LoraCloudModemGeolocationServices): LoraCloudModemGeolocationServices.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: LoraCloudModemGeolocationServices, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoraCloudModemGeolocationServices;
  static deserializeBinaryFromReader(message: LoraCloudModemGeolocationServices, reader: jspb.BinaryReader): LoraCloudModemGeolocationServices;
}

export namespace LoraCloudModemGeolocationServices {
  export type AsObject = {
    token: string,
    modemEnabled: boolean,
    forwardFPortsList: Array<number>,
    gnssUseRxTime: boolean,
    gnssUseGatewayLocation: boolean,
    parseTlv: boolean,
    geolocationBufferTtl: number,
    geolocationMinBufferSize: number,
    geolocationTdoa: boolean,
    geolocationRssi: boolean,
    geolocationGnss: boolean,
    geolocationGnssPayloadField: string,
    geolocationGnssUseRxTime: boolean,
    geolocationWifi: boolean,
    geolocationWifiPayloadField: string,
  }
}

export class CreateLoraCloudIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): LoraCloudIntegration | undefined;
  setIntegration(value?: LoraCloudIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateLoraCloudIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateLoraCloudIntegrationRequest): CreateLoraCloudIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateLoraCloudIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateLoraCloudIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateLoraCloudIntegrationRequest, reader: jspb.BinaryReader): CreateLoraCloudIntegrationRequest;
}

export namespace CreateLoraCloudIntegrationRequest {
  export type AsObject = {
    integration?: LoraCloudIntegration.AsObject,
  }
}

export class GetLoraCloudIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetLoraCloudIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetLoraCloudIntegrationRequest): GetLoraCloudIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetLoraCloudIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetLoraCloudIntegrationRequest;
  static deserializeBinaryFromReader(message: GetLoraCloudIntegrationRequest, reader: jspb.BinaryReader): GetLoraCloudIntegrationRequest;
}

export namespace GetLoraCloudIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetLoraCloudIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): LoraCloudIntegration | undefined;
  setIntegration(value?: LoraCloudIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetLoraCloudIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetLoraCloudIntegrationResponse): GetLoraCloudIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetLoraCloudIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetLoraCloudIntegrationResponse;
  static deserializeBinaryFromReader(message: GetLoraCloudIntegrationResponse, reader: jspb.BinaryReader): GetLoraCloudIntegrationResponse;
}

export namespace GetLoraCloudIntegrationResponse {
  export type AsObject = {
    integration?: LoraCloudIntegration.AsObject,
  }
}

export class UpdateLoraCloudIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): LoraCloudIntegration | undefined;
  setIntegration(value?: LoraCloudIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateLoraCloudIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateLoraCloudIntegrationRequest): UpdateLoraCloudIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateLoraCloudIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateLoraCloudIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateLoraCloudIntegrationRequest, reader: jspb.BinaryReader): UpdateLoraCloudIntegrationRequest;
}

export namespace UpdateLoraCloudIntegrationRequest {
  export type AsObject = {
    integration?: LoraCloudIntegration.AsObject,
  }
}

export class DeleteLoraCloudIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteLoraCloudIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteLoraCloudIntegrationRequest): DeleteLoraCloudIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteLoraCloudIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteLoraCloudIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteLoraCloudIntegrationRequest, reader: jspb.BinaryReader): DeleteLoraCloudIntegrationRequest;
}

export namespace DeleteLoraCloudIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GcpPubSubIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getEncoding(): EncodingMap[keyof EncodingMap];
  setEncoding(value: EncodingMap[keyof EncodingMap]): void;

  getCredentialsFile(): string;
  setCredentialsFile(value: string): void;

  getProjectId(): string;
  setProjectId(value: string): void;

  getTopicName(): string;
  setTopicName(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GcpPubSubIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: GcpPubSubIntegration): GcpPubSubIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GcpPubSubIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GcpPubSubIntegration;
  static deserializeBinaryFromReader(message: GcpPubSubIntegration, reader: jspb.BinaryReader): GcpPubSubIntegration;
}

export namespace GcpPubSubIntegration {
  export type AsObject = {
    applicationId: string,
    encoding: EncodingMap[keyof EncodingMap],
    credentialsFile: string,
    projectId: string,
    topicName: string,
  }
}

export class CreateGcpPubSubIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): GcpPubSubIntegration | undefined;
  setIntegration(value?: GcpPubSubIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateGcpPubSubIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateGcpPubSubIntegrationRequest): CreateGcpPubSubIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateGcpPubSubIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateGcpPubSubIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateGcpPubSubIntegrationRequest, reader: jspb.BinaryReader): CreateGcpPubSubIntegrationRequest;
}

export namespace CreateGcpPubSubIntegrationRequest {
  export type AsObject = {
    integration?: GcpPubSubIntegration.AsObject,
  }
}

export class GetGcpPubSubIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGcpPubSubIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetGcpPubSubIntegrationRequest): GetGcpPubSubIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetGcpPubSubIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGcpPubSubIntegrationRequest;
  static deserializeBinaryFromReader(message: GetGcpPubSubIntegrationRequest, reader: jspb.BinaryReader): GetGcpPubSubIntegrationRequest;
}

export namespace GetGcpPubSubIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetGcpPubSubIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): GcpPubSubIntegration | undefined;
  setIntegration(value?: GcpPubSubIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGcpPubSubIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetGcpPubSubIntegrationResponse): GetGcpPubSubIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetGcpPubSubIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetGcpPubSubIntegrationResponse;
  static deserializeBinaryFromReader(message: GetGcpPubSubIntegrationResponse, reader: jspb.BinaryReader): GetGcpPubSubIntegrationResponse;
}

export namespace GetGcpPubSubIntegrationResponse {
  export type AsObject = {
    integration?: GcpPubSubIntegration.AsObject,
  }
}

export class UpdateGcpPubSubIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): GcpPubSubIntegration | undefined;
  setIntegration(value?: GcpPubSubIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateGcpPubSubIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateGcpPubSubIntegrationRequest): UpdateGcpPubSubIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateGcpPubSubIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateGcpPubSubIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateGcpPubSubIntegrationRequest, reader: jspb.BinaryReader): UpdateGcpPubSubIntegrationRequest;
}

export namespace UpdateGcpPubSubIntegrationRequest {
  export type AsObject = {
    integration?: GcpPubSubIntegration.AsObject,
  }
}

export class DeleteGcpPubSubIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteGcpPubSubIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteGcpPubSubIntegrationRequest): DeleteGcpPubSubIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteGcpPubSubIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteGcpPubSubIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteGcpPubSubIntegrationRequest, reader: jspb.BinaryReader): DeleteGcpPubSubIntegrationRequest;
}

export namespace DeleteGcpPubSubIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class AwsSnsIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getEncoding(): EncodingMap[keyof EncodingMap];
  setEncoding(value: EncodingMap[keyof EncodingMap]): void;

  getRegion(): string;
  setRegion(value: string): void;

  getAccessKeyId(): string;
  setAccessKeyId(value: string): void;

  getSecretAccessKey(): string;
  setSecretAccessKey(value: string): void;

  getTopicArn(): string;
  setTopicArn(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AwsSnsIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: AwsSnsIntegration): AwsSnsIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AwsSnsIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AwsSnsIntegration;
  static deserializeBinaryFromReader(message: AwsSnsIntegration, reader: jspb.BinaryReader): AwsSnsIntegration;
}

export namespace AwsSnsIntegration {
  export type AsObject = {
    applicationId: string,
    encoding: EncodingMap[keyof EncodingMap],
    region: string,
    accessKeyId: string,
    secretAccessKey: string,
    topicArn: string,
  }
}

export class CreateAwsSnsIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): AwsSnsIntegration | undefined;
  setIntegration(value?: AwsSnsIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateAwsSnsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateAwsSnsIntegrationRequest): CreateAwsSnsIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateAwsSnsIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateAwsSnsIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateAwsSnsIntegrationRequest, reader: jspb.BinaryReader): CreateAwsSnsIntegrationRequest;
}

export namespace CreateAwsSnsIntegrationRequest {
  export type AsObject = {
    integration?: AwsSnsIntegration.AsObject,
  }
}

export class GetAwsSnsIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAwsSnsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAwsSnsIntegrationRequest): GetAwsSnsIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetAwsSnsIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAwsSnsIntegrationRequest;
  static deserializeBinaryFromReader(message: GetAwsSnsIntegrationRequest, reader: jspb.BinaryReader): GetAwsSnsIntegrationRequest;
}

export namespace GetAwsSnsIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetAwsSnsIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): AwsSnsIntegration | undefined;
  setIntegration(value?: AwsSnsIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAwsSnsIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAwsSnsIntegrationResponse): GetAwsSnsIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetAwsSnsIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAwsSnsIntegrationResponse;
  static deserializeBinaryFromReader(message: GetAwsSnsIntegrationResponse, reader: jspb.BinaryReader): GetAwsSnsIntegrationResponse;
}

export namespace GetAwsSnsIntegrationResponse {
  export type AsObject = {
    integration?: AwsSnsIntegration.AsObject,
  }
}

export class UpdateAwsSnsIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): AwsSnsIntegration | undefined;
  setIntegration(value?: AwsSnsIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateAwsSnsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateAwsSnsIntegrationRequest): UpdateAwsSnsIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateAwsSnsIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateAwsSnsIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateAwsSnsIntegrationRequest, reader: jspb.BinaryReader): UpdateAwsSnsIntegrationRequest;
}

export namespace UpdateAwsSnsIntegrationRequest {
  export type AsObject = {
    integration?: AwsSnsIntegration.AsObject,
  }
}

export class DeleteAwsSnsIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteAwsSnsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteAwsSnsIntegrationRequest): DeleteAwsSnsIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteAwsSnsIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteAwsSnsIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteAwsSnsIntegrationRequest, reader: jspb.BinaryReader): DeleteAwsSnsIntegrationRequest;
}

export namespace DeleteAwsSnsIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class AzureServiceBusIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getEncoding(): EncodingMap[keyof EncodingMap];
  setEncoding(value: EncodingMap[keyof EncodingMap]): void;

  getConnectionString(): string;
  setConnectionString(value: string): void;

  getPublishName(): string;
  setPublishName(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AzureServiceBusIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: AzureServiceBusIntegration): AzureServiceBusIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AzureServiceBusIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AzureServiceBusIntegration;
  static deserializeBinaryFromReader(message: AzureServiceBusIntegration, reader: jspb.BinaryReader): AzureServiceBusIntegration;
}

export namespace AzureServiceBusIntegration {
  export type AsObject = {
    applicationId: string,
    encoding: EncodingMap[keyof EncodingMap],
    connectionString: string,
    publishName: string,
  }
}

export class CreateAzureServiceBusIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): AzureServiceBusIntegration | undefined;
  setIntegration(value?: AzureServiceBusIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateAzureServiceBusIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateAzureServiceBusIntegrationRequest): CreateAzureServiceBusIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateAzureServiceBusIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateAzureServiceBusIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateAzureServiceBusIntegrationRequest, reader: jspb.BinaryReader): CreateAzureServiceBusIntegrationRequest;
}

export namespace CreateAzureServiceBusIntegrationRequest {
  export type AsObject = {
    integration?: AzureServiceBusIntegration.AsObject,
  }
}

export class GetAzureServiceBusIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAzureServiceBusIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAzureServiceBusIntegrationRequest): GetAzureServiceBusIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetAzureServiceBusIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAzureServiceBusIntegrationRequest;
  static deserializeBinaryFromReader(message: GetAzureServiceBusIntegrationRequest, reader: jspb.BinaryReader): GetAzureServiceBusIntegrationRequest;
}

export namespace GetAzureServiceBusIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetAzureServiceBusIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): AzureServiceBusIntegration | undefined;
  setIntegration(value?: AzureServiceBusIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAzureServiceBusIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAzureServiceBusIntegrationResponse): GetAzureServiceBusIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetAzureServiceBusIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAzureServiceBusIntegrationResponse;
  static deserializeBinaryFromReader(message: GetAzureServiceBusIntegrationResponse, reader: jspb.BinaryReader): GetAzureServiceBusIntegrationResponse;
}

export namespace GetAzureServiceBusIntegrationResponse {
  export type AsObject = {
    integration?: AzureServiceBusIntegration.AsObject,
  }
}

export class UpdateAzureServiceBusIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): AzureServiceBusIntegration | undefined;
  setIntegration(value?: AzureServiceBusIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateAzureServiceBusIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateAzureServiceBusIntegrationRequest): UpdateAzureServiceBusIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateAzureServiceBusIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateAzureServiceBusIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateAzureServiceBusIntegrationRequest, reader: jspb.BinaryReader): UpdateAzureServiceBusIntegrationRequest;
}

export namespace UpdateAzureServiceBusIntegrationRequest {
  export type AsObject = {
    integration?: AzureServiceBusIntegration.AsObject,
  }
}

export class DeleteAzureServiceBusIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteAzureServiceBusIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteAzureServiceBusIntegrationRequest): DeleteAzureServiceBusIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteAzureServiceBusIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteAzureServiceBusIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteAzureServiceBusIntegrationRequest, reader: jspb.BinaryReader): DeleteAzureServiceBusIntegrationRequest;
}

export namespace DeleteAzureServiceBusIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class PilotThingsIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getServer(): string;
  setServer(value: string): void;

  getToken(): string;
  setToken(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PilotThingsIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: PilotThingsIntegration): PilotThingsIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PilotThingsIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PilotThingsIntegration;
  static deserializeBinaryFromReader(message: PilotThingsIntegration, reader: jspb.BinaryReader): PilotThingsIntegration;
}

export namespace PilotThingsIntegration {
  export type AsObject = {
    applicationId: string,
    server: string,
    token: string,
  }
}

export class CreatePilotThingsIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): PilotThingsIntegration | undefined;
  setIntegration(value?: PilotThingsIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreatePilotThingsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreatePilotThingsIntegrationRequest): CreatePilotThingsIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreatePilotThingsIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreatePilotThingsIntegrationRequest;
  static deserializeBinaryFromReader(message: CreatePilotThingsIntegrationRequest, reader: jspb.BinaryReader): CreatePilotThingsIntegrationRequest;
}

export namespace CreatePilotThingsIntegrationRequest {
  export type AsObject = {
    integration?: PilotThingsIntegration.AsObject,
  }
}

export class GetPilotThingsIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetPilotThingsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetPilotThingsIntegrationRequest): GetPilotThingsIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetPilotThingsIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetPilotThingsIntegrationRequest;
  static deserializeBinaryFromReader(message: GetPilotThingsIntegrationRequest, reader: jspb.BinaryReader): GetPilotThingsIntegrationRequest;
}

export namespace GetPilotThingsIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetPilotThingsIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): PilotThingsIntegration | undefined;
  setIntegration(value?: PilotThingsIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetPilotThingsIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetPilotThingsIntegrationResponse): GetPilotThingsIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetPilotThingsIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetPilotThingsIntegrationResponse;
  static deserializeBinaryFromReader(message: GetPilotThingsIntegrationResponse, reader: jspb.BinaryReader): GetPilotThingsIntegrationResponse;
}

export namespace GetPilotThingsIntegrationResponse {
  export type AsObject = {
    integration?: PilotThingsIntegration.AsObject,
  }
}

export class UpdatePilotThingsIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): PilotThingsIntegration | undefined;
  setIntegration(value?: PilotThingsIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdatePilotThingsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdatePilotThingsIntegrationRequest): UpdatePilotThingsIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdatePilotThingsIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdatePilotThingsIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdatePilotThingsIntegrationRequest, reader: jspb.BinaryReader): UpdatePilotThingsIntegrationRequest;
}

export namespace UpdatePilotThingsIntegrationRequest {
  export type AsObject = {
    integration?: PilotThingsIntegration.AsObject,
  }
}

export class DeletePilotThingsIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeletePilotThingsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeletePilotThingsIntegrationRequest): DeletePilotThingsIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeletePilotThingsIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeletePilotThingsIntegrationRequest;
  static deserializeBinaryFromReader(message: DeletePilotThingsIntegrationRequest, reader: jspb.BinaryReader): DeletePilotThingsIntegrationRequest;
}

export namespace DeletePilotThingsIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class IftttIntegration extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  getKey(): string;
  setKey(value: string): void;

  clearUplinkValuesList(): void;
  getUplinkValuesList(): Array<string>;
  setUplinkValuesList(value: Array<string>): void;
  addUplinkValues(value: string, index?: number): string;

  getArbitraryJson(): boolean;
  setArbitraryJson(value: boolean): void;

  getEventPrefix(): string;
  setEventPrefix(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IftttIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: IftttIntegration): IftttIntegration.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IftttIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IftttIntegration;
  static deserializeBinaryFromReader(message: IftttIntegration, reader: jspb.BinaryReader): IftttIntegration;
}

export namespace IftttIntegration {
  export type AsObject = {
    applicationId: string,
    key: string,
    uplinkValuesList: Array<string>,
    arbitraryJson: boolean,
    eventPrefix: string,
  }
}

export class CreateIftttIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): IftttIntegration | undefined;
  setIntegration(value?: IftttIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateIftttIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateIftttIntegrationRequest): CreateIftttIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateIftttIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateIftttIntegrationRequest;
  static deserializeBinaryFromReader(message: CreateIftttIntegrationRequest, reader: jspb.BinaryReader): CreateIftttIntegrationRequest;
}

export namespace CreateIftttIntegrationRequest {
  export type AsObject = {
    integration?: IftttIntegration.AsObject,
  }
}

export class GetIftttIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIftttIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIftttIntegrationRequest): GetIftttIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIftttIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIftttIntegrationRequest;
  static deserializeBinaryFromReader(message: GetIftttIntegrationRequest, reader: jspb.BinaryReader): GetIftttIntegrationRequest;
}

export namespace GetIftttIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GetIftttIntegrationResponse extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): IftttIntegration | undefined;
  setIntegration(value?: IftttIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIftttIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIftttIntegrationResponse): GetIftttIntegrationResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIftttIntegrationResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIftttIntegrationResponse;
  static deserializeBinaryFromReader(message: GetIftttIntegrationResponse, reader: jspb.BinaryReader): GetIftttIntegrationResponse;
}

export namespace GetIftttIntegrationResponse {
  export type AsObject = {
    integration?: IftttIntegration.AsObject,
  }
}

export class UpdateIftttIntegrationRequest extends jspb.Message {
  hasIntegration(): boolean;
  clearIntegration(): void;
  getIntegration(): IftttIntegration | undefined;
  setIntegration(value?: IftttIntegration): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateIftttIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateIftttIntegrationRequest): UpdateIftttIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateIftttIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateIftttIntegrationRequest;
  static deserializeBinaryFromReader(message: UpdateIftttIntegrationRequest, reader: jspb.BinaryReader): UpdateIftttIntegrationRequest;
}

export namespace UpdateIftttIntegrationRequest {
  export type AsObject = {
    integration?: IftttIntegration.AsObject,
  }
}

export class DeleteIftttIntegrationRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteIftttIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteIftttIntegrationRequest): DeleteIftttIntegrationRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteIftttIntegrationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteIftttIntegrationRequest;
  static deserializeBinaryFromReader(message: DeleteIftttIntegrationRequest, reader: jspb.BinaryReader): DeleteIftttIntegrationRequest;
}

export namespace DeleteIftttIntegrationRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GenerateMqttIntegrationClientCertificateRequest extends jspb.Message {
  getApplicationId(): string;
  setApplicationId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GenerateMqttIntegrationClientCertificateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GenerateMqttIntegrationClientCertificateRequest): GenerateMqttIntegrationClientCertificateRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GenerateMqttIntegrationClientCertificateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GenerateMqttIntegrationClientCertificateRequest;
  static deserializeBinaryFromReader(message: GenerateMqttIntegrationClientCertificateRequest, reader: jspb.BinaryReader): GenerateMqttIntegrationClientCertificateRequest;
}

export namespace GenerateMqttIntegrationClientCertificateRequest {
  export type AsObject = {
    applicationId: string,
  }
}

export class GenerateMqttIntegrationClientCertificateResponse extends jspb.Message {
  getTlsCert(): string;
  setTlsCert(value: string): void;

  getTlsKey(): string;
  setTlsKey(value: string): void;

  getCaCert(): string;
  setCaCert(value: string): void;

  hasExpiresAt(): boolean;
  clearExpiresAt(): void;
  getExpiresAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setExpiresAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GenerateMqttIntegrationClientCertificateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GenerateMqttIntegrationClientCertificateResponse): GenerateMqttIntegrationClientCertificateResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GenerateMqttIntegrationClientCertificateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GenerateMqttIntegrationClientCertificateResponse;
  static deserializeBinaryFromReader(message: GenerateMqttIntegrationClientCertificateResponse, reader: jspb.BinaryReader): GenerateMqttIntegrationClientCertificateResponse;
}

export namespace GenerateMqttIntegrationClientCertificateResponse {
  export type AsObject = {
    tlsCert: string,
    tlsKey: string,
    caCert: string,
    expiresAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export interface EncodingMap {
  JSON: 0;
  PROTOBUF: 1;
}

export const Encoding: EncodingMap;

export interface IntegrationKindMap {
  HTTP: 0;
  INFLUX_DB: 1;
  THINGS_BOARD: 2;
  MY_DEVICES: 3;
  LORA_CLOUD: 4;
  GCP_PUB_SUB: 5;
  AWS_SNS: 6;
  AZURE_SERVICE_BUS: 7;
  PILOT_THINGS: 8;
  MQTT_GLOBAL: 9;
  IFTTT: 10;
}

export const IntegrationKind: IntegrationKindMap;

export interface InfluxDbPrecisionMap {
  NS: 0;
  U: 1;
  MS: 2;
  S: 3;
  M: 4;
  H: 5;
}

export const InfluxDbPrecision: InfluxDbPrecisionMap;

export interface InfluxDbVersionMap {
  INFLUXDB_1: 0;
  INFLUXDB_2: 1;
}

export const InfluxDbVersion: InfluxDbVersionMap;

