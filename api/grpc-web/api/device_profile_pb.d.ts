import * as jspb from 'google-protobuf'

import * as google_api_annotations_pb from '../google/api/annotations_pb';
import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';
import * as google_protobuf_empty_pb from 'google-protobuf/google/protobuf/empty_pb';
import * as common_common_pb from '../common/common_pb';


export class DeviceProfile extends jspb.Message {
  getId(): string;
  setId(value: string): DeviceProfile;

  getTenantId(): string;
  setTenantId(value: string): DeviceProfile;

  getName(): string;
  setName(value: string): DeviceProfile;

  getDescription(): string;
  setDescription(value: string): DeviceProfile;

  getRegion(): common_common_pb.Region;
  setRegion(value: common_common_pb.Region): DeviceProfile;

  getMacVersion(): common_common_pb.MacVersion;
  setMacVersion(value: common_common_pb.MacVersion): DeviceProfile;

  getRegParamsRevision(): common_common_pb.RegParamsRevision;
  setRegParamsRevision(value: common_common_pb.RegParamsRevision): DeviceProfile;

  getAdrAlgorithmId(): string;
  setAdrAlgorithmId(value: string): DeviceProfile;

  getPayloadCodecRuntime(): CodecRuntime;
  setPayloadCodecRuntime(value: CodecRuntime): DeviceProfile;

  getPayloadCodecScript(): string;
  setPayloadCodecScript(value: string): DeviceProfile;

  getFlushQueueOnActivate(): boolean;
  setFlushQueueOnActivate(value: boolean): DeviceProfile;

  getUplinkInterval(): number;
  setUplinkInterval(value: number): DeviceProfile;

  getDeviceStatusReqInterval(): number;
  setDeviceStatusReqInterval(value: number): DeviceProfile;

  getSupportsOtaa(): boolean;
  setSupportsOtaa(value: boolean): DeviceProfile;

  getSupportsClassB(): boolean;
  setSupportsClassB(value: boolean): DeviceProfile;

  getSupportsClassC(): boolean;
  setSupportsClassC(value: boolean): DeviceProfile;

  getClassBTimeout(): number;
  setClassBTimeout(value: number): DeviceProfile;

  getClassBPingSlotNbK(): number;
  setClassBPingSlotNbK(value: number): DeviceProfile;

  getClassBPingSlotDr(): number;
  setClassBPingSlotDr(value: number): DeviceProfile;

  getClassBPingSlotFreq(): number;
  setClassBPingSlotFreq(value: number): DeviceProfile;

  getClassCTimeout(): number;
  setClassCTimeout(value: number): DeviceProfile;

  getAbpRx1Delay(): number;
  setAbpRx1Delay(value: number): DeviceProfile;

  getAbpRx1DrOffset(): number;
  setAbpRx1DrOffset(value: number): DeviceProfile;

  getAbpRx2Dr(): number;
  setAbpRx2Dr(value: number): DeviceProfile;

  getAbpRx2Freq(): number;
  setAbpRx2Freq(value: number): DeviceProfile;

  getTagsMap(): jspb.Map<string, string>;
  clearTagsMap(): DeviceProfile;

  getMeasurementsMap(): jspb.Map<string, Measurement>;
  clearMeasurementsMap(): DeviceProfile;

  getAutoDetectMeasurements(): boolean;
  setAutoDetectMeasurements(value: boolean): DeviceProfile;

  getRegionConfigId(): string;
  setRegionConfigId(value: string): DeviceProfile;

  getIsRelay(): boolean;
  setIsRelay(value: boolean): DeviceProfile;

  getIsRelayEd(): boolean;
  setIsRelayEd(value: boolean): DeviceProfile;

  getRelayEdRelayOnly(): boolean;
  setRelayEdRelayOnly(value: boolean): DeviceProfile;

  getRelayEnabled(): boolean;
  setRelayEnabled(value: boolean): DeviceProfile;

  getRelayCadPeriodicity(): CadPeriodicity;
  setRelayCadPeriodicity(value: CadPeriodicity): DeviceProfile;

  getRelayDefaultChannelIndex(): number;
  setRelayDefaultChannelIndex(value: number): DeviceProfile;

  getRelaySecondChannelFreq(): number;
  setRelaySecondChannelFreq(value: number): DeviceProfile;

  getRelaySecondChannelDr(): number;
  setRelaySecondChannelDr(value: number): DeviceProfile;

  getRelaySecondChannelAckOffset(): SecondChAckOffset;
  setRelaySecondChannelAckOffset(value: SecondChAckOffset): DeviceProfile;

