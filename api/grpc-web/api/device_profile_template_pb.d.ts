import * as jspb from 'google-protobuf'

import * as google_api_annotations_pb from '../google/api/annotations_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as common_common_pb from '../common/common_pb';
import * as api_device_profile_pb from '../api/device_profile_pb';


export class DeviceProfileTemplate extends jspb.Message {
  getId(): string;
  setId(value: string): DeviceProfileTemplate;

  getName(): string;
  setName(value: string): DeviceProfileTemplate;

  getDescription(): string;
  setDescription(value: string): DeviceProfileTemplate;

  getVendor(): string;
  setVendor(value: string): DeviceProfileTemplate;

  getFirmware(): string;
  setFirmware(value: string): DeviceProfileTemplate;

  getRegion(): common_common_pb.Region;
  setRegion(value: common_common_pb.Region): DeviceProfileTemplate;

  getMacVersion(): common_common_pb.MacVersion;
  setMacVersion(value: common_common_pb.MacVersion): DeviceProfileTemplate;

  getRegParamsRevision(): common_common_pb.RegParamsRevision;
  setRegParamsRevision(value: common_common_pb.RegParamsRevision): DeviceProfileTemplate;

  getAdrAlgorithmId(): string;
  setAdrAlgorithmId(value: string): DeviceProfileTemplate;

  getPayloadCodecRuntime(): api_device_profile_pb.CodecRuntime;
  setPayloadCodecRuntime(value: api_device_profile_pb.CodecRuntime): DeviceProfileTemplate;

  getPayloadCodecScript(): string;
  setPayloadCodecScript(value: string): DeviceProfileTemplate;

  getFlushQueueOnActivate(): boolean;
  setFlushQueueOnActivate(value: boolean): DeviceProfileTemplate;

  getUplinkInterval(): number;
  setUplinkInterval(value: number): DeviceProfileTemplate;

  getDeviceStatusReqInterval(): number;
  setDeviceStatusReqInterval(value: number): DeviceProfileTemplate;

  getSupportsOtaa(): boolean;
  setSupportsOtaa(value: boolean): DeviceProfileTemplate;

  getSupportsClassB(): boolean;
  setSupportsClassB(value: boolean): DeviceProfileTemplate;

  getSupportsClassC(): boolean;
  setSupportsClassC(value: boolean): DeviceProfileTemplate;

  getClassBTimeout(): number;
  setClassBTimeout(value: number): DeviceProfileTemplate;

  getClassBPingSlotNbK(): number;
  setClassBPingSlotNbK(value: number): DeviceProfileTemplate;

  getClassBPingSlotDr(): number;
  setClassBPingSlotDr(value: number): DeviceProfileTemplate;

  getClassBPingSlotFreq(): number;
  setClassBPingSlotFreq(value: number): DeviceProfileTemplate;

  getClassCTimeout(): number;
  setClassCTimeout(value: number): DeviceProfileTemplate;

  getAbpRx1Delay(): number;
  setAbpRx1Delay(value: number): DeviceProfileTemplate;

  getAbpRx1DrOffset(): number;
  setAbpRx1DrOffset(value: number): DeviceProfileTemplate;

  getAbpRx2Dr(): number;
  setAbpRx2Dr(value: number): DeviceProfileTemplate;

  getAbpRx2Freq(): number;
  setAbpRx2Freq(value: number): DeviceProfileTemplate;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): DeviceProfileTemplate;

  getMeasurementsMap(): jspb.Map<string, api_device_profile_pb.Measurement>;
  clearMeasurementsMap(): DeviceProfileTemplate;

  getAutoDetectMeasurements(): boolean;
  setAutoDetectMeasurements(value: boolean): DeviceProfileTemplate;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeviceProfileTemplate.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceProfileTemplate): DeviceProfileTemplate.AsObject;
  static serializeBinaryToWriter(message: DeviceProfileTemplate, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeviceProfileTemplate;
  static deserializeBinaryFromReader(message: DeviceProfileTemplate, reader: jspb.BinaryReader): DeviceProfileTemplate;
}

