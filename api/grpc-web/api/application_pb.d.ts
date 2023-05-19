import * as jspb from 'google-protobuf'

import * as google_api_annotations_pb from '../google/api/annotations_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';


export class Application extends jspb.Message {
  getId(): string;
  setId(value: string): Application;

  getName(): string;
  setName(value: string): Application;

  getDescription(): string;
  setDescription(value: string): Application;

  getTenantId(): string;
  setTenantId(value: string): Application;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Application.AsObject;
  static toObject(includeInstance: boolean, msg: Application): Application.AsObject;
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
  setId(value: string): ApplicationListItem;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): ApplicationListItem;
  hasCreatedAt(): boolean;
  clearCreatedAt(): ApplicationListItem;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): ApplicationListItem;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): ApplicationListItem;

  getName(): string;
  setName(value: string): ApplicationListItem;

  getDescription(): string;
  setDescription(value: string): ApplicationListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ApplicationListItem.AsObject;
  static toObject(includeInstance: boolean, msg: ApplicationListItem): ApplicationListItem.AsObject;
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
  getApplication(): Application | undefined;
  setApplication(value?: Application): CreateApplicationRequest;
  hasApplication(): boolean;
  clearApplication(): CreateApplicationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateApplicationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateApplicationRequest): CreateApplicationRequest.AsObject;
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
  setId(value: string): CreateApplicationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateApplicationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateApplicationResponse): CreateApplicationResponse.AsObject;
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
  setId(value: string): GetApplicationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetApplicationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetApplicationRequest): GetApplicationRequest.AsObject;
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
  getApplication(): Application | undefined;
  setApplication(value?: Application): GetApplicationResponse;
  hasApplication(): boolean;
  clearApplication(): GetApplicationResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetApplicationResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): GetApplicationResponse;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetApplicationResponse;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): GetApplicationResponse;

  getMeasurementKeysList(): Array<string>;
  setMeasurementKeysList(value: Array<string>): GetApplicationResponse;
  clearMeasurementKeysList(): GetApplicationResponse;
  addMeasurementKeys(value: string, index?: number): GetApplicationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetApplicationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetApplicationResponse): GetApplicationResponse.AsObject;
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
  getApplication(): Application | undefined;
  setApplication(value?: Application): UpdateApplicationRequest;
  hasApplication(): boolean;
  clearApplication(): UpdateApplicationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateApplicationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateApplicationRequest): UpdateApplicationRequest.AsObject;
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
  setId(value: string): DeleteApplicationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteApplicationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteApplicationRequest): DeleteApplicationRequest.AsObject;
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
  setLimit(value: number): ListApplicationsRequest;

  getOffset(): number;
  setOffset(value: number): ListApplicationsRequest;

  getSearch(): string;
  setSearch(value: string): ListApplicationsRequest;

  getTenantId(): string;
  setTenantId(value: string): ListApplicationsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListApplicationsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListApplicationsRequest): ListApplicationsRequest.AsObject;
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
  setTotalCount(value: number): ListApplicationsResponse;

  getResultList(): Array<ApplicationListItem>;
  setResultList(value: Array<ApplicationListItem>): ListApplicationsResponse;
  clearResultList(): ListApplicationsResponse;
  addResult(value?: ApplicationListItem, index?: number): ApplicationListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListApplicationsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListApplicationsResponse): ListApplicationsResponse.AsObject;
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
  setApplicationId(value: string): ListIntegrationsRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListIntegrationsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListIntegrationsRequest): ListIntegrationsRequest.AsObject;
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
  getKind(): IntegrationKind;
  setKind(value: IntegrationKind): IntegrationListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IntegrationListItem.AsObject;
  static toObject(includeInstance: boolean, msg: IntegrationListItem): IntegrationListItem.AsObject;
  static serializeBinaryToWriter(message: IntegrationListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IntegrationListItem;
  static deserializeBinaryFromReader(message: IntegrationListItem, reader: jspb.BinaryReader): IntegrationListItem;
}

export namespace IntegrationListItem {
  export type AsObject = {
    kind: IntegrationKind,
  }
}

