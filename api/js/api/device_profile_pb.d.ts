// package: api
// file: api/device_profile.proto

import * as jspb from "google-protobuf";
import * as google_api_annotations_pb from "../google/api/annotations_pb";
import * as google_protobuf_timestamp_pb from "google-protobuf/google/protobuf/timestamp_pb";
import * as google_protobuf_empty_pb from "google-protobuf/google/protobuf/empty_pb";
import * as common_common_pb from "../common/common_pb";

export class DeviceProfile extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getDescription(): string;
  setDescription(value: string): void;

  getRegion(): common_common_pb.RegionMap[keyof common_common_pb.RegionMap];
  setRegion(value: common_common_pb.RegionMap[keyof common_common_pb.RegionMap]): void;

  getMacVersion(): common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap];
  setMacVersion(value: common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap]): void;

  getRegParamsRevision(): common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap];
  setRegParamsRevision(value: common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap]): void;

  getAdrAlgorithmId(): string;
  setAdrAlgorithmId(value: string): void;

  getPayloadCodecRuntime(): CodecRuntimeMap[keyof CodecRuntimeMap];
  setPayloadCodecRuntime(value: CodecRuntimeMap[keyof CodecRuntimeMap]): void;

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
  getMeasurementsMap(): jspb.Map<string, Measurement>;
  clearMeasurementsMap(): void;
  getAutoDetectMeasurements(): boolean;
  setAutoDetectMeasurements(value: boolean): void;

  getRegionConfigId(): string;
  setRegionConfigId(value: string): void;

  getIsRelay(): boolean;
  setIsRelay(value: boolean): void;

  getIsRelayEd(): boolean;
  setIsRelayEd(value: boolean): void;

  getRelayEdRelayOnly(): boolean;
  setRelayEdRelayOnly(value: boolean): void;

  getRelayEnabled(): boolean;
  setRelayEnabled(value: boolean): void;

  getRelayCadPeriodicity(): CadPeriodicityMap[keyof CadPeriodicityMap];
  setRelayCadPeriodicity(value: CadPeriodicityMap[keyof CadPeriodicityMap]): void;

  getRelayDefaultChannelIndex(): number;
  setRelayDefaultChannelIndex(value: number): void;

  getRelaySecondChannelFreq(): number;
  setRelaySecondChannelFreq(value: number): void;

  getRelaySecondChannelDr(): number;
  setRelaySecondChannelDr(value: number): void;

  getRelaySecondChannelAckOffset(): SecondChAckOffsetMap[keyof SecondChAckOffsetMap];
  setRelaySecondChannelAckOffset(value: SecondChAckOffsetMap[keyof SecondChAckOffsetMap]): void;

  getRelayEdActivationMode(): RelayModeActivationMap[keyof RelayModeActivationMap];
  setRelayEdActivationMode(value: RelayModeActivationMap[keyof RelayModeActivationMap]): void;

  getRelayEdSmartEnableLevel(): number;
  setRelayEdSmartEnableLevel(value: number): void;

  getRelayEdBackOff(): number;
  setRelayEdBackOff(value: number): void;

  getRelayEdUplinkLimitBucketSize(): number;
  setRelayEdUplinkLimitBucketSize(value: number): void;

  getRelayEdUplinkLimitReloadRate(): number;
  setRelayEdUplinkLimitReloadRate(value: number): void;

  getRelayJoinReqLimitReloadRate(): number;
  setRelayJoinReqLimitReloadRate(value: number): void;

  getRelayNotifyLimitReloadRate(): number;
  setRelayNotifyLimitReloadRate(value: number): void;

  getRelayGlobalUplinkLimitReloadRate(): number;
  setRelayGlobalUplinkLimitReloadRate(value: number): void;

  getRelayOverallLimitReloadRate(): number;
  setRelayOverallLimitReloadRate(value: number): void;

  getRelayJoinReqLimitBucketSize(): number;
  setRelayJoinReqLimitBucketSize(value: number): void;

  getRelayNotifyLimitBucketSize(): number;
  setRelayNotifyLimitBucketSize(value: number): void;

  getRelayGlobalUplinkLimitBucketSize(): number;
  setRelayGlobalUplinkLimitBucketSize(value: number): void;

  getRelayOverallLimitBucketSize(): number;
  setRelayOverallLimitBucketSize(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeviceProfile.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceProfile): DeviceProfile.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeviceProfile, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeviceProfile;
  static deserializeBinaryFromReader(message: DeviceProfile, reader: jspb.BinaryReader): DeviceProfile;
}