export namespace DeviceProfileTemplate {
  export type AsObject = {
    id: string,
    name: string,
    description: string,
    vendor: string,
    firmware: string,
    region: common_common_pb.Region,
    macVersion: common_common_pb.MacVersion,
    regParamsRevision: common_common_pb.RegParamsRevision,
    adrAlgorithmId: string,
    payloadCodecRuntime: api_device_profile_pb.CodecRuntime,
    payloadCodecScript: string,
    flushQueueOnActivate: boolean,
    uplinkInterval: number,
    deviceStatusReqInterval: number,
    supportsOtaa: boolean,
    supportsClassB: boolean,
    supportsClassC: boolean,
    classBTimeout: number,
    classBPingSlotNbK: number,
    classBPingSlotDr: number,
    classBPingSlotFreq: number,
    classCTimeout: number,
    abpRx1Delay: number,
    abpRx1DrOffset: number,
    abpRx2Dr: number,
    abpRx2Freq: number,
    tagsMap: Array<[string, string]>,
    measurementsMap: Array<[string, api_device_profile_pb.Measurement.AsObject]>,
    autoDetectMeasurements: boolean,
  }
}

export class DeviceProfileTemplateListItem extends jspb.Message {
  getId(): string;
  setId(value: string): DeviceProfileTemplateListItem;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): DeviceProfileTemplateListItem;
  hasCreatedAt(): boolean;
  clearCreatedAt(): DeviceProfileTemplateListItem;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): DeviceProfileTemplateListItem;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): DeviceProfileTemplateListItem;

  getName(): string;
  setName(value: string): DeviceProfileTemplateListItem;

  getVendor(): string;
  setVendor(value: string): DeviceProfileTemplateListItem;

  getFirmware(): string;
  setFirmware(value: string): DeviceProfileTemplateListItem;

  getRegion(): common_common_pb.Region;
  setRegion(value: common_common_pb.Region): DeviceProfileTemplateListItem;

  getMacVersion(): common_common_pb.MacVersion;
  setMacVersion(value: common_common_pb.MacVersion): DeviceProfileTemplateListItem;

  getRegParamsRevision(): common_common_pb.RegParamsRevision;
  setRegParamsRevision(value: common_common_pb.RegParamsRevision): DeviceProfileTemplateListItem;

  getSupportsOtaa(): boolean;
  setSupportsOtaa(value: boolean): DeviceProfileTemplateListItem;

  getSupportsClassB(): boolean;
  setSupportsClassB(value: boolean): DeviceProfileTemplateListItem;

  getSupportsClassC(): boolean;
  setSupportsClassC(value: boolean): DeviceProfileTemplateListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeviceProfileTemplateListItem.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceProfileTemplateListItem): DeviceProfileTemplateListItem.AsObject;
  static serializeBinaryToWriter(message: DeviceProfileTemplateListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeviceProfileTemplateListItem;
  static deserializeBinaryFromReader(message: DeviceProfileTemplateListItem, reader: jspb.BinaryReader): DeviceProfileTemplateListItem;
}

export namespace DeviceProfileTemplateListItem {
  export type AsObject = {
    id: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    name: string,
    vendor: string,
    firmware: string,
    region: common_common_pb.Region,
    macVersion: common_common_pb.MacVersion,
    regParamsRevision: common_common_pb.RegParamsRevision,
    supportsOtaa: boolean,
    supportsClassB: boolean,
    supportsClassC: boolean,
  }
}

export class CreateDeviceProfileTemplateRequest extends jspb.Message {
  getDeviceProfileTemplate(): DeviceProfileTemplate | undefined;
  setDeviceProfileTemplate(value?: DeviceProfileTemplate): CreateDeviceProfileTemplateRequest;
  hasDeviceProfileTemplate(): boolean;
  clearDeviceProfileTemplate(): CreateDeviceProfileTemplateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateDeviceProfileTemplateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateDeviceProfileTemplateRequest): CreateDeviceProfileTemplateRequest.AsObject;
  static serializeBinaryToWriter(message: CreateDeviceProfileTemplateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateDeviceProfileTemplateRequest;
  static deserializeBinaryFromReader(message: CreateDeviceProfileTemplateRequest, reader: jspb.BinaryReader): CreateDeviceProfileTemplateRequest;
}

export namespace CreateDeviceProfileTemplateRequest {
  export type AsObject = {
    deviceProfileTemplate?: DeviceProfileTemplate.AsObject,
  }
}

export class GetDeviceProfileTemplateRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetDeviceProfileTemplateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDeviceProfileTemplateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDeviceProfileTemplateRequest): GetDeviceProfileTemplateRequest.AsObject;
  static serializeBinaryToWriter(message: GetDeviceProfileTemplateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDeviceProfileTemplateRequest;
  static deserializeBinaryFromReader(message: GetDeviceProfileTemplateRequest, reader: jspb.BinaryReader): GetDeviceProfileTemplateRequest;
}

export namespace GetDeviceProfileTemplateRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetDeviceProfileTemplateResponse extends jspb.Message {
  getDeviceProfileTemplate(): DeviceProfileTemplate | undefined;
  setDeviceProfileTemplate(value?: DeviceProfileTemplate): GetDeviceProfileTemplateResponse;
  hasDeviceProfileTemplate(): boolean;
  clearDeviceProfileTemplate(): GetDeviceProfileTemplateResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetDeviceProfileTemplateResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): GetDeviceProfileTemplateResponse;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetDeviceProfileTemplateResponse;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): GetDeviceProfileTemplateResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDeviceProfileTemplateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDeviceProfileTemplateResponse): GetDeviceProfileTemplateResponse.AsObject;
  static serializeBinaryToWriter(message: GetDeviceProfileTemplateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDeviceProfileTemplateResponse;
  static deserializeBinaryFromReader(message: GetDeviceProfileTemplateResponse, reader: jspb.BinaryReader): GetDeviceProfileTemplateResponse;
}

export namespace GetDeviceProfileTemplateResponse {
  export type AsObject = {
    deviceProfileTemplate?: DeviceProfileTemplate.AsObject,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class UpdateDeviceProfileTemplateRequest extends jspb.Message {
  getDeviceProfileTemplate(): DeviceProfileTemplate | undefined;
  setDeviceProfileTemplate(value?: DeviceProfileTemplate): UpdateDeviceProfileTemplateRequest;
  hasDeviceProfileTemplate(): boolean;
  clearDeviceProfileTemplate(): UpdateDeviceProfileTemplateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateDeviceProfileTemplateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateDeviceProfileTemplateRequest): UpdateDeviceProfileTemplateRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateDeviceProfileTemplateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateDeviceProfileTemplateRequest;
  static deserializeBinaryFromReader(message: UpdateDeviceProfileTemplateRequest, reader: jspb.BinaryReader): UpdateDeviceProfileTemplateRequest;
}

export namespace UpdateDeviceProfileTemplateRequest {
  export type AsObject = {
    deviceProfileTemplate?: DeviceProfileTemplate.AsObject,
  }
}

export class DeleteDeviceProfileTemplateRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteDeviceProfileTemplateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteDeviceProfileTemplateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteDeviceProfileTemplateRequest): DeleteDeviceProfileTemplateRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteDeviceProfileTemplateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteDeviceProfileTemplateRequest;
  static deserializeBinaryFromReader(message: DeleteDeviceProfileTemplateRequest, reader: jspb.BinaryReader): DeleteDeviceProfileTemplateRequest;
}

export namespace DeleteDeviceProfileTemplateRequest {
  export type AsObject = {
    id: string,
  }
}

export class ListDeviceProfileTemplatesRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): ListDeviceProfileTemplatesRequest;

  getOffset(): number;
  setOffset(value: number): ListDeviceProfileTemplatesRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfileTemplatesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfileTemplatesRequest): ListDeviceProfileTemplatesRequest.AsObject;
  static serializeBinaryToWriter(message: ListDeviceProfileTemplatesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListDeviceProfileTemplatesRequest;
  static deserializeBinaryFromReader(message: ListDeviceProfileTemplatesRequest, reader: jspb.BinaryReader): ListDeviceProfileTemplatesRequest;
}

export namespace ListDeviceProfileTemplatesRequest {
  export type AsObject = {
    limit: number,
    offset: number,
  }
}

export class ListDeviceProfileTemplatesResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): ListDeviceProfileTemplatesResponse;

  getResultList(): Array<DeviceProfileTemplateListItem>;
  setResultList(value: Array<DeviceProfileTemplateListItem>): ListDeviceProfileTemplatesResponse;
  clearResultList(): ListDeviceProfileTemplatesResponse;
  addResult(value?: DeviceProfileTemplateListItem, index?: number): DeviceProfileTemplateListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfileTemplatesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfileTemplatesResponse): ListDeviceProfileTemplatesResponse.AsObject;
  static serializeBinaryToWriter(message: ListDeviceProfileTemplatesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListDeviceProfileTemplatesResponse;
  static deserializeBinaryFromReader(message: ListDeviceProfileTemplatesResponse, reader: jspb.BinaryReader): ListDeviceProfileTemplatesResponse;
}

export namespace ListDeviceProfileTemplatesResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<DeviceProfileTemplateListItem.AsObject>,
  }
}