export class ListIntegrationsResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): ListIntegrationsResponse;

  getResultList(): Array<IntegrationListItem>;
  setResultList(value: Array<IntegrationListItem>): ListIntegrationsResponse;
  clearResultList(): ListIntegrationsResponse;
  addResult(value?: IntegrationListItem, index?: number): IntegrationListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListIntegrationsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListIntegrationsResponse): ListIntegrationsResponse.AsObject;
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
  setApplicationId(value: string): HttpIntegration;

  getHeadersMap(): jspb.Map<string, string>;
  clearHeadersMap(): HttpIntegration;

  getEncoding(): Encoding;
  setEncoding(value: Encoding): HttpIntegration;

  getEventEndpointUrl(): string;
  setEventEndpointUrl(value: string): HttpIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HttpIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: HttpIntegration): HttpIntegration.AsObject;
  static serializeBinaryToWriter(message: HttpIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HttpIntegration;
  static deserializeBinaryFromReader(message: HttpIntegration, reader: jspb.BinaryReader): HttpIntegration;
}

export namespace HttpIntegration {
  export type AsObject = {
    applicationId: string,
    headersMap: Array<[string, string]>,
    encoding: Encoding,
    eventEndpointUrl: string,
  }
}

export class CreateHttpIntegrationRequest extends jspb.Message {
  getIntegration(): HttpIntegration | undefined;
  setIntegration(value?: HttpIntegration): CreateHttpIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateHttpIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateHttpIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateHttpIntegrationRequest): CreateHttpIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetHttpIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetHttpIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetHttpIntegrationRequest): GetHttpIntegrationRequest.AsObject;
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
  getIntegration(): HttpIntegration | undefined;
  setIntegration(value?: HttpIntegration): GetHttpIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetHttpIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetHttpIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetHttpIntegrationResponse): GetHttpIntegrationResponse.AsObject;
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
  getIntegration(): HttpIntegration | undefined;
  setIntegration(value?: HttpIntegration): UpdateHttpIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateHttpIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateHttpIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateHttpIntegrationRequest): UpdateHttpIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteHttpIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteHttpIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteHttpIntegrationRequest): DeleteHttpIntegrationRequest.AsObject;
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
  setApplicationId(value: string): InfluxDbIntegration;

  getEndpoint(): string;
  setEndpoint(value: string): InfluxDbIntegration;

  getDb(): string;
  setDb(value: string): InfluxDbIntegration;

  getUsername(): string;
  setUsername(value: string): InfluxDbIntegration;

  getPassword(): string;
  setPassword(value: string): InfluxDbIntegration;

  getRetentionPolicyName(): string;
  setRetentionPolicyName(value: string): InfluxDbIntegration;

  getPrecision(): InfluxDbPrecision;
  setPrecision(value: InfluxDbPrecision): InfluxDbIntegration;

  getVersion(): InfluxDbVersion;
  setVersion(value: InfluxDbVersion): InfluxDbIntegration;

  getToken(): string;
  setToken(value: string): InfluxDbIntegration;

  getOrganization(): string;
  setOrganization(value: string): InfluxDbIntegration;

  getBucket(): string;
  setBucket(value: string): InfluxDbIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InfluxDbIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: InfluxDbIntegration): InfluxDbIntegration.AsObject;
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
    precision: InfluxDbPrecision,
    version: InfluxDbVersion,
    token: string,
    organization: string,
    bucket: string,
  }
}