export namespace DeviceProfile {
  export type AsObject = {
    id: string,
    tenantId: string,
    name: string,
    description: string,
    region: common_common_pb.RegionMap[keyof common_common_pb.RegionMap],
    macVersion: common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap],
    regParamsRevision: common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap],
    adrAlgorithmId: string,
    payloadCodecRuntime: CodecRuntimeMap[keyof CodecRuntimeMap],
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
    measurementsMap: Array<[string, Measurement.AsObject]>,
    autoDetectMeasurements: boolean,
    regionConfigId: string,
    isRelay: boolean,
    isRelayEd: boolean,
    relayEdRelayOnly: boolean,
    relayEnabled: boolean,
    relayCadPeriodicity: CadPeriodicityMap[keyof CadPeriodicityMap],
    relayDefaultChannelIndex: number,
    relaySecondChannelFreq: number,
    relaySecondChannelDr: number,
    relaySecondChannelAckOffset: SecondChAckOffsetMap[keyof SecondChAckOffsetMap],
    relayEdActivationMode: RelayModeActivationMap[keyof RelayModeActivationMap],
    relayEdSmartEnableLevel: number,
    relayEdBackOff: number,
    relayEdUplinkLimitBucketSize: number,
    relayEdUplinkLimitReloadRate: number,
    relayJoinReqLimitReloadRate: number,
    relayNotifyLimitReloadRate: number,
    relayGlobalUplinkLimitReloadRate: number,
    relayOverallLimitReloadRate: number,
    relayJoinReqLimitBucketSize: number,
    relayNotifyLimitBucketSize: number,
    relayGlobalUplinkLimitBucketSize: number,
    relayOverallLimitBucketSize: number,
  }
}

export class Measurement extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getKind(): MeasurementKindMap[keyof MeasurementKindMap];
  setKind(value: MeasurementKindMap[keyof MeasurementKindMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Measurement.AsObject;
  static toObject(includeInstance: boolean, msg: Measurement): Measurement.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Measurement, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Measurement;
  static deserializeBinaryFromReader(message: Measurement, reader: jspb.BinaryReader): Measurement;
}

export namespace Measurement {
  export type AsObject = {
    name: string,
    kind: MeasurementKindMap[keyof MeasurementKindMap],
  }
}

export class DeviceProfileListItem extends jspb.Message {
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
  toObject(includeInstance?: boolean): DeviceProfileListItem.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceProfileListItem): DeviceProfileListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeviceProfileListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeviceProfileListItem;
  static deserializeBinaryFromReader(message: DeviceProfileListItem, reader: jspb.BinaryReader): DeviceProfileListItem;
}

export namespace DeviceProfileListItem {
  export type AsObject = {
    id: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    name: string,
    region: common_common_pb.RegionMap[keyof common_common_pb.RegionMap],
    macVersion: common_common_pb.MacVersionMap[keyof common_common_pb.MacVersionMap],
    regParamsRevision: common_common_pb.RegParamsRevisionMap[keyof common_common_pb.RegParamsRevisionMap],
    supportsOtaa: boolean,
    supportsClassB: boolean,
    supportsClassC: boolean,
  }
}

export class CreateDeviceProfileRequest extends jspb.Message {
  hasDeviceProfile(): boolean;
  clearDeviceProfile(): void;
  getDeviceProfile(): DeviceProfile | undefined;
  setDeviceProfile(value?: DeviceProfile): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateDeviceProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateDeviceProfileRequest): CreateDeviceProfileRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateDeviceProfileRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateDeviceProfileRequest;
  static deserializeBinaryFromReader(message: CreateDeviceProfileRequest, reader: jspb.BinaryReader): CreateDeviceProfileRequest;
}

export namespace CreateDeviceProfileRequest {
  export type AsObject = {
    deviceProfile?: DeviceProfile.AsObject,
  }
}

export class CreateDeviceProfileResponse extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateDeviceProfileResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateDeviceProfileResponse): CreateDeviceProfileResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateDeviceProfileResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateDeviceProfileResponse;
  static deserializeBinaryFromReader(message: CreateDeviceProfileResponse, reader: jspb.BinaryReader): CreateDeviceProfileResponse;
}

export namespace CreateDeviceProfileResponse {
  export type AsObject = {
    id: string,
  }
}

export class GetDeviceProfileRequest extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDeviceProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDeviceProfileRequest): GetDeviceProfileRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDeviceProfileRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDeviceProfileRequest;
  static deserializeBinaryFromReader(message: GetDeviceProfileRequest, reader: jspb.BinaryReader): GetDeviceProfileRequest;
}

export namespace GetDeviceProfileRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetDeviceProfileResponse extends jspb.Message {
  hasDeviceProfile(): boolean;
  clearDeviceProfile(): void;
  getDeviceProfile(): DeviceProfile | undefined;
  setDeviceProfile(value?: DeviceProfile): void;

  hasCreatedAt(): boolean;
  clearCreatedAt(): void;
  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  hasUpdatedAt(): boolean;
  clearUpdatedAt(): void;
  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDeviceProfileResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDeviceProfileResponse): GetDeviceProfileResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetDeviceProfileResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetDeviceProfileResponse;
  static deserializeBinaryFromReader(message: GetDeviceProfileResponse, reader: jspb.BinaryReader): GetDeviceProfileResponse;
}