  getRelayEdActivationMode(): RelayModeActivation;
  setRelayEdActivationMode(value: RelayModeActivation): DeviceProfile;

  getRelayEdSmartEnableLevel(): number;
  setRelayEdSmartEnableLevel(value: number): DeviceProfile;

  getRelayEdBackOff(): number;
  setRelayEdBackOff(value: number): DeviceProfile;

  getRelayEdUplinkLimitBucketSize(): number;
  setRelayEdUplinkLimitBucketSize(value: number): DeviceProfile;

  getRelayEdUplinkLimitReloadRate(): number;
  setRelayEdUplinkLimitReloadRate(value: number): DeviceProfile;

  getRelayJoinReqLimitReloadRate(): number;
  setRelayJoinReqLimitReloadRate(value: number): DeviceProfile;

  getRelayNotifyLimitReloadRate(): number;
  setRelayNotifyLimitReloadRate(value: number): DeviceProfile;

  getRelayGlobalUplinkLimitReloadRate(): number;
  setRelayGlobalUplinkLimitReloadRate(value: number): DeviceProfile;

  getRelayOverallLimitReloadRate(): number;
  setRelayOverallLimitReloadRate(value: number): DeviceProfile;

  getRelayJoinReqLimitBucketSize(): number;
  setRelayJoinReqLimitBucketSize(value: number): DeviceProfile;

  getRelayNotifyLimitBucketSize(): number;
  setRelayNotifyLimitBucketSize(value: number): DeviceProfile;

  getRelayGlobalUplinkLimitBucketSize(): number;
  setRelayGlobalUplinkLimitBucketSize(value: number): DeviceProfile;

  getRelayOverallLimitBucketSize(): number;
  setRelayOverallLimitBucketSize(value: number): DeviceProfile;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeviceProfile.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceProfile): DeviceProfile.AsObject;
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
    region: common_common_pb.Region,
    macVersion: common_common_pb.MacVersion,
    regParamsRevision: common_common_pb.RegParamsRevision,
    adrAlgorithmId: string,
    payloadCodecRuntime: CodecRuntime,
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
    relayCadPeriodicity: CadPeriodicity,
    relayDefaultChannelIndex: number,
    relaySecondChannelFreq: number,
    relaySecondChannelDr: number,
    relaySecondChannelAckOffset: SecondChAckOffset,
    relayEdActivationMode: RelayModeActivation,
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
  setName(value: string): Measurement;

  getKind(): MeasurementKind;
  setKind(value: MeasurementKind): Measurement;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Measurement.AsObject;
  static toObject(includeInstance: boolean, msg: Measurement): Measurement.AsObject;
  static serializeBinaryToWriter(message: Measurement, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Measurement;
  static deserializeBinaryFromReader(message: Measurement, reader: jspb.BinaryReader): Measurement;
}

export namespace Measurement {
  export type AsObject = {
    name: string,
    kind: MeasurementKind,
  }
}

export class DeviceProfileListItem extends jspb.Message {
  getId(): string;
  setId(value: string): DeviceProfileListItem;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): DeviceProfileListItem;
  hasCreatedAt(): boolean;
  clearCreatedAt(): DeviceProfileListItem;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): DeviceProfileListItem;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): DeviceProfileListItem;

  getName(): string;
  setName(value: string): DeviceProfileListItem;

  getRegion(): common_common_pb.Region;
  setRegion(value: common_common_pb.Region): DeviceProfileListItem;

  getMacVersion(): common_common_pb.MacVersion;
  setMacVersion(value: common_common_pb.MacVersion): DeviceProfileListItem;

  getRegParamsRevision(): common_common_pb.RegParamsRevision;
  setRegParamsRevision(value: common_common_pb.RegParamsRevision): DeviceProfileListItem;

  getSupportsOtaa(): boolean;
  setSupportsOtaa(value: boolean): DeviceProfileListItem;

  getSupportsClassB(): boolean;
  setSupportsClassB(value: boolean): DeviceProfileListItem;

  getSupportsClassC(): boolean;
  setSupportsClassC(value: boolean): DeviceProfileListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeviceProfileListItem.AsObject;
  static toObject(includeInstance: boolean, msg: DeviceProfileListItem): DeviceProfileListItem.AsObject;
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
    region: common_common_pb.Region,
    macVersion: common_common_pb.MacVersion,
    regParamsRevision: common_common_pb.RegParamsRevision,
    supportsOtaa: boolean,
    supportsClassB: boolean,
    supportsClassC: boolean,
  }
}