export class CreateInfluxDbIntegrationRequest extends jspb.Message {
  getIntegration(): InfluxDbIntegration | undefined;
  setIntegration(value?: InfluxDbIntegration): CreateInfluxDbIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateInfluxDbIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateInfluxDbIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateInfluxDbIntegrationRequest): CreateInfluxDbIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetInfluxDbIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetInfluxDbIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetInfluxDbIntegrationRequest): GetInfluxDbIntegrationRequest.AsObject;
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
  getIntegration(): InfluxDbIntegration | undefined;
  setIntegration(value?: InfluxDbIntegration): GetInfluxDbIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetInfluxDbIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetInfluxDbIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetInfluxDbIntegrationResponse): GetInfluxDbIntegrationResponse.AsObject;
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
  getIntegration(): InfluxDbIntegration | undefined;
  setIntegration(value?: InfluxDbIntegration): UpdateInfluxDbIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateInfluxDbIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateInfluxDbIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateInfluxDbIntegrationRequest): UpdateInfluxDbIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteInfluxDbIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteInfluxDbIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteInfluxDbIntegrationRequest): DeleteInfluxDbIntegrationRequest.AsObject;
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
  setApplicationId(value: string): ThingsBoardIntegration;

  getServer(): string;
  setServer(value: string): ThingsBoardIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ThingsBoardIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: ThingsBoardIntegration): ThingsBoardIntegration.AsObject;
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
  getIntegration(): ThingsBoardIntegration | undefined;
  setIntegration(value?: ThingsBoardIntegration): CreateThingsBoardIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateThingsBoardIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateThingsBoardIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateThingsBoardIntegrationRequest): CreateThingsBoardIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetThingsBoardIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetThingsBoardIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetThingsBoardIntegrationRequest): GetThingsBoardIntegrationRequest.AsObject;
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
  getIntegration(): ThingsBoardIntegration | undefined;
  setIntegration(value?: ThingsBoardIntegration): GetThingsBoardIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetThingsBoardIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetThingsBoardIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetThingsBoardIntegrationResponse): GetThingsBoardIntegrationResponse.AsObject;
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
  getIntegration(): ThingsBoardIntegration | undefined;
  setIntegration(value?: ThingsBoardIntegration): UpdateThingsBoardIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateThingsBoardIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateThingsBoardIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateThingsBoardIntegrationRequest): UpdateThingsBoardIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteThingsBoardIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteThingsBoardIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteThingsBoardIntegrationRequest): DeleteThingsBoardIntegrationRequest.AsObject;
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
  setApplicationId(value: string): MyDevicesIntegration;

  getEndpoint(): string;
  setEndpoint(value: string): MyDevicesIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MyDevicesIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: MyDevicesIntegration): MyDevicesIntegration.AsObject;
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
  getIntegration(): MyDevicesIntegration | undefined;
  setIntegration(value?: MyDevicesIntegration): CreateMyDevicesIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateMyDevicesIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateMyDevicesIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateMyDevicesIntegrationRequest): CreateMyDevicesIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetMyDevicesIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMyDevicesIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetMyDevicesIntegrationRequest): GetMyDevicesIntegrationRequest.AsObject;
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
  getIntegration(): MyDevicesIntegration | undefined;
  setIntegration(value?: MyDevicesIntegration): GetMyDevicesIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetMyDevicesIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetMyDevicesIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetMyDevicesIntegrationResponse): GetMyDevicesIntegrationResponse.AsObject;
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
  getIntegration(): MyDevicesIntegration | undefined;
  setIntegration(value?: MyDevicesIntegration): UpdateMyDevicesIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateMyDevicesIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateMyDevicesIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateMyDevicesIntegrationRequest): UpdateMyDevicesIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteMyDevicesIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteMyDevicesIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteMyDevicesIntegrationRequest): DeleteMyDevicesIntegrationRequest.AsObject;
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
  setApplicationId(value: string): LoraCloudIntegration;

  getModemGeolocationServices(): LoraCloudModemGeolocationServices | undefined;
  setModemGeolocationServices(value?: LoraCloudModemGeolocationServices): LoraCloudIntegration;
  hasModemGeolocationServices(): boolean;
  clearModemGeolocationServices(): LoraCloudIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoraCloudIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: LoraCloudIntegration): LoraCloudIntegration.AsObject;
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
  setToken(value: string): LoraCloudModemGeolocationServices;

  getModemEnabled(): boolean;
  setModemEnabled(value: boolean): LoraCloudModemGeolocationServices;

  getForwardFPortsList(): Array<number>;
  setForwardFPortsList(value: Array<number>): LoraCloudModemGeolocationServices;
  clearForwardFPortsList(): LoraCloudModemGeolocationServices;
  addForwardFPorts(value: number, index?: number): LoraCloudModemGeolocationServices;

  getGnssUseRxTime(): boolean;
  setGnssUseRxTime(value: boolean): LoraCloudModemGeolocationServices;

  getGnssUseGatewayLocation(): boolean;
  setGnssUseGatewayLocation(value: boolean): LoraCloudModemGeolocationServices;

  getParseTlv(): boolean;
  setParseTlv(value: boolean): LoraCloudModemGeolocationServices;

  getGeolocationBufferTtl(): number;
  setGeolocationBufferTtl(value: number): LoraCloudModemGeolocationServices;

  getGeolocationMinBufferSize(): number;
  setGeolocationMinBufferSize(value: number): LoraCloudModemGeolocationServices;

  getGeolocationTdoa(): boolean;
  setGeolocationTdoa(value: boolean): LoraCloudModemGeolocationServices;

  getGeolocationRssi(): boolean;
  setGeolocationRssi(value: boolean): LoraCloudModemGeolocationServices;

  getGeolocationGnss(): boolean;
  setGeolocationGnss(value: boolean): LoraCloudModemGeolocationServices;

  getGeolocationGnssPayloadField(): string;
  setGeolocationGnssPayloadField(value: string): LoraCloudModemGeolocationServices;

  getGeolocationGnssUseRxTime(): boolean;
  setGeolocationGnssUseRxTime(value: boolean): LoraCloudModemGeolocationServices;

  getGeolocationWifi(): boolean;
  setGeolocationWifi(value: boolean): LoraCloudModemGeolocationServices;

  getGeolocationWifiPayloadField(): string;
  setGeolocationWifiPayloadField(value: string): LoraCloudModemGeolocationServices;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoraCloudModemGeolocationServices.AsObject;
  static toObject(includeInstance: boolean, msg: LoraCloudModemGeolocationServices): LoraCloudModemGeolocationServices.AsObject;
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
  getIntegration(): LoraCloudIntegration | undefined;
  setIntegration(value?: LoraCloudIntegration): CreateLoraCloudIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateLoraCloudIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateLoraCloudIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateLoraCloudIntegrationRequest): CreateLoraCloudIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetLoraCloudIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetLoraCloudIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetLoraCloudIntegrationRequest): GetLoraCloudIntegrationRequest.AsObject;
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
  getIntegration(): LoraCloudIntegration | undefined;
  setIntegration(value?: LoraCloudIntegration): GetLoraCloudIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetLoraCloudIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetLoraCloudIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetLoraCloudIntegrationResponse): GetLoraCloudIntegrationResponse.AsObject;
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
  getIntegration(): LoraCloudIntegration | undefined;
  setIntegration(value?: LoraCloudIntegration): UpdateLoraCloudIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateLoraCloudIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateLoraCloudIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateLoraCloudIntegrationRequest): UpdateLoraCloudIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteLoraCloudIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteLoraCloudIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteLoraCloudIntegrationRequest): DeleteLoraCloudIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GcpPubSubIntegration;

  getEncoding(): Encoding;
  setEncoding(value: Encoding): GcpPubSubIntegration;

  getCredentialsFile(): string;
  setCredentialsFile(value: string): GcpPubSubIntegration;

  getProjectId(): string;
  setProjectId(value: string): GcpPubSubIntegration;

  getTopicName(): string;
  setTopicName(value: string): GcpPubSubIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GcpPubSubIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: GcpPubSubIntegration): GcpPubSubIntegration.AsObject;
  static serializeBinaryToWriter(message: GcpPubSubIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GcpPubSubIntegration;
  static deserializeBinaryFromReader(message: GcpPubSubIntegration, reader: jspb.BinaryReader): GcpPubSubIntegration;
}

