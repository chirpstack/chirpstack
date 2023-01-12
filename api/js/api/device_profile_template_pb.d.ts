// package: api
// file: api/device_profile_template.proto

import * as jspb from "google-protobuf";
import * as google_api_annotations_pb from "../google/api/annotations_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as common_common_pb from "../common/common_pb";
import * as api_device_profile_pb from "../api/device_profile_pb";

export class DeviceProfileTemplate extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

  getVendor(): string;
  setVendor(value: string): void;

  getFirmware(): string;
  setFirmware(value: string): void;

  getRegion(): common_common_pb.RegionMap[keyof common_common_pb.RegionMap];
  setRegion(value: common_common_pb.RegionMap[keyof common_common_pb.RegionMap]): void;

  getMacVersion(): common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap];
  setMacVersion(value: common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap]): void;

  getRegParamsRevision(): common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap];
  setRegParamsRevision(value: common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap]): void;

  getAdrAlgorithmId(): string;
  setAdrAlgorithmId(value: string): void;

  getPayloadCodecRuntime(): api_device_profile_pb.CodecRuntimeMap[keyof api_device_profile_pb.CodecRuntimeMap];
  setPayloadCodecRuntime(value: api_device_profile_pb.CodecRuntimeMap[keyof api_device_profile_pb.CodecRuntimeMap]): void;

  getPayloadCodecScript(): string;
  setPayloadCodecScript(value: string): void;

  getFlushQueueOnActivate(): boolean;
  setFlushQueueOnActivate(value: boolean): void;

  getUplinkInterval(): number;
  setUplinkInterval(value: number): void;

  getDeviceStatusReqInterval(): number;
  setDeviceStatusReqInterval(value: number): void;

  getSupportsOtaa(): boolean;
  setSupportsOtaa(value: boolean): void;

  getSupportsClassB(): boolean;
  setSupportsClassB(value: boolean): void;

  getSupportsClassC(): boolean;
  setSupportsClassC(value: boolean): void;

  getClassBTimeout(): number;
  setClassBTimeout(value: number): void;

  getClassBPingSlotNbK(): number;
  setClassBPingSlotNbK(value: number): void;

  getClassBPingSlotDr(): number;
  setClassBPingSlotDr(value: number): void;

  getClassBPingSlotFreq(): number;
  setClassBPingSlotFreq(value: number): void;

  getClassCTimeout(): number;
  setClassCTimeout(value: number): void;

  getAbpRx1Delay(): number;
  setAbpRx1Delay(value: number): void;

  getAbpRx1DrOffset(): number;
  setAbpRx1DrOffset(value: number): void;

  getAbpRx2Dr(): number;
  setAbpRx2Dr(value: number): void;

  getAbpRx2Freq(): number;
  setAbpRx2Freq(value: number): void;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): void;
  getMeasurementsMap(): jspb.Map<string, api_device_profile_pb.Measurement>;
  clearMeasurementsMap(): void;
  getAutoDetectMeasurements(): boolean;
  setAutoDetectMeasurements(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeviceProfileTemplate.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceProfileTemplate): DeviceProfileTemplate.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
    region: common_common_pb.RegionMap[keyof common_common_pb.RegionMap],
    macVersion: common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap],
    regParamsRevision: common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap],
    adrAlgorithmId: string,
    payloadCodecRuntime: api_device_profile_pb.CodecRuntimeMap[keyof api_device_profile_pb.CodecRuntimeMap],
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

  getVendor(): string;
  setVendor(value: string): void;

  getFirmware(): string;
  setFirmware(value: string): void;

  getRegion(): common_common_pb.RegionMap[keyof common_common_pb.RegionMap];
  setRegion(value: common_common_pb.RegionMap[keyof common_common_pb.RegionMap]): void;

  getMacVersion(): common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap];
  setMacVersion(value: common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap]): void;

  getRegParamsRevision(): common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap];
  setRegParamsRevision(value: common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap]): void;

  getSupportsOtaa(): boolean;
  setSupportsOtaa(value: boolean): void;

  getSupportsClassB(): boolean;
  setSupportsClassB(value: boolean): void;

  getSupportsClassC(): boolean;
  setSupportsClassC(value: boolean): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeviceProfileTemplateListItem.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceProfileTemplateListItem): DeviceProfileTemplateListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
    region: common_common_pb.RegionMap[keyof common_common_pb.RegionMap],
    macVersion: common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap],
    regParamsRevision: common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap],
    supportsOtaa: boolean,
    supportsClassB: boolean,
    supportsClassC: boolean,
  }
}

export class CreateDeviceProfileTemplateRequest extends jspb.Message {
  hasDeviceProfileTemplate(): boolean;
  clearDeviceProfileTemplate(): void;
  getDeviceProfileTemplate(): DeviceProfileTemplate | undefined;
  setDeviceProfileTemplate(value?: DeviceProfileTemplate): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateDeviceProfileTemplateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateDeviceProfileTemplateRequest): CreateDeviceProfileTemplateRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDeviceProfileTemplateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDeviceProfileTemplateRequest): GetDeviceProfileTemplateRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
  hasDeviceProfileTemplate(): boolean;
  clearDeviceProfileTemplate(): void;
  getDeviceProfileTemplate(): DeviceProfileTemplate | undefined;
  setDeviceProfileTemplate(value?: DeviceProfileTemplate): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasUpdatedAt(): boolean;
  clearUpdatedAt(): void;
  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDeviceProfileTemplateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDeviceProfileTemplateResponse): GetDeviceProfileTemplateResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
  hasDeviceProfileTemplate(): boolean;
  clearDeviceProfileTemplate(): void;
  getDeviceProfileTemplate(): DeviceProfileTemplate | undefined;
  setDeviceProfileTemplate(value?: DeviceProfileTemplate): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateDeviceProfileTemplateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateDeviceProfileTemplateRequest): UpdateDeviceProfileTemplateRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteDeviceProfileTemplateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteDeviceProfileTemplateRequest): DeleteDeviceProfileTemplateRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfileTemplatesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfileTemplatesRequest): ListDeviceProfileTemplatesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<DeviceProfileTemplateListItem>;
  setResultList(value: Array<DeviceProfileTemplateListItem>): void;
  addResult(value?: DeviceProfileTemplateListItem, index?: number): DeviceProfileTemplateListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfileTemplatesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfileTemplatesResponse): ListDeviceProfileTemplatesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
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