export namespace GetDeviceProfileResponse {
  export type AsObject = {
    deviceProfile?: DeviceProfile.AsObject,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class UpdateDeviceProfileRequest extends jspb.Message {
  hasDeviceProfile(): boolean;
  clearDeviceProfile(): void;
  getDeviceProfile(): DeviceProfile | undefined;
  setDeviceProfile(value?: DeviceProfile): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateDeviceProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateDeviceProfileRequest): UpdateDeviceProfileRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UpdateDeviceProfileRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateDeviceProfileRequest;
  static deserializeBinaryFromReader(message: UpdateDeviceProfileRequest, reader: jspb.BinaryReader): UpdateDeviceProfileRequest;
}

export namespace UpdateDeviceProfileRequest {
  export type AsObject = {
    deviceProfile?: DeviceProfile.AsObject,
  }
}

export class DeleteDeviceProfileRequest extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteDeviceProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteDeviceProfileRequest): DeleteDeviceProfileRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteDeviceProfileRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteDeviceProfileRequest;
  static deserializeBinaryFromReader(message: DeleteDeviceProfileRequest, reader: jspb.BinaryReader): DeleteDeviceProfileRequest;
}

export namespace DeleteDeviceProfileRequest {
  export type AsObject = {
    id: string,
  }
}

export class ListDeviceProfilesRequest extends jspb.Message {
  getLimit(): number;
  setLimit(value: number): void;

  getOffset(): number;
  setOffset(value: number): void;

  getSearch(): string;
  setSearch(value: string): void;

  getTenantId(): string;
  setTenantId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfilesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfilesRequest): ListDeviceProfilesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListDeviceProfilesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListDeviceProfilesRequest;
  static deserializeBinaryFromReader(message: ListDeviceProfilesRequest, reader: jspb.BinaryReader): ListDeviceProfilesRequest;
}

export namespace ListDeviceProfilesRequest {
  export type AsObject = {
    limit: number,
    offset: number,
    search: string,
    tenantId: string,
  }
}

export class ListDeviceProfilesResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<DeviceProfileListItem>;
  setResultList(value: Array<DeviceProfileListItem>): void;
  addResult(value?: DeviceProfileListItem, index?: number): DeviceProfileListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfilesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfilesResponse): ListDeviceProfilesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListDeviceProfilesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListDeviceProfilesResponse;
  static deserializeBinaryFromReader(message: ListDeviceProfilesResponse, reader: jspb.BinaryReader): ListDeviceProfilesResponse;
}

export namespace ListDeviceProfilesResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<DeviceProfileListItem.AsObject>,
  }
}

export class ListDeviceProfileAdrAlgorithmsResponse extends jspb.Message {
  getTotalCount(): number;
  setTotalCount(value: number): void;

  clearResultList(): void;
  getResultList(): Array<AdrAlgorithmListItem>;
  setResultList(value: Array<AdrAlgorithmListItem>): void;
  addResult(value?: AdrAlgorithmListItem, index?: number): AdrAlgorithmListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfileAdrAlgorithmsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfileAdrAlgorithmsResponse): ListDeviceProfileAdrAlgorithmsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListDeviceProfileAdrAlgorithmsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListDeviceProfileAdrAlgorithmsResponse;
  static deserializeBinaryFromReader(message: ListDeviceProfileAdrAlgorithmsResponse, reader: jspb.BinaryReader): ListDeviceProfileAdrAlgorithmsResponse;
}

export namespace ListDeviceProfileAdrAlgorithmsResponse {
  export type AsObject = {
    totalCount: number,
    resultList: Array<AdrAlgorithmListItem.AsObject>,
  }
}

export class AdrAlgorithmListItem extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getName(): string;
  setName(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AdrAlgorithmListItem.AsObject;
  static toObject(includeInstance: boolean, msg: AdrAlgorithmListItem): AdrAlgorithmListItem.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AdrAlgorithmListItem, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AdrAlgorithmListItem;
  static deserializeBinaryFromReader(message: AdrAlgorithmListItem, reader: jspb.BinaryReader): AdrAlgorithmListItem;
}

export namespace AdrAlgorithmListItem {
  export type AsObject = {
    id: string,
    name: string,
  }
}

export interface CodecRuntimeMap {
  NONE: 0;
  CAYENNE_LPP: 1;
  JS: 2;
}

export const CodecRuntime: CodecRuntimeMap;

export interface MeasurementKindMap {
  UNKNOWN: 0;
  COUNTER: 1;
  ABSOLUTE: 2;
  GAUGE: 3;
  STRING: 4;
}

export const MeasurementKind: MeasurementKindMap;

export interface CadPeriodicityMap {
  SEC_1: 0;
  MS_500: 1;
  MS_250: 2;
  MS_100: 3;
  MS_50: 4;
  MS_20: 5;
}

export const CadPeriodicity: CadPeriodicityMap;

export interface SecondChAckOffsetMap {
  KHZ_0: 0;
  KHZ_200: 1;
  KHZ_400: 2;
  KHZ_800: 3;
  KHZ_1600: 4;
  KHZ_3200: 5;
}

export const SecondChAckOffset: SecondChAckOffsetMap;

export interface RelayModeActivationMap {
  DISABLE_RELAY_MODE: 0;
  ENABLE_RELAY_MODE: 1;
  DYNAMIC: 2;
  END_DEVICE_CONTROLLED: 3;
}

export const RelayModeActivation: RelayModeActivationMap;