export namespace GcpPubSubIntegration {
  export type AsObject = {
    applicationId: string,
    encoding: Encoding,
    credentialsFile: string,
    projectId: string,
    topicName: string,
  }
}

export class CreateGcpPubSubIntegrationRequest extends jspb.Message {
  getIntegration(): GcpPubSubIntegration | undefined;
  setIntegration(value?: GcpPubSubIntegration): CreateGcpPubSubIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateGcpPubSubIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateGcpPubSubIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateGcpPubSubIntegrationRequest): CreateGcpPubSubIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetGcpPubSubIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGcpPubSubIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetGcpPubSubIntegrationRequest): GetGcpPubSubIntegrationRequest.AsObject;
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
  getIntegration(): GcpPubSubIntegration | undefined;
  setIntegration(value?: GcpPubSubIntegration): GetGcpPubSubIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetGcpPubSubIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetGcpPubSubIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetGcpPubSubIntegrationResponse): GetGcpPubSubIntegrationResponse.AsObject;
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
  getIntegration(): GcpPubSubIntegration | undefined;
  setIntegration(value?: GcpPubSubIntegration): UpdateGcpPubSubIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateGcpPubSubIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateGcpPubSubIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateGcpPubSubIntegrationRequest): UpdateGcpPubSubIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteGcpPubSubIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteGcpPubSubIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteGcpPubSubIntegrationRequest): DeleteGcpPubSubIntegrationRequest.AsObject;
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
  setApplicationId(value: string): AwsSnsIntegration;

  getEncoding(): Encoding;
  setEncoding(value: Encoding): AwsSnsIntegration;

  getRegion(): string;
  setRegion(value: string): AwsSnsIntegration;

  getAccessKeyId(): string;
  setAccessKeyId(value: string): AwsSnsIntegration;

  getSecretAccessKey(): string;
  setSecretAccessKey(value: string): AwsSnsIntegration;

  getTopicArn(): string;
  setTopicArn(value: string): AwsSnsIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AwsSnsIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: AwsSnsIntegration): AwsSnsIntegration.AsObject;
  static serializeBinaryToWriter(message: AwsSnsIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AwsSnsIntegration;
  static deserializeBinaryFromReader(message: AwsSnsIntegration, reader: jspb.BinaryReader): AwsSnsIntegration;
}