export class CreateDeviceProfileRequest extends jspb.Message {
  getDeviceProfile(): DeviceProfile | undefined;
  setDeviceProfile(value?: DeviceProfile): CreateDeviceProfileRequest;
  hasDeviceProfile(): boolean;
  clearDeviceProfile(): CreateDeviceProfileRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateDeviceProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateDeviceProfileRequest): CreateDeviceProfileRequest.AsObject;
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
  setId(value: string): CreateDeviceProfileResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateDeviceProfileResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateDeviceProfileResponse): CreateDeviceProfileResponse.AsObject;
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
  setId(value: string): GetDeviceProfileRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDeviceProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetDeviceProfileRequest): GetDeviceProfileRequest.AsObject;
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
  getDeviceProfile(): DeviceProfile | undefined;
  setDeviceProfile(value?: DeviceProfile): GetDeviceProfileResponse;
  hasDeviceProfile(): boolean;
  clearDeviceProfile(): GetDeviceProfileResponse;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetDeviceProfileResponse;
  hasCreatedAt(): boolean;
  clearCreatedAt(): GetDeviceProfileResponse;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): GetDeviceProfileResponse;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): GetDeviceProfileResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetDeviceProfileResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetDeviceProfileResponse): GetDeviceProfileResponse.AsObject;
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
  getDeviceProfile(): DeviceProfile | undefined;
  setDeviceProfile(value?: DeviceProfile): UpdateDeviceProfileRequest;
  hasDeviceProfile(): boolean;
  clearDeviceProfile(): UpdateDeviceProfileRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateDeviceProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateDeviceProfileRequest): UpdateDeviceProfileRequest.AsObject;
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
  setId(value: string): DeleteDeviceProfileRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteDeviceProfileRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteDeviceProfileRequest): DeleteDeviceProfileRequest.AsObject;
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
  setLimit(value: number): ListDeviceProfilesRequest;

  getOffset(): number;
  setOffset(value: number): ListDeviceProfilesRequest;

  getSearch(): string;
  setSearch(value: string): ListDeviceProfilesRequest;

  getTenantId(): string;
  setTenantId(value: string): ListDeviceProfilesRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfilesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfilesRequest): ListDeviceProfilesRequest.AsObject;
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
  setTotalCount(value: number): ListDeviceProfilesResponse;

  getResultList(): Array<DeviceProfileListItem>;
  setResultList(value: Array<DeviceProfileListItem>): ListDeviceProfilesResponse;
  clearResultList(): ListDeviceProfilesResponse;
  addResult(value?: DeviceProfileListItem, index?: number): DeviceProfileListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfilesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfilesResponse): ListDeviceProfilesResponse.AsObject;
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
  setTotalCount(value: number): ListDeviceProfileAdrAlgorithmsResponse;

  getResultList(): Array<AdrAlgorithmListItem>;
  setResultList(value: Array<AdrAlgorithmListItem>): ListDeviceProfileAdrAlgorithmsResponse;
  clearResultList(): ListDeviceProfileAdrAlgorithmsResponse;
  addResult(value?: AdrAlgorithmListItem, index?: number): AdrAlgorithmListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeviceProfileAdrAlgorithmsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeviceProfileAdrAlgorithmsResponse): ListDeviceProfileAdrAlgorithmsResponse.AsObject;
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
  setId(value: string): AdrAlgorithmListItem;

  getName(): string;
  setName(value: string): AdrAlgorithmListItem;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AdrAlgorithmListItem.AsObject;
  static toObject(includeInstance: boolean, msg: AdrAlgorithmListItem): AdrAlgorithmListItem.AsObject;
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

export enum CodecRuntime { 
  NONE = 0,
  CAYENNE_LPP = 1,
  JS = 2,
}
export enum MeasurementKind { 
  UNKNOWN = 0,
  COUNTER = 1,
  ABSOLUTE = 2,
  GAUGE = 3,
  STRING = 4,
}
export enum CadPeriodicity { 
  SEC_1 = 0,
  MS_500 = 1,
  MS_250 = 2,
  MS_100 = 3,
  MS_50 = 4,
  MS_20 = 5,
}
export enum SecondChAckOffset { 
  KHZ_0 = 0,
  KHZ_200 = 1,
  KHZ_400 = 2,
  KHZ_800 = 3,
  KHZ_1600 = 4,
  KHZ_3200 = 5,
}
export enum RelayModeActivation { 
  DISABLE_RELAY_MODE = 0,
  ENABLE_RELAY_MODE = 1,
  DYNAMIC = 2,
  END_DEVICE_CONTROLLED = 3,
}