export namespace AwsSnsIntegration {
  export type AsObject = {
    applicationId: string,
    encoding: Encoding,
    region: string,
    accessKeyId: string,
    secretAccessKey: string,
    topicArn: string,
  }
}

export class CreateAwsSnsIntegrationRequest extends jspb.Message {
  getIntegration(): AwsSnsIntegration | undefined;
  setIntegration(value?: AwsSnsIntegration): CreateAwsSnsIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateAwsSnsIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateAwsSnsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateAwsSnsIntegrationRequest): CreateAwsSnsIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetAwsSnsIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAwsSnsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAwsSnsIntegrationRequest): GetAwsSnsIntegrationRequest.AsObject;
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
  getIntegration(): AwsSnsIntegration | undefined;
  setIntegration(value?: AwsSnsIntegration): GetAwsSnsIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetAwsSnsIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAwsSnsIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAwsSnsIntegrationResponse): GetAwsSnsIntegrationResponse.AsObject;
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
  getIntegration(): AwsSnsIntegration | undefined;
  setIntegration(value?: AwsSnsIntegration): UpdateAwsSnsIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateAwsSnsIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateAwsSnsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateAwsSnsIntegrationRequest): UpdateAwsSnsIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteAwsSnsIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteAwsSnsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteAwsSnsIntegrationRequest): DeleteAwsSnsIntegrationRequest.AsObject;
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
  setApplicationId(value: string): AzureServiceBusIntegration;

  getEncoding(): Encoding;
  setEncoding(value: Encoding): AzureServiceBusIntegration;

  getConnectionString(): string;
  setConnectionString(value: string): AzureServiceBusIntegration;

  getPublishName(): string;
  setPublishName(value: string): AzureServiceBusIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AzureServiceBusIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: AzureServiceBusIntegration): AzureServiceBusIntegration.AsObject;
  static serializeBinaryToWriter(message: AzureServiceBusIntegration, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AzureServiceBusIntegration;
  static deserializeBinaryFromReader(message: AzureServiceBusIntegration, reader: jspb.BinaryReader): AzureServiceBusIntegration;
}

export namespace AzureServiceBusIntegration {
  export type AsObject = {
    applicationId: string,
    encoding: Encoding,
    connectionString: string,
    publishName: string,
  }
}

export class CreateAzureServiceBusIntegrationRequest extends jspb.Message {
  getIntegration(): AzureServiceBusIntegration | undefined;
  setIntegration(value?: AzureServiceBusIntegration): CreateAzureServiceBusIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateAzureServiceBusIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateAzureServiceBusIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateAzureServiceBusIntegrationRequest): CreateAzureServiceBusIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetAzureServiceBusIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAzureServiceBusIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAzureServiceBusIntegrationRequest): GetAzureServiceBusIntegrationRequest.AsObject;
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
  getIntegration(): AzureServiceBusIntegration | undefined;
  setIntegration(value?: AzureServiceBusIntegration): GetAzureServiceBusIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetAzureServiceBusIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAzureServiceBusIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAzureServiceBusIntegrationResponse): GetAzureServiceBusIntegrationResponse.AsObject;
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
  getIntegration(): AzureServiceBusIntegration | undefined;
  setIntegration(value?: AzureServiceBusIntegration): UpdateAzureServiceBusIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateAzureServiceBusIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateAzureServiceBusIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateAzureServiceBusIntegrationRequest): UpdateAzureServiceBusIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteAzureServiceBusIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteAzureServiceBusIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteAzureServiceBusIntegrationRequest): DeleteAzureServiceBusIntegrationRequest.AsObject;
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
  setApplicationId(value: string): PilotThingsIntegration;

  getServer(): string;
  setServer(value: string): PilotThingsIntegration;

  getToken(): string;
  setToken(value: string): PilotThingsIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PilotThingsIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: PilotThingsIntegration): PilotThingsIntegration.AsObject;
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
  getIntegration(): PilotThingsIntegration | undefined;
  setIntegration(value?: PilotThingsIntegration): CreatePilotThingsIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreatePilotThingsIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreatePilotThingsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreatePilotThingsIntegrationRequest): CreatePilotThingsIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetPilotThingsIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetPilotThingsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetPilotThingsIntegrationRequest): GetPilotThingsIntegrationRequest.AsObject;
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
  getIntegration(): PilotThingsIntegration | undefined;
  setIntegration(value?: PilotThingsIntegration): GetPilotThingsIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetPilotThingsIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetPilotThingsIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetPilotThingsIntegrationResponse): GetPilotThingsIntegrationResponse.AsObject;
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
  getIntegration(): PilotThingsIntegration | undefined;
  setIntegration(value?: PilotThingsIntegration): UpdatePilotThingsIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdatePilotThingsIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdatePilotThingsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdatePilotThingsIntegrationRequest): UpdatePilotThingsIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeletePilotThingsIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeletePilotThingsIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeletePilotThingsIntegrationRequest): DeletePilotThingsIntegrationRequest.AsObject;
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
  setApplicationId(value: string): IftttIntegration;

  getKey(): string;
  setKey(value: string): IftttIntegration;

  getUplinkValuesList(): Array<string>;
  setUplinkValuesList(value: Array<string>): IftttIntegration;
  clearUplinkValuesList(): IftttIntegration;
  addUplinkValues(value: string, index?: number): IftttIntegration;

  getArbitraryJson(): boolean;
  setArbitraryJson(value: boolean): IftttIntegration;

  getEventPrefix(): string;
  setEventPrefix(value: string): IftttIntegration;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IftttIntegration.AsObject;
  static toObject(includeInstance: boolean, msg: IftttIntegration): IftttIntegration.AsObject;
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
  getIntegration(): IftttIntegration | undefined;
  setIntegration(value?: IftttIntegration): CreateIftttIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): CreateIftttIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateIftttIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateIftttIntegrationRequest): CreateIftttIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GetIftttIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIftttIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIftttIntegrationRequest): GetIftttIntegrationRequest.AsObject;
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
  getIntegration(): IftttIntegration | undefined;
  setIntegration(value?: IftttIntegration): GetIftttIntegrationResponse;
  hasIntegration(): boolean;
  clearIntegration(): GetIftttIntegrationResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIftttIntegrationResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIftttIntegrationResponse): GetIftttIntegrationResponse.AsObject;
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
  getIntegration(): IftttIntegration | undefined;
  setIntegration(value?: IftttIntegration): UpdateIftttIntegrationRequest;
  hasIntegration(): boolean;
  clearIntegration(): UpdateIftttIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateIftttIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateIftttIntegrationRequest): UpdateIftttIntegrationRequest.AsObject;
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
  setApplicationId(value: string): DeleteIftttIntegrationRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteIftttIntegrationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteIftttIntegrationRequest): DeleteIftttIntegrationRequest.AsObject;
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
  setApplicationId(value: string): GenerateMqttIntegrationClientCertificateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GenerateMqttIntegrationClientCertificateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GenerateMqttIntegrationClientCertificateRequest): GenerateMqttIntegrationClientCertificateRequest.AsObject;
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
  setTlsCert(value: string): GenerateMqttIntegrationClientCertificateResponse;

  getTlsKey(): string;
  setTlsKey(value: string): GenerateMqttIntegrationClientCertificateResponse;

  getCaCert(): string;
  setCaCert(value: string): GenerateMqttIntegrationClientCertificateResponse;

  getExpiresAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setExpiresAt(value?: google_protobuf_timestamp_pb.Timestamp): GenerateMqttIntegrationClientCertificateResponse;
  hasExpiresAt(): boolean;
  clearExpiresAt(): GenerateMqttIntegrationClientCertificateResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GenerateMqttIntegrationClientCertificateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GenerateMqttIntegrationClientCertificateResponse): GenerateMqttIntegrationClientCertificateResponse.AsObject;
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

export enum Encoding { 
  JSON = 0,
  PROTOBUF = 1,
}
export enum IntegrationKind { 
  HTTP = 0,
  INFLUX_DB = 1,
  THINGS_BOARD = 2,
  MY_DEVICES = 3,
  LORA_CLOUD = 4,
  GCP_PUB_SUB = 5,
  AWS_SNS = 6,
  AZURE_SERVICE_BUS = 7,
  PILOT_THINGS = 8,
  MQTT_GLOBAL = 9,
  IFTTT = 10,
}
export enum InfluxDbPrecision { 
  NS = 0,
  U = 1,
  MS = 2,
  S = 3,
  M = 4,
  H = 5,
}
export enum InfluxDbVersion { 
  INFLUXDB_1 = 0,
  INFLUXDB_2 = 1,
}
